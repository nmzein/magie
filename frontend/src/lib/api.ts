import {
	image_name,
	metadata,
	annotations,
	image_list,
	annotation_generator_list,
	websocket,
	InitImageLayers
} from '$lib/stores';
import {
	PUBLIC_HTTP_SCHEME,
	PUBLIC_DOMAIN,
	PUBLIC_BACKEND_PORT,
	PUBLIC_METADATA_SUBDIR,
	PUBLIC_ANNOTATIONS_SUBDIR,
	PUBLIC_UPLOAD_SUBDIR,
	PUBLIC_STORES_SUBDIR,
	PUBLIC_GENERATORS_SUBDIR
} from '$env/static/public';

import type { AnnotationLayer, Metadata, TileRequest } from './types';

const URL = PUBLIC_HTTP_SCHEME + '://' + PUBLIC_DOMAIN + ':' + PUBLIC_BACKEND_PORT;
const METADATA_URL = URL + PUBLIC_METADATA_SUBDIR;
const ANNOTATIONS_URL = URL + PUBLIC_ANNOTATIONS_SUBDIR;
const UPLOAD_URL = URL + PUBLIC_UPLOAD_SUBDIR;
const STORES_URL = URL + PUBLIC_STORES_SUBDIR;
const GENERATORS_URL = URL + PUBLIC_GENERATORS_SUBDIR;

let socket: WebSocket;

export async function ConnectWebSocket() {
	websocket.subscribe((value) => {
		socket = value as WebSocket;
	});
}

export async function LoadImage(imageName: string) {
	image_name.set(imageName);
	GetMetadata(imageName);
	GetAnnotations(imageName);
}

export async function GetTile(tile: TileRequest): Promise<boolean> {
	if (!metadata) {
		return false;
	}
	socket.send(JSON.stringify(tile));
	return true;
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
			GetStores();
		} else {
			console.error('Response Error <Upload>:', response.status, response.statusText);
		}
	} catch (error) {
		console.error('Fetch Error <Upload>:', error);
	}
}

export async function GetGenerators() {
	try {
		const response = await fetch(GENERATORS_URL, { method: 'GET' });

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

export async function GetStores() {
	try {
		const response = await fetch(STORES_URL, { method: 'GET' });

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
			InitImageLayers();
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
