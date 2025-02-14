export type Store = {
	id: number;
	name: string;
};

export type Directory = {
	type: 'Directory';
	parentId: number;
	id: number;
	name: string;
	children: number[];
};

export type Image = {
	type: 'File';
	parentId: number;
	id: number;
	name: string;
};

export type UploaderOptions = {
	name: string;
	encoder: string;
	decoder: string;
	generator: string;
	annotations: 'none' | 'provide' | 'generate';
};

export type WebSocketRequest = TileRequest;

export type TileRequest = {
	store_id: number;
	image_id: number;
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

export type Bounds = { width: number; height: number; left: number; top: number };
export const DEFAULT_BOUND: Bounds = { width: 0, height: 0, left: 0, top: 0 };

export type Point = { x: number; y: number };
export const DEFAULT_POINT: Point = { x: 0, y: 0 };
