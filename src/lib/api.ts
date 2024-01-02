import {
	metadata,
	annotations,
	image_list,
	annotation_generator_list,
	websocket
} from '$lib/stores';

import type { AnnotationLayer, Metadata, Selection } from './types';

// TODO: Better error messages.

// TODO: Move to .env file.
const METADATA_URL = 'http://127.0.0.1:3000/api/metadata';
const ANNOTATIONS_URL = 'http://127.0.0.1:3000/api/annotations';
const UPLOAD_IMAGE_URL = 'http://127.0.0.1:3000/api/upload';
const IMAGE_LIST_URL = 'http://127.0.0.1:3000/api/image-list';
const ANNOTATION_GENERATORS_URL = 'http://127.0.0.1:3000/api/annotation-generators';

let socket: WebSocket;

export async function ConnectWebSocket() {
	websocket.subscribe((value) => {
		socket = value as WebSocket;
	});
}

export async function GetImageSelection(selection: Selection) {
	socket.send(JSON.stringify(selection));
}

export async function SendUploadAssets(
	image_file: File,
	annotation_file: File | undefined,
	annotation_generator: string
) {
	const form_data = new FormData();

	form_data.append('image', image_file);
	if (annotation_file) {
		form_data.append('annotations', annotation_file);
	}
	form_data.append('annotation_generator', annotation_generator);

	try {
		console.log('Sending image to server for processing {}.', form_data);
		const response = await fetch(UPLOAD_IMAGE_URL, {
			method: 'POST',
			body: form_data
		});

		if (response.ok) {
			console.log('Success: Sent image to server for processing.');
			console.log(response);
		} else {
			console.error(
				'Error: Could not send image to the server: ',
				response.status,
				response.statusText
			);
		}
	} catch (error) {
		console.error('Error: Could not reach server: ', error);
	}
}

export async function GetAnnotationGenerators() {
	try {
		const response = await fetch(ANNOTATION_GENERATORS_URL, { method: 'GET' });

		if (response.ok) {
			try {
				const data: string[] = await response.json();
				annotation_generator_list.set(data);
			} catch (error) {
				console.error("Error: Couldn't parse list of image names: ", error);
			}
		} else {
			console.error(
				'Error: List API call returned bad status code: ',
				response.status,
				response.statusText
			);
		}
	} catch (error) {
		console.error('Error during List API call: ', error);
	}
}

export async function GetImagesList() {
	try {
		const response = await fetch(IMAGE_LIST_URL, { method: 'GET' });

		if (response.ok) {
			try {
				const data: string[] = await response.json();
				image_list.set(data);
			} catch (error) {
				console.error("Error: Couldn't parse list of image names: ", error);
			}
		} else {
			console.error(
				'Error: List API call returned bad status code: ',
				response.status,
				response.statusText
			);
		}
	} catch (error) {
		console.error('Error during List API call: ', error);
	}
}

export async function GetMetadata(image_name: string) {
	try {
		const response = await fetch(METADATA_URL, {
			method: 'POST',
			body: image_name
		});

		if (response.ok) {
			try {
				const data: Metadata = await response.json();
				metadata.set(data);
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

export async function GetAnnotations(image_name: string) {
	try {
		const response = await fetch(ANNOTATIONS_URL, {
			method: 'POST',
			body: image_name
		});

		if (response.ok) {
			try {
				const data: AnnotationLayer[] = await response.json();
				annotations.set(data);
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
