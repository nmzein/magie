export type ImageSelection = {
	start: Point;
	end: Point;
};

export type Point = {
	x: number;
	y: number;
};

export type ImageMetadata = {
	cols: number;
	rows: number;
};

export type AnnotationLayer = {
	colours: {
		fill: string;
		stroke: string;
	};
	annotations: Point[][];
};
