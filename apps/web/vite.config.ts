import { svelte } from '@sveltejs/vite-plugin-svelte'
import { defineConfig } from 'vite'
import wasm from "vite-plugin-wasm";

// https://vite.dev/config/
export default defineConfig({
  build: {
    target: 'esnext'
  },
  plugins: [
    svelte(),
    (wasm as any)(),
    {
      name: 'portal-page-rewriter',
      configureServer(server) {
        server.middlewares.use((req, _res, next) => {
          if (req.url === '/swagger-ui' || req.url === '/swagger-ui/') {
            req.url = '/swagger-ui.html';
          } else if (req.url === '/embed-test' || req.url === '/embed-test/') {
            req.url = '/embed-test.html';
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
