import { type Asset, type TiledImageLayer } from '$types';
import { TiledImageRenderer } from './renderer';
import { TiledImageNetworker } from './networker';
import { ImageBitmapCache } from './cache';
import { Bytes } from './shared';

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
	return Atomics.compareExchange(shared, Bytes.Dirty, 1, 0);
}

export function setDirty() {
	return Atomics.store(shared, Bytes.Dirty, 1);
}

function loop() {
	const dirty = setClean();
	if (dirty === 1) {
		const dims = { width: shared[Bytes.Width], height: shared[Bytes.Height] };
		const offset = { x: shared[Bytes.OffsetX], y: shared[Bytes.OffsetY] };
		const scale = shared[Bytes.Scale] / 1e6;

		renderer.updateTransforms(dims, offset, scale);
		const requests = renderer.visible(shared);

		networker.request(requests, shared);
		renderer.render(requests, networker.cache, shared);
	}

	requestAnimationFrame(loop);
}
