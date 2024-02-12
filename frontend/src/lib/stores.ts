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

	metadata.subscribe((value) => {
		local_metadata = value;
	});

	const descriptions: string[] = [
		'Read Event Data into Blob',
		'Read Blob into Uint8Array',
		'Initialise Array',
		'Create Image Object',
		'Create Blob from Uint8Array',
		'Update Layers'
	];

	async function processTile(event: MessageEvent): Promise<void> {
		// Arrays to store timing information and descriptions
		// const timings: number[] = [];

		// let time = performance.now();
		const data: Blob = event.data;
		// timings.push(performance.now() - time);

		// time = performance.now();
		const arr = new Uint8Array(await data.arrayBuffer());
		// timings.push(performance.now() - time);

		let level = arr[0];
		let x = arr[1];
		let y = arr[2];

		// Process the message and update the state
		image.update((layers) => {
			// time = performance.now();
			// Initialize image array rows if not already done.
			if (layers.length !== local_metadata?.length) {
				layers = new Array(local_metadata?.length).fill([]);
			}

			// Initialize image array col for current level if not already done.
			if (layers[level].length === 0) {
				layers[level] = new Array(local_metadata?.[level].rows)
					.fill(0)
					.map(() => new Array(local_metadata?.[level].cols).fill(new Image()));
			}
			// timings.push(performance.now() - time);

			// time = performance.now();
			const new_tile = new Image();
			// timings.push(performance.now() - time);

			// Remove position and level values from start of array.
			// time = performance.now();
			const blob = new Blob([arr.slice(3)], { type: 'image/jpeg' });
			// timings.push(performance.now() - time);

			// time = performance.now();
			new_tile.src = URL.createObjectURL(blob);
			layers[level][y][x] = new_tile;
			// timings.push(performance.now() - time);

			return layers;
		});

		// for (let i = 0; i < timings.length; i++) {
		// 	console.log(`${level}:(${x}, ${y}) ${descriptions[i]}: ${timings[i]} ms`);
		// }
	}

	socket.addEventListener('message', (event: MessageEvent) => {
		processTile(event).catch((error) => {
			console.error('Error processing tile:', error);
		});
	});

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
