export type UploaderSettings = {
	generator: string;
	annotations: 'none' | 'provide' | 'generate';
};

export type TileRequest = {
	id: number;
	level: number;
	x: number;
	y: number;
};

export type MetadataLayer = {
	cols: number;
	rows: number;
	width: number;
	height: number;
};

export type AnnotationLayer = {
	tag: string;
	visible: boolean;
	opacity: number;
	fill: string;
	stroke: string;
	geometry: string;
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
