import type { Image2DState } from './state.svelte';

export type Image2DLayer = {
	tiles: HTMLImageElement[][];
	level: number;
	cols: number;
	rows: number;
	width: number;
	height: number;
};

export type Image2DView = {
	type: 'Image2D';
	state: Image2DState;
	active: boolean;
};

export type { Image2DState };
