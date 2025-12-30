import { untrack } from 'svelte';
import { clamp } from '$helpers';
import type { Asset } from '$types';
import { Bytes, NUM_BYTES } from './shared';

type ViewerOptions = {
	id: string;
	websocketUrl: string;
	asset: Asset;
};

export default class Viewer {
	#id: string;
	#asset: Asset;

	#mouseDown = $state(false);
	#isDragging = $state(false);
	#start = $state({ x: 0, y: 0 });
	#offset = $state({ x: 0, y: 0 });

	#minScale = 1;
	#maxScale = 1000;
	#scale = $state(2);
	#scaleFactor = 1;

	#sharedBuf = new SharedArrayBuffer(Int32Array.BYTES_PER_ELEMENT * NUM_BYTES);
	#shared = new Int32Array(this.#sharedBuf);
	#canvas!: HTMLCanvasElement;
	#worker: Worker | undefined;

	constructor({ id, websocketUrl, asset }: ViewerOptions) {
		this.#id = id;
		this.#asset = asset;

		this.onmousedown = this.onmousedown.bind(this);
		this.onmousemove = this.onmousemove.bind(this);
		this.onwheel = this.onwheel.bind(this);
		this.onmouseup = this.onmouseup.bind(this);
		this.onresize = this.onresize.bind(this);

		$effect.root(() => {
			$effect(() => {
				untrack(() => {
					const canvas = document.getElementById(this.#id) as HTMLCanvasElement | undefined;
					if (!canvas) throw Error('Canvas element not found');

					this.#canvas = canvas;
					this.resetScale();

					const offscreen = this.#canvas.transferControlToOffscreen();
					this.#worker = new Worker(new URL('./worker.ts', import.meta.url), { type: 'module' });

					this.#worker.postMessage(
						{
							type: 'init',
							data: {
								canvas: offscreen,
								sharedBuf: this.#sharedBuf,
								width: window.innerWidth * window.devicePixelRatio,
								height: window.innerHeight * window.devicePixelRatio,
								wsUrl: websocketUrl,
								asset: JSON.stringify(this.#asset)
							}
						},
						[offscreen]
					);

					this.#worker.onmessage = (e) => {
						const { type } = e.data;
						switch (type) {
							case 'connected':
								this.markDirty();
								break;
						}
					};

					this.#canvas.addEventListener('mousedown', this.onmousedown);
					this.#canvas.addEventListener('touchstart', this.ontouchstart);
					this.#canvas.addEventListener('wheel', this.onwheel);

					window.addEventListener('resize', this.onresize);
					window.addEventListener('mousemove', this.onmousemove);
					window.addEventListener('touchmove', this.ontouchmove);
					window.addEventListener('mouseup', this.onmouseup);
					window.addEventListener('touchend', this.ontouchend);

					return () => {
						this.#canvas?.removeEventListener('mousedown', this.onmousedown);
						this.#canvas?.removeEventListener('touchstart', this.ontouchstart);
						this.#canvas?.removeEventListener('wheel', this.onwheel);

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
		if (!this.#canvas) return;

		this.#scale = 2;

		const canvasWidth = this.#canvas.width * window.devicePixelRatio;
		const canvasHeight = this.#canvas.height * window.devicePixelRatio;
		const imageWidth = this.#asset.metadata.width;
		const imageHeight = this.#asset.metadata.height;

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
		mouseX: number = this.#canvas.width / (2 * window.devicePixelRatio),
		mouseY: number = this.#canvas.height / (2 * window.devicePixelRatio)
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
		this.#shared[Bytes.Width] = Math.round(window.innerWidth * window.devicePixelRatio);
		// [2] canvas height
		this.#shared[Bytes.Height] = Math.round(window.innerHeight * window.devicePixelRatio);
		// [3] offset x
		this.#shared[Bytes.OffsetX] = Math.round(this.#offset.x);
		// [4] offset y
		this.#shared[Bytes.OffsetY] = Math.round(this.#offset.y);
		// [5] scale * 1e6
		const actualScale = this.#scale * this.#scaleFactor;
		this.#shared[Bytes.Scale] = Math.floor(actualScale * 1e6);

		// [0] dirty flag
		Atomics.store(this.#shared, Bytes.Dirty, 1);
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
