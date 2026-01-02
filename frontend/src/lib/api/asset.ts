import { defined, request } from '$helpers';
import type { GltfLayer, TiledImageLayer, UploaderOptions } from '$types';
import { HTTP_BASE_URL } from './urls.ts';

type Properties = {
	metadata: TiledImageLayer[];
	annotations: GltfLayer[];
};

export async function properties(storeId: number, assetId: number): Promise<Properties | null> {
	const properties = (await request.get({
		url: `${HTTP_BASE_URL}/api/store/${storeId}/asset/${assetId}/properties`
	})) as Properties | null;

	if (!defined(properties)) return null;

	properties.annotations = properties.annotations.map((a) => ({
		...a,
		url: `${HTTP_BASE_URL}/api/store/${storeId}/asset/${assetId}/annotations/${a.id}`,
		dirty: true
	}));

	return properties;
}

export async function thumbnail(
	storeId: number,
	assetId: number
): Promise<HTMLImageElement | null> {
	const blob: Blob | null = await request.get({
		url: `${HTTP_BASE_URL}/api/store/${storeId}/asset/${assetId}/thumbnail`
	});
	if (!defined(blob)) return null;

	const image = new Image();
	image.src = URL.createObjectURL(blob);
	return image;
}

export async function remove(storeId: number, assetId: number, mode: 'soft' | 'hard') {
	await request.delete({
		url: `${HTTP_BASE_URL}/api/store/${storeId}/asset/${assetId}`,
		query: { mode }
	});
}

export async function move(storeId: number, assetId: number, destinationId: number) {
	await request.patch({
		url: `${HTTP_BASE_URL}/api/store/${storeId}/asset/${assetId}`,
		body: { destination_id: destinationId }
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
		url: `${HTTP_BASE_URL}/api/store/${storeId}/asset/${parentId}/${options.name}`,
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
