import { AssetClientMsgTag } from '$types';
import { BinaryReader, BinaryWriter } from '$lib/helpers/codec';
import { WebSocketManager } from '$lib/helpers/network';
import type { Asset } from '$lib/states/viewer-manager.svelte';

type TileCache = {
	[key: string]: ImageBitmap;
};

type TileIdentifier = { level: number; x: number; y: number };

const TILE_SIZE = 1024;

let offscreenCanvas: OffscreenCanvas | null = null;
let ctx: OffscreenCanvasRenderingContext2D | null = null;
let socketManager: WebSocketManager | null = null;
let tileCache: TileCache = {};
let pendingTileRequests = new Set<string>();
let asset: Asset | null = null;

// SharedArrayBuffer-backed transform
let sharedInts: Int32Array | null = null;

// Track which tiles are currently in view
let visibleTiles: TileIdentifier[] = [];

// Track last transform/size
let lastWidth = 0;
let lastHeight = 0;
let lastTransform = { offsetX: 0, offsetY: 0, scale: 1 };

self.onmessage = function (e) {
	const { type, data } = e.data;

	switch (type) {
		case 'init':
			initCanvas(data.canvas, data.width, data.height);
			asset = JSON.parse(data.asset);
			if (data.sharedBuf) sharedInts = new Int32Array(data.sharedBuf);
			connect(data.wsUrl);
			requestAnimationFrame(loop);
			break;
		case 'close':
			close();
			break;
	}
};

function initCanvas(canvas: OffscreenCanvas, width: number, height: number) {
	offscreenCanvas = canvas;
	ctx = canvas.getContext('2d', { alpha: false });
	if (!ctx || !offscreenCanvas) return;

	offscreenCanvas.width = width;
	offscreenCanvas.height = height;
	lastWidth = width;
	lastHeight = height;
	ctx.imageSmoothingEnabled = false; // TODO: Look into this option.
}

function connect(wsUrl: string) {
	socketManager = new WebSocketManager({
		url: wsUrl,
		maxReconnectAttempts: 8,
		minDelay: 500, // start at 0.5s
		maxDelay: 20000, // cap at 20s
		factor: 2, // exponential growth
		onOpen: () => {
			self.postMessage({ type: 'connected' });
		},
		onMessage: async (event) => {
			await handleTile(event);
		},
		onError: (error) => {
			self.postMessage({ type: 'error', error });
		},
		onClose: (_, willReconnect) => {
			self.postMessage({ type: 'disconnected' });
		}
	});

	socketManager.connect();
}

async function handleTile(event: MessageEvent) {
	const r = new BinaryReader(event.data);
	const _tag = r.u8();
	const level = r.u32();
	const x = r.u32();
	const y = r.u32();
	const tileData = r.bytes();

	if (!asset) return;

	const tileKey = `${level}_${x}_${y}`;
	try {
		const blob = new Blob([tileData], { type: 'image/jpeg' });
		const imageBitmap = await createImageBitmap(blob);

		tileCache[tileKey] = imageBitmap;
		pendingTileRequests.delete(tileKey);

		renderVisibleTiles();
	} catch (error) {
		console.error('Error processing tile:', error);
		pendingTileRequests.delete(tileKey);
	}
}

function calculateVisibleTiles(
	transformer: { offsetX: number; offsetY: number; scale: number },
	width: number,
	height: number
): TileIdentifier[] {
	if (!asset) return [];

	const CTS = TILE_SIZE * transformer.scale;
	const layer = asset.layers[0];
	if (!layer) return [];

	const startX = Math.max(0, Math.floor(-transformer.offsetX / CTS));
	const endX = Math.min(layer.cols - 1, Math.ceil((width - transformer.offsetX) / CTS));
	const startY = Math.max(0, Math.floor(-transformer.offsetY / CTS));
	const endY = Math.min(layer.rows - 1, Math.ceil((height - transformer.offsetY) / CTS));

	const visible: TileIdentifier[] = [];
	const level = 0;

	for (let x = startX; x <= endX; x++) {
		for (let y = startY; y <= endY; y++) {
			visible.push({ level, x, y });
		}
	}

	return visible;
}

function requestTiles(tiles: TileIdentifier[]) {
	if (socketManager?.state !== 'connected' || !asset) return;

	for (const tile of tiles) {
		const tileKey = `${tile.level}_${tile.x}_${tile.y}`;
		if (tileCache[tileKey] || pendingTileRequests.has(tileKey)) continue;

		const layer = asset.layers[tile.level];
		if (!layer || tile.x >= layer.cols || tile.y >= layer.rows) continue;

		pendingTileRequests.add(tileKey);

		const w = new BinaryWriter(1 + 3 * 4);
		w.u8(AssetClientMsgTag.Tile);
		w.u32(tile.level);
		w.u32(tile.x);
		w.u32(tile.y);

		socketManager.send(w.finish());
	}
}

function loop() {
	if (!ctx || !asset || !offscreenCanvas || !sharedInts) {
		requestAnimationFrame(loop);
		return;
	}

	const dirty = Atomics.load(sharedInts, 5);
	if (dirty === 1) {
		Atomics.store(sharedInts, 5, 0);

		const canvasWidth = sharedInts[0];
		const canvasHeight = sharedInts[1];
		const offsetX = sharedInts[2];
		const offsetY = sharedInts[3];
		const scale = sharedInts[4] / 1e6;

		lastWidth = canvasWidth;
		lastHeight = canvasHeight;
		lastTransform = { offsetX, offsetY, scale };

		offscreenCanvas.width = canvasWidth;
		offscreenCanvas.height = canvasHeight;

		visibleTiles = calculateVisibleTiles({ offsetX, offsetY, scale }, canvasWidth, canvasHeight);

		requestTiles(visibleTiles);
		renderVisibleTiles({ offsetX, offsetY, scale }, canvasWidth, canvasHeight);
	}

	requestAnimationFrame(loop);
}

function renderVisibleTiles(transformer = lastTransform, width = lastWidth, height = lastHeight) {
	if (!ctx || !offscreenCanvas || !asset) return;

	const scale = transformer.scale;
	const offsetX = transformer.offsetX;
	const offsetY = transformer.offsetY;

	// Reset and clear
	ctx.setTransform(1, 0, 0, 1, 0, 0);
	ctx.clearRect(0, 0, width, height);

	// Draw tiles
	ctx.setTransform(scale, 0, 0, scale, offsetX, offsetY);
	for (const tile of visibleTiles) {
		const key = `${tile.level}_${tile.x}_${tile.y}`;
		const bmp = tileCache[key];
		if (!bmp) continue;
		ctx.drawImage(bmp, tile.x * TILE_SIZE, tile.y * TILE_SIZE, TILE_SIZE, TILE_SIZE);
	}

	// ---- Debug overlay (bottom-left) ----
	ctx.setTransform(1, 0, 0, 1, 0, 0); // Reset transforms for UI
	ctx.save();

	const panelWidth = 230;
	const panelHeight = 120;
	const padding = 10;

	// Background box
	ctx.fillStyle = 'rgba(0,0,0,0.6)';
	ctx.fillRect(padding, height - panelHeight - padding, panelWidth, panelHeight);

	// Text
	ctx.fillStyle = '#00FF00';
	ctx.font = '12px monospace';

	let y = height - panelHeight - padding + 18;
	const lineHeight = 14;

	ctx.fillText(`offsetX: ${offsetX}`, padding + 8, y);
	y += lineHeight;
	ctx.fillText(`offsetY: ${offsetY}`, padding + 8, y);
	y += lineHeight;
	ctx.fillText(`scale: ${scale.toFixed(5)}`, padding + 8, y);
	y += lineHeight;
	ctx.fillText(`visibleTiles: ${visibleTiles.length}`, padding + 8, y);
	y += lineHeight;
	ctx.fillText(`pending: ${pendingTileRequests.size}`, padding + 8, y);
	y += lineHeight;
	ctx.fillText(`socket: ${socketManager?.state}`, padding + 8, y);
	y += lineHeight;

	ctx.restore();
}

function close() {
	socketManager?.disconnect();
	socketManager = null;

	for (const bmp of Object.values(tileCache)) {
		try {
			bmp.close();
		} catch {}
	}

	tileCache = {};
	pendingTileRequests.clear();
	asset = null;
	offscreenCanvas = null;
	ctx = null;
}
