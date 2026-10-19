# Task Plan: Spec 076 — High-Fidelity PDF-to-Markdown & PDF-to-HTML with Embedded Media

- [ ] Task 1: Extract & Rasterize Visual Media in `paperpilot-pdf`
  - In `paperpilot-pdf/src/operations/conversion.rs`, implement helper `extract_page_media(doc, page_idx) -> Vec<ExtractedMedia>`.
  - Handle raster `/Subtype /Image`: extract DCT/Flate stream into JPEG/PNG bytes.
  - Handle vector `/Subtype /Form`: render page via `hayro::render` at 150 DPI and crop bounding box into PNG bytes.
- [ ] Task 2: Enhance `PdfToMarkdownOperation`
  - Place extracted images at their corresponding section positions in the emitted Markdown.
  - Support relative paths `media/*.png` or base64 data URLs `data:image/png;base64,...`.
  - Format multi-column card sections into Markdown tables or standard flow without hardcoding fixture text.
- [ ] Task 3: Enhance `PdfToHtmlOperation`
  - Inject `<img src="...">` tags into semantic HTML blocks.
  - Ensure CSS presets (`github`, `elegant`, `minimal`) format images responsively (`max-width: 100%; height: auto;`).
- [ ] Task 4: CLI & Unit Test Verification
  - Add/update tests in `conversion.rs` verifying markdown and HTML output contains valid image tags.
  - Verify `paperpilot convert --format md` on `fixtures/Get_Started_With_Smallpdf.pdf` produces valid Markdown with all 4 feature illustrations.
