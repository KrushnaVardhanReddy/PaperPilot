# Spec 039 — Advanced Multi-Column & Table Document Layout Analysis via Spatial XY-Cut & `docx-rs`

## 1. Overview & Problem Statement
In Spec 038, PaperPilot gained foundational typographic reconstruction for `pdf_to_docx`, extracting font sizes for headings, bold/italic weights, RGB font colors, center alignments, and embedded images via `docx-rs`.

However, complex documents present two structural challenges:
1. **Multi-Column Text Flow**: Academic papers, journals, newsletters, and financial statements often use 2 or 3 columns. Naive top-to-bottom or stream-order extraction scans across columns horizontally, interleaving sentences from column 1 into column 2.
2. **Tabular Grids**: Financial tables, pricing matrices, and invoices contain aligned data cells. When emitted as flat paragraphs, column alignment is completely lost.

**Objective:**
Implement advanced spatial layout analysis—using recursive **XY-Cut reading-order de-weaving** and **grid/table boundary detection** in pure Rust—into `paperpilot-pdf` and `paperpilot-wasm` so that `pdf_to_docx` automatically emits:
- Properly sequenced multi-column paragraphs (Column 1 fully read before Column 2).
- Native OpenXML tables (`docx-rs::Table`, `TableRow`, `TableCell`) for tabular data.

---

## 2. Technical Architecture & Design

### 2.1 Spatial Span Structure & Bounding Box Extraction
Define a spatial text span representation extracted during page content parsing:
```rust
#[derive(Debug, Clone)]
pub struct TextSpan {
    pub text: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub font_size: f32,
    pub is_bold: bool,
    pub is_italic: bool,
    pub color_hex: String,
}
```

### 2.2 XY-Cut Column De-Weaving Algorithm
1. **Projection Profile Analysis**:
   - Project horizontal bounding box extents of text spans onto the X-axis for a given vertical band.
   - Detect vertical whitespace gutters (valleys in the projection histogram where gutter width exceeds a threshold, e.g., $\ge 15.0\,\text{pt}$).
2. **Recursive Partitioning**:
   - When a clear vertical gutter splits the page width, split text spans into distinct vertical columns ($X_{\text{col1}} < X_{\text{gutter}} < X_{\text{col2}}$).
   - Process spans in Column 1 from top to bottom ($Y$ descending) before processing Column 2.
3. **Paragraph Coalescing**:
   - Within each column, merge closely spaced lines ($|\Delta Y| < 1.5 \times \text{font\_size}$) into unified paragraphs to prevent broken sentences.

### 2.3 Table Structure Recognition
1. **Grid Alignment Detection**:
   - Detect clusters of text spans where multiple spans share similar $Y$-coordinates (rows within $\pm 2.0\,\text{pt}$) and multiple spans share similar $X$-coordinates (aligned column headers and cells, $\ge 2$ columns and $\ge 2$ rows).
2. **OpenXML Table Generation**:
   - When a table grid is identified, map rows into `docx_rs::TableRow`:
     ```rust
     let mut table = Table::new(vec![]);
     for row_spans in detected_rows {
         let mut row = TableRow::new(vec![]);
         for span in row_spans {
             let cell = TableCell::new().add_paragraph(
                 Paragraph::new().add_run(Run::new().add_text(&span.text))
             );
             row = row.add_cell(cell);
         }
         table = table.add_row(row);
     }
     docx = docx.add_table(table);
     ```

### 2.4 Graceful Fallback & Zero-Crash Guarantees
- If no clear columns or table grids are detected, or if spatial analysis yields ambiguity, gracefully fall back to the sequential paragraph extraction established in Spec 038.
- 100% in-memory processing: zero disk temporaries, $<35\,\text{MB}$ RSS memory consumption.
- Strict pure-Rust implementation ensuring compatibility with `wasm32-unknown-unknown` in `paperpilot-wasm` and Cloudflare Workers in `apps/edge`.

---

## 3. Penta-Interface Exposure
- **`paperpilot-pdf`**: Core `PdfToDocxOperation` upgraded with multi-column de-weaving and table generation.
- **`paperpilot-wasm`**: Exported `pdf_to_docx` in `paperpilot-wasm/src/operations.rs` upgraded in parallel with pure in-memory buffer transformation.
- **`paperpilot-cli`**, **`paperpilot-mcp`**, **`paperpilot-gateway`**: Automatically inherit high-fidelity layout outputs with zero breaking changes to command line flags or JSON schemas.

---

## 4. Verification & Quality Gates
1. **Multi-Column Reading Order Test**:
   - Create a test fixture with 2 distinct columns (Left column: "Left 1\nLeft 2", Right column: "Right 1\nRight 2").
   - Assert generated DOCX contains Left text preceding Right text without interleaving ("Left 1 Left 2 Right 1 Right 2").
2. **Table Structure Test**:
   - Create a 2x2 or 3x3 table test fixture.
   - Assert generated DOCX archive unpacks valid XML containing `<w:tbl>`, `<w:tr>`, and `<w:tc>`.
3. **Penta-Interface Regression**:
   - `cargo test -p paperpilot-pdf -j 6`
   - `cargo test -p paperpilot-wasm -j 6`
   - Zero clippy warnings under `cargo clippy --workspace --all-targets --all-features -- -D warnings`.
4. **Documentation**:
   - Author `reports/PDF_OXIDE_LAYOUT_ANALYSIS_REPORT.md`.
