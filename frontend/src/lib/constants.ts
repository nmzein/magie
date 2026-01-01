export const HTTP_BASE_URL = import.meta.env.DEV ? `http://localhost:${PORT}` : '';
export const WEBSOCKET_BASE_URL = import.meta.env.DEV ? `ws://localhost:${PORT}` : '';

export const STORE_URL = `${HTTP_BASE_URL}/api/store`;
