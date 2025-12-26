import { http } from '$api';
import { defined } from '$helpers';
import Viewer from '$ui/Viewer/viewer.svelte.ts';
import { WEBSOCKET_BASE_URL } from '$constants';

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
	instanceId: string;
	instance: Viewer;
};

export class ViewerManager {
	viewers: ViewerState[] = $state([]);
	activeViewer: ViewerState | undefined = $state();

	constructor() {}

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
		const instance = new Viewer({
			canvasId: instanceId,
			websocketUrl: `${WEBSOCKET_BASE_URL}/api/store/${storeId}/asset/${assetId}/socket`,
			asset
		});
		const viewer = {
			instanceId,
			instance
		};

		this.viewers.push(viewer);
		this.activeViewer = viewer;
	}
}
