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
	parentId: number;
	id: number;
	name: string;
	children: number[];
};

export type Asset = {
	type: 'Asset';
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

export type Bounds = { width: number; height: number; left: number; top: number };
export const DEFAULT_BOUND: Bounds = { width: 0, height: 0, left: 0, top: 0 };

export type Point = { x: number; y: number };
export const DEFAULT_POINT: Point = { x: 0, y: 0 };

export const AssetClientMsgTag = {
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
