import { STORE_URL } from '$constants';
import { request, defined } from '$helpers';
import type { UploaderOptions } from '$types';
import type { Geometry2DLayer } from '$view/Geometry2D/types';
import type { Image2DLayer } from '$view/Image2D/types';
import { GLTFLoader, type GLTF } from 'three/addons/loaders/GLTFLoader.js';

const gltfLoader = new GLTFLoader();

export async function properties(
	storeId: number,
	assetId: number
): Promise<{ metadata: Image2DLayer[]; annotations: Geometry2DLayer[] } | null> {
	return await request.get({ url: `${STORE_URL}/${storeId}/asset/${assetId}/properties` });
}

export async function thumbnail(
	storeId: number,
	assetId: number
): Promise<HTMLImageElement | null> {
	const blob: Blob | null = await request.get({
		url: `${STORE_URL}/${storeId}/asset/${assetId}/thumbnail`
	});
	if (!defined(blob)) return null;

	const image = new Image();
	image.src = URL.createObjectURL(blob);
	return image;
}

export async function geometry2d(storeId: number, assetId: number, layerId: number): Promise<GLTF> {
	return await gltfLoader.loadAsync(
		`${STORE_URL}/${storeId}/asset/${assetId}/annotations/${layerId}`
	);
}

export async function remove(storeId: number, assetId: number, mode: 'soft' | 'hard') {
	await request.delete({
		url: `${STORE_URL}/${storeId}/asset/${assetId}`,
		query: { mode }
	});
}

export async function move(storeId: number, assetId: number, destinationId: number) {
	await request.patch({
		url: `${STORE_URL}/${storeId}/asset/${assetId}`,
		body: { destination_id: destinationId },
		type: 'json'
	});
}

export async function upload(
	storeId: number,
	parentId: number,
	imageFile: File,
	geometryFile: File | undefined,
	options: UploaderOptions
) {
	await request.post({
		url: `${STORE_URL}/${storeId}/asset/${parentId}/${options.name}`,
		body: {
			decoder: options.decoder,
			encoder: options.encoder,
			generator: options.generator,
			image_file: imageFile,
			annotations_file: geometryFile
		},
		type: 'form'
	});
}
