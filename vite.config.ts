import { svelte } from '@sveltejs/vite-plugin-svelte';
import { defineConfig } from 'vite';

/** @type {import('vite').UserConfig} */
export default defineConfig({
  base: './',
  plugins: [svelte({ experimental: {} })],
  build: {
    // inline the wasm engine as a data: URI so the built app works from
    // file:// and non-root mounts (e.g. future Tauri shell)
    assetsInlineLimit: (filePath) => {
      if (filePath.endsWith('.wasm')) return true;
    },
  },
  server: {
    allowedHosts: ["100.122.44.22", "khaldon.manx-luma.ts.net"]
  }
})
