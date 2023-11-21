export type ImageSelection = {
	start: Point;
	end: Point;
};

type Point = {
	x: number;
	y: number;
};

export type ImageMetadata = {
	cols: number;
	rows: number;
};
