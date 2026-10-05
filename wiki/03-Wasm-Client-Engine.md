# Phase 4.8.1: WebAssembly Client Engine

The WebAssembly Client Engine for PaperPilot processes PDF documents fully in memory on the client side, bypassing backend dependency to eliminate user latency and privacy risks.

## Overview
The feature resides inside the `paperpilot-wasm` crate. The core logic executes entirely within pure Rust WebAssembly without C/C++ or system OS components (no POSIX file system I/O, threads, etc.). To interact with the DOM and Frontend UI gracefully, all operations run off the main browser thread via Web Workers.

## Crate Structure
`paperpilot-wasm` depends on `wasm-bindgen`, `js-sys`, and `lopdf` natively without features requesting the file system.
* `lib.rs`: Exposes `WasmPdfEngine` structurally wrapping WASM bindings logic and extracting Uint8Arrays.
* `operations.rs`: Directly hooks `lopdf::Document::load_mem()` handling PDF manipulation logic and byte emission into standard byte streams. Returns standard `Vec<u8>`.
* `tests/wasm_tests.rs`: Tests native functionality independent of WebAssembly DOM targets.

### WasmPdfEngine Exported Methods
1. `merge`: Merges an array of buffers into a single buffer.
2. `rotate`: Rotates targeted PDF pages.
3. `split`: Splits a PDF based on page sequence comma-separated strings (e.g., `"1, 2-3"`).
4. `compress`: Deflates internal binary data.
5. `encrypt`: Secures a document using standard V2 128-bit encryption constraints.
6. `watermark`: Injects textual indicators recursively into page node contents.

## Web Worker Bridge Integration
The WebWorker components orchestrate non-blocking UI behavior. They live inside `apps/desktop/src/lib/wasm/`.
* `pdfWorker.ts`: Imports standard target bindings (e.g. via `pkg/paperpilot_wasm.js`) to parse `MessageEvent` commands gracefully. Emits `success`, `error`, and `result`.
* `index.ts`: Represents the object-oriented Interface API `WasmPdfClient`. Stores generated tasks securely in a promise-dispatch Map awaiting execution state.

## Build and Testing Steps
* Compile the WASM target module: `wasm-pack build --target web paperpilot-wasm`
* Check syntax and wasm compatibility: `cargo check -p paperpilot-wasm --target wasm32-unknown-unknown`
* Execute core tests natively: `cargo test -p paperpilot-wasm`