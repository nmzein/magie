import { http } from '$api';
import { defined } from '$helpers';
import Viewer from '$ui/Viewer/viewer.svelte.ts';
import { WEBSOCKET_BASE_URL } from '$constants';
import { SvelteMap } from 'svelte/reactivity';

export type Layer = {
	level: number;
	cols: number;
	rows: number;
	width: number;
	height: number;
};

export type Asset = {
	storeId: number;
	parentId: number;
	assetId: number;
	name: string;
	width: number;
	height: number;
	levels: number;
	layers: Layer[];
};

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

		const asset: Asset = {
			storeId,
			parentId,
			assetId,
			name,
			width: properties.metadata[0].width,
			height: properties.metadata[0].height,
			levels: properties.metadata.length,
			layers: properties.metadata
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
