import { zip } from '$lib/helpers/array';
import type { AssetMetadata, Dimensions } from '$types';
import { DracoGeometryNetworker, type Networker, TiledImageNetworker } from './networkers';
import { DracoGeometryRenderer, type Renderer, TiledImageRenderer } from './renderers';
import { Fields, shared } from './shared';
import { BufferGeometryStorer, ImageBitmapStorer, type Storer } from './storers';
import type { AssetOptions, InjectedAssetMetadata } from './viewer.svelte';

export type TileIdentifier = { level: number; x: number; y: number };
export type GeometryLayerIdentifier = { url: string };

let canvases: OffscreenCanvas[] = [];
const storers: Storer<unknown>[] = [];
const renderers: Renderer<unknown, unknown>[] = [];
const networkers: Networker<unknown, unknown>[] = [];

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
			const layers: (AssetMetadata & AssetOptions & InjectedAssetMetadata)[] = JSON.parse(
				data.layers
			);
			shared.init(data.sharedBuf);
			canvases = data.canvases;

			const contextIds: AssetOptions['contextId'][] = JSON.parse(data.contextIds);
			const contexts = [];

			for (const [canvas, contextId] of zip(canvases, contextIds)) {
				const ctx = canvas.getContext(contextId);
				if (!ctx) throw Error(`Failed to create ${contextId} rendering context`);
				contexts.push(ctx);

				// ctx.imageSmoothingEnabled = false; // TODO: Look into this option.
			}

			for (const layer of layers) {
				switch (layer.type) {
					case 'tiled-image': {
						const storer = new ImageBitmapStorer();
						storers.push(storer);
						renderers.push(
							new TiledImageRenderer(
								layer,
								storer,
								canvases[layer.contextIndex],
								contexts[layer.contextIndex] as OffscreenCanvasRenderingContext2D
							)
						);
						networkers.push(new TiledImageNetworker(layer, storer));
						break;
					}
					case 'draco-geometry': {
						const storer = new BufferGeometryStorer();
						storers.push(storer);
						renderers.push(
							new DracoGeometryRenderer(
								layer,
								storer,
								canvases[layer.contextIndex],
								contexts[layer.contextIndex] as WebGL2RenderingContext
							)
						);
						networkers.push(new DracoGeometryNetworker(layer, storer));
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
