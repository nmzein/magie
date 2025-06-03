import * as asset from './asset.ts';
import * as directory from './directory.ts';
import * as store from './store.ts';
import * as websocket from './websocket.ts';

import { HTTP_BASE_URL } from '$constants';
import { request } from '$helpers';
import type { Store } from '$types';

async function registry(): Promise<Store[] | null> {
	return await request.get({ url: `${HTTP_BASE_URL}/api/registry` });
}

async function generators(): Promise<string[] | null> {
	return await request.get({ url: `${HTTP_BASE_URL}/api/generators` });
}

const http = (() => {
	return { asset, directory, store, registry, generators };
})();

export { http, websocket };
