import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import Icons from 'unplugin-icons/vite';
import tailwindcss from '@tailwindcss/vite';
import { HTTP_BASE_URL, WEBSOCKET_URL } from './src/lib/constants.ts';

export default defineConfig({
	plugins: [sveltekit(), tailwindcss(), Icons({ compiler: 'svelte' })],
	server: {
		headers: {
			'Content-Security-Policy': `default-src 'none'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline'; img-src 'self' blob:; font-src 'self'; manifest-src 'self'; connect-src 'self' ${HTTP_BASE_URL} ${WEBSOCKET_URL}; frame-src 'self'; frame-ancestors 'none';`,
			'Strict-Transport-Security': 'max-age=31536000; includeSubDomains; preload',
			'Cross-Origin-Opener-Policy': 'same-origin',
			'Cross-Origin-Embedder-Policy': 'require-corp',
			'X-Frame-Options': 'DENY'
		}
	}
});
