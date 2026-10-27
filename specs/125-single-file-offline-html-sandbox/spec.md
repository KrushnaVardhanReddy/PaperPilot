# Spec 125: Single-File Offline Self-Contained HTML Sandbox

## 1. Overview & Problem Statement
Privacy-conscious users in regulated or air-gapped environments (legal, healthcare, defence) want a standalone `.html` file that embeds the entire WASM runtime and web UI inline with zero external network requests and zero CDN dependencies (prompted by community member `spwa4` on Hacker News).

## 2. Technical Architecture & Constraints
A standard WebAssembly build requires a companion `.wasm` file fetched via `fetch('paperpilot_wasm_bg.wasm')`. However, when double-clicking a local `file:///.../paperpilot-portable.html` file in a browser, browser CORS security blocks `fetch()` against local filesystem origins.

To achieve a true **single-file, double-clickable, zero-network portable distribution**:
1. **Base64 Inline WASM**:
   - The compiled `paperpilot_wasm_bg.wasm` binary is base64-encoded.
   - At runtime, the portable runner decodes the base64 string into a `Uint8Array` in memory and initializes the WASM module directly via `WebAssembly.instantiate(bytes, imports)` rather than `WebAssembly.instantiateStreaming(fetch(...))`.
2. **Single-File Bundling**:
   - Inlines all Svelte compiled JavaScript, vanilla CSS tokens, and SVG icons into a single `paperpilot-portable.html`.
3. **Strict Air-Gap Content Security Policy (CSP)**:
   - Includes `<meta http-equiv="Content-Security-Policy" content="default-src 'none'; script-src 'unsafe-inline' 'unsafe-eval'; style-src 'unsafe-inline'; img-src data: blob:; font-src data:; connect-src blob: data:; worker-src blob: data:;">`.
   - Any attempt to reach the internet or send outbound telemetry is blocked by the browser.

## 3. Operations Included in Portable Sandbox
- **PDF Merge**: Combine multiple PDF files into one.
- **PDF Split**: Extract specific pages or split by range.
- **PDF Rotate**: Rotate pages by 90°, 180°, 270°.
- **PDF Delete Pages**: Strip unwanted pages.
- **PDF Compress**: Client-side object stream compression.
- **PDF Extract Text**: In-memory text extraction.

## 4. Deliverables
1. Build tool / script: `scripts/build_portable_html.js`.
2. Output artifact: `dist-portable/paperpilot-portable.html`.
3. Verification: Automated Playwright test verifying 0 network requests and working PDF operations.
4. Make target: `make build-portable`.
