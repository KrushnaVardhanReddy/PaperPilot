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

## 2. Technical Architecture

### 2.1 Font Name Sanitization & Typography
- When extracting spans via `pdf_oxide` / `lopdf`, strip any 6-character subset prefix followed by `+` (e.g. `ABCDEF+Arial-Bold` $\to$ `Arial`).
- Map standard generic fallbacks:
  - Sans-serif: `Arial`, `Calibri`, `Segoe UI`
  - Serif: `Times New Roman`, `Georgia`
  - Monospace: `Courier New`, `Consolas`
- Heading inference:
  - Font size $\ge 20\text{pt}$ or $\ge 1.7\times$ median size $\to$ Heading 1 / title (`Heading1`, bold).
  - Font size $\ge 14\text{pt}$ $\to$ Heading 2 (`Heading2`, bold).
  - Body text $\to$ regular paragraphs.

### 2.2 Layout Analysis & 2-Column Pairing
- For each page:
  - Group text spans into semantic blocks using XY-cut or vertical overlap detection.
  - Detect horizontal adjacency: if an image or icon's bounding box is horizontally adjacent to a text block (e.g., $X_{\text{image}} < X_{\text{text}}$ with overlapping $Y$ intervals), group them into a 2-column row of a borderless `docx-rs::Table`.
  - Column 1: Image (`Pic::new(&png_bytes)`).
  - Column 2: Heading + body paragraph.
- Borderless table styling: set top, bottom, left, right, and inside borders to `BorderType::None` so that the layout looks clean and responsive in LibreOffice and Microsoft Word.

### 2.3 Image Extraction & Normalization
- Extract images using `doc.extract_images(page_idx)`.
- Convert decoded bitmap buffers into PNG format bytes using `image` crate.
- Scale image width/height in EMUs proportionally so they do not exceed cell or page dimensions.

---

## 3. Test Fixtures & Verification Criteria

### 3.1 Fixture
- Input PDF: `fixtures/Get_Started_With_Smallpdf.pdf`
- Test Output: `fixtures/test_smallpdf_output.docx`

### 3.2 Acceptance Criteria
1. **Compilation**: `cargo check -p paperpilot-pdf` and `cargo clippy -p paperpilot-pdf` pass without errors or warnings.
2. **Page Count Integrity**: The converted `fixtures/Get_Started_With_Smallpdf.pdf` must render as **1 page** in LibreOffice / standard DOCX viewers, matching the input document page count.
3. **No Tofu Glyphs**: Converted text must not contain `□□□□` or unmapped character entities in headings.
4. **Image Embedding**: All 4 feature icons/diagrams must be extracted and embedded in the DOCX package (`word/media/image*.png`).
5. **Structural Integrity**: The output file must be a valid ZIP archive containing `word/document.xml`, `word/_rels/document.xml.rels`, and `[Content_Types].xml`.
