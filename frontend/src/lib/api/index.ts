import { request } from '$helpers';
import type { Modules, Store } from '$types';
import * as asset from './asset.ts';
import { socket as broadcast } from './broadcast.ts';
import * as directory from './directory.ts';
import * as store from './store.ts';
import { HTTP_BASE_URL, WEBSOCKET_BASE_URL } from './urls.ts';

async function registry(): Promise<Store[] | null> {
	return await request.get({ url: `${HTTP_BASE_URL}/api/registry` });
}

async function modules(): Promise<Modules | null> {
	return await request.get({ url: `${HTTP_BASE_URL}/api/modules` });
}

const http = (() => {
	return { asset, directory, store, registry, modules };
})();

export { broadcast, http, HTTP_BASE_URL, WEBSOCKET_BASE_URL };
