import adapter from '@sveltejs/adapter-static';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	kit: {
		adapter: adapter({
			pages: 'build',
			assets: 'build',
			fallback: undefined,
			precompress: false,
			strict: true
		}),
		alias: {
			$actions: './src/lib/actions',
			$api: './src/lib/api.svelte.ts',
			$components: './src/lib/components',
			$helpers: './src/lib/helpers.ts',
			$icon: './src/lib/components/Icon.svelte',
			$states: './src/lib/states',
			$types: './src/lib/types.ts',
			$ui: './src/lib/ui',
			$view: './src/lib/view'
		}
	}
};

export default config;
