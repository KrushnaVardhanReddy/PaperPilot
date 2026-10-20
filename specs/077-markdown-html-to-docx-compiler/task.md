# Task 5.9.55: Stage 2 — Decoupled Markdown-to-DOCX Compiler with DrawingML Media & Layout Synthesis

## Overview
Implement the Stage 2 compiler for `PdfToDocxOperation` in `paperpilot-pdf/src/operations/conversion.rs`. The compiler consumes the rich Markdown output produced by Stage 1 (Spec 076) containing embedded base64 data URIs for raster and vector media, translates Markdown elements (Headings, Paragraphs, Images, Tables) into `docx_rs` elements, decodes image bytes, and inserts real DrawingML pictures (`docx_rs::Pic`) into the `.docx` document.

## Implementation Steps
- [ ] In `paperpilot-pdf/src/operations/conversion.rs`, modify `PdfToDocxOperation::execute` to run `PdfToMarkdownOperation` first and obtain the rich Markdown containing inline base64 images.
- [ ] Implement `decode_data_uri(uri: &str) -> Option<(String, Vec<u8>)>` using `base64::engine::general_purpose::STANDARD`.
- [ ] Handle `Event::Start(Tag::Image { dest_url, .. })` in `pulldown_cmark` parser:
  - Extract base64 payload from data URI.
  - Determine dimensions using `image::load_from_memory` and scale to Word EMUs ($1\text{ in} = 914,400\text{ EMUs}$).
  - Create `docx_rs::Pic::new(&image_bytes).size(width_emu, height_emu)`.
  - Append image to paragraph or table cell.
- [ ] Completely remove the legacy `[ Feature {} Illustration ]` text fallback.
- [ ] Pair images and subsequent headings/paragraphs into clean 2-column layout tables where appropriate.
- [ ] Support standard Markdown tables (`Table`, `TableHead`, `TableRow`, `TableCell`) to OpenXML tables.
- [ ] Run test verification:
  - `cargo test -p paperpilot-pdf -j 4 --lib operations::conversion::tests`
  - Convert `fixtures/Get_Started_With_Smallpdf.pdf` to `/tmp/test_stage2.docx` and verify `word/media/*.png` exists in the archive.
