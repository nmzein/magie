import { writable, readable } from 'svelte/store';
import type { Metadata, AnnotationLayer, ImageLayer } from '$lib/types';

// TODO: Investigate leakage threat of out of component stores.

// TODO: Move to .env file.
const WEBSOCKET_URL = 'ws://127.0.0.1:3000/api/connect';

export const image = writable<ImageLayer[]>([[]]);
// TODO: Convert to Metadata array.
export const metadata = writable<Metadata | undefined>();
export const annotations = writable<AnnotationLayer[] | undefined>();

export const image_upload = writable<File | undefined>();
export const annotations_upload = writable<File | undefined>();

export const image_list = writable<string[] | undefined>();
export const annotation_generator_list = writable<string[] | undefined>();

export const websocket = readable({}, (set) => {
	const socket = new WebSocket(WEBSOCKET_URL);

	socket.addEventListener('message', async (event: MessageEvent) => {
		const data: Blob = event.data;
		const arr = new Uint8Array(await data.arrayBuffer());

		image.update((layers) => {
			// TODO: Handle multiple layers.
			if (layers[0].length === 0) {
				// TODO: Remove hardcoding of values.
				layers[0] = new Array(2).fill(0).map(() => new Array(2).fill(new Image()));
			}

			const new_tile = new Image();
			// Remove position integers from start of array.
			const blob = new Blob([arr.slice(2)], { type: 'image/jpeg' });
			new_tile.src = URL.createObjectURL(blob);

			layers[0][arr[1]][arr[0]] = new_tile;

			return layers;
		});
	});

	const send = (data: any) => {
		if (socket.readyState === WebSocket.OPEN) {
			console.log('Sending selection request.');
			socket.send(data);
			console.log('Sent selection request.');
		}
	};

	set({ send });

	return () => {
		socket.close();
	};
});
