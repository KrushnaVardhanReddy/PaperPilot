# Spec 041 — Pure-Rust Chromium-Free URL-to-PDF Archival Engine via `browser_oxide`

## 1. Overview & Problem Statement
Currently, converting dynamic web pages or external URLs to PDF requires either:
1. Spawning system Google Chrome / Chromium via headless CDP flags (heavy, brittle, requires external browser installation, breaks on minimal Linux or containerized environments).
2. Static HTML-to-PDF converters that fail on modern client-rendered JavaScript (React, Vue, Svelte) and web apps.

`browser_oxide` is a pure-Rust, independent headless browser engine featuring its own HTML parser, CSS layout engine, and V8 runtime via `deno_core` with zero dependence on Chromium or CDP.

**Objective:**
Integrate `browser_oxide` into PaperPilot's conversion toolchain to provide a **100% pure-Rust, zero-Chromium `url_to_pdf` conversion and web archiving tool** that renders JavaScript-heavy web pages directly to high-fidelity PDF documents.

---

## 2. Technical Architecture & Design

### 2.1 Operation Specification: `url_to_pdf`
1. **Engine Pipeline**:
   - `browser_oxide::fetch_page` or `render_page` navigates to target URL or loads raw HTML + JS.
   - Evaluates client-side scripts to a stable layout state.
   - Extracts styled DOM layout boxes and vectors.
   - Synthesizes output PDF via PaperPilot's native raster/vector printer (`hayro` / `tiny-skia` / `krilla`).
2. **Options**:
   - `--url <URL>`: Source website.
   - `--delay <MS>`: Wait time for dynamic hydration (default 500ms).
   - `--page-size <A4|Letter>`: Paper format.
   - `--print-background <bool>`: Include CSS backgrounds.

### 2.2 Product Surfaces & Integration
1. **CLI (`paperpilot-cli`)**:
   `paperpilot convert-url --url https://example.com --output page.pdf`
2. **MCP Tool (`paperpilot-mcp`)**:
   `pdf_convert_url` tool enabling AI agents to archive web citations and live articles into PDFs.
3. **REST Gateway (`paperpilot-gateway`)**:
   `POST /api/v1/pdf/convert/url` for enterprise backend web-to-PDF generation.
4. **Desktop UI (`apps/desktop`)**:
   - One-click "Capture Webpage to PDF" tool card in Operations Dock.
   - Side-drawer AI Agent (`PdfChatPanel.svelte`): When examining PDF papers with web links, the agent uses `browser_oxide` to inspect live web citations in-place and verify sources without being blocked by anti-bot captchas.
5. **Template & Certificate Studio (`apps/web`)**:
   - Hydrate dynamic HTML+JS templates (invoices, certificates, charting dashboards) into vector PDFs.

---

## 3. Acceptance Criteria
1. Convert dynamic JS-rendered web pages to PDF without Google Chrome installed.
2. Verified across CLI, MCP, and Gateway interfaces.
3. Tests bounded with `-j 6`.
4. Documented in `reports/URL_TO_PDF_BROWSER_OXIDE_REPORT.md`.
