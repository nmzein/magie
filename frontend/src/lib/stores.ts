import { writable, readable } from 'svelte/store';
import type { Metadata, AnnotationLayer, ImageLayer } from '$lib/types';

// TODO: Investigate leakage threat of out of component stores.

// TODO: Move to .env file.
const WEBSOCKET_URL = 'ws://127.0.0.1:3000/api/connect';

export const image = writable<ImageLayer[]>([[]]);
export const metadata = writable<Metadata[] | undefined>();
export const annotations = writable<AnnotationLayer[] | undefined>();

export const image_upload = writable<File | undefined>();
export const annotations_upload = writable<File | undefined>();

export const image_list = writable<string[] | undefined>();
export const annotation_generator_list = writable<string[] | undefined>();

export const websocket = readable({}, (set) => {
	const socket = new WebSocket(WEBSOCKET_URL);

	let local_metadata: Metadata[] | undefined;
	metadata.subscribe((value) => (local_metadata = value));

	socket.addEventListener('message', (event: MessageEvent) => {
		processTile(event).catch((error) => {
			console.error('Error processing tile:', error);
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
			// Initialize with correct number of rows if not already done.
			if (layers.length !== local_metadata?.length) {
				layers = new Array(local_metadata?.length).fill([]);
			}

			// Initialize with correct number of cols for specific level.
			if (layers[level].length === 0) {
				layers[level] = new Array(local_metadata?.[level].rows)
					.fill(0)
					.map(() => new Array(local_metadata?.[level].cols).fill(new Image()));
			}

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
