import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: vitePreprocess(),

	kit: {
		adapter: adapter({
			pages: 'build',
			assets: 'build',
			fallback: '200.html',
			precompress: false,
			strict: true
		}),
		alias: {
			$actions: './src/lib/actions',
			$api: './src/lib/api',
			$components: './src/lib/components',
			$constants: './src/lib/constants.ts',
			$helpers: './src/lib/helpers.ts',
			$icon: './src/lib/components/Icon.svelte',
			$states: './src/lib/states/index.svelte.ts',
			$types: './src/lib/types.ts',
			$ui: './src/lib/ui',
			$view: './src/lib/view'
		}
	}
};

export default config;
