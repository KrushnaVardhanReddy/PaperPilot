# Spec 088: Full-Fidelity Image & Asset Rendering in HTML & Markdown to PDF Synthesis (`html_to_pdf`, `md_to_pdf`)

## 1. Background & Problem Statement
PaperPilot's Wave 3 (`v0.7.0`) provides bidirectional synthesis:
1. `pdf_to_html` / `pdf_to_markdown`: Extracts text and visual assets (raster photos + vector illustrations as base64 data URIs).
2. `html_to_pdf` / `md_to_pdf`: Uses `fulgur` (a Chromium-free pure-Rust HTML/CSS to PDF layout engine) to compile documents into styled PDFs.

Currently, `HtmlToPdfOperation::render()` instantiates `fulgur::Engine::builder().build()` with no asset bindings. When an HTML or Markdown document contains `<img>` tags referencing:
- Data URIs: `<img src="data:image/png;base64,...">`
- Relative file paths: `<img src="photos/chart.png">`
- Absolute file paths: `<img src="/path/to/diagram.jpg">`

The layout engine ignores the images or leaves empty blank boxes, so generated PDFs only display text and CSS styles without embedded visuals.

## 2. Requirements & Architecture

### 2.1 Asset Pre-Extraction & Bundle Registration
In `paperpilot-pdf/src/operations/conversion.rs`:
Before invoking `fulgur::Engine`, `HtmlToPdfOperation` will:
1. Parse the HTML input for `<img ... src="..." ...>` attributes (or `pulldown_cmark` image events in `md_to_pdf`).
2. Construct a `fulgur::asset::AssetBundle`.
3. For each image source:
   - **Base64 Data URIs (`data:image/...;base64,<data>`)**:
     Decode base64 payload into raw image bytes, allocate a synthetic unique filename (e.g. `img_inline_0.png`), register via `bundle.add_image(name, bytes)`, and rewrite `<img src="...">` in HTML to reference the registered asset name.
   - **Local File Paths (relative or absolute)**:
     Read file bytes from disk (resolving relative paths against the input document's parent directory), register via `bundle.add_image(name, bytes)`, and rewrite HTML src to the asset name.
   - Set `engine_builder.assets(bundle)`.

### 2.2 Fallback Gracefulness
If an image path cannot be found or decoded (corrupted base64 or missing disk file), the conversion must not panic or abort; it logs a warning, skips the missing image asset, and proceeds with the remaining document rendering.

### 2.3 Verification & E2E Validation
1. **Unit Tests in `paperpilot-pdf`**:
   - `test_html_to_pdf_with_base64_image`: Synthesize PDF from HTML with embedded base64 PNG, verify PDF contains XObject image stream using `pdfimages` or byte inspection.
   - `test_md_to_pdf_with_image`: Synthesize PDF from Markdown with `![Test](data:image/png;base64,...)`, verify valid rasterization via `pdftoppm`.
2. **Desktop & CLI Parity**:
   - Run `paperpilot convert --format pdf --input test_samples/image_test.html --output test_samples/image_test_fixed.pdf` and verify embedded image rendering.

## 3. Delegation Strategy
This task touches core Rust conversion logic (`paperpilot-pdf`), requiring HTML/Markdown AST inspection and `fulgur::asset::AssetBundle` wiring. We can dispatch a clean, focused task to Jules or implement locally.
