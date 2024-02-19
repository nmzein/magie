export type TileRequest = {
	image_name: string;
	level: number;
	x: number;
	y: number;
};

export type Metadata = {
	cols: number;
	rows: number;
	width: number;
	height: number;
};

export type AnnotationLayer = {
	tag: string;
	colours: {
		fill: string;
		stroke: string;
	};
	annotations: number[][][];
};

export type ImageLayer = HTMLImageElement[][];
