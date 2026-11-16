import { svelte } from '@sveltejs/vite-plugin-svelte';
import { defineConfig } from 'vite';

/** @type {import('vite').UserConfig} */
export default defineConfig({
  plugins: [svelte({ experimental: {} })],
})
