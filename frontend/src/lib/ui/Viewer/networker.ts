import { BinaryReader, BinaryWriter } from '$lib/helpers/codec';
import { WebSocketManager } from '$lib/helpers/network';
import { AssetClientMsgTag, type Asset, type TiledImageLayer } from '$types';
import { ImageBitmapCache } from './cache';
import { setDirty, type TileIdentifier } from './worker';

export class TiledImageNetworker {
	#asset: Asset<TiledImageLayer>;
	#socketManager: WebSocketManager<ImageBitmapCache>;

	constructor(asset: Asset<TiledImageLayer>, wsUrl: string) {
		this.#asset = asset;
		this.#socketManager = new WebSocketManager({
			url: wsUrl,
			cache: new ImageBitmapCache(),
			onOpen: () => self.postMessage({ type: 'connected' }),
			onMessage: this.#handleMessage,
			onError: (error) => self.postMessage({ type: 'error', error }),
			onClose: (_, __) => self.postMessage({ type: 'disconnected' })
		});

		this.#socketManager.connect();
	}

	get cache() {
		return this.#socketManager.cache;
	}

	async #handleMessage(event: MessageEvent, cache: ImageBitmapCache) {
		const r = new BinaryReader(event.data);
		const _tag = r.u8();
		const level = r.u32();
		const x = r.u32();
		const y = r.u32();
		const tileData = r.bytes();

		const key = `${level}_${x}_${y}`;
		try {
			const blob = new Blob([tileData], { type: 'image/jpeg' });
			const imageBitmap = await createImageBitmap(blob);

			cache.set(key, imageBitmap);
		} catch (error) {
			console.error('Error processing tile:', error);
		}

		return key;
	}

	request(tiles: TileIdentifier[]) {
		for (const tile of tiles) {
			const key = `${tile.level}_${tile.x}_${tile.y}`;
			if (this.cache.has(key) || this.#socketManager.pending(key)) continue;

			const layer = this.#asset.metadata.layers[tile.level];
			if (!layer || tile.x >= layer.cols || tile.y >= layer.rows) continue;

			const w = new BinaryWriter(1 + 3 * 4);
			w.u8(AssetClientMsgTag.Tile);
			w.u32(tile.level);
			w.u32(tile.x);
			w.u32(tile.y);
			const req = w.finish();

			this.#socketManager.send(req, key);
		}
	}

	close() {
		this.#socketManager.disconnect();
	}
}
