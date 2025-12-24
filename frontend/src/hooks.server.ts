import type { Handle } from '@sveltejs/kit';

const headers = {
	'Content-Security-Policy': `default-src 'none'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline'; img-src 'self' blob:; font-src 'self'; manifest-src 'self'; connect-src *; frame-src 'self'; frame-ancestors 'none';`,
	'Strict-Transport-Security': 'max-age=31536000; includeSubDomains; preload',
	'Cross-Origin-Opener-Policy': 'same-origin',
	'Cross-Origin-Embedder-Policy': 'require-corp',
	'X-Frame-Options': 'DENY'
};

export const handle: Handle = async ({ event, resolve }) => {
	const response = await resolve(event);

	// Set headers for dynamic routes
	Object.entries(headers).forEach(([key, value]) => {
		response.headers.set(key, value);
	});

	return response;
};
