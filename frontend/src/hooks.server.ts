import type { Handle } from '@sveltejs/kit';

const securityHeaders = {
	'Cross-Origin-Opener-Policy': 'same-origin',
	'Cross-Origin-Embedder-Policy': 'require-corp'
};

export const handle: Handle = async ({ event, resolve }) => {
	const response = await resolve(event);
	Object.entries(securityHeaders).forEach(([key, value]) => {
		response.headers.set(key, value);
	});
	return response;
};
