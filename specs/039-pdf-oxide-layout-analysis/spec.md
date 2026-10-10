# Spec 039 — Advanced Multi-Column & Table Document Layout Analysis via `pdf-oxide`

## 1. Overview & Problem Statement
Spec 038 introduced style and layout reconstruction for `pdf_to_docx`, mapping font sizes, colors, basic bold/italic weights, and center alignments directly to OpenXML (`docx-rs`).

However, real-world PDFs often contain:
1. **Multi-Column Text Flow**: Academic papers, newsletters, brochures, and financial reports with 2-3 column layouts. Naive vertical scanning reads across columns horizontally, scrambling the reading order.
2. **Tabular Data & Grids**: Invoices, bank statements, and tables with borders or aligned cells without explicit table markup.
3. **Complex Paragraph & Line Wrap Detection**: Distinguishing true paragraph breaks from soft wrapped lines.

**Objective:**
Integrate `pdf_oxide` (or its XY-Cut spatial clustering algorithm) into PaperPilot's document extraction and conversion engine to perform **advanced layout analysis, column de-weaving, and structured table detection** prior to OpenXML generation in `docx-rs`.

---

## 2. Technical Architecture & Design

### 2.1 Spatial Span Clustering & XY-Cut Algorithm
1. **Reading-Order De-Weaving (Recursive XY-Cut)**:
   - Slice bounding boxes of text spans across horizontal and vertical projection profiles.
   - Detect vertical whitespace gutters dividing multi-column regions.
   - Sort and group text blocks in true reading order (top-to-bottom of Column 1, then Column 2) rather than raw stream order.

2. **Table Structure Recognition**:
   - Detect intersecting horizontal and vertical rules/paths or spatial column grids.
   - Cluster cell contents into structured rows and columns.
   - Map discovered tables directly into native OpenXML tables:
     ```rust
     docx = docx.add_table(Table::new(rows));
     ```

3. **Fallback & Memory Guarantees**:
   - Must operate 100% in-memory with zero filesystem access.
   - Retain graceful fallback to Spec 038's lightweight heuristic parser if `pdf_oxide` layout analysis encounters non-standard or corrupt streams.
   - Enforce PaperPilot's $<35\,\text{MB}$ RSS memory ceiling.

---

### 2.2 Penta-Interface & WASM Support
Per PaperPilot's Penta-Interface Mandate:
- **`paperpilot-pdf`**: Provides enhanced `PdfToDocxOperation` with layout awareness.
- **`paperpilot-wasm`**: Compiles cleanly with `wasm-bindgen` (ensuring `pdf_oxide` features used do not depend on native C runtimes or non-WASM OS threads).
- **`paperpilot-cli`**, **`paperpilot-mcp`**, **`paperpilot-gateway`**, **`apps/edge`**: Inherit multi-column and table DOCX export transparently.

---

## 3. Verification & Acceptance Criteria
1. **Multi-Column Assertion**:
   - Convert a two-column PDF fixture to DOCX and assert that text flows sequentially down Column 1 before Column 2 (no interleaved sentences).
2. **Table Assertion**:
   - Convert a 3×3 table PDF fixture and assert the presence of `<w:tbl>`, `<w:tr>`, and `<w:tc>` elements in `word/document.xml`.
3. **Penta-Interface Tests**:
   - Pass all existing and new E2E tests across CLI, MCP, REST, WASM, and Edge.
4. **Clippy & Tests**:
   - Zero clippy warnings under `-D warnings`.
   - Run tests bounded with `-j 6`.
5. **Report**:
   - Generate `reports/PDF_OXIDE_LAYOUT_ANALYSIS_REPORT.md`.
