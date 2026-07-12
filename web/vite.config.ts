import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';
import tailwindcss from '@tailwindcss/vite';
import { viteSingleFile } from 'vite-plugin-singlefile';
import { resolve } from 'path';

// https://vite.dev/config/
export default defineConfig({
	plugins: [svelte(), tailwindcss(), viteSingleFile()],
	resolve: {
		alias: {
			$lib: resolve('./src/lib'),
			$assets: resolve('./src/assets'),
			$wasm: resolve('../pkg')
		}
	},
	build: {
		outDir: '../dist',
		emptyOutDir: true
	}
});
