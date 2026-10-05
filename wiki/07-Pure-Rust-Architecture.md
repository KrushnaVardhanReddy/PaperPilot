# Pure-Rust & Zero-C++ Architecture Guide

This guide documents our strategy for eliminating external C/C++ libraries, OS dependencies, and heavy browser runtimes in favor of pure-Rust ("Rust-only") alternatives that compile natively to WebAssembly (WASM) and Linux/macOS/Windows binaries without host requirements.

Full architectural roadmap is maintained at [`docs/PURE_RUST_ARCHITECTURE_ROADMAP.md`](../docs/PURE_RUST_ARCHITECTURE_ROADMAP.md).

## Quick Direct Comparison

| Feature Module | Traditional Engine (C++ / OS Dependent) | Pure-Rust Counterpart (WASM & Native Friendly) | Notes |
|---|---|---|---|
| **Structure Manipulation** | Poppler / QPDF | **`lopdf`** / **`pdf-writer`** | **Already in use for 12 tools!** In-memory buffer editing. |
| **PDF Page Rendering** | Google PDFium (`pdfium-render`) | **`hayro`** / **`pdf-render`** + **`tiny-skia`** | Interprets PDF content streams to raw framebuffers. |
| **OCR Text Recognition** | Tesseract C++ (`tesseract-ocr`) | **`ocrs`** (via `rten`) | Neural text detection + recognition in pure Rust/WASM. |
| **Neural AI / NLP Intent** | Microsoft ONNX Runtime (`ort`) | **`rten`** or **`tract`** | Executes ONNX models directly on CPU with 0 dynamic libs. |
| **HTML ➔ PDF** | Headless Chrome (`headless_chrome`) | **`typst`** / **`tiny-skia`** | Standalone typesetting engine, eliminates browser daemon. |
| **Office Formats** | LibreOffice / COM Interop | **`docx-rs`** / **`rust_xlsxwriter`** | **Already in use!** Pure-Rust in-memory XML manipulation. |
| **Image Conversion** | `libpng` / `libjpeg` | **`image`** (v0.25) | **Already in use!** Pure Rust encoders/decoders. |
| **Cryptography** | OpenSSL / System Keyrings | **`ring`** / **`rustls`** / **`sha2`** | 100% portable cryptographic primitives. |
