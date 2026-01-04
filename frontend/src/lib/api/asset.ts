import { defined, request } from '$helpers';
import type {
	AssetMetadata,
	GeometryAssetMetadata,
	TiledImageAssetMetadata,
	UploaderOptions
} from '$types';
import { HTTP_BASE_URL, WEBSOCKET_BASE_URL } from './urls.ts';

// TODO: Fix upstream in backend.
export async function properties(
	storeId: number,
	assetId: number
): Promise<AssetMetadata[] | null> {
	const properties: {
		metadata: TiledImageAssetMetadata['layers'];
		annotations: Omit<GeometryAssetMetadata['layers'][number], 'url'>[];
	} | null = await request.get({
		url: `${HTTP_BASE_URL}/api/store/${storeId}/asset/${assetId}/properties`
	});
	if (!defined(properties)) return null;

	const width = properties.metadata[0].width;
	const height = properties.metadata[0].height;

	const tiledImage: TiledImageAssetMetadata = {
		type: 'tiled-image',
		width,
		height,
		primary: true,
		url: `${WEBSOCKET_BASE_URL}/api/store/${storeId}/asset/${assetId}/socket`,
		layers: properties.metadata
	};

	const draco: GeometryAssetMetadata = {
		type: 'draco-geometry',
		width,
		height,
		layers: properties.annotations.map((layer) => ({
			...layer,
			url: `${HTTP_BASE_URL}/api/store/${storeId}/asset/${assetId}/annotations/${layer.id}`
		}))
	};

	return [draco, tiledImage];
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
