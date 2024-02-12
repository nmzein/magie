import {
	metadata,
	annotations,
	image_list,
	annotation_generator_list,
	websocket
} from '$lib/stores';

import type { AnnotationLayer, Metadata, Selection } from './types';

// TODO: Move to .env file.
const METADATA_URL = 'http://127.0.0.1:3000/api/metadata';
const ANNOTATIONS_URL = 'http://127.0.0.1:3000/api/annotations';
const UPLOAD_URL = 'http://127.0.0.1:3000/api/upload';
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
		const response = await fetch(UPLOAD_URL, {
			method: 'POST',
			body: form_data
		});

		if (response.ok) {
			GetImageList();
		} else {
			console.error('Response Error <Upload>:', response.status, response.statusText);
		}
	} catch (error) {
		console.error('Fetch Error <Upload>:', error);
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
				console.error('Parse Error <Annotation Generators>:', error);
			}
		} else {
			console.error(
				'Response Error <Annotation Generators>:',
				response.status,
				response.statusText
			);
		}
	} catch (error) {
		console.error('Fetch Error <Annotation Generators>:', error);
	}
}

export async function GetImageList() {
	try {
		const response = await fetch(IMAGE_LIST_URL, { method: 'GET' });

		if (response.ok) {
			try {
				const data: string[] = await response.json();
				image_list.set(data);
			} catch (error) {
				console.error('Parse Error <Image List>:', error);
			}
		} else {
			console.error('Response Error <Image List>:', response.status, response.statusText);
		}
	} catch (error) {
		console.error('Fetch Error <Image List>:', error);
	}
}

export async function GetMetadata(imageName: string) {
	try {
		const response = await fetch(METADATA_URL, {
			method: 'POST',
			body: imageName
		});

		if (response.ok) {
			try {
				const data: Metadata[] = await response.json();
				metadata.set(data);
			} catch (error) {
				console.error('Parse Error <Metadata: ' + imageName + '>:', error);
			}
		} else {
			console.error(
				'Response Error <Metadata: ' + imageName + '>:',
				response.status,
				response.statusText
			);
		}
	} catch (error) {
		console.error('Fetch Error <Metadata: ' + imageName + '>:', error);
	}
}

export async function GetAnnotations(imageName: string) {
	try {
		const response = await fetch(ANNOTATIONS_URL, {
			method: 'POST',
			body: imageName
		});

		if (response.ok) {
			try {
				const data: AnnotationLayer[] = await response.json();
				annotations.set(data);
			} catch (error) {
				console.error('Parse Error <Annotations: ' + imageName + '>:', error);
			}
		} else {
			console.error(
				'Response Error <Annotations: ' + imageName + '>:',
				response.status,
				response.statusText
			);
		}
	} catch (error) {
		console.error('Fetch Error <Annotations: ' + imageName + '>:', error);
	}
}
