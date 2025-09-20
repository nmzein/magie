import { C_TILE_TAG, S_TILE_TAG, S_ERROR_TAG } from '$constants';

interface TileCache {
	[key: string]: ImageBitmap;
}

interface WorkerState {
	storeId: number;
	id: number;
	layers: Array<{ rows: number; cols: number; width: number; height: number }>;
}

let offscreenCanvas: OffscreenCanvas | null = null;
let ctx: OffscreenCanvasRenderingContext2D | null = null;
let socket: WebSocket | null = null;
let tileCache: TileCache = {};
let pendingTileRequests = new Set<string>();
let workerState: WorkerState | null = null;
let reconnectAttempts = 0;
let maxReconnectAttempts = 5;

// SharedArrayBuffer-backed transform
let sharedInts: Int32Array | null = null;

// Track which tiles are currently in view
let visibleTiles: Array<{ level: number; x: number; y: number }> = [];

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
			connectWebSocket(data.wsUrl);
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
	ctx = canvas.getContext('2d');
	if (!ctx || !offscreenCanvas) return;

	offscreenCanvas.width = width;
	offscreenCanvas.height = height;
	lastWidth = width;
	lastHeight = height;
	ctx.imageSmoothingEnabled = false;
}

function connectWebSocket(wsUrl: string) {
	socket = new WebSocket(wsUrl);
	socket.binaryType = 'arraybuffer';

	socket.onopen = () => {
		reconnectAttempts = 0;
		self.postMessage({ type: 'connected' });
	};

	socket.onmessage = async (event) => {
		await handleWebSocketMessage(event);
	};

	socket.onerror = (error) => {
		self.postMessage({ type: 'error', error });
	};

	socket.onclose = () => {
		self.postMessage({ type: 'disconnected' });
		if (reconnectAttempts < maxReconnectAttempts) {
			reconnectAttempts += 1;
			setTimeout(() => connectWebSocket(wsUrl), 1000 * reconnectAttempts);
		}
	};
}

async function handleWebSocketMessage(event: MessageEvent) {
	const data = new Uint8Array(event.data);
	const dataView = new DataView(data.buffer);

	switch (dataView.getUint8(0)) {
		case S_ERROR_TAG:
			console.error('Server error received');
			self.postMessage({ type: 'error' });
			break;

		case S_TILE_TAG:
			await handleTileResponse(dataView, data);
			break;

		default:
			break;
	}
}

async function handleTileResponse(dataView: DataView, data: Uint8Array) {
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
) {
	if (!workerState) return [];

	const TS = 1024;
	const CTS = TS * transformer.scale;
	const layer = workerState.layers[0];
	if (!layer) return [];

	const startX = Math.max(0, Math.floor(-transformer.offsetX / CTS));
	const endX = Math.min(layer.cols - 1, Math.ceil((width - transformer.offsetX) / CTS));
	const startY = Math.max(0, Math.floor(-transformer.offsetY / CTS));
	const endY = Math.min(layer.rows - 1, Math.ceil((height - transformer.offsetY) / CTS));

	const visible: Array<{ level: number; x: number; y: number }> = [];
	const level = 0;

	for (let x = startX; x <= endX; x++) {
		for (let y = startY; y <= endY; y++) {
			visible.push({ level, x, y });
		}
	}

	return visible;
}

function requestTiles(tiles: Array<{ level: number; x: number; y: number }>) {
	if (!socket || socket.readyState !== WebSocket.OPEN || !workerState) return;

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

		socket.send(new Uint8Array(buffer));
	}
}

function updateWorkerState(data: Partial<WorkerState>) {
	if (workerState) {
		Object.assign(workerState, data);
	}
}

function loop() {
	if (!ctx || !workerState || !offscreenCanvas) {
		requestAnimationFrame(loop);
		return;
	}

	// Read from SharedArrayBuffer if available
	if (sharedInts) {
		const dirty = Atomics.load(sharedInts, 5);
		if (dirty === 1) {
			Atomics.store(sharedInts, 5, 0);

			const viewportWidth = sharedInts[0];
			const viewportHeight = sharedInts[1];
			const offsetX = sharedInts[2];
			const offsetY = sharedInts[3];
			const scale = sharedInts[4] / 1e6;

			lastWidth = viewportWidth;
			lastHeight = viewportHeight;
			lastTransform = { offsetX, offsetY, scale };

			offscreenCanvas.width = viewportWidth;
			offscreenCanvas.height = viewportHeight;

			visibleTiles = calculateVisibleTiles(
				{ offsetX, offsetY, scale },
				viewportWidth,
				viewportHeight
			);
			requestTiles(visibleTiles);

			renderVisibleTiles({ offsetX, offsetY, scale }, viewportWidth, viewportHeight);
		}
	}

	requestAnimationFrame(loop);
}

function renderVisibleTiles(transformer = lastTransform, width = lastWidth, height = lastHeight) {
	if (!ctx || !offscreenCanvas) return;

	ctx.setTransform(1, 0, 0, 1, 0, 0);
	ctx.clearRect(0, 0, width, height);

	const TS = 1024;
	const scale = transformer.scale;
	const offsetX = transformer.offsetX;
	const offsetY = transformer.offsetY;

	ctx.setTransform(scale, 0, 0, scale, offsetX, offsetY);

	for (const tile of visibleTiles) {
		const key = `${tile.level}_${tile.x}_${tile.y}`;
		const bmp = tileCache[key];
		if (!bmp) continue;
		ctx.drawImage(bmp, tile.x * TS, tile.y * TS, TS, TS);
	}

	ctx.setTransform(1, 0, 0, 1, 0, 0);
}

function disconnect() {
	if (socket) {
		socket.close();
		socket = null;
	}

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
