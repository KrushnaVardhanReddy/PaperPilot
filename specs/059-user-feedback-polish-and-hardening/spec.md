# Spec 059: Real-World User Feedback Polish & Engine Hardening

## 1. Overview & Context
Based on end-user testing and real-world feedback from early adopters, several usability gaps, parameter handling bugs, and engine edge cases have been identified. While the underlying libraries pass headless unit tests, users encounter unexpected behaviors in real workflows (missing output paths, page-chunking limitations, lossless-only compression, and font encoding artifacts).

---

## 2. Issues & Scope of Changes

### Issue 1: `excel_to_json` Missing Output Path Bug (Frontend)
- **Problem**: When running `excel_to_json` in the Desktop GUI, the file is not written to disk. In `apps/desktop/src/lib/services/operationArgsBuilder.ts`, the `output` field is omitted from `args`.
- **Fix**: Supply `output: customOutputPath || getOutputPath(`${docName.replace(/\.[^.]+$/i, '')}.json`)`. Ensure the MCP backend writes the formatted JSON string to this file path if provided.

### Issue 2: `excel_to_csv` Output File Extension (Frontend)
- **Problem**: Converted spreadsheet retains the original `.xlsx` extension or fails to save as `.csv`.
- **Fix**: Guarantee that `operationArgsBuilder.ts` and the file save dialog enforce `.csv` extension and write the active/first worksheet as CSV text.

### Issue 3: `split` — Add "Every N Pages" Chunking
- **Problem**: Users wanting to split a 100-page document into 5-page chunks currently receive 100 individual 1-page documents (burst mode).
- **Fix**:
  - Add `--every <N>` flag to `paperpilot split` CLI.
  - In `paperpilot-pdf/src/operations/split.rs`, compute split points at `N, 2N, 3N...`.
  - In Desktop GUI, add a mode toggle: "Split at specific pages (e.g. 2, 5)" vs. "Split every N pages (e.g. 5)".

### Issue 4: `rotate` — Page Range Selection
- **Problem**: Rotating currently rotates all pages in the PDF. Users frequently need to rotate only specific pages (e.g., page 2 which was scanned upside-down).
- **Fix**:
  - Add `--pages <RANGE>` flag (e.g., `--pages 1,3-5` or default `all`) to `paperpilot rotate`.
  - In `paperpilot-pdf/src/operations/rotate.rs`, apply rotation transform only to the specified subset of pages.
  - In Desktop GUI, add a "Page Range" input field (default: "all").

### Issue 5: `watermark` — Text Auto-Fitting & Wrapping
- **Problem**: Horizontal watermarks with longer text strings clip outside the page boundary.
- **Fix**: Compute page width from `MediaBox`. If text width exceeds 80% of page width, dynamically scale down the font size or wrap text to ensure it remains completely within margins.

### Issue 6: `compress` — Real Downsampling / Image Recompression
- **Problem**: Flate stream compression only achieves 0–2% size reduction on PDFs containing JPEG images.
- **Fix**: In `paperpilot-pdf/src/operations/compress.rs`, detect raster image XObjects (`/Subtype /Image`) and re-encode/downsample them according to `--quality` (high/medium/low: 80%, 60%, 40% JPEG quality) to achieve genuine 40–70% file size reduction.

### Issue 7: `ocr` — In-App Model Pack Guidance
- **Problem**: Without ONNX model files in `~/.paperpilot/models/`, OCR silently produces empty output.
- **Fix**: If OCR models are missing, return a clear, user-friendly error with a one-click "Download OCR Model Pack (18MB)" button in the UI, or auto-download on first use.

### Issue 8: `pdf_to_docx` — Image Extraction & CMap Handling
- **Problem**: Extracted docx is missing images and shows symbols for custom-encoded fonts.
- **Fix**:
  - Extract embedded image XObjects and embed them into the generated `.docx` archive via `docx-rs`.
  - Fall back to glyph heuristic mappings when ToUnicode CMaps are missing in PDF font streams.

### Issue 9: Penta-Interface Parity & AI/NLP/LLM Integrity
- **Mandate**: Any parameter additions (such as `every` in `split` or `pages` in `rotate`) must maintain strict parity across all five interfaces:
  1. **CLI (`paperpilot-cli`)**: Add corresponding `--every` and `--pages` flags.
  2. **MCP (`paperpilot-mcp`)**: Add parameters to tool schema definitions and dispatch arms.
  3. **Gateway / REST (`paperpilot-gateway`)**: Update JSON payload schemas and query params in Axum handlers.
  4. **Desktop GUI (`apps/desktop`)**: Expose input controls in `OperationsPanel.svelte` and map through `operationArgsBuilder.ts`.
  5. **WASM / Edge (`paperpilot-wasm`)**: Support parameters in in-memory functions.
- **AI/NLP/LLM Preservation**: Ensure enhancements to `pdf_rag_chunks`, `pdf_to_markdown`, `pdf_extract_text`, and NLP tools in `paperpilot-nlp` retain clean semantic outputs for LLM prompt ingestion without introducing noise or malformed markup.

---

## 3. Delegation Strategy
- **Antigravity (Immediate / Local)**:
  - Fix Issue 1: `excel_to_json` missing output path in `operationArgsBuilder.ts`.
  - Fix Issue 2: `excel_to_csv` output naming and extension handling.
  - Fix Issue 3: Add `every` parameter to `SplitOperation` and CLI.
  - Fix Issue 4: Add `pages` range support to `RotateOperation` and CLI.
- **Jules (Asynchronous Deep Engine Hardening)**:
  - Issue 5: Watermark text auto-fitting / wrapping.
  - Issue 6: Image downsampling and raster recompression in `compress.rs`.
  - Issue 7: OCR missing model detection & guided downloading.
  - Issue 8: PDF to DOCX image extraction and CMap reconstruction.

---

## 4. Verification & Criteria
- [ ] `excel_to_json` writes genuine `.json` file to disk.
- [ ] `excel_to_csv` writes genuine `.csv` file to disk.
- [ ] `split --every 5` splits a 10-page document into two 5-page PDFs (`split_1.pdf` and `split_2.pdf`).
- [ ] `rotate --angle 90 --pages 2` rotates only page 2 of a multi-page PDF.
- [ ] Unit and E2E tests pass cleanly with `-j 4`.
