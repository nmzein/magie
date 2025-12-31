import type { AssetMetadata } from '$types';
import { type Renderer, TiledImageRenderer } from './renderer';
import { Networker, TiledImageNetworker } from './networker';
import { Fields, shared } from './shared';
import { ImageBitmapCache } from './cache';

export type TileIdentifier = { level: number; x: number; y: number };

const renderers: Renderer<any, any>[] = [];
const networkers: Networker<any, any>[] = [];

self.onmessage = (e) => {
	const { type, data } = e.data;

	switch (type) {
		case 'init': {
			const layers: AssetMetadata[] = JSON.parse(data.layers);
			shared.init(data.sharedBuf);

			for (const layer of layers.toReversed()) {
				switch (layer.type) {
					case 'tiled-image': {
						const cache = new ImageBitmapCache();
						renderers.push(
							new TiledImageRenderer(layer, data.canvas, data.width, data.height, cache)
						);
						networkers.push(new TiledImageNetworker(layer, cache));
						break;
					}
					case 'gltf': {
						// renderer = new GLTFRenderer(layer, data.canvas, data.width, data.height);
						// networker = new GLTFNetworker(layer);
						break;
					}
				}
			}

			requestAnimationFrame(loop);

			break;
		}
		case 'close': {
			for (const networker of networkers) {
				networker.close();
			}
			break;
		}
	}
};

function loop() {
	const dirty = shared.setClean();
	if (dirty === 1) {
		const dims = { width: shared.get(Fields.Width), height: shared.get(Fields.Height) };
		const offset = { x: shared.get(Fields.OffsetX), y: shared.get(Fields.OffsetY) };
		const scale = shared.get(Fields.Scale) / 1e6;

		for (const [index, renderer] of renderers.entries()) {
			renderer.updateTransforms(dims, offset, scale);

			const requests = renderer.visible();
			networkers[index].request(requests);
			renderer.render(requests);
		}
	}

	requestAnimationFrame(loop);
}
