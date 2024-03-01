export type TileRequest = {
	id: number;
	level: number;
	x: number;
	y: number;
};

export type Image = {
	id: number;
	path: string;
};

export type Metadata = {
	cols: number;
	rows: number;
	width: number;
	height: number;
};

type Geometry = number[][];

export type AnnotationLayer = {
	tag: string;
	visible: boolean;
	opacity: number;
	fill: string;
	stroke: string;
	annotations: Geometry[];
};

export type ImageLayer = HTMLImageElement[][];

export type DirectoryNode = {
	name: string;
	children: DirectoryNode[];
	files: { name: string; metadata: Image }[];
};
