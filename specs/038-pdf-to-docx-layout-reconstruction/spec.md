# Spec 038 — High-Fidelity Pure-Rust PDF-to-Word (DOCX) Layout & Style Reconstruction

## 1. Overview & Problem Statement
Currently, `paperpilot` provides `pdf_to_docx` via `paperpilot_pdf::operations::conversion::PdfToDocxOperation`. However, the current implementation performs an unformatted raw text dump:
- It strips all alignment (titles, headers, centered text become left-aligned).
- It drops font weights (bold, regular, light) and font sizes (headings look identical to body text).
- It ignores font colors and text formatting from the PDF graphics state.
- It omits embedded images and tables.
- It inserts an awkward plain string `"--- PAGE BREAK ---"` instead of true OpenXML page breaks.

As a result, converted Word documents look plain and unstyled compared to the original PDF.

**Objective:** Upgrade `PdfToDocxOperation` into a **High-Fidelity Pure-Rust Document Layout Reconstruction Engine** using `docx-rs`, `lopdf`, and `image` to preserve structural typography, visual styling, colors, alignment, and page breaks without relying on LibreOffice or cloud converters.

---

## 2. Core Architecture & Reconstruction Mechanics

### 2.1 Paragraph & Typography Analysis
From PDF page text extraction and stream traversal:
1. **Font Size & Heading Inference**:
   - Detect font size changes (`Tf` operator / text matrices).
   - Infer hierarchy:
     - `size >= 20pt`: Heading 1 (`<w:pStyle w:val="Heading1"/>`, bold, size 36-48 half-points).
     - `14pt <= size < 20pt`: Heading 2 (`<w:pStyle w:val="Heading2"/>`, bold, size 28-36 half-points).
     - `size < 14pt`: Body text (`size` matched in half-points).
2. **Text Weight & Emphasis**:
   - Inspect font names for `/Bold`, `/Black`, `/Heavy` ➔ apply `.bold()` on `Run`.
   - Inspect font names for `/Italic`, `/Oblique` ➔ apply `.italic()` on `Run`.
3. **Color Extraction**:
   - Parse RGB/Grayscale operators (`rg`, `g`, `k`) in the content stream.
   - Convert RGB to hex string (e.g. `1.0 0.0 0.0 rg` ➔ `"FF0000"`) and apply `.color(hex)` to `Run`.

### 2.2 Spatial Alignment & Layout
1. **Margin & MediaBox Calculations**:
   - Read page `MediaBox` width $W$ and height $H$.
   - Calculate line bounding box / start position $X_{start}$ and end position $X_{end}$:
     - If line is centered within tolerance $(\pm 15\% \text{ of page center } W/2)$ ➔ set `.align(AlignmentType::Center)`.
     - If line starts near right margin $(X_{start} > 0.6 \times W)$ ➔ set `.align(AlignmentType::Right)`.
     - Otherwise ➔ default `.align(AlignmentType::Left)`.
2. **Native Page Breaks**:
   - Replace literal `"--- PAGE BREAK ---"` text with true OpenXML page breaks:
     ```rust
     docx = docx.add_paragraph(Paragraph::new().page_break_before(true));
     ```
     or add `Run::new().add_page_break()`.

### 2.3 Embedded Images & Visual Media
1. **Embedded Image Traversal**:
   - For each page, iterate over `/Resources` `/XObject` with `/Subtype /Image`.
   - Extract raw image bytes (JPEG, PNG).
   - Insert image into Word document using `docx-rs`'s `Pic::new(&image_bytes)` attached to a paragraph.

---

## 3. Penta-Interface Mandate & E2E Protocol Integration

Per [`wiki/25-Penta-Interface-Mandate-And-E2E-Protocol.md`](../../wiki/25-Penta-Interface-Mandate-And-E2E-Protocol.md), this upgraded converter is supported across all **5 deployment surfaces**:

1. **💻 CLI (`paperpilot-cli`)**:
   ```bash
   paperpilot convert --input document.pdf --format docx --output document.docx
   ```
2. **🤖 MCP Server (`paperpilot-mcp`)**:
   Tool `pdf_to_docx` with `{ "input": "...", "output": "..." }`.
3. **🌐 REST Gateway (`paperpilot-gateway`)**:
   Endpoint `POST /api/v1/pdf/convert` with `{ "input": "...", "output": "...", "format": "docx" }`.
4. **⚡ WASM (`paperpilot-wasm`)**:
   Function `pdf_to_docx(pdf_bytes: &[u8]) -> Result<Vec<u8>, JsValue>`.
5. **☁️ Cloudflare Edge (`apps/edge`)**:
   Endpoint `POST /api/v1/convert?format=docx`.

---

## 4. UI Integration (`apps/desktop`)
- In `PdfChatPanel.svelte` and `GlobalCommandBar.svelte`, natural commands like:
  - `"convert the pdf to word"`
  - `"convert to word"`
  - `"export to docx"`
  Seamlessly map to `pdf_to_docx` with active document context and produce a styled Word document.

---

## 5. Verification & Acceptance Criteria
1. **Compilation & Clippy**:
   - `cargo clippy --workspace --all-targets --all-features -- -D warnings` must pass with zero warnings.
2. **Unit Tests**:
   - `cargo test -p paperpilot-pdf -- docx` validates that:
     - Output is a valid ZIP/OpenXML file readable by `zip::ZipArchive`.
     - Contains `word/document.xml`.
     - Does NOT contain the string `"--- PAGE BREAK ---"`.
     - Preserves headings and bold tags.
3. **Penta-Interface E2E Tests**:
   - 20 assertions in `tools/penta-interface-e2e` for `pdf_to_docx` pass.
4. **Report**:
   - Generate `reports/PDF_TO_DOCX_LAYOUT_RECONSTRUCTION_REPORT.md` documenting fidelity improvements, benchmark timings, and test outputs.
