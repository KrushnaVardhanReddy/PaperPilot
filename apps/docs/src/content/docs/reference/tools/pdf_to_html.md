---
title: "pdf_to_html"
description: "Reference, CLI, MCP, and REST API commands for pdf_to_html."
sidebar:
  order: 47
---

**Status**: ✅ Verified  |  **Format**: PDF (`.pdf`) $\to$ Responsive HTML5 (`.html`)

### What it does
**Converts a PDF into styled, standalone HTML5.**  
Extracts document layout, fonts, headings, paragraphs, and inline styles into responsive web-friendly markup without requiring browser plugins.

- **Standalone Document Generation**: Produces complete HTML5 documents with self-contained `@page` styling, clean layout margins, and print-ready stylesheets.
- **Semantic Text Hierarchy**: Preserves document structure including headers (`h1`–`h4`), ordered and unordered paragraphs, and bold/italic font styling.
- **CSS Presets & Custom Styling**: Supports preset templates or custom injected inline CSS rules when executed through the conversion engine.

---

### CLI
Convert PDF documents to HTML format using the unified `convert` command:

```bash
paperpilot convert \
  --input tests/e2e_fixtures/real/single_page.pdf \
  --format html \
  --output tests/e2e_fixtures/out/penta_e2e/document.html \
  --json
```

#### CLI Parameters
- `--input` *(string, required)*: Path to source PDF document.
- `--format` *(string, required)*: Target output format (`html` or `htm`).
- `--output` *(string, optional, default: `"output.html"`)*: Destination path for HTML file.
- `--css` *(string, optional)*: Path to an external `.css` stylesheet to embed.
- `--css-preset` *(string, optional)*: Built-in CSS theme preset.
- `--inline-css` *(string, optional)*: Custom CSS strings injected into `<style>` block.
- `--json` *(flag, optional)*: Emit structured JSON completion status.

---

### MCP JSON-RPC
Invoke `pdf_to_html` through Model Context Protocol:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_to_html",
    "arguments": {
      "input": "tests/e2e_fixtures/real/single_page.pdf",
      "output": "tests/e2e_fixtures/out/penta_e2e/document.html"
    }
  },
  "id": 1
}
```

#### MCP Schema Parameters
- `input` *(string, required)*: The input PDF file path.
- `output` *(string, required)*: The output HTML destination file path.

#### Example MCP Response
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "content": [
      {
        "type": "text",
        "text": "{\"success\":true,\"message\":\"Converted to HTML successfully.\",\"output_path\":\"tests/e2e_fixtures/out/penta_e2e/document.html\",\"data\":null}"
      }
    ],
    "isError": false
  }
}
```

---

### REST API
Execute HTML conversions via PaperPilot Gateway:

```bash
# Dedicated Gateway Route
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/pdf_to_html \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/document.html"
  }'
```

```bash
# Format Conversion Route
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/convert \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "format": "html",
    "output": "tests/e2e_fixtures/out/penta_e2e/document.html"
  }'
```

```bash
# Universal MCP Tool Route
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_to_html \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/document.html"
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
Direct in-memory buffer `pdf_to_html` compilation is not currently included in `paperpilot-wasm`. In browser environments, convert documents using the REST Gateway API or MCP server.

---

### Known Limitations & Troubleshooting
- **CLI Subcommand Note**: Always use `paperpilot convert --input <INPUT> --format html --output <OUTPUT>`. The standalone subcommand `paperpilot pdf-to-html` is a stub placeholder that prints execution logs but delegates production conversions through `convert`.
- **Absolute Vector Canvas vs Reflowable HTML**: PDFs containing complex intersecting CAD paths or background geometry are converted to semantic text flows. For pixel-exact graphical previews, consider `paperpilot render` to image formats (`.png` / `.webp`).
