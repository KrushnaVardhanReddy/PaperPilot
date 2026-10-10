# Spec 044 — High-Precision Pure-Rust OCR Engine via `rusto-rs` (RapidOCR / PaddleOCR + RTen)

## 1. Executive Summary & Problem Analysis

In PaperPilot Phase 5, the OCR operation (`paperpilot-pdf/src/operations/ocr.rs`) was initially prototyped using `ocrs` backed by `rten` to ensure a 100% pure-Rust architecture (zero C++, zero ONNX Runtime `ort`).

However, testing against real-world scanned legal and government documents (such as the USCIS Form I-140 approval notice) identified three fundamental architectural bottlenecks:
1. **Lack of Word-Level Geometry for PDF Text Overlay**:
   `ocrs` only exposes line-level and text-block structures without exact word-level polygon/quad coordinates. Placing text at coarse approximations results in PDFs that are technically searchable by string matching, but impossible to select and copy intuitively with mouse cursor drag-and-drop in PDF viewers (Acrobat, Chrome, Preview, Evince).
2. **Inference Latency on Sliced Image Strips**:
   Scanned PDFs frequently compress pages into dozens of individual vertical/horizontal image slices (e.g., 26 image strips on the I-140 test notice). Processing 26 neural passes through `ocrs` in debug or unoptimized builds requires multiple minutes per page.
3. **Contour Distortion & Multi-Column Flow**:
   Documents with multi-column text, official stamps, headers, and varying font weights suffer from character dropouts without DBNet (Differentiable Binarization Neural Network) polygon contour detection.

### The Solution: `rusto-rs`
`rusto-rs` is a high-performance, cross-platform pure-Rust OCR engine based on **RapidOCR** and powered by **PaddleOCR v4 / v5 / v6** models:
- **Pure-Rust & Zero C++**: Runs directly on the pure-Rust `rten` tensor engine. Zero CMake, zero OpenCV, and zero dynamic library linking.
- **Word-Level Polygon & Bounding Box Detection**: Emits exact coordinates `(x, y, width, height, polygon)` for every detected word.
- **Sub-Second Performance**: Employs optimized SIMD vector math (AVX-512, AVX2, NEON) and parallel line recognition, completing full page OCR in under 2 seconds.
- **True 1:1 Transparent Text Layer**: Allows PaperPilot to project an invisible font character glyph directly over each visual word in the PDF using standard `/ExtGState` transparency, making the PDF natively mouse-selectable and copyable.

---

## 2. Architecture & Data Pipeline

```
┌────────────────────────────────────────────────────────┐
│                   Scanned PDF Input                    │
└──────────────────────────┬─────────────────────────────┘
                           │
                           ▼
┌────────────────────────────────────────────────────────┐
│  lopdf Image Extractor / Rasterizer                    │
│  - Extracts /XObject images or renders page strip      │
│  - Luminance check: skips blank strips (< 0.2% non-white)│
│  - Downsamples excessive DPI (> 1500px)                │
└──────────────────────────┬─────────────────────────────┘
                           │ DynamicImage (RGB8)
                           ▼
┌────────────────────────────────────────────────────────┐
│  `rusto-rs` Engine Pipeline                            │
│  1. DBNet Text Detection (Polygon contours)            │
│  2. Direction / Angle Classifier (Auto-orientation)   │
│  3. PaddleOCR Text Recognizer (rten tensor weights)    │
└──────────────────────────┬─────────────────────────────┘
                           │ Vec<DetectedWord { text, box: [x,y,w,h], score }>
                           ▼
┌────────────────────────────────────────────────────────┐
│  PDF Coordinate Space Normalization                    │
│  - Pixel space (top-left) -> PDF MediaBox (bottom-left)│
│  - pdf_x = (x / img_w) * page_w                        │
│  - pdf_y = page_h - ((y + h) / img_h) * page_h         │
│  - font_size = (h / img_h) * page_h                    │
└──────────────────────────┬─────────────────────────────┘
                           │
                           ▼
┌────────────────────────────────────────────────────────┐
│  lopdf Content Stream Text Overlay                     │
│  - Resource dictionary: /ExtGState << /ca 0.001 >>    │
│  - Resource dictionary: /Font << /F1 /Helvetica >>     │
│  - Stream: BT /GS_OCR gs /F1 {font_size} Tf Tm Tj ET   │
└──────────────────────────┬─────────────────────────────┘
                           │
                           ▼
┌────────────────────────────────────────────────────────┐
│  Output File:                                          │
│  - If target is PDF: Searchable & Selectable PDF       │
│  - If target is TXT: Spatially sorted plaintext stream │
└────────────────────────────────────────────────────────┘
```

---

## 3. Detailed Technical Requirements

### 3.1 Dependencies & Workspace Configuration
- Add `rusto-rs = "0.3"` to `paperpilot-pdf/Cargo.toml`.
- Configure features for pure-Rust mode (default `image` processing, without OpenCV or MNN C++ backend).
- In root `Cargo.toml`, maintain release/dev profile optimizations:
  ```toml
  [profile.dev.package.rten]
  opt-level = 3
  [profile.dev.package.rten-tensor]
  opt-level = 3
  [profile.dev.package.rusto-rs]
  opt-level = 3
  ```

### 3.2 Model Management & Discovery
`rusto-rs` requires three lightweight model files:
1. `ch_PP-OCRv4_det.rten` (or v5/v6 detection model, ~2.5 MB)
2. `ch_PP-OCRv4_rec.rten` (recognition model, ~10 MB)
3. `ppocr_keys_v1.txt` (dictionary character map, ~30 KB)

The engine must resolve models in the following priority order:
1. Custom directory specified via environment variable `PAPERPILOT_OCR_MODELS_DIR`.
2. Standard user directory: `~/.local/share/paperpilot/models/ocr/`.
3. Workspace directory: `models/ocr/`.
4. If neural model files are not found, return a descriptive error or fall back to extracting existing embedded text without crashing.

### 3.3 Word-Level Coordinate Transformation
Given:
- Source image strip dimensions: `img_w`, `img_h`
- Image placement on page: `rect = [x0, y0, x1, y1]`
- Page dimensions: `page_w = MediaBox.width`, `page_h = MediaBox.height`
- Detected word bounding box: `box = [box_x, box_y, box_w, box_h]`

Transformation Formulas:
$$\text{norm\_x} = \frac{\text{box\_x}}{\text{img\_w}}, \quad \text{norm\_y} = \frac{\text{box\_y}}{\text{img\_h}}$$
$$\text{norm\_w} = \frac{\text{box\_w}}{\text{img\_w}}, \quad \text{norm\_h} = \frac{\text{box\_h}}{\text{img\_h}}$$

PDF Page Coordinates:
$$\text{strip\_w} = x_1 - x_0, \quad \text{strip\_h} = y_1 - y_0$$
$$\text{pdf\_x} = x_0 + (\text{norm\_x} \times \text{strip\_w})$$
$$\text{pdf\_y} = y_0 + (\text{strip\_h} - ((\text{norm\_y} + \text{norm\_h}) \times \text{strip\_h}))$$
$$\text{font\_size} = \max(6.0, \text{norm\_h} \times \text{strip\_h})$$

### 3.4 PDF Stream Injection Syntax
Inject an invisible graphics state `/GS_OCR` into each page's resource dictionary:
```pdf
/Resources <<
  /ExtGState <<
    /GS_OCR << /Type /ExtGState /ca 0.001 /CA 0.001 >>
  >>
  /Font <<
    /F1 << /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>
  >>
>>
```
For each detected word:
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
*Note: Using `/ca 0.001` (nearly zero alpha) instead of `3 Tr` ensures that modern PDF renderers maintain bounding-box geometry for mouse cursor hit-testing and drag selection, whereas `3 Tr` is discarded by some viewer selection engines.*

### 3.5 Spatially Sorted Text Output (`.txt`)
When the target output is `.txt`:
1. Group detected words into lines based on vertical overlap:
   $$\Delta y < 0.5 \times \text{font\_size}$$
2. Sort lines from top to bottom (`pdf_y` descending).
3. Sort words within each line from left to right (`pdf_x` ascending).
4. Join words with single spaces, and lines with newlines.

---

## 4. Penta-Interface Implementation Matrix

| Interface | Command / Route / Call | Input | Output |
|---|---|---|---|
| **CLI** | `paperpilot ocr -i scan.pdf -o out_ocr.pdf` | Scanned PDF | Searchable & selectable PDF |
| **CLI (Text)**| `paperpilot ocr -i scan.pdf -o out.txt` | Scanned PDF | Formatted text stream |
| **MCP** | `pdf_ocr {"input": "...", "output": "..."}` | JSON params | Formatted output + JSON response |
| **Gateway** | `POST /api/v1/pdf/ocr` | Multipart / Path | Downloadable OCR PDF |
| **Desktop** | UI Operations Sidebar: "OCR Text Recognition" | File selection | Default `${base}_ocr.pdf` |
| **WASM** | `paperpilot_wasm::ocr()` | `Uint8Array` | `Uint8Array` PDF |

---

## 5. Acceptance Criteria & Verification Plan

1. **Pure Rust Invariant**: Zero C++ dependencies or shared libraries (`ort`, `opencv`, etc.) in `cargo tree`.
2. **Sub-Second Performance**: A standard 300 DPI page processes in under 5 seconds with `-j 6`.
3. **Word-Level Mouse Selection**: In `/home/krushna/Downloads/I-140_approval_notice_ocr.pdf`, clicking and dragging over text lines (e.g. "USCIS", "Form I-140", "Receipt Number") highlights and copies the exact words.
4. **All Tests Pass**: `cargo test -p paperpilot-pdf --lib ocr -j 6` passes with zero regressions.
5. **Report Produced**: `reports/RUSTO_RS_OCR_MIGRATION_REPORT.md` documenting performance benchmarks, accuracy metrics, and coordinate precision.
