export type UploaderOptions = {
	name: string;
	encoder: string;
	decoder: string;
	generator: string;
	annotations: 'none' | 'provide' | 'generate';
};

export type WebSocketRequest = TileRequest;

export type TileRequest = {
	id: number;
	level: number;
	x: number;
	y: number;
};

export type Properties = {
	metadata: MetadataLayer[];
	annotations: AnnotationLayer[];
};

export type MetadataLayer = {
	cols: number;
	rows: number;
	width: number;
	height: number;
};

export type AnnotationLayer = {
	id: number;
	tag: string;
	visible: boolean;
	opacity: number;
	fill: string;
	stroke: string;
};

export type ImageLayer = HTMLImageElement[][];

export type Directory = {
	type: 'directory';
	id: number;
	name: string;
	files: Image[];
	subdirectories: Directory[];
};

export type Image = {
	type: 'image';
	id: number;
	name: string;
};

export type Path = string[];
export type Route = number[];

export type Navigable<T = Image | Directory> = {
	path: Path;
	route: Route;
	data: T;
};

export type Clipboard = {
	mode: 'cut' | 'copy' | undefined;
	items: (Image | Directory)[];
};

export type Bounds = { width: number; height: number; left: number; top: number };
export const DEFAULT_BOUND: Bounds = { width: 0, height: 0, left: 0, top: 0 };

export type Point = { x: number; y: number };
export const DEFAULT_POINT: Point = { x: 0, y: 0 };
