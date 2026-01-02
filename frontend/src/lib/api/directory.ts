import { request } from '$helpers';
import { HTTP_BASE_URL } from './urls.ts';

export async function create(storeId: number, parentDirectoryId: number, name: string) {
	await request.post({
		url: `${HTTP_BASE_URL}/api/store/${storeId}/directory/${parentDirectoryId}/${name}`
	});
}

export async function remove(storeId: number, directoryId: number, mode: 'soft' | 'hard') {
	await request.delete({
		url: `${HTTP_BASE_URL}/api/store/${storeId}/directory/${directoryId}`,
		query: { mode }
	});
}

export async function move(storeId: number, directoryId: number, destinationId: number) {
	await request.patch({
		url: `${HTTP_BASE_URL}/api/store/${storeId}/directory/${directoryId}`,
		body: { destination_id: destinationId }
	});
}
