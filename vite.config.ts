import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import { enhancedImages } from '@sveltejs/enhanced-img';
import wasmPack from 'vite-plugin-wasm-pack';

export default defineConfig({
	plugins: [enhancedImages(), sveltekit()]
});
