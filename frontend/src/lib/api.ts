import {
	loadedImage,
	metadata,
	annotations,
	registry,
	generators,
	selectedGenerator,
	image
} from '$stores';
import {
	PUBLIC_HTTP_SCHEME,
	PUBLIC_WS_SCHEME,
	PUBLIC_DOMAIN,
	PUBLIC_BACKEND_PORT,
	// Directory routes.
	PUBLIC_DIRECTORY_CREATE_SUBDIR,
	PUBLIC_DIRECTORY_DELETE_SUBDIR,
	PUBLIC_DIRECTORY_RENAME_SUBDIR,
	PUBLIC_DIRECTORY_MOVE_SUBDIR,
	// Image routes.
	PUBLIC_IMAGE_UPLOAD_SUBDIR,
	PUBLIC_IMAGE_DELETE_SUBDIR,
	PUBLIC_IMAGE_METADATA_SUBDIR,
	PUBLIC_IMAGE_ANNOTATIONS_SUBDIR,
	PUBLIC_IMAGE_TILES_SUBDIR,
	// General routes.
	PUBLIC_REGISTRY_SUBDIR,
	PUBLIC_GENERATORS_SUBDIR
} from '$env/static/public';

import type { AnnotationLayer, MetadataLayer, Image, Directory } from './types';

const URL = '://' + PUBLIC_DOMAIN + ':' + PUBLIC_BACKEND_PORT;
const HTTP_URL = PUBLIC_HTTP_SCHEME + URL;
const WS_URL = PUBLIC_WS_SCHEME + URL;

// Directory routes.
const DIRECTORY_CREATE_URL = HTTP_URL + PUBLIC_DIRECTORY_CREATE_SUBDIR;
const DIRECTORY_DELETE_URL = HTTP_URL + PUBLIC_DIRECTORY_DELETE_SUBDIR;
const DIRECTORY_RENAME_URL = HTTP_URL + PUBLIC_DIRECTORY_RENAME_SUBDIR;
const DIRECTORY_MOVE_URL = HTTP_URL + PUBLIC_DIRECTORY_MOVE_SUBDIR;

// Image routes.
const IMAGE_UPLOAD_URL = HTTP_URL + PUBLIC_IMAGE_UPLOAD_SUBDIR;
const IMAGE_DELETE_URL = HTTP_URL + PUBLIC_IMAGE_DELETE_SUBDIR;
const IMAGE_METADATA_URL = HTTP_URL + PUBLIC_IMAGE_METADATA_SUBDIR;
const IMAGE_ANNOTATIONS_URL = HTTP_URL + PUBLIC_IMAGE_ANNOTATIONS_SUBDIR;
export const WEBSOCKET_URL = WS_URL + PUBLIC_IMAGE_TILES_SUBDIR;

// General routes.
const REGISTRY_URL = HTTP_URL + PUBLIC_REGISTRY_SUBDIR;
const GENERATORS_URL = HTTP_URL + PUBLIC_GENERATORS_SUBDIR;

export async function LoadImage(image: Image) {
	loadedImage.value = image;
	GetMetadata(image.id);
}

export async function CreateDirectory(parentDirectoryID: number | undefined, name: string) {
	const data = {
		parent_id: parentDirectoryID,
		name: name
	};

	try {
		const response = await fetch(DIRECTORY_CREATE_URL, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(data)
		});

		if (response.ok) {
			GetRegistry();
		} else {
			console.error('Response Error <Create Directory>:', response.status, response.statusText);
		}
	} catch (error) {
		console.error('Fetch Error <Create Directory>:', error);
	}
}

export async function SendUploadAssets(
	parentDirectoryID: number,
	imageFile: File,
	annotationsFile: File | undefined,
	generatorName: string
) {
	const formData = new FormData();

	formData.append('parent_directory_id', parentDirectoryID.toString());
	formData.append('image_file', imageFile);
	if (annotationsFile) {
		formData.append('annotations_file', annotationsFile);
	}
	formData.append('generator_name', generatorName);

	try {
		const response = await fetch(IMAGE_UPLOAD_URL, {
			method: 'POST',
			body: formData
		});

		if (response.ok) {
			GetRegistry();
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

export async function GetRegistry() {
	try {
		const response = await fetch(REGISTRY_URL, { method: 'GET' });

		if (response.ok) {
			try {
				const data: Directory = await response.json();
				registry.value = data;
			} catch (error) {
				console.error('Parse Error <Registry>:', error);
			}
		} else {
			console.error('Response Error <Registry>:', response.status, response.statusText);
		}
	} catch (error) {
		console.error('Fetch Error <Registry>:', error);
	}
}

export async function GetMetadata(id: number) {
	try {
		const response = await fetch(IMAGE_METADATA_URL, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(id)
		});

		if (response.ok) {
			try {
				const data: MetadataLayer[] = await response.json();
				metadata.value = data;

				// On success, initialise the image grid and get annotations.
				image.init();
				GetAnnotations(id);
			} catch (error) {
				console.error('Parse Error <Metadata: ' + loadedImage.value?.name + '>:', error);
			}
		} else {
			console.error(
				'Response Error <Metadata: ' + loadedImage.value?.name + '>:',
				response.status,
				response.statusText
			);
		}
	} catch (error) {
		console.error('Fetch Error <Metadata: ' + loadedImage.value?.name + '>:', error);
	}
}

export async function GetAnnotations(id: number) {
	try {
		const response = await fetch(IMAGE_ANNOTATIONS_URL, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(id)
		});

		if (response.ok) {
			try {
				const data: AnnotationLayer[] = await response.json();
				annotations.value = data;
				// // TODO: Move to server.
				// annotations.value = data.map((annotationLayer) => ({
				// 	...annotationLayer,
				// 	annotations: annotationLayer.annotations.map((annotation) => sort(annotation))
				// }));
			} catch (error) {
				console.error('Parse Error <Annotations: ' + loadedImage.value?.name + '>:', error);
			}
		} else {
			console.error(
				'Response Error <Annotations: ' + loadedImage.value?.name + '>:',
				response.status,
				response.statusText
			);
		}
	} catch (error) {
		console.error('Fetch Error <Annotations: ' + loadedImage.value?.name + '>:', error);
	}
}

// // TODO: Move to server.
// // Credit: https://stackoverflow.com/questions/54719326/sorting-points-in-a-clockwise-direction
// function sort(annotation: number[][]): number[][] {
// 	const length = annotation.length;

// 	// Get the center (mean value) using reduce.
// 	const center = annotation.reduce(
// 		(acc, [x, y]) => {
// 			acc[0] += x / length;
// 			acc[1] += y / length;
// 			return acc;
// 		},
// 		[0, 0]
// 	);

// 	return (
// 		annotation
// 			// Add an angle property to each point using:
// 			// angle = arctan(y/x) then convert to degrees.
// 			.map(([x, y]) => {
// 				return [x, y, Math.atan2(y - center[1], x - center[0]) * (180 / Math.PI)];
// 			})
// 			// Sort by angle.
// 			.sort((a, b) => a[2] - b[2])
// 			// Remove the angle property.
// 			.map(([x, y, _]) => [x, y])
// 	);
// }
