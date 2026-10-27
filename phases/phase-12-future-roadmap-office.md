## Phase 9 — Future Roadmap: Office Document Ecosystem (`.docx`, `.xlsx`, `.pptx`) `[POST-LAUNCH]`

> Full architectural strategy documented at [`docs/OFFICE_EXPANSION_STRATEGY.md`](docs/OFFICE_EXPANSION_STRATEGY.md).
> Extends PaperPilot's pure-Rust, zero-dependency, sub-10ms execution engine to Microsoft Office / OpenDocument formats (`.docx`, `.xlsx`, `.pptx`).

| # | Task | Notes |
|---|---|---|
| 9.1 | `paperpilot-office` Crate Scaffold | Pure-Rust streaming ZIP (`zip-rs`) & zero-copy OpenXML parser (`quick-xml`) with WASM compatibility |
| 9.2 | Word Document Suite (`.docx`) | `docx_read_markdown`, `docx_redact` (PII scrub), `docx_template_fill` (variable injection), `docx_redline_diff` |
| 9.3 | Excel Spreadsheet Suite (`.xlsx`) | `xlsx_query` (instant SQL-like filtering), `xlsx_sanitize` (strip hidden sheets/PII), `xlsx_formula_eval` |
| 9.4 | PowerPoint Deck Suite (`.pptx`) | `pptx_assemble` (JSON/Markdown outline to slide deck), `pptx_redact`, slide extraction. Evaluate & integrate `deckprobe` (`deckflow/deckprobe`) for zero-copy OpenXML slide inspection, notes harvesting, and deck linting. |
| 9.5 | Office MCP & CLI Parity | Register all Office operations in `paperpilot-mcp` for Claude Desktop, Cursor, and OpenCode |
| 9.6 | Universal Office-to-PDF Synthesis | Direct in-memory conversion from OpenXML DOM to PDF/A via PaperPilot layout engine |
| 9.7 | Enterprise Governance & DLP | Background Watch Folder daemon auditing/sanitizing outbound spreadsheets & shared template registry |
| 9.8 | Future Storage: Embedded Turso / libSQL Persistence Engine | Evaluate & integrate Turso (`libsql`) in local embedded mode (`./data/paperpilot.db`) with native vector search (`libsql_vector`) for persistent multi-document indexing, audit trails, and optional edge replication. |
| 9.9 | Semantic PDF AST & Layout Extraction (`unpdf`) | Evaluate `unpdf` (`iyulab/unpdf`) as a high-fidelity Rust/WASM AST extractor for semantic text, bounding boxes, and complex document structure reconstruction. |
| 9.10 | High-Fidelity Typesetting Engine (`typst`) | Evaluate the `typst` compiler as an engine for Markdown/HTML → publication-grade PDF synthesis with native math formulas, customizable layout grids, and reproducible export. |
| 9.11 | Embedded Local Embedding & Semantic RAG (`fastembed-rs`) | Integrate pure-Rust/ONNX local embeddings (`fastembed-rs`) paired with `libsql_vector` for 100% offline document search, semantic chunking, and air-gapped RAG pipelines. |

---

