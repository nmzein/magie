import { ASSET_URL } from '$constants';
import { request, defined } from '$helpers';
import type { UploaderOptions } from '$types';
import type { Geometry2DLayer } from '$view/Geometry2D/types';
import type { Image2DLayer } from '$view/Image2D/types';
import { GLTFLoader, type GLTF } from 'three/addons/loaders/GLTFLoader.js';

const gltfLoader = new GLTFLoader();

export async function properties(
	storeId: number,
	id: number
): Promise<{ metadata: Image2DLayer[]; annotations: Geometry2DLayer[] } | null> {
	return await request.get({ url: `${ASSET_URL}/${storeId}/${id}/properties` });
}

export async function thumbnail(storeId: number, id: number): Promise<HTMLImageElement | null> {
	const blob: Blob | null = await request.get({
		url: `${ASSET_URL}/${storeId}/${id}/thumbnail`
	});
	if (!defined(blob)) return null;

	const image = new Image();
	image.src = URL.createObjectURL(blob);
	return image;
}

export async function geometry2d(storeId: number, id: number, layerId: number): Promise<GLTF> {
	return await gltfLoader.loadAsync(`${ASSET_URL}/${storeId}/${id}/annotations/${layerId}`);
}

export async function remove(storeId: number, id: number, mode: 'soft' | 'hard') {
	await request.delete({
		url: `${ASSET_URL}/${storeId}/${id}`,
		query: { mode }
	});
}

export async function move(storeId: number, id: number, destinationId: number) {
	await request.patch({
		url: `${ASSET_URL}/${storeId}/${id}`,
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
		url: `${ASSET_URL}/${storeId}/${parentId}/${options.name}`,
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
