// FIXME: Env var not available during Nix build.
// import { PUBLIC_PORT } from '$env/static/public';
const PUBLIC_PORT = 3000;

export const HTTP_BASE_URL =
	process.env.NODE_ENV === 'development' ? `http://localhost:${PUBLIC_PORT}` : '';

export const WEBSOCKET_BASE_URL =
	process.env.NODE_ENV === 'development' ? `ws://localhost:${PUBLIC_PORT}` : '';

export const STORE_URL = HTTP_BASE_URL + '/api/store';
export const BROADCAST_URL = WEBSOCKET_BASE_URL + '/api/broadcast';
