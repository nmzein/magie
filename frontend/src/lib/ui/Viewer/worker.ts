import { AssetClientMsgTag } from '$types';
import { BinaryReader, BinaryWriter } from '$lib/helpers/codec';
import { WebSocketManager } from '$lib/helpers/network';
import type { Asset } from '$lib/states/viewer-manager.svelte';
import { Renderer } from './renderer';
import { ImageBitmapCache } from './cache';

export type TileIdentifier = { level: number; x: number; y: number };

let asset: Asset;
let sharedInts: Int32Array;

let cache = new ImageBitmapCache();
let renderer: Renderer;
let socketManager: WebSocketManager;

self.onmessage = function (e) {
	const { type, data } = e.data;

	switch (type) {
		case 'init':
			asset = JSON.parse(data.asset);
			sharedInts = new Int32Array(data.sharedBuf);

			renderer = new Renderer(asset, data.canvas, data.width, data.height);
			socketManager = new WebSocketManager({
				url: data.wsUrl,
				onOpen: () => self.postMessage({ type: 'connected' }),
				onMessage: handleTile,
				onError: (error) => self.postMessage({ type: 'error', error }),
				onClose: (_, willReconnect) => self.postMessage({ type: 'disconnected' })
			});

			socketManager.connect();
			requestAnimationFrame(loop);

			break;
		case 'close':
			cache.clear();
			socketManager?.disconnect();
			break;
	}
};

async function handleTile(event: MessageEvent) {
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
		setDirty();
	} catch (error) {
		console.error('Error processing tile:', error);
	}

	return key;
}

function requestTiles(tiles: TileIdentifier[]) {
	for (const tile of tiles) {
		const key = `${tile.level}_${tile.x}_${tile.y}`;
		if (cache.has(key) || socketManager.pending(key)) continue;

		const layer = asset.layers[tile.level];
		if (!layer || tile.x >= layer.cols || tile.y >= layer.rows) continue;

		const w = new BinaryWriter(1 + 3 * 4);
		w.u8(AssetClientMsgTag.Tile);
		w.u32(tile.level);
		w.u32(tile.x);
		w.u32(tile.y);
		const req = w.finish();

		socketManager.send(req, key);
	}
}

function setClean() {
	return Atomics.compareExchange(sharedInts, 0, 1, 0);
}

function setDirty() {
	return Atomics.store(sharedInts, 0, 1);
}

function loop() {
	const dirty = setClean();
	if (dirty === 1) {
		const canvasWidth = sharedInts[1];
		const canvasHeight = sharedInts[2];

		const offset = { x: sharedInts[3], y: sharedInts[4] };
		const scale = sharedInts[5] / 1e6;

		renderer.updateCanvasDimensions(canvasWidth, canvasHeight);
		renderer.updateTransforms(offset, scale);
		const visibleTiles = renderer.calculateVisibleTiles();

		requestTiles(visibleTiles);

		const debugCallback = (ctx: OffscreenCanvasRenderingContext2D) => {
			ctx.setTransform(1, 0, 0, 1, 0, 0);
			ctx.save();

			const panelWidth = 230;
			const panelHeight = 120;
			const padding = 10;

			// Background box
			ctx.fillStyle = 'rgba(0,0,0,0.6)';
			ctx.fillRect(padding, canvasHeight - panelHeight - padding, panelWidth, panelHeight);

			// Text
			ctx.fillStyle = '#00FF00';
			ctx.font = '12px monospace';

			let y = canvasHeight - panelHeight - padding + 18;
			const lineHeight = 14;

			ctx.fillText(`offset: ${offset.x}, ${offset.y}`, padding + 8, y);
			y += lineHeight;
			ctx.fillText(`scale: ${scale.toFixed(5)}`, padding + 8, y);
			y += lineHeight;
			ctx.fillText(`visibleTiles: ${visibleTiles.length}`, padding + 8, y);
			y += lineHeight;
			ctx.fillText(`pending: ${socketManager.numPending()}`, padding + 8, y);
			y += lineHeight;
			ctx.fillText(`socket: ${socketManager.state}`, padding + 8, y);
			y += lineHeight;
		};

		renderer.renderVisibleTiles(cache, visibleTiles, debugCallback);
	}

	requestAnimationFrame(loop);
}
