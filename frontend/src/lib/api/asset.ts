import { STORE_URL } from '$constants';
import { request, defined } from '$helpers';
import type { GltfLayer, TiledImageLayer, UploaderOptions } from '$types';

export async function properties(
	storeId: number,
	assetId: number
): Promise<{ metadata: TiledImageLayer[]; annotations: GltfLayer[] } | null> {
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
