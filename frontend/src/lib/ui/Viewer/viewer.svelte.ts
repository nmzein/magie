import { untrack } from 'svelte';
import { clamp } from '$helpers';
import { DEFAULT_POINT, type AssetMetadata } from '$types';
import { Fields, NUM_FIELDS } from './shared';

type ViewerOptions = {
	id: string;
	primary: number;
	layers: AssetMetadata[];
};

export default class Viewer {
	#id: ViewerOptions['id'];
	#primary: ViewerOptions['primary'];
	#layers: ViewerOptions['layers'];

	#mouseDown = $state(false);
	#isDragging = $state(false);
	#start = $state(DEFAULT_POINT);
	#offset = $state(DEFAULT_POINT);

	#minScale = 1;
	#maxScale = 1000;
	#scale = $state(2);
	#scaleFactor = 1;

	#div!: HTMLDivElement;
	#canvases: HTMLCanvasElement[] = [];
	#offscreenCanvases: OffscreenCanvas[] = [];
	#sharedBuf = new SharedArrayBuffer(Int32Array.BYTES_PER_ELEMENT * NUM_FIELDS);
	#shared = new Int32Array(this.#sharedBuf);
	#worker: Worker | undefined;

	constructor({ id, primary, layers }: ViewerOptions) {
		if (primary < 0 || primary >= layers.length) {
			throw Error('Invalid primary layer index');
		}

		this.#id = id;
		this.#primary = primary;
		this.#layers = layers;

		this.onmousedown = this.onmousedown.bind(this);
		this.onmousemove = this.onmousemove.bind(this);
		this.onwheel = this.onwheel.bind(this);
		this.onmouseup = this.onmouseup.bind(this);
		this.onresize = this.onresize.bind(this);

		$effect.root(() => {
			$effect(() => {
				untrack(() => {
					const div = document.getElementById(this.#id) as HTMLDivElement | undefined;
					if (!div) throw Error('Div element not found');

					this.#div = div;

					const canvasDefs = [{ ctx: '2d' }, { ctx: 'webgl2' }];

					for (const index of canvasDefs.keys()) {
						const canvasEl = document.createElement('canvas');

						canvasEl.width = window.innerWidth * window.devicePixelRatio;
						canvasEl.height = window.innerHeight * window.devicePixelRatio;
						canvasEl.style.position = 'absolute';
						canvasEl.style.width = `100%`;
						canvasEl.style.height = `100%`;
						canvasEl.style.zIndex = (Number(div.style.zIndex) + index).toString();
						div.appendChild(canvasEl);

						this.#canvases.push(canvasEl);
						this.#offscreenCanvases.push(canvasEl.transferControlToOffscreen());
					}

					this.resetScale();

					this.#worker = new Worker(new URL('./worker.ts', import.meta.url), { type: 'module' });
					this.#worker.postMessage(
						{
							type: 'init',
							data: {
								canvases: this.#offscreenCanvases,
								canvasDefs: JSON.stringify(canvasDefs),
								sharedBuf: this.#sharedBuf,
								width: window.innerWidth * window.devicePixelRatio,
								height: window.innerHeight * window.devicePixelRatio,
								primary: this.#primary,
								layers: JSON.stringify(this.#layers.toReversed())
							}
						},
						this.#offscreenCanvases
					);

					this.#div.addEventListener('mousedown', this.onmousedown);
					this.#div.addEventListener('touchstart', this.ontouchstart);
					this.#div.addEventListener('wheel', this.onwheel);

					window.addEventListener('resize', this.onresize);
					window.addEventListener('mousemove', this.onmousemove);
					window.addEventListener('touchmove', this.ontouchmove);
					window.addEventListener('mouseup', this.onmouseup);
					window.addEventListener('touchend', this.ontouchend);

					return () => {
						this.#div?.removeEventListener('mousedown', this.onmousedown);
						this.#div?.removeEventListener('touchstart', this.ontouchstart);
						this.#div?.removeEventListener('wheel', this.onwheel);

						window.removeEventListener('resize', this.onresize);
						window.removeEventListener('mousemove', this.onmousemove);
						window.removeEventListener('touchmove', this.ontouchmove);
						window.removeEventListener('onmouseup', this.onmouseup);
						window.removeEventListener('ontouchend', this.ontouchend);

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

	get scale() {
		return this.#scale;
	}

	atMinScale(): boolean {
		return this.#scale === this.#minScale;
	}

	atMaxScale(): boolean {
		return this.#scale === this.#maxScale;
	}

	resetScale() {
		if (!this.#div) return;

		this.#scale = 2;

		const { width, height } = this.#div.getBoundingClientRect();

		const canvasWidth = width * window.devicePixelRatio;
		const canvasHeight = height * window.devicePixelRatio;

		const imageWidth = this.#layers[this.#primary].width;
		const imageHeight = this.#layers[this.#primary].height;

		// Fit to smallest dimension (ensures entire image is visible)
		const scaleX = canvasWidth / imageWidth;
		const scaleY = canvasHeight / imageHeight;
		this.#scaleFactor = Math.min(scaleX, scaleY) * 0.5;

		// Center the image in the canvas
		const scaledImageWidth = imageWidth * this.#scaleFactor * this.#scale;
		const scaledImageHeight = imageHeight * this.#scaleFactor * this.#scale;
		this.#offset.x = (canvasWidth - scaledImageWidth) / 2;
		this.#offset.y = (canvasHeight - scaledImageHeight) / 2;

		this.#start = { x: 0, y: 0 };
		this.markDirty();
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
		mouseX: number = this.#canvases[0].width / (2 * window.devicePixelRatio),
		mouseY: number = this.#canvases[0].height / (2 * window.devicePixelRatio)
	) {
		const prevScale = this.#scale;

		// Exponential zoom feels natural on wheel / pinch
		const zoomFactor = Math.exp(-0.005 * delta);
		const nextScale = clamp(prevScale * zoomFactor, this.#minScale, this.#maxScale);

		// Early-out if scale didn't change (avoids jitter & extra math)
		if (nextScale === prevScale) return;

		// Screen → canvas
		const canvasX = mouseX * window.devicePixelRatio;
		const canvasY = mouseY * window.devicePixelRatio;

		// Keep cursor anchored during zoom
		const scaleRatio = nextScale / prevScale;
		this.#offset.x = canvasX - (canvasX - this.#offset.x) * scaleRatio;
		this.#offset.y = canvasY - (canvasY - this.#offset.y) * scaleRatio;

		this.#scale = nextScale;
		this.markDirty();
	}

	markDirty() {
		// [1] canvas width
		this.#shared[Fields.Width] = Math.round(window.innerWidth * window.devicePixelRatio);
		// [2] canvas height
		this.#shared[Fields.Height] = Math.round(window.innerHeight * window.devicePixelRatio);
		// [3] offset x
		this.#shared[Fields.OffsetX] = Math.round(this.#offset.x);
		// [4] offset y
		this.#shared[Fields.OffsetY] = Math.round(this.#offset.y);
		// [5] scale * 1e6
		const actualScale = this.#scale * this.#scaleFactor;
		this.#shared[Fields.Scale] = Math.floor(actualScale * 1e6);

		// [0] dirty flag
		Atomics.store(this.#shared, Fields.Dirty, 1);
	}

	onmousedown(e: MouseEvent) {
		e.preventDefault();
		this.panStart(e.clientX, e.clientY);
	}

	ontouchstart(te: TouchEvent) {
		const e = te.touches[0];
		this.panStart(e.clientX, e.clientY);
	}

	onmousemove(e: MouseEvent) {
		e.preventDefault();
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

	onresize() {
		this.markDirty();
	}
}
