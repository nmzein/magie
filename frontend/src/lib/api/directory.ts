import { DIRECTORY_URL } from '$constants';
import { request } from '$helpers';

export async function create(storeId: number, parentDirectoryId: number, name: string) {
	await request.post({
		url: `${DIRECTORY_URL}/${storeId}/${parentDirectoryId}/${name}`
	});
}

export async function remove(storeId: number, directoryId: number, mode: 'soft' | 'hard') {
	await request.delete({
		url: `${DIRECTORY_URL}/${storeId}/${directoryId}`,
		query: { mode }
	});
}

export async function move(storeId: number, directoryId: number, destinationId: number) {
	await request.patch({
		url: `${DIRECTORY_URL}/${storeId}/${directoryId}`,
		body: { destination_id: destinationId },
		type: 'json'
	});
}
