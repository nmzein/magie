import { writable, readable } from 'svelte/store';
import type { ImageMetadata } from '$lib/types';
import { WebSocketURL } from '$lib/urls';

export const ImageStore = writable<HTMLImageElement[]>([]);
export const MetadataStore = writable<ImageMetadata | undefined>();

export const ImageUploadStore = writable<File | undefined>();
export const AnnotationFileUploadStore = writable<File | undefined>();

export const AnnotationGeneratorStore = writable<string>('');
export const AutogenerateAnnotationsStore = writable<boolean>(true);

export const WebSocketStore = readable({}, (set) => {
	const socket = new WebSocket(WebSocketURL);

	socket.addEventListener('message', (event: MessageEvent) => {
		// Assuming each message is an image binary data.
		// Read binary data from the Blob.
		const imageData: Blob = event.data;

		// Convert Blob to HTMLImageElement.
		const image = new Image();
		image.src = URL.createObjectURL(imageData);

		// Update the images store.
		ImageStore.update((images) => [...images, image]);
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
