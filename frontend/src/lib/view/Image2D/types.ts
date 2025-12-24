import type { Image2DState } from './state.svelte';
import type Viewer from './viewer.svelte';

export type Image2DLayer = {
	level: number;
	cols: number;
	rows: number;
	width: number;
	height: number;
};

export type Image2DView = {
	type: 'Image2D';
	state: Image2DState;
	viewer: Viewer;
	active: boolean;
};

export type { Image2DState };
