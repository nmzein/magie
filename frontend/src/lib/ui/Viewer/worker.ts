import { type Asset, type TiledImageLayer } from '$types';
import { TiledImageRenderer } from './renderer';
import { TiledImageNetworker } from './networker';
import { Fields, shared } from './shared';
import { ImageBitmapCache } from './cache';

export type TileIdentifier = { level: number; x: number; y: number };

let cache: ImageBitmapCache;
let renderer: TiledImageRenderer;
let networker: TiledImageNetworker;

self.onmessage = (e) => {
	const { type, data } = e.data;

	switch (type) {
		case 'init':
			const asset: Asset<TiledImageLayer> = JSON.parse(data.asset);
			shared.init(data.sharedBuf);

			cache = new ImageBitmapCache();
			renderer = new TiledImageRenderer(asset, data.canvas, data.width, data.height, cache);
			networker = new TiledImageNetworker(asset, data.url, cache);

			requestAnimationFrame(loop);

			break;
		case 'close':
			networker.close();
			break;
	}
};

function loop() {
	const dirty = shared.setClean();
	if (dirty === 1) {
		const dims = { width: shared.get(Fields.Width), height: shared.get(Fields.Height) };
		const offset = { x: shared.get(Fields.OffsetX), y: shared.get(Fields.OffsetY) };
		const scale = shared.get(Fields.Scale) / 1e6;

		renderer.updateTransforms(dims, offset, scale);

		const requests = renderer.visible();
		networker.request(requests);
		renderer.render(requests);
	}

	requestAnimationFrame(loop);
}
