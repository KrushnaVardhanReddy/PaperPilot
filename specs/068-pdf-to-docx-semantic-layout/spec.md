# Spec 068 — High-Fidelity PDF-to-DOCX Semantic Layout & Vector Illustration Reconstruction

## Status: APPROVED / READY FOR IMPLEMENTATION

## 1. Problem Statement & Comparison

When converting `fixtures/Get_Started_With_Smallpdf.pdf` to Word DOCX:
- **Actual Output (`fixtures/Get_Started_With_Smallpdf.docx`)**: All text strings are dumped into a single squashed paragraph block at the top left of the page with no section divisions, and 0 illustrations appear.
- **Expected Document Structure (`fixtures/Get_Started_With_Smallpdf_EXPECTED.docx`)**:
  - Centered document title (`"Welcome to Smallpdf"` in bold 22pt).
  - Centered subtitle (`"Ready to take document management to the next level?"` in 11pt gray).
  - Four distinct alternating feature rows formatted as borderless 2-column tables (`docx_rs::Table`):
    - Row 1: Left column illustration / badge, right column title + body.
    - Row 2: Left column title + body, right column illustration / badge.
    - Row 3: Left column illustration / badge, right column title + body.
    - Row 4: Left column title + body, right column illustration / badge.
  - Page margin geometry fixed at 0.5 inches (`720 dxa`), ensuring the document remains strictly **1 page**.

### Why Previous Approaches Failed:
1. **Blind `extract_text()` linear dump**: Calling `doc.extract_text(&[*page_id])` reads raw PDF content stream tokens in arbitrary operator order, concatenating lines without spatial coordinate or block structure awareness.
2. **Missing Vector Art / Form XObjects**: In modern brochures and vector PDFs, illustrations are often stored as Form XObjects (`/Subtype /Form`) with bezier curves, rather than raw JPEG/PNG image streams (`/Subtype /Image`). Looking solely for `/Subtype /Image` yields 0 images.

---

## 2. Technical Architecture & Implementation Details

Modify `paperpilot-pdf/src/operations/conversion.rs` in `PdfToDocxOperation::execute`:

### 2.1 Spatial Text Block Clustering (or Markdown AST Layout)
Use the structured text extractor (already present in `pdf_oxide::api::Pdf::to_markdown` or `extract-text`), which correctly segments the PDF into semantic blocks:
```markdown
# Welcome to Smallpdf
Ready to take document management to the next level?

#### Enhance Documents in One Click
When you right-click on a file, we’ll present you with an array of options to convert, compress, or modify it.

#### Digital Documents—All In One Place
With the new Smallpdf experience, you can freely upload, organize, and share digital documents...
```

Parse this hierarchy into:
1. **Page Header**: Level 1 heading $\to$ Centered paragraph, bold, size 44 (22pt).
2. **Page Subtitle**: Paragraph following header $\to$ Centered paragraph, size 22 (11pt), muted color `#757E8A`.
3. **Feature Cards**: Pairs of `(Heading, Paragraph)`:
   - Card 1: "Digital Documents—All In One Place" + Body.
   - Card 2: "Enhance Documents in One Click" + Body.
   - Card 3: "Access Files Anytime, Anywhere" + Body.
   - Card 4: "Collaborate With Others" + Body.

### 2.2 Reconstructing 2-Column Feature Tables with `docx-rs`
For each detected feature card, construct a 2-column borderless OpenXML table:
```rust
use docx_rs::*;

let mut table = Table::new(vec![])
    .set_borders(TableBorders::new().clear_all());

let mut cell_text = TableCell::new().width(5200, WidthType::Dxa);
cell_text = cell_text.add_paragraph(
    Paragraph::new().add_run(
        Run::new()
            .add_text(card_title)
            .bold()
            .size(24) // 12pt
            .fonts(RunFonts::new().ascii(&font_family))
            .color("1F242E")
    )
);
cell_text = cell_text.add_paragraph(
    Paragraph::new().add_run(
        Run::new()
            .add_text(card_body)
            .size(19) // 9.5pt
            .fonts(RunFonts::new().ascii(&font_family))
            .color("525C68")
    )
);

// Left/Right column placement
let cell_graphic = TableCell::new().width(2400, WidthType::Dxa);
// If an image or rasterized Form XObject exists, add Pic. Otherwise add styled illustration badge.
let row = if is_left_graphic {
    TableRow::new(vec![cell_graphic, cell_text])
} else {
    TableRow::new(vec![cell_text, cell_graphic])
};

table = table.add_row(row);
docx = docx.add_table(table);
```

### 2.3 Font Sanitization & Page Geometry
1. Apply `sanitize_font_name` so all font runs reference `Source Sans Pro`, `Arial`, or standard system fonts with zero subset prefixes (`JFGNNE+`).
2. Maintain compact page margins:
   ```rust
   let docx = Docx::new().page_margin(
       docx_rs::PageMargin::new()
           .top(720)
           .bottom(720)
           .left(720)
           .right(720),
   );
   ```

---

## 3. Verification Protocol
1. Run conversion on `fixtures/Get_Started_With_Smallpdf.pdf`.
2. Compare the output `fixtures/Get_Started_With_Smallpdf.docx` with `fixtures/Get_Started_With_Smallpdf_EXPECTED.docx`.
3. Verify that `word/document.xml` in the generated DOCX contains `<w:tbl>` elements with separate feature rows and centered titles.
4. Run `cargo test -p paperpilot-pdf --lib test_pdf_to_docx -j 4`.
