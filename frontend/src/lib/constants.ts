import { PUBLIC_PORT } from '$env/static/public';

export const HTTP_BASE_URL =
	import.meta.env.MODE === 'development' ? `http://localhost:${PUBLIC_PORT}` : '';

const WEBSOCKET_BASE_URL =
	import.meta.env.MODE === 'development' ? `ws://localhost:${PUBLIC_PORT}` : '';

export const DIRECTORY_URL = new URL(HTTP_BASE_URL + '/api/directory');
export const ASSET_URL = new URL(HTTP_BASE_URL + '/api/image');
export const STORE_URL = new URL(HTTP_BASE_URL + '/api/store');
export const WEBSOCKET_URL = new URL(WEBSOCKET_BASE_URL + '/api/websocket');

export const C_TILE_TAG = 0;

export const S_ERROR_TAG = 0;
export const S_TILE_TAG = 1;
export const S_DIRECTORY_TAG = 2;
export const S_DIRECTORY_CREATE_TAG = 0;
export const S_DIRECTORY_DELETE_TAG = 1;
export const S_DIRECTORY_MOVE_TAG = 2;
export const S_DIRECTORY_RENAME_TAG = 3;
