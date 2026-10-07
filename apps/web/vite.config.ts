import { svelte } from '@sveltejs/vite-plugin-svelte'
import { defineConfig } from 'vite'
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";

// https://vite.dev/config/
export default defineConfig({
  build: {
    target: 'esnext'
  },
  plugins: [
    svelte(),
    (wasm as any)()
  ],
  server: {
    fs: {
      allow: ['..', '../../paperpilot-wasm/pkg']
    },
    proxy: {
      '/swagger-ui': {
        target: 'http://127.0.0.1:7823',
        changeOrigin: true
      },
      '/api-docs': {
        target: 'http://127.0.0.1:7823',
        changeOrigin: true
      }
    }
  }
})
