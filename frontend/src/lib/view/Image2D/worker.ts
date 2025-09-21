import { C_TILE_TAG, S_TILE_TAG } from '$constants';
import { WebSocketManager } from '$lib/helpers/network';

type TileCache = {
	[key: string]: ImageBitmap;
};

type WorkerState = {
	storeId: number;
	id: number;
	layers: Array<{ rows: number; cols: number; width: number; height: number }>;
};

type TileIdentifier = { level: number; x: number; y: number };

const TILE_SIZE = 1024;

let offscreenCanvas: OffscreenCanvas | null = null;
let ctx: OffscreenCanvasRenderingContext2D | null = null;
let socketManager: WebSocketManager | null = null;
let tileCache: TileCache = {};
let pendingTileRequests = new Set<string>();
let workerState: WorkerState | null = null;
let reconnectAttempts = 0;
let maxReconnectAttempts = 5;

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
			workerState = {
				storeId: data.storeId,
				id: data.id,
				layers: JSON.parse(data.layers)
			};
			if (data.sharedBuf) sharedInts = new Int32Array(data.sharedBuf);
			connect(data.wsUrl);
			requestAnimationFrame(loop);
			break;
		case 'updateState':
			updateWorkerState(data);
			break;
		case 'disconnect':
			disconnect();
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
	const data = new Uint8Array(event.data);
	const dataView = new DataView(data.buffer);
	if (dataView.getUint8(0) !== S_TILE_TAG) return;

	const level = dataView.getUint32(9);
	const x = dataView.getUint32(13);
	const y = dataView.getUint32(17);
	const tileData = data.slice(29);

	if (!workerState) return;

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
	if (!workerState) return [];

	const CTS = TILE_SIZE * transformer.scale;
	const layer = workerState.layers[0];
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
	if (socketManager?.state !== 'connected' || !workerState) return;

	for (const tile of tiles) {
		const tileKey = `${tile.level}_${tile.x}_${tile.y}`;
		if (tileCache[tileKey] || pendingTileRequests.has(tileKey)) continue;

		const layer = workerState.layers[tile.level];
		if (!layer || tile.x >= layer.cols || tile.y >= layer.rows) continue;

		pendingTileRequests.add(tileKey);

		const buffer = new ArrayBuffer(1 + 5 * 4);
		const view = new DataView(buffer);
		view.setUint8(0, C_TILE_TAG);
		view.setUint32(1, workerState.storeId);
		view.setUint32(5, workerState.id);
		view.setUint32(9, tile.level);
		view.setUint32(13, tile.x);
		view.setUint32(17, tile.y);

		socketManager.send(new Uint8Array(buffer));
	}
}

function updateWorkerState(data: Partial<WorkerState>) {
	if (workerState) {
		Object.assign(workerState, data);
	}
}

function loop() {
	if (!ctx || !workerState || !offscreenCanvas || !sharedInts) {
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
	if (!ctx || !offscreenCanvas) return;

	ctx.setTransform(1, 0, 0, 1, 0, 0);
	ctx.clearRect(0, 0, width, height);

	const scale = transformer.scale;
	const offsetX = transformer.offsetX;
	const offsetY = transformer.offsetY;

	ctx.setTransform(scale, 0, 0, scale, offsetX, offsetY);

	for (const tile of visibleTiles) {
		const key = `${tile.level}_${tile.x}_${tile.y}`;
		const bmp = tileCache[key];
		if (!bmp) continue;
		ctx.drawImage(bmp, tile.x * TILE_SIZE, tile.y * TILE_SIZE, TILE_SIZE, TILE_SIZE);
	}

	ctx.setTransform(1, 0, 0, 1, 0, 0);
}

function disconnect() {
	socketManager?.disconnect();
	socketManager = null;

	for (const bmp of Object.values(tileCache)) {
		try {
			bmp.close();
		} catch {}
	}

	tileCache = {};
	pendingTileRequests.clear();
	workerState = null;
	offscreenCanvas = null;
	ctx = null;
}
