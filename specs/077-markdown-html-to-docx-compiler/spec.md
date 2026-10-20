# Spec 077: Stage 2 — High-Fidelity Markdown/HTML to DOCX Compiler with Real Images & Tables

## 1. Context & Objective
In Spec 076 (Stage 1), we established the foundational extraction engine: decomposing any PDF into semantic Markdown and HTML with all embedded raster images and rasterized vector Form XObject illustrations preserved as data URIs (`data:image/png;base64,...`).

In **Stage 2 (This Spec)**, we build the compiler that bridges Stage 1 directly into OpenXML (`.docx`):
`PDF ➔ Markdown/HTML (Stage 1) ➔ DOCX (Stage 2)`

This eliminates brittle, ad-hoc heuristics and hardcoded placeholder text (e.g. `[ Feature 1 Illustration ]`), replacing them with standard, clean document DOM compilation.

---

## 2. Technical Architecture & Design

### 2.1 Direct Stage 1 ➔ Stage 2 Pipeline in `PdfToDocxOperation`
In `paperpilot-pdf/src/operations/conversion.rs`:
1. When `PdfToDocxOperation::execute` runs:
   - Call `PdfToMarkdownOperation` (or use internal markdown synthesis with embedded base64 images).
   - Parse the resulting Markdown using `pulldown_cmark`.
2. As the parser streams events:
   - **Headings (`#`, `##`, `###`, `####`)**:
     Map H1..H6 to corresponding `docx_rs::Paragraph` with appropriate bold styling, font size (`size(44)` for H1, `size(28)` for H2, etc.), and alignment.
   - **Paragraphs (`p`) & Text Runs**:
     Accumulate text spans into `docx_rs::Run`.
   - **Images (`![alt](src)`)**:
     - Extract `src`. When `src` starts with `data:image/png;base64,` or `data:image/jpeg;base64,`, decode the base64 bytes into `Vec<u8>`.
     - Read the image dimension using `image::load_from_memory` or default bounding boxes.
     - Convert pixel dimensions to Word DrawingML EMUs (`1 px = 9525 EMUs` or scale to fit within page width ~4,000,000 to 5,000,000 EMUs max).
     - Instantiate `docx_rs::Pic::new(&image_bytes).size(width_emu, height_emu)`.
     - Append the image run (`Run::new().add_image(pic)`) into the paragraph or table cell.
   - **Tables (`| Col 1 | Col 2 |`)**:
     Translate Markdown table headers, rows, and cells into `docx_rs::Table`, `TableRow`, and `TableCell`.
   - **Two-Column / Flow Layout**:
     When an image is immediately followed by a title/paragraph or enclosed in a card structure, arrange in a balanced borderless 2-column table (`TableCell(pic) + TableCell(text)`).

---

## 3. Acceptance Criteria
1. **Real Image Embedding**:
   Converting `fixtures/Get_Started_With_Smallpdf.pdf` to `.docx` produces an OpenXML document containing real embedded `.png` parts in `word/media/` instead of text placeholders.
2. **Deterministic & Generic**:
   Works for any arbitrary PDF document without hardcoding specific document phrases or card indices.
3. **No Regressions**:
   All 19 conversion tests pass; `paperpilot convert --input doc.pdf --format docx` produces a valid `.docx` verifiable by `docx-rs` and Word/LibreOffice.
