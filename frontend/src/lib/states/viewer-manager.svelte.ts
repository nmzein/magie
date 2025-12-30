import { http } from '$api';
import { defined } from '$helpers';
import Viewer from '$ui/Viewer/viewer.svelte.ts';
import { WEBSOCKET_BASE_URL } from '$constants';
import { SvelteMap } from 'svelte/reactivity';
import type { Asset, TiledImageLayer } from '$types';

export type ViewerState = {
	instance: Viewer;
	position: 'top-left' | 'top-right' | 'bottom-left' | 'bottom-right';
};

export class ViewerManager {
	viewers: SvelteMap<string, Viewer> = new SvelteMap();
	activeViewerId: string | undefined = $state();
	activeViewer: Viewer | undefined = $derived(
		this.activeViewerId ? this.viewers.get(this.activeViewerId) : undefined
	);

	async load(storeId: number, parentId: number, assetId: number, name: string) {
		const properties = await http.asset.properties(storeId, assetId);

		if (!defined(properties) || properties.metadata.length === 0) return;

		const asset: Asset<TiledImageLayer> = {
			type: 'Asset',
			storeId,
			parentId,
			id: assetId,
			name,
			metadata: {
				width: properties.metadata[0].width,
				height: properties.metadata[0].height,
				layers: properties.metadata
			}
		};

		const instanceId = `viewer-${storeId}-${assetId}`;
		const viewer = new Viewer({
			id: `viewer-${storeId}-${assetId}`,
			websocketUrl: `${WEBSOCKET_BASE_URL}/api/store/${storeId}/asset/${assetId}/socket`,
			asset
		});

		this.viewers.set(instanceId, viewer);
		this.activeViewerId = instanceId;
	}
}
