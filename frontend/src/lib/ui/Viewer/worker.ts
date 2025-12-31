import type { AssetMetadata, Dimensions } from '$types';
import { GltfRenderer, type Renderer, TiledImageRenderer } from './renderers';
import { GltfNetworker, Networker, TiledImageNetworker } from './networkers';
import { Fields, shared } from './shared';
import { type Store, GltfStore, ImageBitmapStore } from './stores';
import { zip } from '$lib/helpers/array';

export type TileIdentifier = { level: number; x: number; y: number };
export type GltfLayerIdentifier = { url: string };

let canvases: OffscreenCanvas[] = [];
const stores: Store<any>[] = [];
const renderers: Renderer<any, any>[] = [];
const networkers: Networker<any, any>[] = [];

function setCanvasDims(dims: Dimensions) {
	canvases.forEach((canvas) => {
		canvas.width = dims.width;
		canvas.height = dims.height;
	});
}

self.onmessage = (e) => {
	const { type, data } = e.data;

	switch (type) {
		case 'init': {
			const layers: AssetMetadata[] = JSON.parse(data.layers);
			shared.init(data.sharedBuf);

			canvases = data.canvases;
			setCanvasDims({ width: data.width, height: data.height });

			const canvasDefs: { ctx: '2d' | 'webgl2' }[] = JSON.parse(data.canvasDefs);
			const contexts = [];

			for (const [index, canvas] of canvases.entries()) {
				const canvasDef = canvasDefs[index];
				const ctx = canvas.getContext(canvasDef.ctx);
				if (!ctx) throw Error(`Failed to create ${canvasDef.ctx} rendering context`);

				// ctx.imageSmoothingEnabled = false; // TODO: Look into this option.
				contexts.push(ctx);
			}

			for (const layer of layers) {
				switch (layer.type) {
					case 'tiled-image': {
						const store = new ImageBitmapStore();
						stores.push(store);
						renderers.push(new TiledImageRenderer(layer, store, canvases[0], contexts[0]));
						networkers.push(new TiledImageNetworker(layer, store));
						break;
					}
					case 'gltf': {
						const store = new GltfStore();
						stores.push(store);
						renderers.push(new GltfRenderer(layer, store, canvases[1], contexts[1]));
						networkers.push(new GltfNetworker(layer, store));
						break;
					}
				}
			}

			requestAnimationFrame(loop);

			break;
		}
		case 'close': {
			for (const [store, networker] of zip(stores, networkers)) {
				store.clear();
				networker.close();
			}
			break;
		}
	}
};

function loop() {
	const wasDirty = shared.setCleanIfDirty();
	if (wasDirty) {
		const dims = { width: shared.get(Fields.Width), height: shared.get(Fields.Height) };
		const offset = { x: shared.get(Fields.OffsetX), y: shared.get(Fields.OffsetY) };
		const scale = shared.get(Fields.Scale) / 1e6;

		setCanvasDims(dims);

		for (const [renderer, networker] of zip(renderers, networkers)) {
			const requests = renderer.render(dims, offset, scale);
			networker.request(requests);
		}
	}

	requestAnimationFrame(loop);
}
