---
title: "pdf_header_footer"
description: "Reference, CLI, MCP, and REST API commands for pdf_header_footer."
sidebar:
  order: 28
---

**Status**: ✅ Verified  |  **Format**: PDF (`.pdf`)

### What it does
**Adds customizable headers and footers with configurable positioning, margins, and text alignment across pages.**  
Stamps notices, organizational branding, confidential classifications, dates, or custom dynamic headers across all pages in a PDF document.

---

### CLI
Apply headers or footers via PaperPilot CLI:

#### 1. Add Header Banner
```bash
paperpilot header-footer \
  --input tests/e2e_fixtures/single_page.pdf \
  --text "CONFIDENTIAL - INTERNAL AUDIT" \
  --position header \
  --output tests/e2e_fixtures/out/header_stamped.pdf \
  --json
```

#### 2. Add Footer Banner
```bash
paperpilot header-footer \
  --input tests/e2e_fixtures/single_page.pdf \
  --text "Approved for Release • PaperPilot v0.7.0" \
  --position footer \
  --output tests/e2e_fixtures/out/footer_stamped.pdf \
  --json
```

#### CLI Parameters
- `--input` *(string, required)*: Path to the input PDF file.
- `--output` *(string, required)*: Path to save the resulting PDF document.
- `--text` *(string, required)*: The header or footer text string to stamp.
- `--position` *(string, optional, default: `"header"`)*: Target placement (`"header"` or `"footer"`).
- `--json` *(flag, optional)*: Emit structured JSON execution result.

---

### MCP JSON-RPC
When integrating via Model Context Protocol (MCP), call the tool `pdf_header_footer`:

#### 1. Stamp Header via MCP
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_header_footer",
    "arguments": {
      "input": "tests/e2e_fixtures/single_page.pdf",
      "output": "tests/e2e_fixtures/out/header_stamped.pdf",
      "text": "CONFIDENTIAL - INTERNAL AUDIT",
      "position": "top"
    }
  },
  "id": 1
}
```

#### 2. Dual Header & Footer Support via MCP
The MCP implementation also supports specifying simultaneous header and footer targets:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_header_footer",
    "arguments": {
      "input": "tests/e2e_fixtures/single_page.pdf",
      "output": "tests/e2e_fixtures/out/dual_stamped.pdf",
      "header_left": "CONFIDENTIAL",
      "footer_center": "Page Approved"
    }
  },
  "id": 2
}
```

#### MCP Schema Parameters
- `input` *(string, required)*: Input PDF path.
- `output` *(string, required)*: Destination PDF path.
- `text` *(string, required for standard invocation)*: Content to add.
- `position` *(string, required for standard invocation)*: Positioning (`"top"`, `"bottom"`, `"header"`, or `"footer"`).
- `header_left` / `header` *(string, optional)*: Left header text string.
- `footer_center` / `footer` *(string, optional)*: Center footer text string.

---

### REST API
Execute header and footer stamping via PaperPilot Gateway:

#### 1. Dedicated Route (`/api/v1/pdf/pdf_header_footer`)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/pdf_header_footer \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/single_page.pdf",
    "output": "tests/e2e_fixtures/out/header_stamped.pdf",
    "text": "CONFIDENTIAL - INTERNAL AUDIT",
    "position": "header"
  }'
```

#### 2. Tool Route (`/api/v1/pdf/tools/pdf_header_footer`)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_header_footer \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/single_page.pdf",
    "output": "tests/e2e_fixtures/out/header_stamped.pdf",
    "text": "CONFIDENTIAL - INTERNAL AUDIT",
    "position": "header"
  }'
```

#### Response Example
```json
{
  "success": true,
  "request_id": "73fb17eb-3446-4251-9f6f-b9d5e2ab3b79",
  "message": "Header/footer added successfully.",
  "output": {
    "isError": false,
    "resultType": "complete",
    "content": [
      {
        "type": "text",
        "text": "Header/footer added successfully."
      }
    ]
  }
}
```

---

### WebAssembly (WASM)
In browser or worker contexts using `@paperpilot/wasm`:

```javascript
import { WasmPdfEngine } from 'paperpilot-wasm';

// Stamps header and footer strings across all pages in the PDF
const stampedPdfBytes = WasmPdfEngine.header_footer(
  pdfBytes,
  "CONFIDENTIAL HEADER", // header text
  "INTERNAL USE ONLY"    // footer text
);
```

---

### Known Limitations & Troubleshooting
- **Sequential Header and Footer via CLI:** The CLI command applies either a header or a footer in a single pass depending on `--position`. To apply both a header and footer via CLI, chain two commands sequentially or pass both via the MCP / REST API (`header_left` and `footer_center`).
- **Dynamic Page Numbers vs Static Headers:** For automated dynamic page counter numbering (e.g., `Page {n} of {total}`), use `paperpilot page-numbers` (`pdf_page_numbers`) or `pdf_bates` instead of static headers.
- **Proxy Environments:** In environments with corporate proxies configured (`HTTP_PROXY`/`HTTPS_PROXY`), pass `--noproxy '*'` or `NO_PROXY=127.0.0.1` to avoid proxy interception of loopback requests.
