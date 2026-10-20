# Task 5.9.55: Stage 2 — High-Fidelity Markdown to DOCX Compiler with Real Embedded Images

## Overview
Implement the Stage 2 compiler for `PdfToDocxOperation` in `paperpilot-pdf/src/operations/conversion.rs`. The compiler consumes the rich Markdown output produced by Stage 1 (Spec 076) containing embedded base64 data URIs for raster and vector media, translates Markdown elements (Headings, Paragraphs, Images, Tables) into `docx_rs` elements, decodes image bytes, and inserts real DrawingML pictures (`docx_rs::Pic`) into the `.docx` document.

## Subtasks
- [ ] Parse Markdown output from `PdfToMarkdownOperation` (or shared intermediate generator) in `PdfToDocxOperation`.
- [ ] In `pulldown_cmark` event loop, handle `Tag::Image` / `Event::Start(Tag::Image { dest_url, .. })`.
- [ ] Decode base64 data URLs (`data:image/png;base64,...`) into PNG bytes.
- [ ] Convert dimensions into EMUs and add to `docx_rs::Run::add_image(Pic::new(&bytes).size(w, h))`.
- [ ] Replace hardcoded `[ Feature X Illustration ]` text runs with real image embedding.
- [ ] Verify using `fixtures/Get_Started_With_Smallpdf.pdf` and check zip contents of the resulting `.docx` for `word/media/image*.png`.
- [ ] Run full test suite with `cargo test -p paperpilot-pdf -j 4 --lib operations::conversion::tests`.
