# Pure-Rust OCR Architecture

## 1. Introduction
The PaperPilot OCR (Optical Character Recognition) pipeline is built entirely in Rust, enabling on-device text recognition with zero external dependencies (no C/C++ libraries such as Tesseract or Leptonica). This ensures perfect cross-platform portability, smaller container sizes, and WASM compatibility.

## 2. Core Components
The architecture relies on the following pure-Rust components:

1. **`ocrs` (v0.10.x)**: A high-performance text detection and recognition library. It handles finding text lines (via DBNet) and decoding character sequences (via a Transformer/CTC decoder).
2. **`rten` & `rten-tensor`**: A lightweight tensor execution runtime built from scratch in Rust, featuring AVX-512/SIMD acceleration. `ocrs` uses `rten` to execute ONNX models locally without requiring Microsoft's ONNX Runtime C++ binaries.
3. **`image`**: Used to process and decode raster streams embedded inside PDFs (XObjects) before passing them to the OCR engine.

## 3. Execution Flow
When a user invokes the OCR operation via CLI, MCP, or the API:
1. **Document Loading**: The PDF is parsed via `lopdf`.
2. **Image Extraction**: The engine scans `XObject` dictionaries for embedded images (`DCTDecode`, `FlateDecode`).
3. **Tensor Conversion**: Images are converted to `NdTensor` layouts expected by `rten-imageproc`.
4. **Inference (DBNet + Recognition)**: The `ocrs` engine maps bounded text segments.
5. **PDF Injection**: Extracted text coordinates are translated into invisible text layers (or plain searchable text).
6. **Fallback Mechanism**: If model weights are missing or a PDF contains no raster layers, the engine safely hands off the document to `ExtractTextOperation`.

## 4. Portability & WASM
Because this pipeline eschews FFI bindings, it can be compiled directly to the `wasm32-unknown-unknown` target. The Web Worker can download quantized `rten` models and run OCR locally inside the browser.
