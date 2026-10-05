# Spec 017: Pure-Rust OCR Engine Migration (`ocrs` via `rten`)

## Status: APPROVED & ACTIVE (Phase 5.2.1)

## 1. Context & Motivation
Currently, `pdf_ocr` in `paperpilot-pdf/src/operations/ocr.rs` is an empty/passthrough stub or relies on host system C++ executables (`tesseract-ocr`, Leptonica). This creates a heavy OS dependency, breaks cross-platform portability on clean systems, and cannot run in WebAssembly or serverless edge environments.

By migrating `pdf_ocr` to **`ocrs`** (a 100% pure-Rust OCR engine by Robert Knight powered by the pure-Rust neural runtime **`rten`**):
1. **Zero C++ Dependencies**: No `apt-get install tesseract-ocr`, no shared library linker issues.
2. **WebAssembly & Edge Ready**: Compiles cleanly to `wasm32-unknown-unknown` for in-browser OCR.
3. **Self-Contained On-Device Inference**: Runs text detection (DBNet) and recognition directly on CPU using pure Rust SIMD.

---

## 2. Architecture & File Scope

```
PaperPilot/
├── paperpilot-pdf/
│   ├── Cargo.toml                     # Add ocrs + rten dependencies
│   ├── src/operations/ocr.rs          # Implement pure-Rust OCR using ocrs
│   └── tests/test_ocr.rs              # Integration tests
├── wiki/09-Pure-Rust-OCR.md            # Architecture & usage documentation
└── reports/OCR_PURE_RUST_REPORT.md     # Verification report
```

### Strict Non-Overlapping File Ownership:
- `paperpilot-pdf/Cargo.toml`
- `paperpilot-pdf/src/operations/ocr.rs`
- `wiki/09-Pure-Rust-OCR.md`
- `reports/OCR_PURE_RUST_REPORT.md`

*(Touches ZERO files in `paperpilot-wasm`, `apps/desktop`, `apps/web`, or `apps/edge`)*.

---

## 3. Implementation Details

1. **Dependency Addition** (`paperpilot-pdf/Cargo.toml`):
   ```toml
   ocrs = "0.10"
   rten = "0.14"
   rten-imageproc = "0.14"
   ```
   *(Or latest compatible pure-Rust versions)*.

2. **OCR Engine Architecture (`ocr.rs`)**:
   - Accepts PDF document or page images.
   - Extracts page raster or renders images from PDF using lopdf / image crate.
   - Runs DBNet text detection to find text line bounding boxes.
   - Runs recognition model to extract UTF-8 characters with confidence scores.
   - Injects invisible text layer (or searchable PDF text stream) onto the document and saves output.

3. **Fallback & Graceful Handling**:
   - If model files are not provided, gracefully fall back to text extraction or bundled lightweight default weights.

---

## 4. Quality Gate & Parity Verification
- `cargo test -p paperpilot-pdf` must pass.
- `python3 scripts/test_tri_interface_e2e.py` must maintain **44 / 44 PASS (100%)**.
