# Task 164: Implement Phase 11 PaperPilot Browser Companion Extension

## Checklist
- [ ] Scaffold `apps/extension` with Manifest V3, Vite, Svelte 5, and TypeScript
- [ ] Implement `manifest.json` with permissions (`contextMenus`, `activeTab`, `storage`, `scripting`)
- [ ] Implement Service Worker (`src/background/index.ts`) with context menus and PaperPilot Gateway detector (`http://127.0.0.1:7823`)
- [ ] Implement WASM bridge (`src/wasm/wasmBridge.ts`) linking `paperpilot-wasm` for client-side instant PDF operations
- [ ] Implement Content Script (`src/content/`) for clutter-free web article extraction
- [ ] Implement Toolbar Popup UI (`src/popup/App.svelte`) showing daemon health badge, quick tool buttons, and drag-and-drop file processing
- [ ] Add build scripts in root `package.json` (`build:extension`) and extension `package.json`
- [ ] Add Playwright test in `apps/extension/tests/` to verify manifest validity and extension compilation
- [ ] Verify clean build and zero workspace regressions

