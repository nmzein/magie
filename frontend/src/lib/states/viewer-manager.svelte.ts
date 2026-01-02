import { SvelteMap } from 'svelte/reactivity';
import { http, WEBSOCKET_BASE_URL } from '$api';
import { defined } from '$helpers';
import type { Asset } from '$types';
import Viewer from '$ui/Viewer/viewer.svelte.ts';

type Position = 'top-left' | 'top-right' | 'bottom-left' | 'bottom-right';

export type ViewerState = {
	instance: Viewer;
	asset: Asset;
};

export class ViewerManager {
	#width: 50 | 100 = 100;
	#height: 50 | 100 = 100;
	viewers: SvelteMap<Position, ViewerState> = new SvelteMap();
	active: string | undefined = $state();

	get width() {
		return this.#width;
	}

	get height() {
		return this.#height;
	}

	get activeViewer() {
		return this.active ? this.viewers.get(this.active as Position)?.instance : undefined;
	}

	#nextPosition() {
		const positions: Position[] = ['top-left', 'top-right', 'bottom-left', 'bottom-right'];
		return positions[0]; // TEMP: Allow only one viewer for now.
		// return positions[this.viewers.size];
	}

	async load(storeId: number, parentId: number, assetId: number, name: string) {
		const properties = await http.asset.properties(storeId, assetId);

		if (!defined(properties) || properties.metadata.length === 0) return;

		const asset: Asset = {
			type: 'Asset',
			storeId,
			parentId,
			id: assetId,
			name
		};

		const width = properties.metadata[0].width;
		const height = properties.metadata[0].height;

		const position = this.#nextPosition();
		const instance = new Viewer({
			id: `${position}-${storeId}-${assetId}`,
			primary: 1,
			layers: [
				{
					type: 'gltf',
					width,
					height,
					layers: properties.annotations
				},
				{
					type: 'tiled-image',
					width,
					height,
					url: `${WEBSOCKET_BASE_URL}/api/store/${storeId}/asset/${assetId}/socket`,
					layers: properties.metadata
				}
			]
		});

		// TEMP: Allow only one viewer for now.
		// this.#width = this.viewers.size === 0 ? 100 : 50;
		// this.#height = this.viewers.size <= 1 ? 100 : 50;
		this.viewers.set(position, { instance, asset });
		this.active = position;
	}
}
