import { MetadataURL, ProcessImageURL, ProcessImageAnnotationsURL, ImageListURL } from '$lib/urls';
import { MetadataStore } from '$lib/stores';
import type { ImageMetadata } from './types';

export async function SendImage(Image: File, AnnotationGenerator: string) {
	const formData = new FormData();
	formData.append('image', Image);
	formData.append('annotation_generator', AnnotationGenerator);

	try {
		console.log('Sending image to server for processing {}.', formData);
		const response = await fetch(ProcessImageURL, {
			method: 'POST',
			body: formData
		});

		if (response.ok) {
			console.log('Success: Sent image to server for processing.');
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

export async function SendFiles(Image: File, AnnotationFile: File, AnnotationGenerator: string) {
	const formData = new FormData();
	formData.append('image', Image);
	formData.append('annotations', AnnotationFile);
	formData.append('annotation_generator', AnnotationGenerator);

	try {
		const response = await fetch(ProcessImageAnnotationsURL, { method: 'POST', body: formData });

		if (response.ok) {
			console.log('Success: Sent files to server for processing.');
		} else {
			console.error(
				'Error: Could not send files to the server: ',
				response.status,
				response.statusText
			);
		}
	} catch (error) {
		console.error('Error: Could not reach server: ', error);
	}
}

export async function GetAnnotationGenerators(): Promise<string[]> {
	return ['TIA Toolbox', 'Example 2'];
}

export async function GetImagesList(): Promise<string[]> {
	try {
		const response = await fetch(ImageListURL, { method: 'POST' });

		if (response.ok) {
			try {
				const data: string[] = await response.json();
				return data;
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

	return [];
}

export async function GetMetadata() {
	try {
		const response = await fetch(MetadataURL, { method: 'POST' });

		if (response.ok) {
			try {
				const data: ImageMetadata = await response.json();
				MetadataStore.set(data);
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
