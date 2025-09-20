import { http } from '$api';
import { defined } from '$helpers';
import { views } from '$states';
import { Transformer } from './transformer.svelte.ts';
import type { Image2DLayer } from './types.ts';
import type { Geometry2DLayer } from '$view/Geometry2D/types.ts';

class Image2DState {
	width: number;
	height: number;
	levels: number;
	layers: Image2DLayer[] = $state([]);
	geometries: Geometry2DLayer[] = $state([]);
	transformer: Transformer;

	constructor(
		public storeId: number,
		public parentId: number,
		public id: number,
		public name: string,
		layers: Image2DLayer[],
		geometries: Geometry2DLayer[]
	) {
		this.layers = layers;
		this.geometries = geometries;
		this.width = layers[0].width;
		this.height = layers[0].height;
		this.levels = layers.length;
		this.transformer = new Transformer(layers);
	}
}

export async function load(storeId: number, parentId: number, id: number, name: string) {
	const properties = await http.asset.properties(storeId, id);

	if (!defined(properties) || properties.metadata.length === 0) return;

	const state = new Image2DState(
		storeId,
		parentId,
		id,
		name,
		properties.metadata,
		properties.annotations
	);

	views[0] = {
		type: 'Image2D',
		state,
		active: true
	};
}

export type { Image2DState };
