import { metadataURL, processURL } from '$lib/urls';
import { metadataStore } from '$lib/stores';
import type { ImageMetadata } from './types';

export async function sendProcessRequest() {
	try {
		const response = await fetch(processURL, { method: 'POST' });

		if (response.ok) {
			sendMetadataRequest();
		} else {
			console.error(
				'Error: Process API call returned bad status code: ',
				response.status,
				response.statusText
			);
		}
	} catch (error) {
		console.error('Error during Process API call: ', error);
	}
}

export async function sendMetadataRequest() {
	try {
		const response = await fetch(metadataURL, { method: 'POST' });

		if (response.ok) {
			try {
				const data: ImageMetadata = await response.json();
				metadataStore.set(data);
			} catch (error) {
				console.error("Error: Couldn't parse metadata: ", error);
			}
		} else {
			console.error(
				'Error: Metadata API call returned bad status code: ',
				response.status,
				response.statusText
			);
		}
	} catch (error) {
		console.error('Error during Metadata API call: ', error);
	}
}
