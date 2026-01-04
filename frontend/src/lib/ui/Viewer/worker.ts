import { zip } from '$lib/helpers/array';
import type { AssetMetadata } from '$types';
import { DracoGeometryNetworker, type Networker, TiledImageNetworker } from './networkers';
import { DracoGeometryRenderer, type Renderer, TiledImageRenderer } from './renderers';
import { Fields, shared } from './shared';
import { BufferGeometryStorer, ImageBitmapStorer, type Storer } from './storers';
import type { AssetOptions, InjectedAssetMetadata } from './viewer.svelte';

export type TileIdentifier = { level: number; x: number; y: number };
export type GeometryLayerIdentifier = { url: string };

let canvases: OffscreenCanvas[] = [];
let actors: [Storer<unknown>, Renderer<unknown, unknown>, Networker<unknown, unknown>][];

self.onmessage = (e) => {
	const { type, data } = e.data;

	switch (type) {
		case 'init': {
			const layers: (AssetMetadata & AssetOptions & InjectedAssetMetadata)[] = JSON.parse(
				data.layers
			);
			shared.init(data.sharedBuf);
			canvases = data.canvases;

			const contextIds: AssetOptions['contextId'][] = JSON.parse(data.contextIds);
			const contexts: OffscreenRenderingContext[] = [];

			for (const [canvas, contextId] of zip(canvases, contextIds)) {
				const ctx = canvas.getContext(contextId);
				if (!ctx) throw Error(`Failed to create ${contextId} rendering context`);
				contexts.push(ctx);

				// ctx.imageSmoothingEnabled = false; // TODO: Look into this option.
			}

			actors = layers.map((layer) => {
				switch (layer.type) {
					case 'tiled-image': {
						const storer = new ImageBitmapStorer();
						const renderer = new TiledImageRenderer(
							layer,
							storer,
							canvases[layer.contextIndex],
							contexts[layer.contextIndex] as OffscreenCanvasRenderingContext2D
						);
						const networker = new TiledImageNetworker(layer, storer);
						return [storer, renderer, networker];
					}
					case 'draco-geometry': {
						const storer = new BufferGeometryStorer();
						const renderer = new DracoGeometryRenderer(
							layer,
							storer,
							canvases[layer.contextIndex],
							contexts[layer.contextIndex] as WebGL2RenderingContext
						);
						const networker = new DracoGeometryNetworker(layer, storer);
						return [storer, renderer, networker];
					}
					default:
						throw Error(`Unsupported asset type.`);
				}
			});

			requestAnimationFrame(loop);
			break;
		}
		case 'close': {
			for (const [storer, _, networker] of actors) {
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

		canvases.forEach((canvas) => {
			canvas.width = dims.width;
			canvas.height = dims.height;
		});

		for (const [_, renderer, networker] of actors) {
			const requests = renderer.render(dims, offset, scale);
			networker.request(requests);
		}
	}

	requestAnimationFrame(loop);
}
