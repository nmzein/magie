import { untrack } from 'svelte';
import type { Image2DLayer } from './types.ts';
import { clamp } from '$helpers';

export default class Viewer {
	#metadata: Image2DLayer[];

	#mouseDown = $state(false);
	#isDragging = $state(false);
	#start = $state({ x: 0, y: 0 });
	#offset = $state({ x: 0, y: 0 });

	#minScale = 1;
	#maxScale = 200;
	#scale = $state(2);
	#scaleFactor = 1;
	#scaleBreakpoints: number[] = [];

	#minLevel = 0;
	#maxLevel: number;
	#currentLevel: number = $state(0);

	#sharedBuf = new SharedArrayBuffer(Int32Array.BYTES_PER_ELEMENT * 6);
	#sharedInts = new Int32Array(this.#sharedBuf);
	#canvasId: string;
	#canvas: HTMLCanvasElement | undefined = $state();
	#worker: Worker | undefined = $state();

	constructor({
		canvasId,
		websocketUrl,
		metadata
	}: {
		canvasId: string;
		websocketUrl: string;
		metadata: Image2DLayer[];
	}) {
		this.#canvasId = canvasId;
		this.#metadata = metadata;
		this.#maxLevel = metadata.length - 1;
		this.#currentLevel = metadata.length - 1;

		const lowestResolution = metadata[this.#maxLevel].width * metadata[this.#maxLevel].height;

		// Start at highest resolution (minLevel) and go till second lowest (maxLevel - 1).
		for (let i = this.#minLevel; i < this.#maxLevel; i++) {
			this.#scaleBreakpoints.push(
				Math.sqrt((metadata[i].width * metadata[i].height) / lowestResolution)
			);
		}

		this.onmousedown = this.onmousedown.bind(this);
		this.onmousemove = this.onmousemove.bind(this);
		this.onwheel = this.onwheel.bind(this);
		this.onmouseup = this.onmouseup.bind(this);
		this.onresize = this.onresize.bind(this);

		$effect.root(() => {
			$effect(() => {
				untrack(() => {
					this.#canvas = document.getElementById(this.#canvasId) as HTMLCanvasElement | undefined;

					if (!this.#canvas) throw Error('Canvas element not found');

					this.resetScale();

					const offscreen = this.#canvas.transferControlToOffscreen();
					this.#worker = new Worker(new URL('./worker.ts', import.meta.url), { type: 'module' });

					this.#worker.postMessage(
						{
							type: 'init',
							data: {
								canvas: offscreen,
								width: window.innerWidth * window.devicePixelRatio,
								height: window.innerHeight * window.devicePixelRatio,
								wsUrl: websocketUrl,
								layers: JSON.stringify(metadata),
								sharedBuf: this.#sharedBuf
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

					window.addEventListener('resize', this.onresize);
					window.addEventListener('mousemove', this.onmousemove);
					window.addEventListener('touchmove', this.ontouchmove);
					window.addEventListener('mouseup', this.onmouseup);
					window.addEventListener('touchend', this.ontouchend);
					window.addEventListener('wheel', this.onwheel);

					return () => {
						this.#canvas?.removeEventListener('mousedown', this.onmousedown);
						this.#canvas?.removeEventListener('touchstart', this.ontouchstart);

						window.removeEventListener('resize', this.onresize);
						window.removeEventListener('mousemove', this.onmousemove);
						window.removeEventListener('touchmove', this.ontouchmove);
						window.removeEventListener('onmouseup', this.onmouseup);
						window.removeEventListener('ontouchend', this.ontouchend);
						window.removeEventListener('wheel', this.onwheel);
						this.#worker?.postMessage({ type: 'close' });
						this.#worker?.terminate();
					};
				});
			});
		});
	}

	get canvasId() {
		return this.#canvasId;
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

	get maxLevel(): number {
		return this.#maxLevel;
	}

	resetScale() {
		if (!this.#canvas) return;

		this.#scale = 2;

		const { width, height } = this.#canvas.getBoundingClientRect();

		const canvasWidth = width * window.devicePixelRatio;
		const canvasHeight = height * window.devicePixelRatio;
		const imageWidth = this.#metadata[this.#minLevel].width;
		const imageHeight = this.#metadata[this.#minLevel].height;

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

	// FIXME: default to half canvas width/height
	zoom(
		delta: number,
		mouseX: number = screen.availWidth / 2,
		mouseY: number = screen.availHeight / 2,
		dpr: number = window.devicePixelRatio
	) {
		const prevScale = this.#scale;

		// Exponential zoom feels natural on wheel / pinch
		const zoomFactor = Math.exp(-0.005 * delta);
		const nextScale = clamp(prevScale * zoomFactor, this.#minScale, this.#maxScale);

		// Early-out if scale didn't change (avoids jitter & extra math)
		if (nextScale === prevScale) return;

		// Screen → canvas
		const canvasX = mouseX * dpr;
		const canvasY = mouseY * dpr;

		// Keep cursor anchored during zoom
		const scaleRatio = nextScale / prevScale;
		this.#offset.x = canvasX - (canvasX - this.#offset.x) * scaleRatio;
		this.#offset.y = canvasY - (canvasY - this.#offset.y) * scaleRatio;

		this.#scale = nextScale;
		this.markDirty();
	}

	markDirty() {
		if (!this.#sharedInts) return;

		// [0] canvas width
		this.#sharedInts[0] = Math.round(window.innerWidth * window.devicePixelRatio);
		// [1] canvas height
		this.#sharedInts[1] = Math.round(window.innerHeight * window.devicePixelRatio);
		// [2] #offset x
		this.#sharedInts[2] = Math.round(this.#offset.x);
		// [3] #offset y
		this.#sharedInts[3] = Math.round(this.#offset.y);
		// [4] scale * 1e6
		const actualScale = this.#scale * this.#scaleFactor;
		this.#sharedInts[4] = Math.floor(actualScale * 1e6);
		// [5] dirty flag
		Atomics.store(this.#sharedInts, 5, 1);
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

	// #handleLevelChange(delta: number) {
	// 	if (
	// 		this.#currentLevel === undefined ||
	// 		this.#maxLevel === undefined ||
	// 		this.#scaleBreakpoints === undefined
	// 	)
	// 		return;

	// 	// If at highest detail level and zooming in,
	// 	// or if at lowest detail level and zooming out, do nothing.
	// 	if (
	// 		(this.#currentLevel == this.#minLevel && delta < 0) ||
	// 		(this.#currentLevel == this.#maxLevel && delta > 0)
	// 	) {
	// 		console.log(
	// 			'At level',
	// 			this.#currentLevel,
	// 			'and zooming',
	// 			delta < 0 ? 'in' : 'out' + '. Skip computation.'
	// 		);
	// 		return;
	// 	}

	// 	// If zooming out (not at lowest detail)
	// 	// check current breakpoint (at #currentLevel)
	// 	// if scale < sB[cL] then cL += 1 (move to lower reso.)
	// 	// e.g. sB = [32, 8] and currently at level 1 and zooming out
	// 	// desired result: move to level 2 (cL + 1)
	// 	// should happen when: scale < 8 (sB[cl])
	// 	// result: cL += 1 (cL = 2)
	// 	if (delta > 0 && this.#scale < this.#scaleBreakpoints[this.#currentLevel]) {
	// 		this.#currentLevel += 1;
	// 		console.log('Switching to lower resolution level:', this.#currentLevel + '.');
	// 	}

	// 	// If zooming in (not at highest detail),
	// 	// check next breakpoint (at #currentLevel - 1)
	// 	// if scale > sB[cL - 1] then cL -= 1 (move to higher reso.)
	// 	if (delta < 0 && this.#scale > this.#scaleBreakpoints[this.#currentLevel - 1]) {
	// 		this.#currentLevel -= 1;
	// 		console.log('Switching to higher resolution level:', this.#currentLevel + '.');
	// 	}
	// }
}
