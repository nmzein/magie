export type Store = {
	id: number;
	name: string;
};

export type Modules = {
	generators: string[];
	decoders: string[];
	encoders: string[];
};

export type Directory = {
	type: 'Directory';
	storeId: number;
	parentId: number;
	id: number;
	name: string;
	children: number[];
};

export type Asset<L = TiledImageLayer | GltfLayer> = {
	type: 'Asset';
	storeId: number;
	parentId: number;
	id: number;
	name: string;
	metadata: AssetMetadata<L>;
};

export type AssetMetadata<L> = {
	width: number;
	height: number;
	layers: L[];
};

export type TiledImageLayer = {
	level: number;
	cols: number;
	rows: number;
	width: number;
	height: number;
};

export type GltfLayer = {
	id: number;
	tag: string;
	visible: boolean;
	opacity: number;
	fill: string;
	stroke: string;
};

export type UploaderOptions = {
	name: string;
	encoder: string;
	decoder: string;
	generator: string;
	annotations: 'none' | 'provide' | 'generate';
};

export type Bounds = { width: number; height: number; left: number; top: number };
export const DEFAULT_BOUND: Bounds = { width: 0, height: 0, left: 0, top: 0 };

export type Point = { x: number; y: number };
export const DEFAULT_POINT: Point = { x: 0, y: 0 };

export const AssetClientMsgTag = {
	Tile: 0
};

export const AssetServerMsgTag = {
	Error: 0,
	Tile: 1
};

export const GeneralServerMsgTag = {
	Error: 0,
	Directory: 1
};

export const DirectoryServerMsgTag = {
	Create: 0,
	Delete: 1,
	Move: 2,
	Rename: 3
};
