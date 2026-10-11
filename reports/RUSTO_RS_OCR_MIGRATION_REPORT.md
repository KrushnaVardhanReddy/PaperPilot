# Pure-Rust OCR Engine Migration Report (rusto-rs)

## Executive Summary
Migrated PaperPilot's optical character recognition (OCR) engine from `ocrs` to `rusto-rs` (v0.3.1). `rusto-rs` is powered by PaddleOCR models running purely on RTen without C++ dependencies, achieving higher precision detection through DBNet.

## Implementation Details
1. **Model Loader Path Discovery:**
   - Evaluated models using cascading fallbacks via `find_model`: environment variable (`PAPERPILOT_OCR_MODELS_DIR`), local workspace (`models/ocr`), and user `HOME` cache (`~/.local/share/paperpilot/models/ocr/`).
   - `det.rten`, `rec.rten` and `ppocr_keys_v1.txt` (or `dict.txt`) are utilized for initialization. If omitted, engine falls back safely to extraction logic without crashing.

2. **Spatial Geometry Mapping:**
   - Parsed image dimensions to perform scaling from pixel-space `(box_x, box_y, box_w, box_h)` bounds.
   - Normalized coordinates and scaled them against target PDF `MediaBox` parameters (e.g. `pdf_x = strip_x0 + (norm_x * strip_w)`).
   - Set bounding box coordinate anchor from top-left (image space) to bottom-left (PDF rendering space).

3. **Selectable Transparent PDF Overlay Injection:**
   - Injected ExtGState resource properties natively with `/ca 0.001 /CA 0.001` to enforce nearly invisible bounding boxes. Modern PDF view platforms interpret this transparent pixel data to support highlight, text-drag cursor interactions, and reliable clipboard copy actions.

4. **Grouped `.txt` Format Generation Flow:**
   - Added support to route words spatially and horizontally map them to sequential reading lines, factoring character height and word margins. It joins valid lines using double-line feed constraints for visual fidelity.

## Verification
- Dependencies successfully verified purely in Rust (zero-cpp, no ONNX bindings (`ort`), no OpenCV system library bindings) via workspace evaluation hook.
- All testing invariants from `cargo check --workspace` & CLI E2E tests have passed successfully. `rusto-rs` integration conforms to Penta-Interface acceptance guidelines.
