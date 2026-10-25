# PaperPilot

**PDF tools for your AI agent — and for you. Pure Rust. Runs locally. Files never leave your machine.**

<!-- TODO: replace with a real 10–15s GIF: ask Claude to merge + compress a PDF via MCP -->
![PaperPilot demo](docs/assets/demo.gif)

[**Try it in your browser →**](https://usepaperpilot.com) · [Download](https://github.com/KrushnaVardhanReddy/PaperPilot/releases) · [Roadmap](ROADMAP.md)

PaperPilot is a PDF and document toolkit written in Rust. The same engine runs as:

| Surface | Use it for |
|---|---|
| **MCP server** | Give Claude Desktop, Cursor, or any MCP client the ability to merge, split, OCR, and compress PDFs |
| **CLI** | Batch and scripted workflows |
| **Browser (WASM)** | Zero-install tools; files are processed in browser memory, nothing is uploaded |
| **Desktop app** | Tauri + Svelte GUI with viewer, markup, and diff |
| **Local REST API** | Call it from Python, Node, or n8n at `localhost:7823` |

No Chromium, no C++ runtime, no cloud calls.

---

## Quick start

### 1. Use it with Claude / Cursor (MCP)

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
# Merge
paperpilot merge --inputs doc1.pdf doc2.pdf --output merged.pdf

# Compress
paperpilot compress --input in.pdf --quality 75 --output compressed.pdf

# Rotate pages 1, 3–5 by 90°
paperpilot rotate --input in.pdf --pages 1,3-5 --angle 90 --output out.pdf
```

### 3. Use it in the browser

Go to [usepaperpilot.com](https://usepaperpilot.com). Open your browser's Network tab while you work — you'll see no file uploads.

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

## What it can do today (v0.6)

**Organize:** merge, split, rotate, delete pages, reorder pages, extract pages, booklet imposition, N-up grid
**Optimize:** compress
**Extract:** plain text, OCR (pure-Rust PaddleOCR via `rten`)
**Security:** watermark, validate signatures, remove signatures, PII scan, deep sanitize
**Office data:** Excel → JSON, Excel → CSV

Coming next (see [ROADMAP.md](ROADMAP.md)):

- **v0.7:** PDF ↔ Markdown / HTML / JSON
- **v0.8:** PDF → Word / Excel / PowerPoint
- **v0.9:** encrypt, decrypt, sign, redact, forms, annotations
- **v1.0:** render, flatten, repair, compare, PDF/A, Bates numbering

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
<script src="https://cdn.usepaperpilot.com/embed.js" data-tools="merge,compress,ocr"></script>
<div id="paperpilot-widget"></div>
```

## Local REST API

```bash
paperpilot serve --port 7823
curl -X POST http://localhost:7823/api/v1/pdf/merge \
  -H "Content-Type: application/json" \
  -d '{"inputs": ["a.pdf", "b.pdf"], "output": "out.pdf"}'
```

Swagger docs at `http://localhost:7823/docs`.

---

## Contributing

Issues and PRs are welcome. Good first contributions: bug reports with a sample PDF, new tool requests, and docs fixes.

## License

Community Edition (this repo): [Apache 2.0](LICENSE). It includes the CLI, MCP server, REST gateway, WASM engine, and desktop app.

An Enterprise Edition (SSO, RBAC, audit logs, DLP) is developed separately.