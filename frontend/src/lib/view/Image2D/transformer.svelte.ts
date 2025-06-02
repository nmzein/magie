import type { Image2DLayer } from './types.ts';

/// Handles zoom and offset calculations.
export class Transformer {
	MIN_SCALE = 0.1;
	MAX_SCALE = 100;
	MIN_LEVEL = 0;
	maxLevel: number | undefined = $state();
	currentLevel: number | undefined = $state();
	isDragging = $state(false);
	panStartX = $state(0);
	panStartY = $state(0);
	offsetX = $state(0);
	offsetY = $state(0);
	#scale = $state(1);
	#scaleBreakpoints: number[] = [];
	atMinScale: boolean = $derived(this.#scale === this.MIN_SCALE);
	atMaxScale: boolean = $derived(this.#scale === this.MAX_SCALE);

	constructor(metadata: Image2DLayer[]) {
		// TODO: FIX
		for (let i = 0; i < metadata.length; i++) {
			if (metadata[i].cols <= 4 || metadata[i].rows <= 4) {
				this.maxLevel = i - 1;
				this.currentLevel = i - 1;
				break;
			}
		}

		// this.maxLevel = metadata.length - 1;
		// this.currentLevel = metadata.length - 1;

		const lowestResolution = metadata[this.maxLevel!].width * metadata[this.maxLevel!].height;

		// Start at highest resolution (MIN_LEVEL) and go till second lowest (maxLevel - 1).
		for (let i = this.MIN_LEVEL; i < this.maxLevel!; i++) {
			this.#scaleBreakpoints.push(
				Math.sqrt((metadata[i].width * metadata[i].height) / lowestResolution)
			);
		}
	}

	get scale() {
		return this.#scale;
	}

	resetScale() {
		this.offsetX = 0;
		this.offsetY = 0;
		this.#scale = 1;
	}

	panStart(x: number, y: number) {
		this.isDragging = true;
		this.panStartX = x;
		this.panStartY = y;
	}

	pan(x: number, y: number) {
		if (!this.isDragging) return;

		this.offsetX += x - this.panStartX;
		this.offsetY += y - this.panStartY;

		this.panStartX = x;
		this.panStartY = y;
	}

	panStop() {
		this.isDragging = false;
	}

	zoom(
		delta: number,
		mouseX: number = screen.availWidth / 2,
		mouseY: number = screen.availHeight / 2
	) {
		let newScale = this.#scale * Math.exp(delta * -0.005);

		// Limit the scale factor within a reasonable range.
		if (newScale < this.MIN_SCALE) {
			newScale = this.MIN_SCALE;
		} else if (newScale > this.MAX_SCALE) {
			newScale = this.MAX_SCALE;
		}

		const ratio = 1 - newScale / this.#scale;

		this.offsetX += (mouseX - this.offsetX) * ratio;
		this.offsetY += (mouseY - this.offsetY) * ratio;

		this.#scale = newScale;

		this.#handleLevelChange(delta);
	}

	#handleLevelChange(delta: number) {
		if (
			this.currentLevel === undefined ||
			this.maxLevel === undefined ||
			this.#scaleBreakpoints === undefined
		)
			return;

		// If at highest detail level and zooming in,
		// or if at lowest detail level and zooming out, do nothing.
		if (
			(this.currentLevel == this.MIN_LEVEL && delta < 0) ||
			(this.currentLevel == this.maxLevel && delta > 0)
		) {
			console.log(
				'At level',
				this.currentLevel,
				'and zooming',
				delta < 0 ? 'in' : 'out' + '. Skip computation.'
			);
			return;
		}

		// If zooming out (not at lowest detail)
		// check current breakpoint (at currentLevel)
		// if scale <>> sB[cL] then cL += 1 (move to lower reso.)
		// e.g. sB = [32, 8] and currently at level 1 and zooming out
		// desired result: move to level 2 (cL + 1)
		// should happen when: scale < 8 (sB[cl])
		// result: cL += 1 (cL = 2)
		if (delta > 0 && this.#scale < this.#scaleBreakpoints[this.currentLevel]) {
			this.currentLevel += 1;
			console.log('Switching to lower resolution level:', this.currentLevel + '.');
		}

		// If zooming in (not at highest detail),
		// check next breakpoint (at currentLevel - 1)
		// if scale > sB[cL - 1] then cL -= 1 (move to higher reso.)
		if (delta < 0 && this.#scale > this.#scaleBreakpoints[this.currentLevel - 1]) {
			this.currentLevel -= 1;
			console.log('Switching to higher resolution level:', this.currentLevel + '.');
		}
	}
}
