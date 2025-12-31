import { http } from '$api';
import { defined } from '$helpers';
import Viewer from '$ui/Viewer/viewer.svelte.ts';
import { WEBSOCKET_BASE_URL } from '$constants';
import { SvelteMap } from 'svelte/reactivity';
// import type { Asset } from '$types';

// export type ViewerState = {
// 	instance: Viewer;
// 	asset: Asset;
// 	position: 'top-left' | 'top-right' | 'bottom-left' | 'bottom-right';
// };

export class ViewerManager {
	viewers: SvelteMap<string, Viewer> = new SvelteMap();
	activeViewerId: string | undefined = $state();
	activeViewer: Viewer | undefined = $derived(
		this.activeViewerId ? this.viewers.get(this.activeViewerId) : undefined
	);

	async load(storeId: number, _parentId: number, assetId: number, _name: string) {
		const properties = await http.asset.properties(storeId, assetId);

		if (!defined(properties) || properties.metadata.length === 0) return;

		// const asset: Asset = {
		// 	type: 'Asset',
		// 	storeId,
		// 	parentId,
		// 	id: assetId,
		// 	name
		// };

		const instanceId = `viewer-${storeId}-${assetId}`;
		const viewer = new Viewer({
			id: `viewer-${storeId}-${assetId}`,
			primary: 1,
			layers: [
				{
					type: 'gltf',
					width: properties.metadata[0].width,
					height: properties.metadata[0].height,
					layers: properties.annotations
				},
				{
					type: 'tiled-image',
					url: `${WEBSOCKET_BASE_URL}/api/store/${storeId}/asset/${assetId}/socket`,
					width: properties.metadata[0].width,
					height: properties.metadata[0].height,
					layers: properties.metadata
				}
			]
		});

		this.viewers.clear();
		this.viewers.set(instanceId, viewer);
		this.activeViewerId = instanceId;
	}
}
