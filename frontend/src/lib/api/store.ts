import { STORE_URL } from '$constants';
import { request } from '$helpers';
import type { Asset, Directory } from '$types';

export async function get(storeId: number): Promise<(Directory | Asset)[] | null> {
	return await request.get({ url: `${STORE_URL}/${storeId}` });
}
