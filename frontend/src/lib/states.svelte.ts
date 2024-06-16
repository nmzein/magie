import type {
	MetadataLayer,
	AnnotationLayer,
	ImageLayer,
	Image,
	TileRequest,
	Directory
} from '$types';
import { WEBSOCKET_URL } from '$api';

export function state<T>(initial: T): { value: T };
export function state<T = undefined>(initial?: T): { value: T };
export function state<T>(initial?: T) {
	let state = $state({ value: initial });
	return state;
}

export const loadedImage = state<Image | undefined>();
export const registry = state<Directory | undefined>();
export const generators = state<string[] | undefined>();
export const selectedGenerator = state<string | undefined>();
export const metadata = state<MetadataLayer[] | undefined>();
export const annotations = state<AnnotationLayer[] | undefined>();
export const imageUpload = state<File | undefined>();
export const annotationsUpload = state<File | undefined>();

export const fileExplorerState = state<{ activeDirectoryId: string }>({
	activeDirectoryId: ''
});
export const autogenerateAnnotations = state<boolean>(false);

export const image = (() => {
	let image = state<ImageLayer[]>();

	// Run as soon as metadata is parsed and loaded in GetMetadata.
	const init = () => {
		const levels = metadata.value?.length;
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
	let socket = state<WebSocket>(new WebSocket(WEBSOCKET_URL));

	socket.value.addEventListener('message', (event: MessageEvent) => {
		processTile(event).catch((error) => {
			console.error('Tile Processing Error:', error);
		});
	});

	async function processTile(event: MessageEvent): Promise<void> {
		if (image.state.value === undefined) return;

		const data: Blob = event.data;
		const arr = new Uint8Array(await data.arrayBuffer());

		const level = arr[0];
		const x = arr[1];
		const y = arr[2];

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
