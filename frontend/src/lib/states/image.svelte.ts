import type { ImageLayer, Image, TileRequest, Properties } from '$types';
import { http, websocket } from '$api';
import { defined } from '$helpers';
import { Transformer } from './transformer.svelte';

export class ImageViewer {
	initialised = $state(false);
	// @ts-ignore
	info: Image = $state();
	// @ts-ignore
	properties: Properties = $state();
	// @ts-ignore
	width: number;
	// @ts-ignore
	height: number;
	// @ts-ignore
	levels: number;
	tiles: ImageLayer[] = $state([]);
	// @ts-ignore
	transformer: Transformer;

	constructor(info: Image) {
		http.image.properties(info.id).then((properties) => {
			if (!defined(properties) || properties.metadata.length === 0) return;

			this.info = info;
			this.properties = properties;
			this.levels = properties.metadata.length;
			// TODO: Figure out why this scaling is needed.
			this.width = this.properties.metadata[0].width * 1.003;
			this.height = this.properties.metadata[0].height * 1.003;

			this.tiles = new Array(this.levels).fill([]);

			for (let level = 0; level < this.levels; level++) {
				this.tiles[level] = new Array(properties.metadata[level].rows)
					.fill(0)
					.map(() => new Array(properties.metadata[level].cols).fill(new Image()));
			}

			this.transformer = new Transformer(properties.metadata);

			this.initialised = true;
		});
	}

	async getTile(data: TileRequest): Promise<boolean> {
		return websocket.send(data);
	}

	async insertTile(event: MessageEvent) {
		const data: Blob = event.data;
		const arr = new Uint8Array(await data.arrayBuffer());

		const level = arr[0];
		const x = arr[1];
		const y = arr[2];

		const newTile = new Image();
		// Remove position and level values from start of array.
		const blob = new Blob([arr.slice(3)], { type: 'image/jpeg' });
		newTile.src = URL.createObjectURL(blob);
		this.tiles[level][y][x] = newTile;
	}
}
