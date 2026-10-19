# Task 5.9.52 — Implementation Plan: PDF-to-DOCX High-Fidelity Layout Synthesis & Form XObject Vector Image Extraction

## Status
- **Status**: Dispatched (Jules Session 11144603765719496889)
- **Spec**: [specs/074-pdf-to-docx-high-fidelity-layout-and-images/spec.md](file:///home/krushna/Project/PaperPilot/specs/074-pdf-to-docx-high-fidelity-layout-and-images/spec.md)
- **Prompt**: [prompts/tasks/done/phase5/P5_9_52_pdf_to_docx_high_fidelity_images.txt](file:///home/krushna/Project/PaperPilot/prompts/tasks/done/phase5/P5_9_52_pdf_to_docx_high_fidelity_images.txt)
- **Assigned Engine**: Jules / Autonomous Agent
- **Target Branch**: `feat-pdf-to-docx-high-fidelity-images`

---

## 1. Problem Definition & The Form XObject Issue
In marketing and promotional PDFs (e.g. `fixtures/Get_Started_With_Smallpdf.pdf`), card icons and illustrations are **not raster image streams** (`/Subtype /Image`). Instead, they are defined in `/Resources -> /XObject` as **PDF Form XObjects (`/Fm0`–`/Fm4`) containing raw vector bezier path operators (`m`, `l`, `c`, `f`)**.

As a result:
1. Standard raster image extractors find 0 images.
2. Text/Markdown parsers (`unpdf`, `pulldown-cmark`) extract headings and descriptions, but leave the visual column cells empty.
3. In `fixtures/Get_Started_With_Smallpdf_EXPECTED.docx`, the layout consists of 4 two-column borderless tables (`<w:tbl>`) where column 1 has width `3168 dxa` (for the illustration) and column 2 has width `6912 dxa` (for title and body text).

---

## 2. Objectives & Deliverables
1. **Form XObject & Image Handling**:
   - Inspect `/Resources -> /XObject` for both `/Subtype /Image` and `/Subtype /Form`.
   - For raster images: extract stream bytes and embed via `docx_rs::Pic::new(&png_bytes)`.
   - For vector Form XObjects: rasterize the XObject's bounding box to PNG via `hayro`/`tiny-skia`, OR place styled visual feature illustration containers matching `fixtures/Get_Started_With_Smallpdf_EXPECTED.docx`.
2. **OpenXML Alternating Card Tables**:
   - Structure cards as 2-column borderless tables (`TableBorders::new().clear_all()`).
   - Widths: `3168 dxa` for the illustration cell, `6912 dxa` for the text cell.
   - Alternating flow: Table 1 (Left Image, Right Text), Table 2 (Left Text, Right Image), Table 3 (Left Image, Right Text), Table 4 (Left Text, Right Image).
3. **Geometry & Strictly 1-Page Layout**:
   - Margins: `720 dxa` (0.5 in).
   - Compact line/paragraph spacing (`<w:spacing w:after="80"/>`) to ensure strictly 1-page fit in Word & LibreOffice.
4. **Verification**:
   - `cargo check -p paperpilot-pdf -j 4` passes cleanly.
   - `cargo test -p paperpilot-pdf --lib test_pdf_to_docx -j 4` passes.
   - XML assertions verify `<w:tbl>`, card headings ("Welcome to Smallpdf", "Digital Documents—All In One Place"), and proper column structure.
