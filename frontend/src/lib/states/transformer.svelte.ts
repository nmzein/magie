import { image } from '$states';

/// Handles zoom and offset calculations.
export const Transformer = () => {
	const MIN_SCALE = 0.1;
	const MAX_SCALE = 100;
	const MIN_LEVEL = 0;
	let maxLevel: number | undefined = $state();
	let currentLevel: number | undefined = $state();

	let offsetX = $state(0);
	let offsetY = $state(0);
	let scale = $state(1);
	const scaleBreakpoints: number[] | undefined = $derived.by(() => {
		if (!image.initialised || image.metadata === undefined || maxLevel === undefined) return;

		const lowestResolution = image.metadata[maxLevel].width * image.metadata[maxLevel].height;
		let scaleBreakpoints = [];
		// Start at highest resolution (minLevel) and go till second lowest (maxLevel - 1).
		for (let i = MIN_LEVEL; i < maxLevel; i++) {
			scaleBreakpoints.push(
				Math.sqrt((image.metadata[i].width * image.metadata[i].height) / lowestResolution)
			);
		}

		return scaleBreakpoints;
	});

	function atMinScale() {
		return scale === MIN_SCALE;
	}

	function atMaxScale() {
		return scale === MAX_SCALE;
	}

	function resetScale() {
		offsetX = 0;
		offsetY = 0;
		scale = 1;
	}

	function zoom(
		delta: number,
		mouseX: number = screen.availWidth / 2,
		mouseY: number = screen.availHeight / 2
	) {
		let newScale = scale * Math.exp(delta * -0.005);

		// Limit the scale factor within a reasonable range.
		if (newScale < MIN_SCALE) {
			newScale = MIN_SCALE;
		} else if (newScale > MAX_SCALE) {
			newScale = MAX_SCALE;
		}

		const ratio = 1 - newScale / scale;

		offsetX += (mouseX - offsetX) * ratio;
		offsetY += (mouseY - offsetY) * ratio;

		scale = newScale;

		handleLevelChange(delta);
	}

	function handleLevelChange(delta: number) {
		if (currentLevel === undefined || maxLevel === undefined || scaleBreakpoints === undefined)
			return;

		// If at highest detail level and zooming in,
		// or if at lowest detail level and zooming out, do nothing.
		if ((currentLevel == MIN_LEVEL && delta < 0) || (currentLevel == maxLevel && delta > 0)) {
			console.log(
				'At level',
				currentLevel,
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
		if (delta > 0 && scale < scaleBreakpoints[currentLevel]) {
			currentLevel += 1;
			console.log('Switching to lower resolution level:', currentLevel + '.');
		}

		// If zooming in (not at highest detail),
		// check next breakpoint (at currentLevel - 1)
		// if scale > sB[cL - 1] then cL -= 1 (move to higher reso.)
		if (delta < 0 && scale > scaleBreakpoints[currentLevel - 1]) {
			currentLevel -= 1;
			console.log('Switching to higher resolution level:', currentLevel + '.');
		}
	}

	return {
		get scale() {
			return scale;
		},
		get offsetX() {
			return offsetX;
		},
		set offsetX(value: number) {
			offsetX = value;
		},
		get offsetY() {
			return offsetY;
		},
		set offsetY(value: number) {
			offsetY = value;
		},
		get currentLevel() {
			return currentLevel;
		},
		set currentLevel(value: number | undefined) {
			currentLevel = value;
		},
		set maxLevel(value: number | undefined) {
			maxLevel = value;
		},
		atMinScale,
		atMaxScale,
		resetScale,
		zoom
	};
};
