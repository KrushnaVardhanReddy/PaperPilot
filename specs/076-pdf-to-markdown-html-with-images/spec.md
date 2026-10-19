# Spec 076: High-Fidelity PDF-to-Markdown & PDF-to-HTML with Embedded Media (Raster Images & Vector Form XObjects)

## 1. Context & Motivation
Directly translating raw PDF byte streams into OpenXML (`.docx`) in a single step leads to complex, hard-to-maintain heuristics and tight coupling to specific document fixtures. 

The industry-standard architecture (used by Pandoc, Mathpix, and Calibre) uses a **two-stage pipeline**:
1. **Stage 1 (This Spec)**: Decompile PDF into rich, semantic **Markdown (`.md`)** or **HTML (`.html`)** with all visual assets preserved (raster images and vector Form XObject illustrations).
2. **Stage 2**: Compile the resulting clean Markdown/HTML into DOCX, EPUB, or other target formats using standard document DOM translators.

By mastering Stage 1, users and downstream tools can inspect and use the extracted Markdown/HTML directly (for AI RAG pipelines, CMS publishing, and web previews), while Stage 2 becomes a clean, deterministic compiler.

---

## 2. Technical Architecture & Design

### 2.1 Media Extraction Strategy
Every PDF page may contain two types of visual assets in its `/Resources -> /XObject` dictionary:
1. **Raster Images (`/Subtype /Image`)**:
   - Extract raw DCT/Flate stream into JPEG/PNG bytes.
   - Save to disk as `media/image_p{page}_{idx}.png` (or format as inline base64 data URLs `data:image/png;base64,...`).
2. **Vector Form XObjects (`/Subtype /Form`)**:
   - Vector graphics containing Bézier path commands (`m`, `l`, `c`, `f`) that text extractors ignore.
   - Use `hayro::render` (at 150 DPI) to render the page to a pixel buffer and crop the vector illustration bounding box into a standalone PNG image.

### 2.2 Markdown Representation (`PdfToMarkdownOperation`)
In `paperpilot-pdf/src/operations/conversion.rs`:
- Combine `unpdf` / `pdf_oxide` structural layout analysis with extracted media.
- Emit standard Markdown image syntax:
  ```markdown
  ![Illustration 1](media/image_1.png)
  ```
  or inline base64 if a single standalone file is requested:
  ```markdown
  ![Illustration 1](data:image/png;base64,iVBORw0KGgo...)
  ```
- For multi-column cards (like brochures), emit Markdown tables or standard block sequences that represent the visual flow without hardcoding fixture-specific text.

### 2.3 HTML Representation (`PdfToHtmlOperation`)
In `paperpilot-pdf/src/operations/conversion.rs`:
- Use semantic HTML tags: `<article>`, `<section>`, `<h1>`..`<h6>`, `<p>`, `<table>`.
- Images are embedded via `<img src="media/image_1.png" alt="..." style="max-width: 100%;">` or `<img src="data:image/png;base64,...">`.
- Apply responsive typography and CSS presets (`github`, `elegant`, `minimal`).

---

## 3. Command-Line Interface (CLI)
Support outputting either a standalone file with embedded base64 data URLs, or a folder with a `media/` subfolder:
```bash
# Standalone Markdown with images
paperpilot convert --format md --input input.pdf --output output.md

# Standalone HTML with images
paperpilot convert --format html --input input.pdf --output output.html
```

---

## 4. Acceptance Criteria & Test Plan
1. **Fixture Test (`fixtures/Get_Started_With_Smallpdf.pdf`)**:
   - Running `convert --format md` produces Markdown containing image links for the 4 feature illustrations.
   - Running `convert --format html` produces an HTML page viewable in a browser with the 4 feature illustrations and headers intact.
2. **Standard PDF Test**:
   - Regular text documents convert to clean Markdown without spurious image tags.
   - Documents with raster photos convert with images placed inline with surrounding text.
3. **Automated Verification**:
   - `cargo test -p paperpilot-pdf test_pdf_to_markdown` passes.
   - `cargo test -p paperpilot-pdf test_pdf_to_html` passes.
