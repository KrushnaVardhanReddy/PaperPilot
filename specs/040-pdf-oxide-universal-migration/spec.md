# Spec 040 — Universal `pdf_oxide` Core Engine Migration & High-Performance Extraction Suite

## 1. Overview & Vision
`pdf_oxide` (v0.3.78) provides a pure-Rust, permissive (MIT/Apache), sub-millisecond (0.8ms mean) PDF parsing engine with built-in XY-Cut layout analysis, perfect word spacing, and native Markdown/HTML synthesis.

Currently, PaperPilot relies on low-level object iteration via `lopdf` for text extraction, searches, and conversions. While functional, this produces known limitations:
- `pdf_convert_markdown` misses multi-column flows and table markdown (`| Col 1 | Col 2 |`).
- `pdf_convert_html` generates basic unstructured text containers without semantic typography.
- `pdf_extract_text` occasionally produces merged words or disjointed character lines.
- `pdf_search` scans flat unindexed streams without returning precise bounding-box coordinates for desktop viewer highlight overlays.

**Objective:**
Integrate `pdf_oxide` across PaperPilot's extraction and document conversion toolchain to achieve **sub-1ms high-accuracy extraction**, **structured table/column Markdown & HTML conversion**, and **spatial search indexing** across all 5 deployment surfaces.

---

## 2. Target Operations & Technical Upgrades

### 2.1 `pdf_convert_markdown` (High-Fidelity Document-to-Markdown)
- **Current State:** Basic line-by-line text dump with rough indentation heuristics.
- **Upgrade:**
  - Route through `pdf_oxide::to_markdown(&bytes, options)` (or layout-aware markdown exporter).
  - Preserve:
    - Headings (`#`, `##`, `###`) detected via font metrics and text matrix scales.
    - True reading order across 2-column and 3-column articles (eliminating horizontal sentence interleaving).
    - Markdown tables (`| Header | Header |` / `|---|---|`) for detected grids.
    - Inline formatting (`**bold**`, `*italic*`) based on font descriptors.

### 2.2 `pdf_convert_html` (Semantic HTML5 Document Synthesis)
- **Current State:** Simple `<pre>` or raw `<div>` wrapping without semantic DOM elements.
- **Upgrade:**
  - Generate clean semantic HTML5 (`<article>`, `<section>`, `<h1>`-`<h6>`, `<p>`, `<table>`, `<thead>`, `<tbody>`).
  - Embed extracted images (`<img src="data:image/png;base64,..."/>`) directly in layout flow.
  - Retain word spacing and font family hints.

### 2.3 `pdf_extract_text` (High-Accuracy Spatial Text Extraction)
- **Current State:** Character stream concatenation; occasionally misses whitespace or encounters kerning-induced word collisions.
- **Upgrade:**
  - Leverage `pdf_oxide`'s word-spacing heuristic and XY-cut flow.
  - Return flawless text stream with proper whitespace, line endings, and paragraph breaks.
  - Support `--layout` flag:
    - `--layout flow`: Logical reading order (columns de-weaved).
    - `--layout raw`: Standard top-to-bottom stream extraction.

### 2.4 `pdf_search` (Bounding-Box Aware Spatial Search)
- **Current State:** Returns only matching string snippets and page numbers.
- **Upgrade:**
  - Return exact bounding boxes: `{"page": 1, "text": "...", "bbox": {"x": 100.5, "y": 250.0, "width": 80.2, "height": 12.0}}`.
  - Enables instant visual highlighting in Desktop UI (`apps/desktop`) and Web Viewer (`apps/web`).

### 2.5 Phase 5.8: Zero-Python Local AI / RAG Ingestion Pipeline
- **New Tool: `pdf_rag_chunks`**:
  - High-speed document chunker designed for Vector DBs / LLM context windows.
  - Splits documents by semantic sections (headers, tables, paragraphs) with source page and bounding box provenance.
  - Sub-5ms latency per 10-page document (50× faster than Python Docling/Unstructured).

---

## 3. Architecture & Constraints

1. **Pure-Rust & WASM Compatibility**:
   - `pdf_oxide` must be configured with features compatible with `wasm32-unknown-unknown` (no native C dependencies, no blocking OS threads).
   - If optional ML/OCR features in `pdf_oxide` require external C libraries, guard them behind optional flags (`default-features = false`).
2. **Memory Ceiling**:
   - Keep RSS memory strictly below the $35\,\text{MB}$ limit across long-running gateway sessions.
3. **Penta-Interface Coverage**:
   - CLI: `paperpilot convert --format markdown`, `paperpilot convert --format html`, `paperpilot text`, `paperpilot search`.
   - MCP: `pdf_convert_markdown`, `pdf_convert_html`, `pdf_extract_text`, `pdf_search`, `pdf_rag_chunks`.
   - Gateway REST API: `/api/v1/pdf/convert`, `/api/v1/pdf/tools/{tool}`.
   - WASM & Cloudflare Edge: 100% in-memory byte buffers.

---

## 4. Acceptance Criteria & Quality Gates
1. **Fidelity Assertions**:
   - Multi-column academic paper test fixture converted to Markdown preserves column reading order.
   - Financial report fixture with tables converts to valid Markdown tables and HTML `<table>`.
2. **Performance Benchmarks**:
   - `pdf_extract_text` and `pdf_convert_markdown` benchmarked at $<5\,\text{ms}$ on standard 5-page documents.
3. **Test Suite & Concurrency**:
   - Pass all tests across `paperpilot-pdf`, `paperpilot-mcp`, `paperpilot-gateway`, `paperpilot-wasm`, `penta-interface-e2e`.
   - Run tests bounded to 6 cores (`-j 6`).
4. **Report**:
   - Author `reports/PDF_OXIDE_UNIVERSAL_MIGRATION_REPORT.md`.
