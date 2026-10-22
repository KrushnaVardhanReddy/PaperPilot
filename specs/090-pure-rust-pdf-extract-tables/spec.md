# Spec 090: Pure-Rust Tabular Data Extraction (`pdf_extract_tables`)

## 1. Background & Executive Summary
PDF documents frequently contain structured data (financial statements, invoices, rate sheets, lab results, and research tables). However, standard text extractors (`pdf_extract_text`) flatten table rows into unordered text streams, losing column boundaries, headers, and cell relationships.

**Spec 090** introduces `pdf_extract_tables` across all 5 PaperPilot surfaces. It provides:
1. **Lattice & Stream Detection**: Identification of bordered tables (vector lines/rectangles) and borderless/stream tables (whitespace column alignments).
2. **Structured Cell Matrices**: Returns clean rows, columns, headers, and bounding boxes (`x, y, w, h`).
3. **Multi-Format Export**: Generates structured JSON (with cell coordinates) or standard RFC 4180 CSV for spreadsheet and AI ingestion.
4. **Penta-Interface Parity**: Full execution across CLI, MCP, REST Gateway, client-side WASM, and Cloudflare Edge.

---

## 2. Technical Architecture & Data Model

### 2.1 Single Core Runner
In `paperpilot-pdf/src/operations/table.rs`:
```rust
pub fn extract_tables_bytes(input_bytes: &[u8], options: &TableExtractOptions) -> Result<TableExtractResult, PdfError>
```

### 2.2 Options & Output Schema
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableExtractOptions {
    /// Target pages ("all", "1,2", or "1-3")
    #[serde(default = "default_all")]
    pub pages: String,
    /// Output format: "json" | "csv"
    #[serde(default = "default_format")]
    pub format: String,
    /// Minimum confidence score threshold (0.0 - 1.0)
    #[serde(default = "default_confidence")]
    pub min_confidence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableExtractResult {
    pub total_tables: usize,
    pub tables: Vec<ExtractedTable>,
    pub csv_output: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedTable {
    pub page: u32,
    pub table_index: usize,
    pub bbox: [f32; 4], // [x, y, width, height]
    pub rows: usize,
    pub cols: usize,
    pub headers: Vec<String>,
    pub data: Vec<Vec<String>>,
}
```

### 2.3 Extraction Heuristic
1. **Word & Text Positioning**: Parse text chunks with their `(x, y)` coordinates, font heights, and widths.
2. **Row Clustering**: Group words sharing overlapping $Y$-coordinates (within standard line threshold) into horizontal rows, sorted by $Y$ descending.
3. **Column Clustering**: Project $X$-coordinates to discover column gutters. Words spanning consistent $X$-intervals are assigned to cell $(row, col)$.
4. **Header Detection**: If the top row contains distinct styling (bold font, uppercase, or header separator), mark as `headers`.
5. **CSV Serialization**: If `format == "csv"`, format cell strings with standard RFC 4180 escaping (`"cell with, commas"`).

---

## 3. Penta-Interface Wrappers

1. **Native CLI (`paperpilot-cli`)**:
   `paperpilot extract-tables --input invoice.pdf --output tables.json --format json`
   `paperpilot extract-tables --input invoice.pdf --format csv`
2. **MCP (`paperpilot-mcp`)**:
   Tool name: `pdf_extract_tables`
   Schema accepts `input` (path), `output` (optional path), `pages` (optional), `format` ("json" | "csv").
3. **REST Gateway (`paperpilot-gateway`)**:
   `POST /api/v1/pdf/extract-tables` (multipart or JSON payload).
4. **Browser WASM (`paperpilot-wasm`)**:
   `WasmPdfEngine.extract_tables(input_bytes: &[u8], options_json: &str) -> Result<String, JsValue>`
5. **Edge Worker (`apps/edge`)**:
   `POST /api/v1/extract-tables`

---

## 4. UI Integration (`OperationsPanel.svelte`)
- Add `pdf_extract_tables` under the **Conversions & AI** catalog.
- Configuration controls:
  - Output format: `Structured JSON (.json)` vs `Spreadsheet (.csv)`
  - Page range selector (`all`, `1-3`, etc.)
- When executed on Desktop, supports direct copy to clipboard or saving to file.

---

## 5. Verification & Testing
- Unit tests in `paperpilot-pdf/src/operations/table.rs`:
  - `test_extract_tables_simple_matrix`: Verifies 3x3 table with headers.
  - `test_extract_tables_csv_formatting`: Verifies comma/quote escaping.
  - `test_extract_tables_empty_document`: Graceful fallback returning empty tables array.
- Penta-Interface E2E suite (`tools/penta-interface-e2e/src/cases/table.rs`):
  - 10 real semantic assertions verifying matching output across CLI, MCP, REST, WASM, Edge.
