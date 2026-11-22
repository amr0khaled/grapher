import { svelte } from '@sveltejs/vite-plugin-svelte';
import { compression } from "vite-plugin-compression2"
import { defineConfig } from 'vite';

/** @type {import('vite').UserConfig} */
export default defineConfig({
  base: './',
  plugins: [svelte({ experimental: {} }),
  compression({
    algorithms: ["brotliCompress"],
    include: [/\.(js|mjs|json|css|html|svg|ttf|otf|eot)$/],
    threshold: 300 * 1024,
    deleteOriginalAssets: false
  })
  ],
  build: {
    // inline the wasm engine as a data: URI so the built app works from
    // file:// and non-root mounts (e.g. future Tauri shell)
    assetsInlineLimit: (filePath) => {
      if (filePath.endsWith('.wasm')) return true;
    },
    rolldownOptions: {
      output: {
        codeSplitting: {
          groups: [
            {
              name: "wasm",
              test: "/src[/]wasm",
              priority: 30
            },
            {
              name: "components",
              test: "/src[/]components",
              maxSize: 500_000,
            },
            {
              name: "utils",
              test: "/src[/]utils",
              maxSize: 200_000,
            }
          ]
        }
      }
    }
  },
})
