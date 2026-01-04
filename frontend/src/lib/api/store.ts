import { request } from '$helpers';
import type { Asset, Directory } from '$types';
import { HTTP_BASE_URL } from './urls.ts';

export async function get(storeId: number): Promise<(Directory | Asset)[] | null> {
	return await request.get({ url: `${HTTP_BASE_URL}/api/store/${storeId}` });
}
