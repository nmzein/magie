import { type Asset, type TiledImageLayer } from '$types';
import { TiledImageRenderer } from './renderer';
import { TiledImageNetworker } from './networker';
import { ImageBitmapCache } from './cache';
import { Fields } from './shared';

export type TileIdentifier = { level: number; x: number; y: number };

let asset: Asset<TiledImageLayer>;
let shared: Int32Array;

let cache: ImageBitmapCache;
let renderer: TiledImageRenderer;
let networker: TiledImageNetworker;

self.onmessage = (e) => {
	const { type, data } = e.data;

	switch (type) {
		case 'init':
			asset = JSON.parse(data.asset);
			shared = new Int32Array(data.sharedBuf);

			cache = new ImageBitmapCache();
			renderer = new TiledImageRenderer(asset, data.canvas, data.width, data.height);
			networker = new TiledImageNetworker(asset, data.wsUrl);

			requestAnimationFrame(loop);

			break;
		case 'close':
			cache.clear();
			networker.close();
			break;
	}
};

function setClean() {
	return Atomics.compareExchange(shared, Fields.Dirty, 1, 0);
}

export function setDirty() {
	return Atomics.store(shared, Fields.Dirty, 1);
}

function loop() {
	const dirty = setClean();
	if (dirty === 1) {
		const dims = { width: shared[Fields.Width], height: shared[Fields.Height] };
		const offset = { x: shared[Fields.OffsetX], y: shared[Fields.OffsetY] };
		const scale = shared[Fields.Scale] / 1e6;

		renderer.updateTransforms(dims, offset, scale);
		const requests = renderer.visible();

		networker.request(requests);
		renderer.render(requests, networker.cache);
		setDirty();
	}

	requestAnimationFrame(loop);
}
