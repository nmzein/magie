export type UploaderOptions = {
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
	id: number;
	name: string;
	files: Image[];
	subdirectories: Directory[];
};

export type Image = {
	id: number;
	name: string;
};

export type Path = string[];
export type Route = number[];

export type ItemExt = DirectoryExt | ImageExt;

export type ImageExt = {
	path: Path;
	route: Route;
	data: Image;
};

export type DirectoryExt = {
	path: Path;
	route: Route;
	data: Directory;
};

export type Bounds = { width: number; height: number; left: number; top: number };
export const DEFAULT_BOUND: Bounds = { width: 0, height: 0, left: 0, top: 0 };

export type Point = { x: number; y: number };
export const DEFAULT_POINT: Point = { x: 0, y: 0 };
