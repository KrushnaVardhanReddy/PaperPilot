import { svelte } from '@sveltejs/vite-plugin-svelte';
import { defineConfig, type Plugin } from 'vite';
import wasm from 'vite-plugin-wasm';
import path from 'node:path';
import fs from 'node:fs';
import { fileURLToPath } from 'node:url';

const rootDir = fileURLToPath(new URL('.', import.meta.url));

function extensionPackagerPlugin(): Plugin {
  return {
    name: 'extension-packager',
    closeBundle() {
      // 1. Relocate dist/src/popup/popup.html to dist/popup.html
      const nestedPopup = path.resolve(rootDir, 'dist/src/popup/popup.html');
      const targetPopup = path.resolve(rootDir, 'dist/popup.html');
      if (fs.existsSync(nestedPopup)) {
        let html = fs.readFileSync(nestedPopup, 'utf-8');
        // Fix asset URLs from nested ../../assets or absolute /assets to ./assets
        html = html.replace(/(href|src)="(?:\.\.\/)+/g, '$1="./');
        html = html.replace(/(href|src)="\/assets\//g, '$1="./assets/');
        fs.writeFileSync(targetPopup, html);
        fs.rmSync(path.resolve(rootDir, 'dist/src'), { recursive: true, force: true });
      }

      // 2. Copy manifest.json to dist/manifest.json
      const srcManifest = path.resolve(rootDir, 'manifest.json');
      const distManifest = path.resolve(rootDir, 'dist/manifest.json');
      if (fs.existsSync(srcManifest)) {
        fs.copyFileSync(srcManifest, distManifest);
      }

      // 3. Copy icons/ to dist/icons/
      const srcIcons = path.resolve(rootDir, 'icons');
      const distIcons = path.resolve(rootDir, 'dist/icons');
      if (fs.existsSync(srcIcons)) {
        fs.cpSync(srcIcons, distIcons, { recursive: true });
      }

      // 4. Copy paperpilot_wasm_bg.wasm to dist/
      const wasmPkg = path.resolve(rootDir, '../../paperpilot-wasm/pkg/paperpilot_wasm_bg.wasm');
      const wasmDist = path.resolve(rootDir, 'dist/paperpilot_wasm_bg.wasm');
      if (fs.existsSync(wasmPkg)) {
        fs.copyFileSync(wasmPkg, wasmDist);
      }
    },
  };
}

export default defineConfig({
  base: './',
  plugins: [
    svelte(),
    (wasm as any)(),
    extensionPackagerPlugin(),
  ],
  build: {
    target: 'esnext',
    outDir: 'dist',
    emptyOutDir: true,
    rollupOptions: {
      input: {
        popup: path.resolve(rootDir, 'src/popup/popup.html'),
        background: path.resolve(rootDir, 'src/background/index.ts'),
        content: path.resolve(rootDir, 'src/content/index.ts'),
      },
      output: {
        entryFileNames: (chunkInfo) => {
          if (chunkInfo.name === 'background') {
            return 'background.js';
          }
          if (chunkInfo.name === 'content') {
            return 'content.js';
          }
          return 'assets/[name]-[hash].js';
        },
        chunkFileNames: 'assets/[name]-[hash].js',
        assetFileNames: 'assets/[name]-[hash].[ext]',
      },
    },
  },
  server: {
    fs: {
      allow: ['..', '../../paperpilot-wasm/pkg'],
    },
  },
});
