import { defineConfig } from 'vite';
import { sveltekit } from '@sveltejs/kit/vite';
import Icons from 'unplugin-icons/vite';

import tailwindcss from 'tailwindcss';
import autoprefixer from 'autoprefixer';

export default defineConfig({
	plugins: [
		sveltekit(),
		Icons({
			compiler: 'svelte'
		})
	],
	server: {
		proxy: {
			'/axum-api': {
				target: 'http://localhost:5000',
				changeOrigin: true,
				rewrite: (path) => path.replace(/^\/axum-api/, '') // This line rewrites the URL path
			}
		}
	},
	css: {
		postcss: {
			plugins: [tailwindcss(), autoprefixer()]
		}
	}
});
