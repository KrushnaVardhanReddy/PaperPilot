# Mobile Touch-First WASM Architecture

## Overview
The PaperPilot Mobile Architecture utilizes the **pure in-memory `paperpilot-wasm` engine** to run heavy document processing natively within the mobile browser environment (WebView/PWA).
By completely bypassing traditional native mobile cross-compilation (e.g. via Tauri 2 native bridging) and scoped storage boundaries (`content://` URIs), we guarantee a 100% offline, private, and frictionless emergency operation toolset.

## Key Components

1. **WasmPdfEngine (`packages/paperpilot-wasm`)**
   - Built with `wasm-bindgen`, this provides a 0-dependency interface directly to `lopdf` and our internal pipelines via `WasmPdfEngine`.
   - Uses raw `Uint8Array` buffers exclusively for input and output, bypassing the filesystem.
   - Core tools enabled: Merge, Rotate, Compress, Encrypt, Split/Extract, Clean Blank Pages, PDF Metadata, OCR, Images to PDF.

2. **Mobile PDF WebWorker (`pdfWorker.ts`)**
   - Keeps the main UI thread lag-free by chunking the execution of the `WasmPdfEngine` into a background worker module.

3. **Touch-First UI Drawer (`Widget.svelte`)**
   - Utilizes CSS media queries (`max-width: 640px`) to pivot the primary layout to a tactile bottom-sheet modal.
   - All tap areas are standardized to `min-height: 48px; min-width: 48px` to satisfy WCAG touch guidelines.
   - Safely protects content via `env(safe-area-inset-bottom)`.

4. **Hardware Integrations**
   - `CameraScannerModal` accesses native camera streams via `navigator.mediaDevices.getUserMedia()`, snaps multi-page queues, handles Kaagaz-style polygon/crop matrix math, and then pipes the Array of Uint8Arrays down to `WasmPdfEngine.images_to_pdf`.
   - Native OS Sharing via `navigator.share()` instantly pushes the resulting PDF blob back to the mobile system intent registry (WhatsApp, Email, Files, etc.), completely avoiding downloads.

## Discarded Concepts
- **`process_mobile_buffer` Tauri Bridge**: We removed the backend bridging logic because piping heavy Base64 encodings between WebView and native Rust logic induced massive memory spikes and broke due to Android's strict IO permissions.
