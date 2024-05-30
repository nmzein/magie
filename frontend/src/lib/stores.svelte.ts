import type { MetadataLayer, AnnotationLayer, ImageLayer, Image, TileRequest } from '$types';
import { WEBSOCKET_URL } from '$api';

const state = <T>(initial: T | undefined = undefined) => {
	let state = $state({ value: initial });
	return state;
};

const definedState = <T>(initial: T) => {
	let state = $state({ value: initial });
	return state;
};

export const loadedImage = state<Image>();
export const stores = state<Image[]>();
export const generators = state<string[]>();
export const selectedGenerator = state<string>();
export const metadata = state<MetadataLayer[]>();
export const annotations = state<AnnotationLayer[]>();
export const imageUpload = state<File>();
export const annotationsUpload = state<File>();

export const autogenerateAnnotations = definedState<boolean>(true);

export const image = (() => {
	let image = state<ImageLayer[]>();

	// Run as soon as metadata is parsed and loaded in GetMetadata.
	const init = () => {
		let levels = metadata.value?.length;
		if (image === undefined || levels === undefined) return;

		image.value = new Array(levels).fill([]);

		for (let level = 0; level < levels; level++) {
			image.value[level] = new Array(metadata.value?.[level].rows)
				.fill(0)
				.map(() => new Array(metadata.value?.[level].cols).fill(new Image()));
		}
	};

	return { state: image, init };
})();

const _websocket = () => {
	let socket = definedState<WebSocket>(new WebSocket(WEBSOCKET_URL));

	socket.value.addEventListener('message', (event: MessageEvent) => {
		processTile(event).catch((error) => {
			console.error('Tile Processing Error:', error);
		});
	});

	async function processTile(event: MessageEvent): Promise<void> {
		if (image.state.value === undefined) return;

		const data: Blob = event.data;
		const arr = new Uint8Array(await data.arrayBuffer());

		let level = arr[0];
		let x = arr[1];
		let y = arr[2];

		const newTile = new Image();
		// Remove position and level values from start of array.
		const blob = new Blob([arr.slice(3)], { type: 'image/jpeg' });
		newTile.src = URL.createObjectURL(blob);
		image.state.value[level][y][x] = newTile;
	}

	async function GetTile(data: TileRequest): Promise<boolean> {
		if (metadata === undefined || socket.value.readyState !== WebSocket.OPEN) {
			return false;
		}

		socket.value.send(JSON.stringify(data));
		return true;
	}

	return { socket, GetTile };
};

type WebSocketState = {
	socket: { value: WebSocket };
	GetTile: (data: TileRequest) => Promise<boolean>;
};

export let websocket: WebSocketState;

export async function ConnectWebSocket() {
	websocket = _websocket();
}
