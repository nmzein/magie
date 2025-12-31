import { http } from '$api';
import { defined } from '$helpers';
import Viewer from '$ui/Viewer/viewer.svelte.ts';
import { WEBSOCKET_BASE_URL } from '$constants';
import { SvelteMap } from 'svelte/reactivity';
import type { Asset } from '$types';

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
	activeViewer: Viewer | undefined = $derived(
		this.active ? this.viewers.get(this.active as Position)?.instance : undefined
	);

	get width() {
		return this.#width;
	}

	get height() {
		return this.#height;
	}

	#nextPosition() {
		const positions: Position[] = ['top-left', 'top-right', 'bottom-left', 'bottom-right'];
		return positions[this.viewers.size];
	}

	async load(storeId: number, parentId: number, assetId: number, name: string) {
		if (this.viewers.size === 2) return;

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
			id: position,
			primary: 0,
			// primary: 1,
			layers: [
				// {
				// 	type: 'gltf',
				// 	width,
				// 	height,
				// 	layers: properties.annotations
				// },
				{
					type: 'tiled-image',
					width,
					height,
					url: `${WEBSOCKET_BASE_URL}/api/store/${storeId}/asset/${assetId}/socket`,
					layers: properties.metadata
				}
			]
		});

		this.#width = this.viewers.size === 0 ? 100 : 50;
		this.#height = this.viewers.size <= 1 ? 100 : 50;
		this.viewers.set(position, { instance, asset });
		this.active = position;
	}
}
