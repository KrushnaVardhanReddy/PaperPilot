---
title: "pdf_convert_markdown"
description: "Reference, CLI, MCP, and REST API commands for pdf_convert_markdown."
sidebar:
  order: 16
---

**Status**: ✅ Verified  |  **Format**: Markdown (`.md`) $\to$ Publication PDF (`.pdf`)

### What it does
**Renders Markdown source documents directly into beautifully styled, publication-ready PDF files.**  
Parses standard CommonMark and GitHub Flavored Markdown (tables, blockquotes, syntax-highlighted code fences, task lists) into structured semantic layouts, then styles and paginates them into high-fidelity PDF documents.

- **Markdown Features Supported**: Headers (`h1`–`h6`), bullet and ordered lists, GFM tables with column alignment, blockquotes, code fences, and emphasis.
- **Theme Presets**: Integrated typography presets including `github`, `elegant`, `minimal`, `branded`, and `compact`.
- **CSS Customization**: Inject external CSS files or custom inline styles for headers, footers, margins, and page breaks.

---

### CLI

Compile Markdown documents to PDF using `paperpilot md-to-pdf` or the unified `convert` command:

```bash
# Direct Subcommand
paperpilot md-to-pdf \
  --input tests/e2e_fixtures/real/test.md \
  --output tests/e2e_fixtures/out/penta_e2e/document.pdf \
  --json
```

```bash
# Unified Convert Command with Theme Preset
paperpilot convert \
  --input tests/e2e_fixtures/real/test.md \
  --format pdf \
  --css-preset elegant \
  --output tests/e2e_fixtures/out/penta_e2e/document.pdf \
  --json
```

*(Aliases supported: `md-to-pdf`, `md_to_pdf`, `convert-markdown`, `pdf-convert-markdown`, `pdf_convert_markdown`)*

#### CLI Parameters
- `--input` *(string, required)*: Path to input Markdown document (`.md`).
- `--output` *(string, optional, default: `"output.pdf"`)*: Destination path for generated PDF document.
- `--format` *(string, required for `convert`)*: Set to `pdf`.
- `--css` *(string, optional)*: Path to an external CSS stylesheet.
- `--css-preset` *(string, optional)*: Theme preset (`github`, `elegant`, `minimal`, `branded`, `compact`).
- `--inline-css` *(string, optional)*: Custom CSS rules injected directly into `<style>`.
- `--json` *(flag, optional)*: Output structured JSON execution result.

#### Example CLI Output
```json
{"success":true,"operation":"MdToPdf","error":null}
```

---

### MCP JSON-RPC

Invoke `pdf_convert_markdown` via Model Context Protocol:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_convert_markdown",
    "arguments": {
      "input": "tests/e2e_fixtures/real/test.md",
      "output": "tests/e2e_fixtures/out/penta_e2e/document.pdf",
      "preset": "elegant",
      "page_size": "A4",
      "orientation": "portrait"
    }
  },
  "id": 1
}
```

#### MCP Schema Parameters
- `input` *(string, required)*: Path to the input Markdown file.
- `output` *(string, required)*: Destination path for the generated PDF.
- `preset` *(string, optional)*: CSS preset theme (`github`, `elegant`, `minimal`, `branded`, `compact`).
- `custom_css` *(string, optional)*: Custom CSS styling overrides.
- `page_size` *(string, optional)*: Page size (e.g. `A4`, `Letter`).
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
        "text": "{\"success\":true,\"message\":\"Markdown converted to PDF successfully.\",\"output_path\":\"tests/e2e_fixtures/out/penta_e2e/document.pdf\",\"data\":null}"
      }
    ],
    "isError": false
  }
}
```

---

### REST API

Execute Markdown to PDF conversions via PaperPilot Gateway:

```bash
# Standard Tool API Route
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_convert_markdown \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/test.md",
    "output": "tests/e2e_fixtures/out/penta_e2e/document.pdf",
    "preset": "elegant"
  }'
```

```bash
# Unified Convert Route
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/convert \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/test.md",
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
Markdown parsing and layout styling rely on native layout primitives and font metrics, run natively in the CLI, REST API Gateway, or MCP daemon.

---

### Known Limitations & Troubleshooting
- **Tables and Page Breaks**: By default, table rows include `page-break-inside: avoid`. Exceptionally tall cells containing large bodies of text may cause awkward page gaps if they exceed page height.
- **Directionality Note**: To extract text from a PDF *into* Markdown, use `pdf_to_markdown` (`paperpilot convert --input doc.pdf --format md`). `pdf_convert_markdown` renders *from* Markdown *to* PDF.
