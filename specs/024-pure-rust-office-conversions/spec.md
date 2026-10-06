# Spec 024: Pure-Rust High-Speed Office & Document Conversions (Zero-Chrome)

## Status: APPROVED / IN IMPLEMENTATION (Phase 5.6)

## 1. Executive Summary & Problem Statement
Currently, PaperPilot achieves sub-10ms performance across 41 out of 44 PDF operations. However, three conversion tools (`pdf_convert_excel`, `pdf_convert_markdown`, and `pdf_convert_html`) rely on spawning a headless Chromium browser instance via `headless_chrome`.

While Chromium provides layout support, launching a full browser process imposes a **~2,400ms – 2,750ms** cold-start latency bottleneck, increases memory overhead, requires an external Chrome binary, and prevents compiling these conversion routines into client-side WebAssembly.

By replacing `headless_chrome` with pure-Rust vector document generators:
1. **Excel to PDF (`pdf_convert_excel`)**: Replaced with **`genpdf`** (or `printpdf`) reading cell tabular matrices via **`calamine`**. Latency drops from **~2,500ms to < 25ms** (a **100x speedup**).
2. **Markdown to PDF (`pdf_convert_markdown`)**: Replaced with **`typst`**, producing publication-grade typography, syntax-highlighted code blocks, and math directly in **< 35ms**.
3. **HTML to PDF (`pdf_convert_html`)**: Replaced with **`fulgur`** (pure-Rust HTML/CSS-to-PDF engine with CSS Paged Media `@page` support). Latency drops from **~2,750ms to < 50ms** with zero external browser dependencies.

---

## 2. Technical Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ 1. EXCEL PIPELINE: calamine -> genpdf -> lopdf                              │
│    • Ingestion: calamine parses .xlsx / .xls sheets into in-memory 2D cells │
│    • Layout: genpdf constructs paginated table grids, auto column sizing    │
│    • Output: Crisp vector PDF bytes (< 25ms, zero browser process)          │
├─────────────────────────────────────────────────────────────────────────────┤
│ 2. MARKDOWN PIPELINE: pulldown-cmark -> typst -> Vector PDF                 │
│    • Ingestion: Markdown content parsed into structured typographic blocks  │
│    • Layout: typst layout engine (headers, code blocks, lists, quotes)      │
│    • Output: High-speed native vector PDF bytes (< 35ms)                    │
├─────────────────────────────────────────────────────────────────────────────┤
│ 3. HTML/CSS PIPELINE: fulgur -> Vector PDF                                  │
│    • Ingestion: Raw HTML string with embedded/external CSS                  │
│    • Layout: fulgur pure-Rust CSS Paged Media box model & pagination        │
│    • Output: Sub-50ms native vector PDF without Chromium process            │
├─────────────────────────────────────────────────────────────────────────────┤
│ 4. MANDATORY VERIFICATION GATE                                              │
│    • scripts/test_tri_interface_e2e.py MUST score 44/44 PASS                │
│    • All 132 assertions (44 tools x 3 interfaces: CLI, MCP, REST API) green │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 3. Implementation Plan

### Phase 1: Dependency Integration (`paperpilot-pdf/Cargo.toml`)
- Add `genpdf = "0.2"` (pure-Rust table/document generator).
- Add `typst = "0.13"` or latest compatible (pure-Rust typesetting compiler).
- Add `fulgur = "0.40"` (pure-Rust HTML/CSS-to-PDF rendering engine).
- Remove or deprecate `headless_chrome`.
- Validate with `cargo check -p paperpilot-pdf`.

### Phase 2: Operations Overhaul (`paperpilot-pdf/src/operations/conversion.rs`)
- Refactor `ExcelToPdfOperation`:
  - Read workbook sheets via `calamine::open_workbook_auto`.
  - Format grid with clean borders, header wrapping, and cell padding.
  - Render directly to destination file using `genpdf`.
- Refactor `MarkdownToPdfOperation`:
  - Compile markdown/typst AST directly via `typst::compile`.
  - Export vector PDF bytes with standard fallback font embedding.
- Refactor `HtmlToPdfOperation`:
  - Render HTML string with CSS stylesheets directly via `fulgur::render_to_pdf` (or `fulgur` builder).
  - Eliminate all `headless_chrome::Browser` and tempfile browser spawning.

### Phase 3: Tri-Interface Parity Validation
- Run unit tests: `cargo test -p paperpilot-pdf`.
- Run E2E tri-interface suite: `python3 scripts/test_tri_interface_e2e.py`.
- Ensure all 44 tools across CLI, MCP, and REST API produce valid `%PDF-` files and pass all assertions.
- Measure and record latency drop in `reports/PURE_RUST_OFFICE_CONVERSIONS_REPORT.md`.
