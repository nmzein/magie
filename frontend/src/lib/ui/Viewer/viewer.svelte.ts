import { untrack } from 'svelte';
import { on } from 'svelte/events';
import { clamp } from '$helpers';
import { type AssetMetadata, type Bounds, DEFAULT_BOUND, DEFAULT_POINT } from '$types';
import { Fields, NUM_FIELDS } from './shared';

type ViewerOptions = {
	id: string;
	layers: AssetMetadata[];
};

export default class Viewer {
	#id: ViewerOptions['id'];
	#layers: ViewerOptions['layers'];
	#primary: number;

	#mouseDown = $state(false);
	#isDragging = $state(false);
	#start = $state(DEFAULT_POINT);
	#offset = $state(DEFAULT_POINT);
	#bounds: Bounds = $state(DEFAULT_BOUND);

	#minScale = 0.5;
	#maxScale = 500;
	#scale = $state(1);
	#effectiveScale = $state(this.#scale);

	#div!: HTMLDivElement;
	#canvases: HTMLCanvasElement[] = [];
	#offscreenCanvases: OffscreenCanvas[] = [];
	#sharedBuf = new SharedArrayBuffer(Int32Array.BYTES_PER_ELEMENT * NUM_FIELDS);
	#shared = new Int32Array(this.#sharedBuf);
	#worker: Worker | undefined;

	constructor({ id, layers }: ViewerOptions) {
		this.#id = id;
		this.#primary = layers.findIndex((layer) => layer.primary);
		this.#layers = layers;

		this.onmousedown = this.onmousedown.bind(this);
		this.onmousemove = this.onmousemove.bind(this);
		this.onwheel = this.onwheel.bind(this);
		this.onmouseup = this.onmouseup.bind(this);

		$effect.root(() => {
			$effect(() => {
				untrack(() => {
					const div = document.getElementById(this.#id) as HTMLDivElement | undefined;
					if (!div) throw Error('Div element not found');

					this.#div = div;
					this.#setBounds(this.#div.getBoundingClientRect());

					const canvasDefs = [{ ctx: '2d' }, { ctx: 'webgl2' }];

					for (const index of canvasDefs.keys()) {
						const canvas = document.createElement('canvas');

						canvas.width = this.#bounds.width;
						canvas.height = this.#bounds.height;
						canvas.style.position = 'absolute';
						canvas.style.width = `100%`;
						canvas.style.height = `100%`;
						canvas.style.zIndex = (Number(div.style.zIndex) + index).toString();
						div.appendChild(canvas);

						this.#canvases.push(canvas);
						this.#offscreenCanvases.push(canvas.transferControlToOffscreen());
					}

					this.reset();

					this.#worker = new Worker(new URL('./worker.ts', import.meta.url), { type: 'module' });
					this.#worker.postMessage(
						{
							type: 'init',
							data: {
								canvases: this.#offscreenCanvases,
								canvasDefs: JSON.stringify(canvasDefs),
								sharedBuf: this.#sharedBuf,
								layers: JSON.stringify(this.#layers.toReversed())
							}
						},
						this.#offscreenCanvases
					);

					const removeListeners = [
						on(this.#div, 'mousedown', this.onmousedown),
						on(this.#div, 'touchstart', this.ontouchstart),
						on(this.#div, 'wheel', this.onwheel),
						on(window, 'mousemove', this.onmousemove),
						on(window, 'touchmove', this.ontouchmove),
						on(window, 'mouseup', this.onmouseup),
						on(window, 'touchend', this.ontouchend)
					];

					const resizeObserver = new ResizeObserver((entries) => {
						for (const entry of entries) {
							if (entry.target === this.#div) {
								this.#setBounds(entry.contentRect);
								this.markDirty();
							}
						}
					});

					resizeObserver.observe(this.#div);

					return () => {
						for (const removeListener of removeListeners) {
							removeListener();
						}

						resizeObserver.disconnect();

						this.#worker?.postMessage({ type: 'close' });
						this.#worker?.terminate();
					};
				});
			});
		});
	}

	get id() {
		return this.#id;
	}

	get isDragging() {
		return this.#isDragging;
	}

	get dims() {
		return this.#bounds;
	}

	get offset() {
		return this.#offset;
	}

	get effectiveScale() {
		return this.#effectiveScale;
	}

	get scale() {
		return this.#scale;
	}

	atMinScale(): boolean {
		return this.#scale === this.#minScale;
	}

	atMaxScale(): boolean {
		return this.#scale === this.#maxScale;
	}

	#setBounds(bounds: Bounds) {
		const dpr = window.devicePixelRatio;

		this.#bounds = {
			width: bounds.width * dpr,
			height: bounds.height * dpr,
			left: bounds.left * dpr,
			top: bounds.top * dpr
		};
	}

	panStart(x: number, y: number) {
		this.#mouseDown = true;
		this.#start = { x, y };
	}

	pan(x: number, y: number) {
		if (!this.#mouseDown) return;

		this.#isDragging = true;

		this.#offset.x += x - this.#start.x;
		this.#offset.y += y - this.#start.y;

		this.#start = { x, y };
		this.markDirty();
	}

	panStop() {
		this.#isDragging = false;
		this.#mouseDown = false;
	}

	zoom(
		delta: number,
		mouseX: number = this.#bounds.width / 2,
		mouseY: number = this.#bounds.height / 2
	) {
		const prevScale = this.#scale;

		const scaledDelta = Math.exp(-0.005 * delta);
		const nextScale = clamp(prevScale * scaledDelta, this.#minScale, this.#maxScale);

		if (nextScale === prevScale) return;

		const scaleRatio = nextScale / prevScale;

		// DOM -> canvas
		const canvasX = mouseX * window.devicePixelRatio;
		const canvasY = mouseY * window.devicePixelRatio;

		this.#offset.x = canvasX - (canvasX - this.#offset.x) * scaleRatio;
		this.#offset.y = canvasY - (canvasY - this.#offset.y) * scaleRatio;
		this.#scale = nextScale;

		const layer = this.#layers[this.#primary];
		const scaleFactor = Math.min(
			this.#bounds.width / layer.width,
			this.#bounds.height / layer.height
		);

		this.#effectiveScale = this.#scale * scaleFactor;

		this.markDirty();
	}

	reset() {
		this.#scale = 1;
		this.#start = { x: 0, y: 0 };

		const layer = this.#layers[this.#primary];
		const scaleFactor = Math.min(
			this.#bounds.width / layer.width,
			this.#bounds.height / layer.height
		);

		this.#effectiveScale = this.#scale * scaleFactor;

		this.#offset.x = (this.#bounds.width - layer.width * this.#effectiveScale) / 2;
		this.#offset.y = (this.#bounds.height - layer.height * this.#effectiveScale) / 2;

		this.markDirty();
	}

	markDirty() {
		this.#shared[Fields.Width] = Math.round(this.#bounds.width);
		this.#shared[Fields.Height] = Math.round(this.#bounds.height);
		this.#shared[Fields.OffsetX] = Math.round(this.#offset.x);
		this.#shared[Fields.OffsetY] = Math.round(this.#offset.y);
		this.#shared[Fields.Scale] = Math.round(this.#effectiveScale * 1e6);

		Atomics.store(this.#shared, Fields.Dirty, 1);
	}

	onmousedown(e: MouseEvent) {
		this.panStart(e.clientX, e.clientY);
	}

	ontouchstart(te: TouchEvent) {
		const e = te.touches[0];
		this.panStart(e.clientX, e.clientY);
	}

	onmousemove(e: MouseEvent) {
		this.pan(e.clientX, e.clientY);
	}

	ontouchmove(te: TouchEvent) {
		const e = te.touches[0];
		this.pan(e.clientX, e.clientY);
	}

	onmouseup() {
		this.panStop();
	}

	ontouchend() {
		this.panStop();
	}

	onwheel(e: WheelEvent) {
		this.zoom(e.deltaY, e.clientX, e.clientY);
	}
}
