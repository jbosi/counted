import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import tailwindcss from '@tailwindcss/vite';

// https://vite.dev/config/
export default defineConfig({
	plugins: [tailwindcss(), react()],
	server: {
		proxy: {
			'/api': 'http://127.0.0.1:8080',
		},
	},
	build: {
		// Ensure no inline scripts/styles in production
		assetsInlineLimit: 0,
		cssCodeSplit: true,
	},
});
