import { PUBLIC_HTTP_SCHEME, PUBLIC_WS_SCHEME, PUBLIC_BACKEND_URL } from '$env/static/public';

export const BASE_URL = '://' + PUBLIC_BACKEND_URL;
export const HTTP_URL = PUBLIC_HTTP_SCHEME + BASE_URL + '/api';
export const DIRECTORY_URL = new URL(HTTP_URL + '/directory');
export const ASSET_URL = new URL(HTTP_URL + '/image');
export const STORE_URL = new URL(HTTP_URL + '/store');
export const WEBSOCKET_URL = new URL(PUBLIC_WS_SCHEME + BASE_URL + '/api/websocket');

export const C_TILE_TAG = 0;

export const S_ERROR_TAG = 0;
export const S_TILE_TAG = 1;
export const S_DIRECTORY_TAG = 2;
export const S_DIRECTORY_CREATE_TAG = 0;
export const S_DIRECTORY_DELETE_TAG = 1;
export const S_DIRECTORY_MOVE_TAG = 2;
export const S_DIRECTORY_RENAME_TAG = 3;
