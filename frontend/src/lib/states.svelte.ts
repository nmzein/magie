import type {
	MetadataLayer,
	AnnotationLayer,
	ImageLayer,
	Image,
	Route,
	Directory,
	DirectoryExt,
	UploaderSettings,
	TileRequest
} from '$types';
import { http, websocket } from '$api';
import { DEFAULT_BOUND, type Bounds, DEFAULT_POINT, type Point } from '$types';
import { appendPx, defined } from '$helpers';
import { SvelteSet } from 'svelte/reactivity';

export function state<T>(initial: T): { value: T };
export function state<T = undefined>(initial?: T): { value: T };
export function state<T>(initial?: T) {
	let state = $state({ value: initial });
	return state;
}

export class SelectionBox<T = any> {
	private _dragging: boolean = $state(false);
	private startPosition: Point = DEFAULT_POINT;
	private selectionBox: HTMLElement;
	private bounds: Bounds = $state(DEFAULT_BOUND);
	private parentBounds: DOMRect | Bounds;
	private intersected: SvelteSet<T> = new SvelteSet();

	constructor(selectionBox: HTMLElement, parentBounds: DOMRect | Bounds) {
		this.selectionBox = selectionBox;
		this.parentBounds = parentBounds;
	}

	public get dragging(): boolean {
		return this._dragging;
	}

	public start(cursor: Point) {
		if (this._dragging) return;

		this._dragging = true;

		this.startPosition = {
			x: cursor.x - this.parentBounds.left,
			y: cursor.y - this.parentBounds.top
		};

		this.bounds = {
			width: 0,
			height: 0,
			left: this.startPosition.x,
			top: this.startPosition.y
		};

		Object.assign(this.selectionBox.style, appendPx(this.bounds));
	}

	public update(cursor: Point) {
		if (!this._dragging) return;

		// Clamp current mouse position between 0 and parent's width/height.
		const currentX = Math.max(
			0,
			Math.min(cursor.x - this.parentBounds.left, this.parentBounds.width)
		);
		const currentY = Math.max(
			0,
			Math.min(cursor.y - this.parentBounds.top, this.parentBounds.height)
		);

		const width = currentX - this.startPosition.x;
		const height = currentY - this.startPosition.y;

		this.bounds = {
			width: Math.abs(width),
			height: Math.abs(height),
			left: width < 0 ? currentX : this.startPosition.x,
			top: height < 0 ? currentY : this.startPosition.y
		};

		Object.assign(this.selectionBox.style, appendPx(this.bounds));
	}

	public stop(): T[] {
		if (!this._dragging) return [];

		let intersected = Array.from(this.intersected);

		this.startPosition = DEFAULT_POINT;
		this.bounds = DEFAULT_BOUND;
		this.intersected.clear();

		this._dragging = false;

		return intersected;
	}

	public intersecting(target: DOMRect | Bounds, item: T | undefined = undefined): boolean {
		if (!this._dragging) return false;

		let targetLeft = target.left - this.parentBounds.left;
		let targetTop = target.top - this.parentBounds.top;

		let isIntersecting = !(
			this.bounds.left + this.bounds.width < targetLeft ||
			targetLeft + target.width < this.bounds.left ||
			this.bounds.top + this.bounds.height < targetTop ||
			targetTop + target.height < this.bounds.top
		);

		if (defined(item)) {
			let isTracked = this.intersected.has(item);

			if (isIntersecting && !isTracked) {
				this.intersected.add(item);
			} else if (!isIntersecting && isTracked) {
				this.intersected.delete(item);
			}
		}

		return isIntersecting;
	}
}

export const explorer = (() => {
	// Holds information about the directory structure.
	let registry: Directory | undefined = $state();
	// Selected directories (in main panel).
	let selected: (Directory | Image)[] = $state([]);
	// Pinned directories (in side panel).
	let pinned: DirectoryExt[] = $state([]);
	// Stack of directories to keep track of navigation.
	let stack: Route[] = $state([[0]]);
	// Pointer to current directory in stack (for back and forward).
	let stackPointer = $state(0);
	// Current directory in stack pointed to by stackPointer.
	let currentRoute: Route = $derived(stack[stackPointer]);
	// Actual current directory information obtained from registry.
	let currentDirectory = $derived.by(() => {
		if (registry === undefined) return;

		let path = [];
		let currentDirectory = registry; // Initial root node.

		for (const index of currentRoute) {
			currentDirectory = currentDirectory.subdirectories[index];
			path.push(currentDirectory.name);
		}

		return { path, directory: currentDirectory };
	});

	let showUploader: boolean = $state(false);
	let showDirectoryCreator: boolean = $state(false);

	function insertIntoStack(dir: number[]) {
		// Slice stack to current pointer and insert new directory.
		stack = stack.slice(0, stackPointer + 1);
		stack.push(dir);
		stackPointer += 1;
	}

	// Defaults to going up to parent directory.
	function up(index: number = currentRoute.length - 2) {
		if (currentRoute.length <= 1) return;

		deselectAll();

		let dir = currentRoute.slice(0, index + 1);

		let current = currentDirectory?.directory;
		insertIntoStack(dir);

		if (current === undefined) return;
		select(current);
	}

	function backward() {
		if (stackPointer <= 0) return;

		deselectAll();

		let current = currentDirectory?.directory;
		stackPointer -= 1;

		if (current === undefined) return;
		select(current);
	}

	function forward() {
		if (stackPointer >= stack.length - 1) return;

		deselectAll();

		let current = currentDirectory?.directory;
		stackPointer += 1;

		if (current === undefined) return;
		select(current);
	}

	function navigateTo(index: number) {
		deselectAll();

		// Important: concat() creates a copy of current.
		let dir = currentRoute.concat(index);

		insertIntoStack(dir);
	}

	function isSelected(item: Directory | Image): boolean {
		return selected.includes(item);
	}

	function select(item: Directory | Image) {
		selected.push(item);
	}

	function deselect(item: Directory | Image) {
		selected = selected.filter((d) => d !== item);
	}

	function deselectAll() {
		selected = [];
	}

	function pin(dir: DirectoryExt) {
		// Check not already pinned.
		if (pinned.some((pinnedDir) => pinnedDir === dir)) return;
		pinned.push(dir);
	}

	function unpin(dir: DirectoryExt) {
		// Search for index of dir in pinned.
		let index = pinned.findIndex((pinnedDir) => pinnedDir === dir);
		if (index === -1) return;
		pinned.splice(index, 1);
	}

	async function loadRegistry() {
		let _registry = await http.GetRegistry();
		if (_registry === undefined) return;
		registry = _registry;
	}

	return {
		get registry() {
			return registry;
		},
		set registry(value: Directory | undefined) {
			registry = value;
		},
		set selected(value: (Directory | Image)[]) {
			selected = value;
		},
		get currentDirectory() {
			return currentDirectory;
		},
		get showUploader() {
			return showUploader;
		},
		set showUploader(value: boolean) {
			showUploader = value;
		},
		get showDirectoryCreator() {
			return showDirectoryCreator;
		},
		set showDirectoryCreator(value: boolean) {
			showDirectoryCreator = value;
		},
		up,
		backward,
		forward,
		navigateTo,
		isSelected,
		select,
		deselect,
		deselectAll,
		pin,
		unpin,
		loadRegistry
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
	// TODO: Rename to options
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
		set parentDirectoryId(value: number | undefined) {
			parentDirectoryId = value;
		},
		set image(value: File | undefined) {
			image = value;
		},
		set annotations(value: File | undefined) {
			annotations = value;
		},
		settings,
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
