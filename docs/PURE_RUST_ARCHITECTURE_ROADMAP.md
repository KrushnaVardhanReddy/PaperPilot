# Pure-Rust & Zero-C++ Architecture Roadmap (WASM & Edge Portability)

> **Goal**: Guide PaperPilot toward becoming a 100% standalone, zero-OS-dependency, zero-C++ binary across Desktop, Mobile (iOS/Android), Browser Web (WASM), and Cloudflare Edge Workers.

---

## 1. Executive Summary

While PaperPilot currently achieves 100% test parity and runs 44 operations across CLI, MCP, REST, and Desktop, several advanced modules rely on external C/C++ dynamic libraries or host OS executables. 

Replacing these dependencies with modern **pure-Rust ("Rust-only")** equivalents unlocks:
1. **100% Client-Side WebAssembly (WASM)** execution without external toolchains or emscripten.
2. **Sub-10ms Serverless Edge Deployment** on Cloudflare Workers and Fastly Compute@Edge.
3. **Frictionless Cross-Platform Desktop & Mobile**: No external `apt-get` packages (`tesseract-ocr`), no shared library linker issues (`libonnxruntime.so`, `libpdfium.so`), and zero system dependencies on Windows/macOS/Linux.

---

## 2. Dependency Audit & Pure-Rust Replacements

| Domain / Operation | Current Engine | Current Dependency Type | Pure-Rust Replacement | Key Benefits & Crate Details |
|---|---|---|---|---|
| **PDF Manipulation**<br>*(Merge, Split, Rotate, Compress, Encrypt, Watermark, Delete, Extract, Crop, Reorder, Flatten, Metadata)* | `lopdf` | **Already Pure Rust ✅** | `lopdf` + `pdf-writer` | Pure in-memory AST manipulation. Zero OS calls. Formats raw `Vec<u8>` buffers in <5ms. |
| **OCR Text Recognition**<br>(`pdf_ocr`) | `tesseract-ocr` | C++ binary + Leptonica + `.traineddata` files | **`ocrs`** *(by Robert Knight)* | Pure-Rust OCR engine running DBNet detection + recognition models on CPU/WASM via `rten`. Zero C++ compilation. |
| **PDF Page Rendering & Rasterization**<br>(`pdf_render`, `pdf_to_images`) | `pdfium-render` | Google PDFium prebuilt C++ dynamic library (`libpdfium.so`/`.dylib`) | **`hayro`** / **`pdf-render`** / **`pdfox`** + **`tiny-skia`** | Interprets PDF vector content streams directly into raw pixel framebuffers. Compiles to `wasm32-unknown-unknown`. |
| **Neural AI / NLP Intent Inference**<br>(`paperpilot-nlp` Layer 2) | `ort` (Microsoft ONNX Runtime) | C++ shared libraries (`libonnxruntime.so`, 25MB+) | **`rten`** or **`tract`** *(Sonos)* | 100% pure-Rust neural network inference engines for ONNX models. Fully runs on CPU and WASM with zero external runtime. |
| **HTML ➔ PDF Conversion**<br>(`pdf_convert_html`) | `headless_chrome` | Host Chromium/Chrome executable | **`typst`** / **`resvg`** + **`tiny-skia`** | Typst is a modern, blazing-fast pure-Rust document typesetting engine. Eliminates heavy browser processes. |
| **Office Formats**<br>(`pdf_to_docx`, `pdf_to_xlsx`, `excel_to_pdf`) | `docx-rs`, `rust_xlsxwriter`, `calamine` | **Already Pure Rust ✅** | *(Kept)* | In-memory XML generation and zip compression. Ready for direct WASM edge usage. |
| **Image Encoding / Decoding**<br>(`images_to_pdf`, `extract_images`) | `image` crate (v0.25) | **Already Pure Rust ✅** | `image` | Encodes/decodes PNG, JPEG, WebP, GIF natively without `libpng` or `libjpeg`. |
| **Integrity & Signatures**<br>(`pdf_hash`, `pdf_sign`, `pdf_validate`) | OpenSSL / System crypto | Host OS cryptographic keychains | **`ring`** / **`rustls`** / **`sha2`** | Pure-Rust cryptographic primitives for SHA-256 integrity, RSA/Ed25519 signatures, and self-signed certificates. |

---

## 3. Deep Dive into Pure-Rust Crates

### 3.1. `ocrs` (Pure-Rust OCR)
- **Repository**: `github.com/robertknight/ocrs`
- **Architecture**:
  - Uses modern deep-learning models (DBNet for text detection, sequence-to-sequence for recognition).
  - Powered by **`rten`**, a pure-Rust neural runtime with SIMD optimization.
  - Weights are packaged into compact, memory-mappable `.rten` models (~10MB total).
- **Impact for PaperPilot**:
  - Enables in-browser OCR directly inside `apps/web/` without uploading documents.
  - Eliminates the `apt-get install tesseract-ocr` Linux requirement.

### 3.2. `hayro` & `tiny-skia` (Pure-Rust PDF Rendering)
- **`hayro`**: A modular, pure-Rust PDF rasterizer.
- **`tiny-skia`**: A zero-dependency 2D vector graphics library implementing Skia’s path rasterization algorithms in pure Rust.
- **Impact for PaperPilot**:
  - Enables thumbnail generation and page previews in WASM and Cloudflare Workers.
  - Removes the need to package Google PDFium binaries across Windows, Linux, macOS, and mobile architectures.

### 3.3. `rten` / `tract` (Pure-Rust ONNX Inference)
- **`rten`**: Lightweight, clean neural network engine designed specifically for Rust and WASM.
- **`tract`**: Sonos's industrial-grade pure-Rust ONNX execution engine with INT8 quantization support.
- **Impact for PaperPilot**:
  - Replaces `ort` and the 25MB `libonnxruntime.so` binary.
  - Embeds `TinyBERT` directly into the binary with zero dynamic library dependencies.

### 3.4. `typst` (Pure-Rust Document Engine)
- **Repository**: `github.com/typst/typst`
- **Architecture**: A full document layout and typesetting compiler written from scratch in Rust.
- **Impact for PaperPilot**:
  - Replaces headless Chrome for HTML/Markdown document generation.
  - Produces publication-quality PDFs in milliseconds.

---

## 4. Phased Migration Strategy

### Phase 4 (Current)
- Complete the 12 pure-Rust tools in `paperpilot-wasm` using `lopdf`.
- Deploy the client-side Web App (`apps/web/`) and Cloudflare Workers edge microservice.

### Phase 5.1 (Pure-Rust Image & Hash Port)
- Port `images_to_pdf`, `extract_images`, and `pdf_hash` into `paperpilot-wasm` using `image` and `sha2`.

### Phase 5.2 (Pure-Rust OCR & Inference Migration)
- Replace `tesseract-ocr` with **`ocrs`** for 100% portable on-device and in-browser OCR.
- Benchmark **`rten`** against `ort` for embedded `TinyBERT` intent classification.

### Phase 5.3 (Rendering & Typst Engine)
- Introduce **`hayro` + `tiny-skia`** as the universal pure-Rust page rasterizer for WASM and headless environments.
- Introduce **`typst`** for ultra-fast in-memory Markdown/HTML to PDF generation.

---

## 5. Mandatory Migration Quality Gates & Zero-Regression Rules

> [!CAUTION]
> **NON-NEGOTIABLE CI/CD REQUIREMENT**:
> When replacing any C++ / OS dependency (e.g. Tesseract ➔ `ocrs`, PDFium ➔ `hayro`, ONNX ➔ `rten`, Chrome ➔ `typst`), the entire PaperPilot test matrix MUST continue passing with **100% green parity**. 
> 
> Under NO circumstances may a tool be degraded, stubbed, disabled, or removed from CLI, MCP, REST, or Desktop UI during or after migration.

### Mandatory Verification Harnesses:

Every migration step/PR must execute and pass the following suites before merge:

1. **Tri-Interface Master Verification Harness (`scripts/test_tri_interface_e2e.py`)**:
   - **Target**: All 44 PDF operations.
   - **Verification**: CLI (`paperpilot-cli`), MCP (`paperpilot-mcp`), and REST API (`paperpilot-gateway` on `:7823`).
   - **Required Scorecard**: `44 / 44 CLI (100% PASS)`, `44 / 44 MCP (100% PASS)`, `44 / 44 API (100% PASS)` recorded in `reports/TRI_INTERFACE_E2E_100_VERIFIED.md`.

2. **Desktop UI Playwright Parity Suite (`apps/desktop/tests/e2e_44_operations_parity.spec.ts`)**:
   - **Target**: All 44 PDF operations triggered through the desktop user interface.
   - **Verification**: Search, tool selection, parameter input, operation run, toast verification, and back-navigation.
   - **Required Scorecard**: `44 / 44 PASS (0 Failed)` recorded in `reports/UI_44_OPERATIONS_E2E_SCORECARD.md`.

3. **WASM In-Browser Suite (`apps/web/tests/e2e_wasm_12_tools.spec.ts`)**:
   - **Target**: In-browser WebAssembly execution via Web Worker thread.
   - **Verification**: Valid `%PDF-` binary output and latency benchmarking.
   - **Required Scorecard**: `12 / 12 PASS (0 Failed)` recorded in `reports/WASM_12_TOOLS_E2E_SCORECARD.md`.

4. **Rust Unit & Integration Test Suites**:
   - `cargo test --workspace` must pass 100% with 0 warnings or regressions.
