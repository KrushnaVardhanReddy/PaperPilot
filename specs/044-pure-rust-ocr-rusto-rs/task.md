# Task 5.9.19 — Implementation Plan: Pure-Rust OCR Engine Migration (`rusto-rs`)

## Status
- **Status**: Ready for Execution
- **Spec**: [specs/044-pure-rust-ocr-rusto-rs/spec.md](file:///home/krushna/Project/PaperPilot/specs/044-pure-rust-ocr-rusto-rs/spec.md)
- **Prompt**: [prompts/tasks/phase5/P5_9_19_rusto_rs_ocr_migration.txt](file:///home/krushna/Project/PaperPilot/prompts/tasks/phase5/P5_9_19_rusto_rs_ocr_migration.txt)
- **Assigned Engine**: Jules / Autonomous Agent

---

## 1. Objectives & Deliverables
1. Integrate `rusto-rs` (v0.3) in `paperpilot-pdf` with pure-Rust runtime (`rten` backend, zero OpenCV, zero C++ FFI).
2. Replace `ocrs::OcrEngine` pipeline in `paperpilot-pdf/src/operations/ocr.rs` with `rusto-rs` session execution.
3. Implement DBNet word-level polygon/quad coordinates mapping to PDF point units.
4. Inject transparent overlay `/ExtGState << /ca 0.001 /CA 0.001 >>` so every detected word is directly mouse-selectable and copyable in PDF viewers.
5. Support multi-line spatial layout for plain text targets (`.txt`).
6. Maintain multi-directory neural model discovery:
   - `$PAPERPILOT_OCR_MODELS_DIR`
   - `~/.local/share/paperpilot/models/ocr/`
   - `models/ocr/`
7. Ensure all operations pass under `-j 6` constraint and produce completion report `reports/RUSTO_RS_OCR_MIGRATION_REPORT.md`.

---

## 2. Step-by-Step Implementation Breakdown

### Step 1: Cargo Workspace & Dependency Configuration
- Edit `paperpilot-pdf/Cargo.toml`:
  - Add `rusto-rs = { version = "0.3", default-features = true }`.
  - Verify absence of `opencv`, `ort`, or C++ build scripts via `cargo tree -p rusto-rs`.
- Edit root `Cargo.toml`:
  - Ensure dev and release profiles specify `opt-level = 3` for `rusto-rs`, `rten`, and `rten-tensor`.

### Step 2: Model Management & Initialization
- Create/update model loader in `paperpilot-pdf/src/operations/ocr.rs`:
  - Locate `det.rten`, `rec.rten`, and `dict.txt` (or PaddleOCR v4 keys).
  - Initialize `rusto::RustO::initialize(config)`.
  - Handle graceful fallback: if models are not present, perform embedded text extraction without panicking.

### Step 3: Image Extraction & Preprocessing Optimization
- Extract images from `lopdf` page XObjects.
- Filter out blank/solid white strips:
  - Calculate luminance on sample pixels; if > 99.8% white, skip neural inference instantly.
- Cap strip width to 1500px to maintain sub-second inference speeds.

### Step 4: Word Bounding-Box Detection & Coordinate Mapping
- Run `ocr.detect_text(&image_source, &run_options)`.
- Extract detected word bounding boxes `(text, x, y, w, h)`.
- Transform image pixel space (top-left origin) to PDF MediaBox point space (bottom-left origin):
  ```rust
  let norm_x = word.x as f32 / img_width as f32;
  let norm_y = word.y as f32 / img_height as f32;
  let norm_w = word.w as f32 / img_width as f32;
  let norm_h = word.h as f32 / img_height as f32;

  let pdf_x = strip_x0 + (norm_x * strip_w);
  let pdf_y = strip_y0 + (strip_h - ((norm_y + norm_h) * strip_h));
  let font_size = (norm_h * strip_h).max(6.0);
  ```

### Step 5: PDF Transparent Text Layer Injection
- Ensure page resources include:
  - `/ExtGState << /GS_OCR << /Type /ExtGState /ca 0.001 /CA 0.001 >> >>`
  - `/Font << /F1 << /Type /Font /Subtype /Type1 /BaseFont /Helvetica >> >>`
- Write content stream operations for each word:
  ```pdf
  q
  /GS_OCR gs
  BT
  /F1 {font_size} Tf
  1 0 0 1 {pdf_x} {pdf_y} Tm
  ({escaped_text}) Tj
  ET
  Q
  ```

### Step 6: Spatial Text Extraction (`.txt`)
- Group words into horizontal lines based on y-coordinate proximity.
- Sort lines top-to-bottom and words left-to-right.
- Output clean text formatted with natural line breaks.

### Step 7: Penta-Interface & Verification
- Verify CLI: `cargo run -p paperpilot-cli -j 6 -- ocr --input <scan> --output <out_ocr.pdf>`
- Verify MCP: `cargo test -p paperpilot-mcp -j 6`
- Verify Gateway: `cargo test -p paperpilot-gateway -j 6`
- Verify Desktop UI: Confirm operations panel binds output to `${base}_ocr.pdf`.
- Compile and test WASM: `make build-wasm` / `wasm-pack build crates/paperpilot-wasm`.

---

## 3. Concurrency & Performance Rules
- **MAX CORES**: All cargo builds and tests MUST include `-j 6`.
- **NO PYTHON**: Never use python scripts to verify OCR or PDF outputs. Use `paperpilot-cli` or Rust integration tests.
- **ZERO C++**: Maintain strict pure-Rust invariant.
