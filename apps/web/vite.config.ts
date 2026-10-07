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
    (wasm as any)(),
    {
      name: 'swagger-ui-rewriter',
      configureServer(server) {
        server.middlewares.use((req, res, next) => {
          if (req.url === '/swagger-ui' || req.url === '/swagger-ui/') {
            req.url = '/swagger-ui.html';
          }
          next();
        });
      }
    }
  ],
  server: {
    fs: {
      allow: ['..', '../../paperpilot-wasm/pkg']
    }
  }
})
