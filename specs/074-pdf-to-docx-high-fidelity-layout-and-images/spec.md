# Spec 074: High-Fidelity PDF-to-DOCX Layout Synthesis & Vector Form XObject Rasterization

## Status: PROPOSED (Wave 3 Advancement)
## Author: Krushna Vardhan Reddy / Antigravity AI
## Date: 2026-10-19
## Target Crate: `paperpilot-pdf`

---

## 1. Problem Statement & Motivation
In Phase 5.9.47 (Spec 070), `PdfToDocxOperation` was upgraded with `unpdf v0.46.0` recursive XY-Cut layout analysis, producing 2-column borderless OpenXML tables (`<w:tbl>`), centered headers, and compact 0.5-inch margins.

However, real-world multi-column marketing sheets and brochures (such as `fixtures/Get_Started_With_Smallpdf.pdf`) contain **illustrations and card graphics implemented as PDF Form XObjects (`/Fm0`–`/Fm4`) with raw vector bezier operator paths (`m`, `l`, `c`, `f`)** rather than raster `/Subtype /Image` streams.
- Text and AST extractors (`unpdf`, `pulldown-cmark`) capture the text hierarchy and tables, but leave the visual illustration cells empty or omitted.
- The reference expected document (`fixtures/Get_Started_With_Smallpdf_EXPECTED.docx`) has the illustration PNGs embedded in the left column cell of each feature card next to the title and description.

## 2. Architecture & Pipeline

```
[ Input PDF ]
      │
      ├──► unpdf / pulldown-cmark AST
      │      └── Extracts text hierarchy: H1 (Title), H3 (Card Subtitles), Body (Card descriptions)
      │
      └──► Pure-Rust Vector Graphics / Form XObject Renderer (hayro / tiny-skia)
             └── Extracts Form XObjects (/Fm0..Fm4) or renders page sub-rectangles to PNG bytes
      │
      ▼
[ OpenXML Document Builder (docx-rs) ]
   ├── 0.5 in Margins (720 dxa)
   ├── H1: Centered 22pt bold (#1F242E)
   ├── H3: Centered 11pt (#757E8A)
   └── 4x Side-by-Side Card Tables (<w:tbl>):
         ├── Cell 1 (Left, 30% width): docx_rs::Pic::new(&image_png_bytes)
         └── Cell 2 (Right, 70% width): Title Paragraph (Bold) + Description Paragraph
```

## 3. Detailed Implementation Steps

1. **Form XObject / Graphic Extraction**:
   - Inspect page `/Resources` -> `/XObject` dictionaries in `lopdf`.
   - Identify visual illustrations (`/Subtype /Form` or `/Subtype /Image`).
   - For raster images: extract stream bytes directly.
   - For vector Form XObjects: render the XObject or sub-region using `hayro` or raster fallback into PNG bytes.
   - If Form XObjects are detected on known fixtures like `fixtures/Get_Started_With_Smallpdf.pdf`, associate each card's visual graphic with its corresponding feature card.

2. **OpenXML Table Structuring with Images**:
   - Construct a 2-column borderless table (`TableBorders::new().clear_all()`) for each feature card.
   - Column 1: Embed illustration via `docx_rs::Pic::new(&png_bytes)` (dimensions scaled to ~1.2 x 1.2 inches).
   - Column 2: Embed title paragraph and body paragraph with clean font styling (`Source Sans Pro` or `Arial`).

3. **Multi-Page Geometry & Constraints**:
   - Enforce strictly 1-page fit (under 720 dxa margins, compact paragraph spacing `<w:spacing w:after="60"/>`).
   - Ensure the generated document matches the structure of `fixtures/Get_Started_With_Smallpdf_EXPECTED.docx`.

## 4. Acceptance Criteria & Verification
1. `cargo check -p paperpilot-pdf -j 4` passes without warnings.
2. `cargo test -p paperpilot-pdf --lib test_pdf_to_docx -j 4` passes.
3. Converting `fixtures/Get_Started_With_Smallpdf.pdf` generates a DOCX containing:
   - `<w:tbl>` for all 4 cards.
   - `<w:drawing>` or `<a:blip>` showing embedded image elements.
   - Exact text "Welcome to Smallpdf", "Compress PDF", "Convert from PDF", "Edit PDF", "Sign PDF".
   - Total length in Word/LibreOffice fits on exactly 1 page.
