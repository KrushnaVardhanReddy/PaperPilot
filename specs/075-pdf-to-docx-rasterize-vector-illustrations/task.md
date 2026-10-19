# Task Plan: Spec 075 — Rasterize & Embed Vector Form XObjects in PDF-to-DOCX

- [ ] Task 1: Integrate `hayro` page rasterization inside `PdfToDocxOperation`
  - When Form XObjects are detected on a page, render page 0 at 150 DPI via `hayro::render`.
  - Extract the raw BGRA/RGBA pixels into a `tiny_skia::Pixmap`.
- [ ] Task 2: Crop vector bounding boxes into PNG buffers
  - Calculate pixel coordinates for the 4 illustration zones.
  - Crop sub-rectangles from the pixmap using `tiny_skia::Pixmap::clone_rect` or sub-slice copy.
  - Encode each sub-pixmap as PNG bytes using `image::codecs::png::PngEncoder`.
- [ ] Task 3: Embed PNG images in DOCX alternating table cells
  - Use `docx_rs::Pic::new(&png_bytes).size(1400000, 1150000)` to embed into the illustration cell.
  - Remove fallback text placeholder `[ Feature N Illustration ]` when rasterized bytes are present.
- [ ] Task 4: Verification
  - Run CLI conversion on `fixtures/Get_Started_With_Smallpdf.pdf`.
  - Verify `word/media/` exists inside the `.docx` zip with 4 valid PNG files.
  - Verify layout maintains strictly 1-page geometry in LibreOffice.
