# PaperPilot

**PDF tools for your AI agent — and for you. Pure Rust. Runs locally. Files never leave your machine.**

<!-- TODO: replace with a real 10–15s GIF: ask Claude to merge + compress a PDF via MCP -->
![PaperPilot demo](docs/assets/demo.gif)

[**Try it in your browser →**](https://usepaperpilot.com) · [Download](https://github.com/KrushnaVardhanReddy/PaperPilot/releases) · [Roadmap](ROADMAP.md)

PaperPilot is a PDF and document toolkit written in Rust. The same engine runs as:

| Surface | Use it for |
|---|---|
| **MCP server** | Give Codex, Claude Code, Kimi Code, Pi, Claude Desktop, Cursor, or any MCP client the ability to merge, split, OCR, and compress PDFs |
| **CLI** | Batch and scripted workflows |
| **Browser (WASM)** | Zero-install tools; files are processed in browser memory, nothing is uploaded |
| **Desktop app** | Tauri + Svelte GUI with viewer, markup, and diff |
| **Local REST API** | Call it from Python, Node, or n8n at `localhost:7823` |

No Chromium, no C++ runtime, no cloud calls.

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

- **Organize & Layout:** merge, split, rotate, delete pages, reorder pages, extract pages, booklet imposition, N-up grid, multi-sheet poster tile slicing (`pdf_posterize`)
- **Optimize & Compress:** lossy compression, stream deduplication, CMYK safety guards
- **Extract & Ingest:** structured text, detected Markdown tables, native Open-WebUI & Apache Tika ingestion adapter (`/tika/text`, `/tika/json/md`, `/rmeta/text`), pure-Rust OCR (PaddleOCR via `rten`)
- **Security & Analysis:** watermark, validate signatures, remove signatures, PII scan, deep sanitize, malformed fuzzing & panic-free error handling
- **Office Data:** Excel → JSON, Excel → CSV

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

---

## Contributing & Community

Issues and PRs are welcome. Good first contributions: bug reports with a sample PDF, new tool requests, and docs fixes.

Special thanks to all community members and contributors recognized in [CONTRIBUTORS.md](CONTRIBUTORS.md).

## License

Community Edition (this repo): [Apache 2.0](LICENSE). It includes the CLI, MCP server, REST gateway, WASM engine, and desktop app.

An Enterprise Edition (SSO, RBAC, audit logs, DLP) is developed separately.
