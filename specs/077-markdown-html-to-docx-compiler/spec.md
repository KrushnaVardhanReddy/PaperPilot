# Spec 077: Stage 2 — Decoupled Markdown-to-DOCX Compiler with DrawingML Media & Layout Synthesis

## 1. Problem Statement & Motivation
In previous versions of `PdfToDocxOperation`, DOCX creation suffered from three major architectural flaws:
1. **Tight Coupling to Specific Fixtures**: The parser relied on hardcoded `card_index` counting and assumed documents had alternating left/right cards with placeholders like `[ Feature {} Illustration ]`.
2. **Missing Real Vector Graphics in DOCX**: Even though vector illustrations were detected as Form XObjects (`is_form == true`), the code explicitly fell back to printing italicized blue text `[ Feature {} Illustration ]` instead of embedding real image bytes (`docx_rs::Pic`).
3. **Bypassing the Decoupled Pipeline**: `PdfToDocxOperation` ran its own separate PDF scan (`unpdf::to_markdown`) rather than consuming the unified Stage 1 AST/Markdown emitted by `PdfToMarkdownOperation`.

In **Stage 2**, we decouple the pipeline completely:
$$\text{PDF} \xrightarrow{\text{Stage 1: Spec 076}} \text{Markdown with inline Base64 data URIs} \xrightarrow{\text{Stage 2: Spec 077}} \text{Production OpenXML DOCX}$$

---

## 2. Detailed Technical Design & Implementation Architecture

### 2.1 Pipeline Flow in `PdfToDocxOperation::execute`
File: [`paperpilot-pdf/src/operations/conversion.rs`](file:///home/krushna/Project/PaperPilot/paperpilot-pdf/src/operations/conversion.rs)

1. **Invoke Stage 1 (`PdfToMarkdownOperation`)**:
   - Execute `PdfToMarkdownOperation` on `document`.
   - Retrieve the resulting Markdown string `markdown` from `extracted_markdown`.
   - This Markdown contains the complete document hierarchy: `# Page 1`, `#### Headings`, paragraphs, and embedded media tags:
     `![Illustration Fm0](data:image/png;base64,iVBORw0KGgo...)`

2. **AST Parsing via `pulldown_cmark`**:
   - Initialize `Parser::new_ext(&markdown, Options::all())`.
   - Maintain the compiler state machine:
     ```rust
     struct DocxCompilerState {
         docx: Docx,
         current_heading_level: u32,
         current_paragraph: Paragraph,
         current_run: Option<Run>,
         pending_image: Option<(Vec<u8>, u32, u32)>, // (png_bytes, width_emu, height_emu)
         in_table: bool,
         table_rows: Vec<TableRow>,
         current_row_cells: Vec<TableCell>,
         current_cell_paragraphs: Vec<Paragraph>,
     }
     ```

3. **Event Dispatch Logic**:
   - **`Event::Start(Tag::Image { dest_url, .. })`**:
     - Parse `dest_url`. When it begins with `data:image/` (e.g. `data:image/png;base64,` or `data:image/jpeg;base64,`):
       1. Strip the header prefix up to `,`.
       2. Decode the base64 string into `Vec<u8>` using `base64::engine::general_purpose::STANDARD.decode()`.
       3. Inspect the image geometry using `image::load_from_memory(&bytes)` to get `(width_px, height_px)`.
       4. Calculate OpenXML DrawingML EMUs:
          - $1\text{ inch} = 914,400\text{ EMUs}$.
          - At $150\text{ DPI}$: $\text{scale} = 914400 / 150 = 6096\text{ EMUs per pixel}$.
          - Bound the maximum width to fit printable page margins:
            Page width ($8.5" = 7,772,400\text{ EMUs}$) minus 1-inch margins ($1,440,000\text{ EMUs}$) = max $4,500,000\text{ EMUs}$.
            If $\text{width\_emu} > 4,500,000$, scale proportionally down.
       5. Store in `pending_image = Some((bytes, width_emu, height_emu))`.
   - **`Event::End(TagEnd::Image)`**:
     - Instantiate `docx_rs::Pic::new(&bytes).size(width_emu, height_emu)`.
     - Construct a picture run:
       ```rust
       let pic_run = Run::new().add_image(pic);
       let pic_para = Paragraph::new().align(AlignmentType::Center).add_run(pic_run);
       ```
     - Attach `pic_para` to `pending_image_paragraph` or flush directly to `docx`.
   - **Pairing Images with Descriptions (Visual Card / 2-Column Synthesis)**:
     - When a heading or paragraph immediately follows a `pending_image_paragraph`:
       - Construct a 2-column borderless `docx_rs::Table`:
         - Column 1 ($3,168\text{ dxa} \approx 2.2" $): The illustration cell containing the picture.
         - Column 2 ($6,912\text{ dxa} \approx 4.8" $): The text cell containing the heading + paragraph runs.
       - Emit `Table::new(vec![TableRow::new(vec![illus_cell, text_cell])])` with `.set_borders(TableBorders::new().clear_all())`.
   - **`Event::Start(Tag::Heading { level, .. })` & `Event::End(TagEnd::Heading)`**:
     - Map `pulldown_cmark::HeadingLevel`:
       - `H1` (Title): Size 44 ($22\text{pt}$), bold, `#1F242E`.
       - `H2` (Subtitle): Size 32 ($16\text{pt}$), bold, `#2D3748`.
       - `H3` / `H4` (Card Header): Size 24 ($12\text{pt}$), bold, `#1F242E`.
   - **`Event::Text(text)`**:
     - Append to `current_run` with appropriate font hierarchy (default body text: size 19 ($9.5\text{pt}$), color `#525C68`).

4. **Eliminating All Placeholder Text**:
   - Zero occurrences of `[ Feature {} Illustration ]` or hardcoded index strings.
   - Any document containing raster or vector images embeds the authentic pixel data directly into the OpenXML DrawingML relationship (`word/media/image1.png`, etc.).

---

## 3. Data Structures & Types
```rust
/// EMU scaling constants for OpenXML DrawingML
pub const EMUS_PER_INCH: u32 = 914_400;
pub const MAX_DOCX_PAGE_WIDTH_EMUS: u32 = 5_500_000;
pub const DEFAULT_CARD_IMAGE_WIDTH_EMUS: u32 = 1_800_000;
pub const DEFAULT_CARD_IMAGE_HEIGHT_EMUS: u32 = 1_800_000;

pub fn decode_data_uri(uri: &str) -> Option<(String, Vec<u8>)> {
    if !uri.starts_with("data:image/") {
        return None;
    }
    let parts: Vec<&str> = uri.splitn(2, ',').collect();
    if parts.len() != 2 {
        return None;
    }
    let mime = parts[0].strip_prefix("data:image/")?.strip_suffix(";base64")?;
    use base64::{engine::general_purpose::STANDARD, Engine as _};
    let bytes = STANDARD.decode(parts[1]).ok()?;
    Some((mime.to_string(), bytes))
}
```

---

## 4. Verification & Quality Gates
1. **Automated Conversion Suite**:
   `cargo test -p paperpilot-pdf -j 4 --lib operations::conversion::tests` must pass 100%.
2. **ZIP Content Verification**:
   Inspect the generated `.docx` output with `unzip -l output.docx`:
   - `word/media/image1.png`
   - `word/media/image2.png`
   - `word/media/image3.png`
   - `word/media/image4.png`
   - `word/media/image5.png`
   - `word/document.xml`
   Must exist with non-zero size, confirming real DrawingML binary embedding.
3. **Regression Safety**:
   Zero regressions on table detection, multi-column reading order, and office conversions.
