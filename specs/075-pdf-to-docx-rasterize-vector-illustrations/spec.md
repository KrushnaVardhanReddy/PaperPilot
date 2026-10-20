# Spec 075: Generic Dynamic Layout, N-Column Table Generation & Image Extraction for PDF-to-DOCX

## 1. Executive Summary & Objective
Previous iterations of `PdfToDocxOperation` hardcoded a 2-column alternating table layout tailored specifically for `fixtures/Get_Started_With_Smallpdf.pdf`. When converting standard PDFs (such as single-column reports, multi-column scientific papers, or data tables), this produced unintended formatting artifacts (e.g. bundling arbitrary `Heading 3` tags into 2-column tables and injecting text placeholders `[ Feature N Illustration ]`).

The goal of Spec 075 is to make `PdfToDocxOperation` **completely generic and adaptive**:
1. **Dynamic Table Reconstruction**: Parse standard Markdown tables from `unpdf` (`Event::Start(Tag::Table)`) into dynamic `N`-column OpenXML tables (`<w:tbl>`) where column widths scale proportionally to the available page width (10,080 dxa).
2. **Standard Document Flow**: Normal headings (`#`, `##`, `###`), lists (`*`, `1.`), and paragraphs (`<w:p>`) flow naturally as full-width blocks unless explicitly grouped in a multi-column or table structure.
3. **Multi-Column / Brochure Card Detection**: Detect side-by-side elements only when spatial or structural layout indicates multiple columns or accompanying graphic assets.
4. **Real Image Extraction & Embedding**: Extract raster images (`/Subtype /Image`) or rasterize vector illustration regions into high-resolution PNG bytes using `hayro` and `image`, embedding them via `docx_rs::Pic`. Never inject dummy text placeholders like `[ Feature N Illustration ]`.

---

## 2. Technical Architecture

### 2.1 Dynamic Table Parser (`pulldown-cmark` Table Events)
Instead of hardcoding 2-column tables on `Heading 3`, support dynamic tables via standard `pulldown-cmark` events:
- `Event::Start(Tag::Table(alignments))`: Initialize a new table state with `N = alignments.len()`. Calculate column widths: `width_per_col = 10080 / N`.
- `Event::Start(Tag::TableHead)`: Track header row styling (bold text, subtle shading if appropriate).
- `Event::Start(Tag::TableRow)`: Begin a new `TableRow`.
- `Event::Start(Tag::TableCell)`: Begin a new `TableCell` with calculated width `width_per_col` (WidthType::Dxa).
- `Event::End(TagEnd::TableCell)`: Push cell to current row.
- `Event::End(TagEnd::TableRow)`: Push row to current table.
- `Event::End(TagEnd::Table)`: Add completed `Table` to `docx`.

### 2.2 Adaptive Multi-Column & Card Handling
When `unpdf` parses multi-column sections (e.g. brochures or academic papers with XY-Cut):
- If content has accompanying side-by-side media (Form XObjects or images):
  - Rasterize or extract the graphic into PNG bytes.
  - Construct a borderless layout container table only when media is present.
- If content is standard text:
  - Render as standard flowing paragraphs with proper margins and indentation.

### 2.3 Image Extraction and Vector Rasterization
- **Raster Images (`/Subtype /Image`)**: Decode DCT/Flate stream into JPEG/PNG bytes, embed directly with `docx_rs::Pic`.
- **Vector Graphics (`/Subtype /Form`)**: When bounding boxes are known, rasterize using `hayro::render` at 150 DPI and crop to PNG bytes.
- **Strict Rule**: Zero hardcoded strings `[ Feature N Illustration ]` in the generated DOCX. If no image is present, no placeholder is inserted.

---

## 3. Success Criteria & Verification
1. **Generic Test**: Convert a standard multi-page text PDF (e.g. `tests/e2e_fixtures/real/sample.pdf` or multi-page docs) -> Produces standard flowing Word paragraphs without spurious 2-column tables.
2. **Table Test**: Convert a PDF containing tabular data -> Produces a proper `N`-column OpenXML table matching the table dimensions.
3. **Brochure Test**: Convert `fixtures/Get_Started_With_Smallpdf.pdf` -> Produces 4 cards with actual embedded PNG illustrations, strictly 1-page geometry, and zero text placeholders.
4. **Compilation & CI**:
   - `cargo test -p paperpilot-pdf test_pdf_to_docx -j 4` passes.
   - `cargo test -p paperpilot-cli --test all50_cleanroom_audit -j 4` passes.
