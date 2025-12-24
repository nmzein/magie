// import { PUBLIC_PORT } from '$env/static/public';

// TODO: Fix import issue.
const PUBLIC_PORT = 3000;

export const HTTP_BASE_URL =
	process.env.NODE_ENV === 'development' ? `http://localhost:${PUBLIC_PORT}` : '';

export const WEBSOCKET_BASE_URL =
	process.env.NODE_ENV === 'development' ? `ws://localhost:${PUBLIC_PORT}` : '';

export const DIRECTORY_URL = HTTP_BASE_URL + '/api/directory';
export const ASSET_URL = HTTP_BASE_URL + '/api/image';
export const STORE_URL = HTTP_BASE_URL + '/api/store';
export const WEBSOCKET_URL = WEBSOCKET_BASE_URL + '/api/websocket';

export const C_TILE_TAG = 0;

export const S_ERROR_TAG = 0;
export const S_TILE_TAG = 1;
export const S_DIRECTORY_TAG = 2;
export const S_DIRECTORY_CREATE_TAG = 0;
export const S_DIRECTORY_DELETE_TAG = 1;
export const S_DIRECTORY_MOVE_TAG = 2;
export const S_DIRECTORY_RENAME_TAG = 3;
