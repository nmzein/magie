import adapter from '@sveltejs/adapter-auto';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: vitePreprocess(),

	kit: {
		adapter: adapter(),
		alias: {
			$actions: './src/lib/actions',
			$api: './src/lib/api.svelte.ts',
			$components: './src/lib/components',
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
