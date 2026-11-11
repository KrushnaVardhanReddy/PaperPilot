# PaperPilot

**PDF tools for your AI agent — and for you. Pure Rust. Runs locally. Files never leave your machine.**

[![CI](https://github.com/KrushnaVardhanReddy/PaperPilot/actions/workflows/rust.yml/badge.svg)](https://github.com/KrushnaVardhanReddy/PaperPilot/actions)
[![OpenSSF Scorecard](https://api.securityscorecards.dev/projects/github.com/KrushnaVardhanReddy/PaperPilot/badge)](https://securityscorecards.dev/viewer/?uri=github.com/KrushnaVardhanReddy/PaperPilot)
[![Security Policy](https://img.shields.io/badge/Security-Policy_Enforced-brightgreen.svg)](SECURITY.md)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE)
[![Rust 1.75+](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org)
[![MCP Compatible](https://img.shields.io/badge/MCP-100%25_Compatible-purple.svg)](https://modelcontextprotocol.io)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](CONTRIBUTING.md)

![PaperPilot demo](docs/assets/gifs/demo_01_ask_natural_language.gif)

[**Try it in your browser →**](https://usepaperpilot.com) · [Download](https://github.com/KrushnaVardhanReddy/PaperPilot/releases) · [Contributing](CONTRIBUTING.md) · [Roadmap](ROADMAP.md)

PaperPilot is a PDF and document toolkit written in Rust. The same engine runs as:

| Surface | Use it for |
|---|---|
| **MCP server** | Give Codex, Claude Code, Gemini CLI, Cursor, Claude Desktop, or any MCP client instant document superpowers |
| **CLI** | High-speed batch workflows and automated terminal pipelines |
| **Browser Extension** | In-tab right-click actions on web/arXiv PDFs, clean article capture, and WASM toolkit |
| **Browser (WASM)** | Zero-install web tools; files are processed in browser memory, nothing is uploaded |
| **Desktop app** | Tauri + Svelte 5 GUI with interactive canvas, viewer, and pipeline builder |
| **Local REST API** | Call it from Python, Node.js, n8n, LangChain, or LlamaIndex at `localhost:7823` |

No Chromium, no C++ runtime, no cloud calls.

---

## ⚡ 10-Second Installation

### Homebrew (macOS & Linux)
```bash
brew install KrushnaVardhanReddy/tap/paperpilot
```

### One-Line Install Script
**macOS & Linux (POSIX):**
```bash
curl -fsSL https://raw.githubusercontent.com/KrushnaVardhanReddy/PaperPilot/main/scripts/install.sh | sh
```

**Windows (PowerShell):**
```powershell
irm https://raw.githubusercontent.com/KrushnaVardhanReddy/PaperPilot/main/scripts/install.ps1 | iex
```

### Self-Healing System Diagnostics
Verify your environment, gateway port, temp storage, and OCR models with one command:
```bash
paperpilot doctor
# PaperPilot System Diagnostics (v0.1.0)
# ==================================================
# Status:    Healthy
# [✓] Temp Directory: /tmp (writable & ephemeral cleanup verified)
# [✓] Gateway Port: 7823 available
# [✓] OCR Models: models/ocr/ppocrv4 (det.onnx, rec.onnx found)
# [✓] Disk Space: > 5 GB available
```

**Open-WebUI integration (source builds):** use the local gateway as a
PDF-only Tika-compatible extraction server. See the
[Open-WebUI setup guide](docs/integrations/open_webui.md).

---

## Quick start

### 1. Use it with your coding agent (MCP)

Codex, Claude Code, Kimi Code, and current Pi releases connect to the same local
stdio server. See the [coding agent setup guide](docs/coding-agents.md) for each
client's configuration and verification steps. Builds from this source can also
print ready-to-merge configuration:

```bash
paperpilot-mcp --client-config codex
# Also: claude-code, kimi-code, pi, claude-desktop, cursor
```

Download `paperpilot-mcp` from [Releases](https://github.com/KrushnaVardhanReddy/PaperPilot/releases), then add it to `claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "paperpilot": {
      "command": "/path/to/paperpilot-mcp"
    }
  }
}
```

Restart Claude and try: *"Merge these two PDFs and compress the result."*

### 2. Use the CLI

```bash
# Semantic Chunking for AI Agents & RAG (Spec 159)
paperpilot pdf-chunk --input contract.pdf --target-tokens 500 --overlap 50 --json

# Merge multiple documents
paperpilot merge --inputs doc1.pdf doc2.pdf --output merged.pdf

# High-efficiency compression
paperpilot compress --input in.pdf --quality 75 --output compressed.pdf

# Search pattern or keywords across document pages
paperpilot search --input in.pdf --query "Invoice" --json

# Rotate pages 1, 3–5 by 90°
paperpilot rotate --input in.pdf --pages 1,3-5 --angle 90 --output out.pdf
```

### 3. Use it in the browser & extension

- **Web App**: Go to [usepaperpilot.com](https://usepaperpilot.com) for zero-install client-side WebAssembly tools.
- **Companion Extension**: Load the Manifest V3 browser extension from [`apps/extension`](apps/extension) for in-tab context menus, right-click PDF actions, and clean web article capture.

### 4. Build from source

```bash
git clone https://github.com/KrushnaVardhanReddy/PaperPilot.git
cd PaperPilot
cargo build --release -p paperpilot-cli -p paperpilot-mcp -p paperpilot-gateway
```

Desktop app:

```bash
cd apps/desktop && pnpm install && pnpm tauri dev
```

Requires Rust 1.75+, Node.js, and pnpm.

---

## What it can do (91 Tools Across 5 Interfaces)

- **AI Agent Intelligence & RAG**:
  - `pdf_chunk`: Structure-aware semantic chunker respecting `#`/`##` headings, `\n\n` paragraphs, and keeping Markdown tables (`|...|`) atomic with dynamic overlap rewinding and boilerplate running header/footer suppression.
  - `pdf_search` & `workspace_search`: Sub-millisecond grep and pattern searching across multi-document repositories.
  - `pdf_extract_patterns`: High-precision regex and pattern extraction for invoices, order IDs, tracking numbers, and dates.
  - `pdf_classify_type`: Automatic document classification (invoices, receipts, legal briefs, tax filings, research papers).
- **Core Layout & Imposition**:
  - Merge, split, rotate, delete pages, reorder pages, extract pages, booklet imposition, N-up grid, and multi-sheet poster tile slicing (`pdf_posterize`).
  - Interactive forms: fill (`pdf_form_fill`), extract form fields (`pdf_form_extract`), and flatten.
  - Digital signatures (`pdf_sign`), bookmarks/TOC (`pdf_bookmarks`), and crop/margins (`pdf_crop`, `pdf_margin`).
- **Office & Cross-Format Transformations**:
  - Bidirectional PDF ⇄ Word (`pdf_to_docx`, `docx_to_pdf`).
  - Bidirectional PDF ⇄ Excel (`pdf_to_xlsx`, `excel_to_csv`, `excel_to_json`).
  - PowerPoint slides (`pdf_to_pptx`), HTML (`pdf_to_html`), and clean Markdown (`pdf_to_markdown`, `office_to_markdown`).
- **OCR & Document Vision**:
  - Neural PP-OCRv4 detection & recognition via `rten` and `ocrs` (det.onnx + rec.onnx).
  - Searchable OCR text layer injection (`pdf_ocr_layer`) and auto-deskewing (`pdf_deskew`).
- **Security & PII Redaction**:
  - Automated PII scanning (`pdf_pii_scanner`) detecting SSNs, credit cards, emails, phone numbers.
  - Cryptographic validation & hashing (`pdf_validate`, `pdf_hash`), password protection (`pdf_protect`, `pdf_unlock`), and irreversible redaction (`pdf_redact`).
- **Zero-Copy High Performance Engine**:
  - Zero-copy memory-mapped I/O (`DocumentSource` via `memmap2`) for files $\ge 10\text{ MB}$, keeping resident memory flat (< 100 MB) during search, chunking, and PII scanning of 500+ page documents.
  - Universal streaming page batching (`PageBatchIterator`) to prevent OOM errors.

---

## Why another PDF tool?

- **Private by default.** Everything runs locally or in your browser tab.
- **Agent-ready.** MCP support is first-class, not an afterthought.
- **One engine, many surfaces.** CLI, MCP, REST, WASM, and desktop share the same Rust core, so behavior is consistent.
- **Small and dependency-light.** Pure Rust, built on [`lopdf`](https://github.com/J-F-Liu/lopdf), [`pdf_oxide`](https://crates.io/crates/pdf_oxide), [`rten`](https://github.com/robertknight/rten), and friends.

## Benchmarks

<!-- TODO: add a real table: operation | input size | time | machine -->
Run `cargo bench --workspace` to reproduce. Methodology and results: [SYSTEM_BENCHMARK_REPORT.md](SYSTEM_BENCHMARK_REPORT.md).

---

## Architecture

```
Desktop (Tauri) · CLI · REST (Axum) · MCP (stdio) · WASM (browser / edge)
                              │
                       Rust core workspace
   paperpilot-core · paperpilot-pdf · paperpilot-nlp · paperpilot-gateway
   paperpilot-mcp · paperpilot-wasm · paperpilot-storage
```

## Embed in your own site

```html
<script src="https://usepaperpilot.com/embed.js" data-tools="merge,compress,ocr"></script>
<div id="paperpilot-widget"></div>
```

## Local REST API & Document Ingestion
 
```bash
paperpilot serve --port 7823
# Standard PDF Operations (JSON & multipart)
curl -X POST http://localhost:7823/api/v1/pdf/merge \
  -H "Content-Type: application/json" \
  -d '{"inputs": ["a.pdf", "b.pdf"], "output": "out.pdf"}'

# Open-WebUI & Tika In-Memory Ingestion (sub-5ms)
curl -X PUT http://localhost:7823/tika/text \
  -H "Content-Type: application/pdf" \
  --data-binary @sample.pdf
```

Interactive Swagger OpenAPI docs at `http://localhost:7823/swagger-ui`.

## Security & Supply Chain Integrity

PaperPilot processes sensitive, proprietary, and personal documents on-device without cloud dependencies. We maintain rigorous software supply chain and security compliance:

- **OpenSSF Scorecard**: **8.2 / 10**
- **Token Permissions**: 100% scoped to `permissions: read-all` (principle of least privilege).
- **SAST & Secret Gate**: Continuous Semgrep OSS and Trivy security audits gating all commits.
- **Dependency Hardening**: All third-party GitHub Actions pinned to immutable 40-character commit SHAs.
- **Responsible Disclosure**: See [`SECURITY.md`](SECURITY.md) for our vulnerability disclosure policy and 48-hour response SLA.

---

## Contributing & Community


Issues and PRs are welcome. Good first contributions: bug reports with a sample PDF, new tool requests, and docs fixes.

Special thanks to all community members and contributors recognized in [CONTRIBUTORS.md](CONTRIBUTORS.md).

## License

Community Edition (this repo): [Apache 2.0](LICENSE). It includes the CLI, MCP server, REST gateway, WASM engine, and desktop app.

An Enterprise Edition (SSO, RBAC, audit logs, DLP) is developed separately.
