# Spec 044 — High-Precision Pure-Rust OCR Engine via `rusto-rs` (RapidOCR / PaddleOCR + RTen)

## 1. Overview & Problem Statement
Currently, PaperPilot's OCR operation (`paperpilot-pdf/src/operations/ocr.rs`) uses `ocrs` backed by `rten`. While this preserved the pure-Rust architecture (zero C++ / zero ONNX Runtime `ort`), real-world production scans (such as the USCIS I-140 approval notice) exposed several limitations:
1. **Lack of Word-Level Bounding Boxes**: `ocrs` outputs flat text blocks without accurate spatial geometry coordinates. As a result, recognized text cannot be accurately positioned under scanned visual words, preventing intuitive mouse-drag selection and cursor copying in PDF viewers.
2. **Inference Latency on High-Resolution Slices**: `ocrs` takes 10–25s per sliced image strip in unoptimized builds. Documents containing multiple slices or high-DPI scans can take minutes.
3. **Orientation & Contour Sensitivity**: Documents with angled text, multi-column tables, or faint stamps suffer from low recognition accuracy without DBNet polygon contour detection.

### The Solution: `rusto-rs`
`rusto-rs` is a high-performance, cross-platform, pure-Rust OCR engine and toolkit based on **RapidOCR** and powered by **PaddleOCR v4** models.
- **Pure Rust Runtime**: Utilizes the pure-Rust **RTen** inference engine—requiring zero C++, zero CMake, and zero `ort` dynamic libraries.
- **Exact Spatial Geometry**: Provides DBNet polygon contour detection and word-level bounding box coordinates (`x, y, width, height, polygon`).
- **Sub-Second Performance**: Highly optimized vector math with SIMD acceleration (AVX-512, AVX2, NEON) for sub-second inference per page.
- **Transparent Selectable Text Layer**: Enables PaperPilot to inject an invisible text character layer directly matching each visual word's exact spatial coordinates on the scanned PDF page, making mouse selection and copy-paste identical to native digital PDFs.

---

## 2. Architecture & Pipeline

```
[Scanned PDF Page] 
       │
       ▼
[lopdf Page Extraction] ──▶ Extracts XObject Image / Bitmaps
       │
       ▼
[`rusto-rs` Engine]
   ├─ DBNet Detection (Polygon Contours & Word Boxes)
   ├─ Direction/Orientation Classifier (Auto-rotate text)
   └─ Recognition Model (PaddleOCR v4 / RTen weights)
       │
       ▼
[Spatial Word Bounding Boxes: (text, x, y, width, height, confidence)]
       │
       ▼
[lopdf Content Stream Injection]
   ├─ Font: Helvetica / TrueType standard font (`/F1`)
   ├─ Graphics State: Transparent overlay (`/ExtGState << /ca 0.001 /CA 0.001 >>`)
   └─ Exact Coordinate Transform: `Tm` + `Tj` for each word bounding box
       │
       ▼
[Searchable & Selectable Output PDF (`_ocr.pdf`) / Clean Text File (`_ocr.txt`)]
```

---

## 3. Detailed Technical Requirements

### 3.1 Model Management & Zero External Bloat
- Support dual model formats: `.rten` (FlatBuffers) and `.onnx`.
- Store models in standard user-space directories:
  1. `$PAPERPILOT_OCR_MODELS_DIR`
  2. `~/.local/share/paperpilot/models/ocr/`
  3. Workspace `models/ocr/`
- Automatic fallback: If neural models are missing, gracefully fall back to embedded font text extraction without crashing.

### 3.2 Exact Word-Level PDF Text Layer Injection
For each detected word `(text, x, y, w, h)`:
1. Translate image pixel coordinates into PDF point coordinates:
   $$\text{pdf\_x} = \frac{x}{\text{img\_width}} \times \text{page\_width}$$
   $$\text{pdf\_y} = \text{page\_height} - \left(\frac{y + h}{\text{img\_height}} \times \text{page\_height}\right)$$
2. Compute matching font scale:
   $$\text{font\_size} = \frac{h}{\text{img\_height}} \times \text{page\_height}$$
3. Inject the text using Lopdf stream operations:
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
4. This guarantees that dragging the mouse cursor over any visual word in Acrobat, Chrome, Preview, or Evince selects that exact word.

---

## 4. Penta-Interface Support
- **CLI**:
  - `paperpilot ocr --input scanned.pdf --output searchable.pdf`
  - `paperpilot ocr --input scanned.pdf --output text.txt --format json` (includes bounding boxes)
- **MCP (`paperpilot-mcp`)**:
  - `pdf_ocr`: Arguments `{ input, output, language? }`
- **REST Gateway (`paperpilot-gateway`)**:
  - `POST /api/v1/pdf/ocr`
- **Desktop UI (`apps/desktop`)**:
  - Operations sidebar "OCR Text Recognition" defaults to `${base}_ocr.pdf`.
  - Progress indicator showing page-by-page OCR completion.
- **WASM (`paperpilot-wasm`)**:
  - Compatible with WASM through pure-Rust `rten` inference.

---

## 5. Acceptance Criteria
1. `paperpilot-pdf` integrates `rusto-rs` without introducing any C++ or `ort` dependencies.
2. High-speed OCR: Real-world scanned documents (including multi-strip scans like the I-140 approval notice) process under 5 seconds per page on local CPU.
3. Word-level cursor selection: Clicking and dragging over any visual word in the generated `_ocr.pdf` selects and copies the correct text.
4. All unit, integration, and E2E tests pass under `cargo test -j 6`.
5. Author completion report at `reports/RUSTO_RS_OCR_MIGRATION_REPORT.md`.
