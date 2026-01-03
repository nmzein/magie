import { SvelteMap } from 'svelte/reactivity';
import { http } from '$api';
import { defined } from '$helpers';
import type { Asset, AssetMetadata } from '$types';
import Viewer, { type AssetOptions } from '$ui/Viewer/viewer.svelte.ts';

type Position = 'top-left' | 'top-right' | 'bottom-left' | 'bottom-right';

export type ViewerState = {
	instance: Viewer;
	asset: Asset;
};

export class ViewerManager {
	#width: 50 | 100 = 100;
	#height: 50 | 100 = 100;
	viewers: SvelteMap<string, ViewerState> = new SvelteMap();
	active: string | undefined = $state();
	activeViewer: Viewer | undefined = $derived(
		this.active ? this.viewers.get(this.active)?.instance : undefined
	);

	get width() {
		return this.#width;
	}

	get height() {
		return this.#height;
	}

	#nextPosition() {
		const positions: Position[] = ['top-left', 'top-right', 'bottom-left', 'bottom-right'];
		return positions[0]; // TEMP: Allow only one viewer for now.
		// return positions[this.viewers.size];
	}

	async load(storeId: number, parentId: number, assetId: number, name: string) {
		const group = await http.asset.properties(storeId, assetId);
		if (!defined(group) || group.length === 0) return;

		const asset: Asset = {
			type: 'Asset',
			storeId,
			parentId,
			id: assetId,
			name
		};

		const layers: (AssetMetadata & AssetOptions)[] = group.map((asset) => {
			if (asset.type === 'tiled-image') return { ...asset, contextId: '2d' };
			if (asset.type === 'gltf') return { ...asset, contextId: 'webgl2' };
			throw Error('Unsupported asset type.');
		});

		const position = this.#nextPosition();
		const id = `${position}-${storeId}-${assetId}`;
		const instance = new Viewer({ id, layers });

		// TEMP: Allow only one viewer for now.
		// this.#width = this.viewers.size === 0 ? 100 : 50;
		// this.#height = this.viewers.size <= 1 ? 100 : 50;
		this.viewers.clear();
		this.viewers.set(id, { instance, asset });
		this.active = id;
	}
}
