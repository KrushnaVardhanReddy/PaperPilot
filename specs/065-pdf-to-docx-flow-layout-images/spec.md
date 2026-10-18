# Spec 065 — Responsive Flow Layout Reconstruction & Inline Image Extraction for PDF-to-DOCX

## 1. Overview & Problem Statement
Currently, `paperpilot_pdf::operations::conversion::PdfToDocxOperation` converts PDFs to DOCX files either via `pdf_oxide`'s layout mode or flow mode:
1. **Layout mode (`to_docx`)**: Emits absolutely-positioned `<w:framePr>` paragraphs and floating DrawingML shapes. Differences in font metrics between PDF engines and Word/LibreOffice cause vertical overflows, turning 1 PDF page into multiple unintended DOCX pages with misaligned images.
2. **Standard flow mode (`to_docx_bytes_flow`)**: Discards all embedded images and retains subsetted PDF font names (`JFGNNE+SourceSansPro-Bold`), resulting in missing visual assets and "tofu" glyph boxes (`□□□□□`) in standard office viewers.
3. **Multi-Column Content Linearization**: Side-by-side feature cards (e.g. icon on left, heading + description on right) are flattened into a single vertical column without preserving their 2-column or grid relationship.

**Objective:**
Implement a high-fidelity, responsive flow layout engine for PDF $\to$ DOCX conversion utilizing `docx-rs` and `pdf_oxide` extraction APIs:
- Strip font subset prefixes (`JFGNNE+...`) and set cross-platform font families (`Source Sans Pro`, `Arial`, `Helvetica`, `sans-serif`) to eliminate tofu glyph boxes (`□□□□□`).
- Reconstruct side-by-side / 2-column sections into borderless OpenXML tables (`docx-rs::Table`), guaranteeing that icons and descriptive text remain pinned side-by-side without overflowing pages.
- Extract embedded raster images (`doc.extract_images(i)`) and place them inline alongside their associated text blocks using `docx-rs::Pic`.
- Maintain single-page fidelity for single-page input documents like marketing sheets and brochures.

---

## 2. Technical Architecture & Precise Implementation Details

### 2.1 Font Name Sanitization & Typography
PDF font names frequently carry subset tags (e.g., `JFGNNE+SourceSansPro-Bold`, `F1`, `G1`). When emitted verbatim into OpenXML `w:rFonts`, Word/LibreOffice fails to match installed system fonts, showing "tofu" glyph boxes (`□□□□□`).

**Sanitization Algorithm**:
```rust
pub fn sanitize_font_name(raw_name: &str) -> String {
    // 1. Strip 6-character random uppercase subset tag and '+' (e.g. "JFGNNE+SourceSansPro-Bold" -> "SourceSansPro-Bold")
    let cleaned = if let Some((prefix, rest)) = raw_name.split_once('+') {
        if prefix.len() == 6 && prefix.chars().all(|c| c.is_ascii_uppercase()) {
            rest
        } else {
            raw_name
        }
    } else {
        raw_name
    };

    // 2. Separate font family from weight suffix
    let family = cleaned
        .trim_end_matches("-Bold")
        .trim_end_matches("-Regular")
        .trim_end_matches("-Italic")
        .trim_end_matches("-BoldItalic")
        .trim_end_matches(",Bold")
        .trim_end_matches(",Italic");

    // 3. Insert spaces between CamelCase words if needed (e.g., "SourceSansPro" -> "Source Sans Pro")
    let mut normalized = String::new();
    for (i, ch) in family.chars().enumerate() {
        if i > 0 && ch.is_ascii_uppercase() {
            if let Some(prev) = family.chars().nth(i - 1) {
                if prev.is_ascii_lowercase() {
                    normalized.push(' ');
                }
            }
        }
        normalized.push(ch);
    }

    if normalized.is_empty() || normalized.starts_with('F') && normalized.len() <= 3 {
        "Arial".to_string()
    } else {
        normalized
    }
}
```

### 2.2 Layout Extraction & Pairing (2-Column Grid Detection)
A PDF page has dimensions `(page_w, page_h)`.
Using `pdf_oxide::PdfDocument::extract_spans(page_idx)` and `extract_images(page_idx)`:

1. **Extract Images**:
   - For each image, retrieve its bounding box `Rect { x, y, width, height }` (PDF origin at bottom-left; top $Y = page_h - y - height$).
   - Convert decompressed image bytes to PNG via the `image` crate.
   - Discard 1x1 tracking pixels or tiny decorative artifacts ($width < 10$ or $height < 10$).

2. **Cluster Spans into Text Blocks**:
   - Sort spans top-to-bottom ($Y_{top}$ descending).
   - Merge spans with horizontal proximity and matching $Y$ lines into `TextLine`.
   - Cluster consecutive lines into `TextBlock` based on line-spacing threshold (gap $< 1.5 \times font\_size$).
   - Determine block horizontal bounds: `x_min`, `x_max`, `y_top`, `y_bottom`.

3. **Horizontal Pairing (Left Image / Right Text or Vice-Versa)**:
   - For each image, search for a `TextBlock` whose vertical span significantly overlaps ($|Y_{center,img} - Y_{center,block}| < 0.6 \times \max(H_{img}, H_{block})$).
   - If found:
     - If $X_{img} < X_{block}$, classify as **Left Image, Right Text**.
     - If $X_{img} > X_{block}$, classify as **Left Text, Right Image**.
     - Emit as a **2-column borderless OpenXML Table** row.
   - If an image or text block does not pair horizontally:
     - Check horizontal alignment:
       - If $|X_{center} - (page\_w / 2)| < 0.15 \times page\_w$, set center alignment (`AlignmentType::Center`).
       - Otherwise left align (`AlignmentType::Left`).
     - Emit as a standard `Paragraph` (or `Pic` paragraph for standalone images).

### 2.3 Borderless OpenXML Table Construction with `docx-rs`
To prevent page overflows and ensure natural responsive wrapping:
```rust
use docx_rs::*;

let mut table = Table::new(vec![])
    .set_borders(TableBorders::new().clear_all()); // zero border

let cell_img = TableCell::new()
    .width(2800, WidthType::Dxa) // ~2 inches
    .add_paragraph(Paragraph::new().add_run(Run::new().add_image(pic)));

let mut cell_text = TableCell::new()
    .width(5600, WidthType::Dxa); // ~4 inches

// Add heading
cell_text = cell_text.add_paragraph(
    Paragraph::new()
        .add_run(
            Run::new()
                .add_text(heading_text)
                .bold()
                .size(24) // 12pt
                .fonts(RunFonts::new().ascii(&font_family))
        )
);

// Add body paragraph
cell_text = cell_text.add_paragraph(
    Paragraph::new()
        .add_run(
            Run::new()
                .add_text(body_text)
                .size(20) // 10pt
                .fonts(RunFonts::new().ascii(&font_family))
        )
);

let row = TableRow::new(vec![cell_img, cell_text]);
table = table.add_row(row);
docx = docx.add_table(table);
```

### 2.4 Single-Page Geometry & Margins
- Set section page margins to standard compact margins:
  - Top: 720 dxa (0.5 in)
  - Bottom: 720 dxa (0.5 in)
  - Left: 720 dxa (0.5 in)
  - Right: 720 dxa (0.5 in)
- Keep line spacing tight (line 240, after 60 dxa) to prevent unintended page overflow.

---

## 3. Test Fixture & Verification Protocol

### 3.1 Fixture
- `fixtures/Get_Started_With_Smallpdf.pdf` (69 KB, 1 page)

### 3.2 Verification Command
```bash
cargo run -p paperpilot-cli -- convert --input "fixtures/Get_Started_With_Smallpdf.pdf" --format docx --output "fixtures/test_smallpdf_output.docx"
```

### 3.3 Acceptance Checklist
- [ ] Output DOCX is a valid zip package containing `word/document.xml` and `word/media/image*.png`.
- [ ] Heading "Welcome to Smallpdf" is properly decoded with **zero tofu boxes** (`□□□□`).
- [ ] Sub-heading "Ready to take document management to the next level?" is centered.
- [ ] All 4 feature cards are rendered side-by-side with their corresponding icons.
- [ ] Output is strictly **1 page** when opened in LibreOffice / Microsoft Word.
- [ ] `cargo clippy --workspace --all-targets --all-features -- -D warnings` passes with 0 warnings.
