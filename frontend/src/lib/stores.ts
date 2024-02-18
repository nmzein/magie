import { writable, readable } from 'svelte/store';
import type { Metadata, AnnotationLayer, ImageLayer } from '$lib/types';
import {
	PUBLIC_WS_SCHEME,
	PUBLIC_DOMAIN,
	PUBLIC_BACKEND_PORT,
	PUBLIC_WEBSOCKET_SUBDIR
} from '$env/static/public';

// TODO: Investigate leakage threat of out of component stores.

const WEBSOCKET_URL =
	PUBLIC_WS_SCHEME + '://' + PUBLIC_DOMAIN + ':' + PUBLIC_BACKEND_PORT + PUBLIC_WEBSOCKET_SUBDIR;

export const image_name = writable<string | undefined>();
export const image = writable<ImageLayer[]>([[]]);

export const metadata = writable<Metadata[] | undefined>();
export const annotations = writable<AnnotationLayer[] | undefined>();

export const image_upload = writable<File | undefined>();
export const annotations_upload = writable<File | undefined>();

export const image_list = writable<string[] | undefined>();
export const annotation_generator_list = writable<string[] | undefined>();

let local_metadata: Metadata[] | undefined;
metadata.subscribe((value) => (local_metadata = value));

export function InitImageLayers() {
	let layers = new Array(local_metadata?.length).fill([]);
	for (let level = 0; level < layers.length; level++) {
		layers[level] = new Array(local_metadata?.[level].rows)
			.fill(0)
			.map(() => new Array(local_metadata?.[level].cols).fill(new Image()));
	}

	image.set(layers);
}

export const websocket = readable({}, (set) => {
	const socket = new WebSocket(WEBSOCKET_URL);

	socket.addEventListener('message', (event: MessageEvent) => {
		processTile(event).catch((error) => {
			console.error('Tile Processing Error:', error);
		});
	});

	async function processTile(event: MessageEvent): Promise<void> {
		const data: Blob = event.data;
		const arr = new Uint8Array(await data.arrayBuffer());

		let level = arr[0];
		let x = arr[1];
		let y = arr[2];

		// Wait for metadata to be set before updating image array.
		while (local_metadata === undefined) {
			await new Promise((resolve) => setTimeout(resolve, 100));
		}

		image.update((layers) => {
			const newTile = new Image();

			// Remove position and level values from start of array.
			const blob = new Blob([arr.slice(3)], { type: 'image/jpeg' });
			newTile.src = URL.createObjectURL(blob);
			layers[level][y][x] = newTile;

			return layers;
		});
	}

	const send = (data: string) => {
		if (socket.readyState === WebSocket.OPEN) {
			socket.send(data);
		}
	};

	set({ send });

	return () => {
		socket.close();
	};
});
