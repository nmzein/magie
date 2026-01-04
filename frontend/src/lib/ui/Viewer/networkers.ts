import type { BufferGeometry } from 'three';
import { DRACOLoader } from 'three/addons/loaders/DRACOLoader.js';
import { ByteReader, ByteWriter } from '$lib/helpers/byte';
import { WebSocketManager } from '$lib/helpers/network';
import {
	AssetClientMsgTag,
	type AssetMetadata,
	type GeometryAssetMetadata,
	type TiledImageAssetMetadata
} from '$types';
import { shared } from './shared';
import type { BufferGeometryStorer, ImageBitmapStorer, Storer } from './storers';
import type { GeometryLayerIdentifier, TileIdentifier } from './worker';

export class Networker<T, S> {
	constructor(_metadata: AssetMetadata, _store: Storer<S>) {
		if (new.target === Networker) {
			throw new Error('Networker is abstract and cannot be instantiated');
		}
	}

	async request(_requests: T[]) {
		throw new Error('request(1) must be implemented');
	}

	close() {
		throw new Error('close() must be implemented');
	}
}

export class TiledImageNetworker extends Networker<TileIdentifier, ImageBitmap> {
	#storer: ImageBitmapStorer;
	#metadata: TiledImageAssetMetadata;
	#socketManager: WebSocketManager;

	constructor(metadata: TiledImageAssetMetadata, storer: ImageBitmapStorer) {
		super(metadata, storer);

		this.#storer = storer;
		this.#metadata = metadata;
		this.#socketManager = new WebSocketManager({
			url: metadata.url,
			onMessage: (event) => this.#handleMessage(event),
			onError: (error) => self.postMessage({ type: 'error', error })
		});

		this.#socketManager.connect();
	}

	async #handleMessage(event: MessageEvent) {
		const r = new ByteReader(event.data);
		const _tag = r.u8();
		const level = r.u32();
		const x = r.u32();
		const y = r.u32();
		const tileData = r.bytes();

		const key = `${level}_${x}_${y}`;
		try {
			const blob = new Blob([tileData], { type: 'image/jpeg' });
			const imageBitmap = await createImageBitmap(blob);

			this.#storer.set(key, imageBitmap);
			shared.setDirty();
		} catch (error) {
			console.error('Error processing tile:', error);
		}

		return key;
	}

	async request(tiles: TileIdentifier[]) {
		for (const tile of tiles) {
			const key = `${tile.level}_${tile.x}_${tile.y}`;
			if (this.#storer.has(key) || this.#socketManager.pending(key)) continue;

			const layer = this.#metadata.layers[tile.level];
			if (!layer || tile.x >= layer.cols || tile.y >= layer.rows) continue;

			const w = new ByteWriter(1 + 3 * 4);
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

export class DracoGeometryNetworker extends Networker<GeometryLayerIdentifier, BufferGeometry> {
	#storer: BufferGeometryStorer;
	#pending: Set<string> = new Set();
	#loader: DRACOLoader;

	constructor(metadata: GeometryAssetMetadata, storer: BufferGeometryStorer) {
		super(metadata, storer);

		this.#storer = storer;
		this.#loader = new DRACOLoader();
		this.#loader.setDecoderPath('/draco/');
	}

	async request(requests: GeometryLayerIdentifier[]) {
		for (const layer of requests) {
			if (this.#storer.has(layer.url) || this.#pending.has(layer.url)) {
				continue;
			}

			this.#pending.add(layer.url);

			this.#loader
				.loadAsync(layer.url)
				.then((file) => {
					this.#storer.set(layer.url, file);
					shared.setDirty();
				})
				.catch((err) => {
					console.error(`Failed to load ${layer.url}`, err);
				})
				.finally(() => {
					this.#pending.delete(layer.url);
				});
		}
	}

	close() {
		this.#loader.dispose();
	}
}
