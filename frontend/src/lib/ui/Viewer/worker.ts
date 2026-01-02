import { zip } from '$lib/helpers/array';
import type { AssetMetadata, Dimensions } from '$types';
import { GltfNetworker, type Networker, TiledImageNetworker } from './networkers';
import { GltfRenderer, type Renderer, TiledImageRenderer } from './renderers';
import { Fields, shared } from './shared';
import { GltfStorer, ImageBitmapStorer, type Storer } from './storers';

export type TileIdentifier = { level: number; x: number; y: number };
export type GltfLayerIdentifier = { url: string };

let canvases: OffscreenCanvas[] = [];
const storers: Storer<any>[] = [];
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
						const storer = new ImageBitmapStorer();
						storers.push(storer);
						renderers.push(new TiledImageRenderer(layer, storer, canvases[0], contexts[0]));
						networkers.push(new TiledImageNetworker(layer, storer));
						break;
					}
					case 'gltf': {
						const storer = new GltfStorer();
						storers.push(storer);
						renderers.push(new GltfRenderer(layer, storer, canvases[1], contexts[1]));
						networkers.push(new GltfNetworker(layer, storer));
						break;
					}
				}
			}

			requestAnimationFrame(loop);

			break;
		}
		case 'close': {
			for (const [storer, networker] of zip(storers, networkers)) {
				storer.clear();
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
