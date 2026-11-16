---
title: "pdf_convert_html"
description: "Reference, CLI, MCP, and REST API commands for pdf_convert_html."
sidebar:
  order: 15
---

**Status**: ✅ Verified  |  **Format**: HTML (`.html`) $\to$ Publication PDF (`.pdf`)

### What it does
**Renders HTML source documents and web pages directly into styled, publication-ready PDF files.**  
Powered by a lightweight headless layout engine, `pdf_convert_html` transforms HTML markup, tables, images, and embedded CSS into high-fidelity PDF documents with pagination, custom margins, and headers/footers.

- **CSS Theme Presets**: Built-in typography presets including `github`, `elegant`, `minimal`, `branded`, and `compact`.
- **Custom Stylesheet Injection**: Inject external `.css` files or inline CSS overrides directly into the rendering pipeline.
- **Page Geometry Control**: Configurable page formats (e.g. `A4`, `Letter`), margins (`margin_mm`), and orientations (`portrait`, `landscape`).

---

### CLI

Compile HTML to PDF using `paperpilot html-to-pdf` or the unified `convert` command:

```bash
# Direct Subcommand
paperpilot html-to-pdf \
  --input tests/e2e_fixtures/real/test.html \
  --output tests/e2e_fixtures/out/penta_e2e/document.pdf \
  --json
```

```bash
# Unified Convert Command with Custom Stylesheet Preset
paperpilot convert \
  --input tests/e2e_fixtures/real/test.html \
  --format pdf \
  --css-preset github \
  --output tests/e2e_fixtures/out/penta_e2e/document.pdf \
  --json
```

*(Aliases supported: `html-to-pdf`, `html_to_pdf`, `convert-html`, `pdf-convert-html`, `pdf_convert_html`)*

#### CLI Parameters
- `--input` *(string, required)*: Path to source HTML file.
- `--output` *(string, optional, default: `"output.pdf"`)*: Destination path for rendered PDF document.
- `--format` *(string, required for `convert`)*: Set to `pdf`.
- `--css` *(string, optional)*: Path to an external CSS stylesheet.
- `--css-preset` *(string, optional)*: Theme preset (`github`, `elegant`, `minimal`, `branded`, `compact`).
- `--inline-css` *(string, optional)*: Custom CSS rules injected directly into the document.
- `--json` *(flag, optional)*: Output structured JSON result.

#### Example CLI Output
```json
{"success":true,"operation":"HtmlToPdf","error":null}
```

---

### MCP JSON-RPC

Invoke `pdf_convert_html` via Model Context Protocol:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_convert_html",
    "arguments": {
      "input": "tests/e2e_fixtures/real/test.html",
      "output": "tests/e2e_fixtures/out/penta_e2e/document.pdf",
      "preset": "github",
      "page_size": "A4",
      "orientation": "portrait"
    }
  },
  "id": 1
}
```

#### MCP Schema Parameters
- `input` *(string, required)*: Path to input HTML file.
- `output` *(string, required)*: Destination path for generated PDF file.
- `preset` *(string, optional)*: CSS preset theme (`github`, `elegant`, `minimal`, `branded`, `compact`).
- `custom_css` *(string, optional)*: Custom CSS rules to apply.
- `page_size` *(string, optional)*: Page dimensions (e.g. `A4`, `Letter`).
- `orientation` *(string, optional)*: Page orientation (`portrait` or `landscape`).

#### Example MCP Response
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "content": [
      {
        "type": "text",
        "text": "{\"success\":true,\"message\":\"HTML converted to PDF successfully.\",\"output_path\":\"tests/e2e_fixtures/out/penta_e2e/document.pdf\",\"data\":null}"
      }
    ],
    "isError": false
  }
}
```

---

### REST API

Execute HTML to PDF conversions via PaperPilot Gateway:

```bash
# Standard Tool API Route
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_convert_html \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/test.html",
    "output": "tests/e2e_fixtures/out/penta_e2e/document.pdf",
    "preset": "github"
  }'
```

```bash
# Unified Convert Route
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/convert \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/test.html",
    "format": "pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/document.pdf"
  }'
```

#### Example Gateway Response
```json
{
  "success": true,
  "data": null,
  "error": null
}
```

---

### WASM Support
Headless HTML layout rendering requires OS-level font loading and filesystem access, so execution runs natively via the CLI, REST API Gateway, or MCP daemon.

---

### Known Limitations & Troubleshooting
- **Inline SVG Entities**: Malformed or unescaped XML entities within inline `<svg>` blocks (such as unencoded `&` instead of `&amp;`) will cause XML layout parser warnings and fall back gracefully to skipping that SVG element. Ensure XML attributes and inline SVG elements are well-formed.
- **External Network Assets**: External CSS stylesheets or remote fonts (`http://` / `https://`) are not fetched in strict offline sandbox mode. Embed fonts or supply local stylesheets via `--css`.
- **Directionality Note**: To convert a PDF *to* HTML instead, use `pdf_to_html` (`paperpilot convert --input doc.pdf --format html`). `pdf_convert_html` converts *from* HTML *to* PDF.
