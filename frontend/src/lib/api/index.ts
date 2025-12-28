import * as asset from './asset.ts';
import * as directory from './directory.ts';
import * as store from './store.ts';
import * as websocket from './websocket.ts';

import { HTTP_BASE_URL } from '$constants';
import { request } from '$helpers';
import type { Modules, Store } from '$types';

async function registry(): Promise<Store[] | null> {
	return await request.get({ url: `${HTTP_BASE_URL}/api/registry` });
}

async function modules(): Promise<Modules | null> {
	return await request.get({ url: `${HTTP_BASE_URL}/api/modules` });
}

const http = (() => {
	return { asset, directory, store, registry, modules };
})();

export { http, websocket };
