import { writable, readable } from 'svelte/store';
import type { Metadata, AnnotationLayer } from '$lib/types';

// TODO: Investigate leakage threat of out of component stores.

// TODO: Move to .env file.
const WEBSOCKET_URL = 'ws://127.0.0.1:3000/api/connect';

export const image = writable<HTMLImageElement[]>([]);
export const metadata = writable<Metadata | undefined>();
export const annotations = writable<AnnotationLayer[] | undefined>();

export const image_upload = writable<File | undefined>();
export const annotations_upload = writable<File | undefined>();

export const image_list = writable<string[] | undefined>();
export const annotation_generator_list = writable<string[] | undefined>();

export const websocket = readable({}, (set) => {
	const socket = new WebSocket(WEBSOCKET_URL);

	// TODO: Grid system instead of buffer.
	socket.addEventListener('message', (event: MessageEvent) => {
		// Assuming each message is an image binary data.
		// Read binary data from the Blob.
		const image_data: Blob = event.data;

		// Convert Blob to HTMLImageElement.
		const new_tiles = new Image();
		new_tiles.src = URL.createObjectURL(image_data);

		// Update the images store.
		image.update((old_tiles) => [...old_tiles, new_tiles]);
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
