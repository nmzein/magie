import {
	loadedImage,
	metadata,
	annotations,
	stores,
	generators,
	selectedGenerator,
	image
} from '$stores';
import {
	PUBLIC_HTTP_SCHEME,
	PUBLIC_WS_SCHEME,
	PUBLIC_DOMAIN,
	PUBLIC_BACKEND_PORT,
	PUBLIC_ANNOTATIONS_SUBDIR,
	PUBLIC_CREATE_DIR_SUBDIR,
	PUBLIC_GENERATORS_SUBDIR,
	PUBLIC_METADATA_SUBDIR,
	PUBLIC_STORES_SUBDIR,
	PUBLIC_WEBSOCKET_SUBDIR,
	PUBLIC_UPLOAD_SUBDIR
} from '$env/static/public';

import type { AnnotationLayer, Metadata, Image } from './types';

const URL = '://' + PUBLIC_DOMAIN + ':' + PUBLIC_BACKEND_PORT;
const HTTP_URL = PUBLIC_HTTP_SCHEME + URL;
const WS_URL = PUBLIC_WS_SCHEME + URL;

export const WEBSOCKET_URL = WS_URL + PUBLIC_WEBSOCKET_SUBDIR;
const ANNOTATIONS_URL = HTTP_URL + PUBLIC_ANNOTATIONS_SUBDIR;
const CREATE_DIR_URL = HTTP_URL + PUBLIC_CREATE_DIR_SUBDIR;
const GENERATORS_URL = HTTP_URL + PUBLIC_GENERATORS_SUBDIR;
const METADATA_URL = HTTP_URL + PUBLIC_METADATA_SUBDIR;
const STORES_URL = HTTP_URL + PUBLIC_STORES_SUBDIR;
const UPLOAD_URL = HTTP_URL + PUBLIC_UPLOAD_SUBDIR;

export async function LoadImage(image: Image) {
	console.log('Loading image:', image);
	loadedImage.value = image;
	GetMetadata(image.id);
	GetAnnotations();
}

export async function CreateDirectory(
	parentDirectoryID: number | undefined,
	directoryName: string
) {
	const data = {
		parent_directory_id: parentDirectoryID,
		directory_name: directoryName
	};

	try {
		const response = await fetch(CREATE_DIR_URL, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(data)
		});

		if (response.ok) {
			GetStores();
		} else {
			console.error('Response Error <Create Directory>:', response.status, response.statusText);
		}
	} catch (error) {
		console.error('Fetch Error <Create Directory>:', error);
	}
}

export async function SendUploadAssets(
	directoryPath: string,
	imageUpload: File,
	annotationsUpload: File | undefined,
	generator: string
) {
	const formData = new FormData();

	formData.append('directory_path', directoryPath);
	formData.append('image', imageUpload);
	if (annotationsUpload) {
		formData.append('annotations', annotationsUpload);
	}
	formData.append('annotation_generator', generator);

	try {
		const response = await fetch(UPLOAD_URL, {
			method: 'POST',
			body: formData
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
				generators.value = data;
				selectedGenerator.value = generators.value?.[0];
			} catch (error) {
				console.error('Parse Error <Generators>:', error);
			}
		} else {
			console.error('Response Error <Generators>:', response.status, response.statusText);
		}
	} catch (error) {
		console.error('Fetch Error <Generators>:', error);
	}
}

export async function GetStores() {
	try {
		const response = await fetch(STORES_URL, { method: 'GET' });

		if (response.ok) {
			try {
				const data: Image[] = await response.json();
				stores.value = data;
			} catch (error) {
				console.error('Parse Error <Stores>:', error);
			}
		} else {
			console.error('Response Error <Stores>:', response.status, response.statusText);
		}
	} catch (error) {
		console.error('Fetch Error <Stores>:', error);
	}
}

export async function GetMetadata(id: number) {
	try {
		const response = await fetch(METADATA_URL, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(id)
		});

		if (response.ok) {
			try {
				const data: Metadata[] = await response.json();
				metadata.value = data;
			} catch (error) {
				console.error('Parse Error <Metadata: ' + loadedImage.value?.path + '>:', error);
			}

			image.initialise();
		} else {
			console.error(
				'Response Error <Metadata: ' + loadedImage.value?.path + '>:',
				response.status,
				response.statusText
			);
		}
	} catch (error) {
		console.error('Fetch Error <Metadata: ' + loadedImage.value?.path + '>:', error);
	}
}

export async function GetAnnotations() {
	try {
		const response = await fetch(ANNOTATIONS_URL, {
			method: 'GET'
		});

		if (response.ok) {
			try {
				const data: AnnotationLayer[] = await response.json();
				annotations.value = data;
			} catch (error) {
				console.error('Parse Error <Annotations: ' + loadedImage.value?.path + '>:', error);
			}
		} else {
			console.error(
				'Response Error <Annotations: ' + loadedImage.value?.path + '>:',
				response.status,
				response.statusText
			);
		}
	} catch (error) {
		console.error('Fetch Error <Annotations: ' + loadedImage.value?.path + '>:', error);
	}
}
