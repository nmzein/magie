import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import wasm from "vite-plugin-wasm";
import topLevelAwait from 'vite-plugin-top-level-await';

export default defineConfig({
	plugins: [wasm(), topLevelAwait(), sveltekit()],
	define: {
		'process.env.NODE_ENV': '"production"',
	}
});
