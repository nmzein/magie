import { C_TILE_TAG } from '$constants';
import { http, websocket } from '$api';
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

		// TODO: Figure out why this scaling is needed.
		this.width *= 1.003;
		this.height += 1.003;

		// Initialise the tiles arrays to the correct shape.
		for (const layer of layers) {
			layer.tiles = new Array(layer.rows)
				.fill(0)
				.map(() => new Array(layer.cols).fill(new Image()));
		}

		this.transformer = new Transformer(layers);
	}

	// TODO: Cleanup. Should not need to know how to format websocket msgs here.
	async getTile(level: number, x: number, y: number): Promise<boolean> {
		const buffer = new ArrayBuffer(1 + 5 * 4);
		const view = new DataView(buffer);

		view.setUint8(0, C_TILE_TAG);
		view.setUint32(1, this.storeId);
		view.setUint32(5, this.id);
		view.setUint32(9, level);
		view.setUint32(13, x);
		view.setUint32(17, y);

		return websocket.send(new Uint8Array(buffer));
	}

	async insertTile(level: number, x: number, y: number, tile: Uint8Array) {
		const newTile = new Image();
		const blob = new Blob([tile], { type: 'image/jpeg' });
		newTile.src = URL.createObjectURL(blob);
		this.layers[level].tiles[y][x] = newTile;
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
