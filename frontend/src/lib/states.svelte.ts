import type {
	MetadataLayer,
	AnnotationLayer,
	ImageLayer,
	Image,
	Directory,
	UploaderSettings,
	TileRequest
} from '$types';
import { http, websocket } from '$api';

export function state<T>(initial: T): { value: T };
export function state<T = undefined>(initial?: T): { value: T };
export function state<T>(initial?: T) {
	let state = $state({ value: initial });
	return state;
}

export const registry = (() => {
	let value: Directory | undefined = $state();

	async function init() {
		let registry = await http.GetRegistry();
		if (registry === undefined) return;
		value = registry;
	}
	const reload = init;

	return {
		get value() {
			return value;
		},
		init,
		reload
	};
})();

export const generators = (() => {
	let value: string[] = $state([]);

	async function init() {
		let generators = await http.GetGenerators();
		if (generators === undefined) return;
		value = generators;
		uploader.settings.generator = value[0];
	}

	return {
		get value() {
			return value;
		},
		init
	};
})();

export const uploader = (() => {
	let parentDirectoryId: number | undefined = $state();
	let image: File | undefined = $state();
	let annotations: File | undefined = $state();
	let settings: UploaderSettings = {
		generator: '',
		annotations: 'none'
	};

	async function upload() {
		if (parentDirectoryId === undefined || image === undefined) return;

		if (settings.annotations === 'provide') {
			await http.SendUploadAssets(parentDirectoryId, image, annotations, settings);
		} else {
			await http.SendUploadAssets(parentDirectoryId, image, undefined, settings);
		}

		reset();
	}

	function reset() {
		parentDirectoryId = undefined;
		image = undefined;
		annotations = undefined;
	}

	return {
		get settings() {
			return settings;
		},
		set parentDirectoryId(value: number | undefined) {
			parentDirectoryId = value;
		},
		set image(value: File | undefined) {
			image = value;
		},
		set annotations(value: File | undefined) {
			annotations = value;
		},
		set settings(value: UploaderSettings) {
			settings = value;
		},
		set settings_generator(value: string) {
			settings.generator = value;
		},
		set settings_annotations(value: 'none' | 'provide' | 'generate') {
			settings.annotations = value;
		},
		upload,
		reset
	};
})();

export const image = (() => {
	let info: Image | undefined = $state();
	let metadata: MetadataLayer[] = $state([]);
	// TODO: Figure out why this scaling is needed.
	let width = $derived.by(() => {
		if (metadata.length === 0) return undefined;
		return metadata[0].width * 1.003;
	});

	let height = $derived.by(() => {
		if (metadata.length === 0) return undefined;
		return metadata[0].height * 1.003;
	});
	let levels: number = $derived(metadata.length);
	let tiles: ImageLayer[] = $state([]);
	let annotations: AnnotationLayer[] = $state([]);
	let initialised: boolean = $state(false);

	// Run as soon as metadata is parsed and loaded in GetMetadata.
	async function load(_info: Image) {
		initialised = false;
		reset();
		transformer.resetScale();

		info = _info;

		const _metadata = await http.GetMetadata(info.id);
		if (_metadata === undefined || _metadata.length === 0) {
			reset();
			return;
		}
		metadata = _metadata;

		tiles = new Array(levels).fill([]);

		for (let level = 0; level < levels; level++) {
			tiles[level] = new Array(metadata[level].rows)
				.fill(0)
				.map(() => new Array(metadata[level].cols).fill(new Image()));
		}

		// Ready to receive new image tiles.
		initialised = true;

		// TODO: Move annotation metadata into GetMetadata request
		// TODO: and have the actual geometry info be requested by each
		// TODO: layer individually inside of a web worker.
		const _annotations = await http.GetAnnotations(info.id);
		if (_annotations === undefined) {
			reset();
			return;
		}
		annotations = _annotations;
	}

	async function getTile(data: TileRequest): Promise<boolean> {
		if (!initialised) {
			return false;
		}

		return websocket.send(data);
	}

	async function insertTile(event: MessageEvent) {
		if (!initialised) return;

		const data: Blob = event.data;
		const arr = new Uint8Array(await data.arrayBuffer());

		const level = arr[0];
		const x = arr[1];
		const y = arr[2];

		const newTile = new Image();
		// Remove position and level values from start of array.
		const blob = new Blob([arr.slice(3)], { type: 'image/jpeg' });
		newTile.src = URL.createObjectURL(blob);
		tiles[level][y][x] = newTile;
	}

	function reset() {
		info = undefined;
		metadata = [];
		tiles = [];
		annotations = [];
	}

	return {
		get initialised() {
			return initialised;
		},
		get info() {
			return info;
		},
		get metadata() {
			return metadata;
		},
		get width() {
			return width;
		},
		get height() {
			return height;
		},
		get levels() {
			return levels;
		},
		get tiles() {
			return tiles;
		},
		get annotations() {
			return annotations;
		},
		load,
		getTile,
		insertTile
	};
})();

/// Handles zoom and offset calculations.
export const transformer = (() => {
	const MIN_SCALE = 0.1;
	const MAX_SCALE = 100;
	const MIN_LEVEL = 0;
	let maxLevel: number | undefined = $state();
	let currentLevel: number | undefined = $state();

	let offsetX = $state(0);
	let offsetY = $state(0);
	let scale = $state(1);
	let scaleBreakpoints: number[] | undefined = $derived.by(() => {
		if (!image.initialised || image.metadata === undefined || maxLevel === undefined) return;

		let lowestResolution = image.metadata[maxLevel].width * image.metadata[maxLevel].height;
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

		let ratio = 1 - newScale / scale;

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
})();
