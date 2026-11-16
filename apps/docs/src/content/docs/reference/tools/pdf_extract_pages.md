---
title: "pdf_extract_pages"
description: "Reference, CLI, MCP, and REST API commands for pdf_extract_pages."
sidebar:
  order: 23
---

**Status**: ✅ Verified  |  **Format**: PDF (`.pdf`)

### What it does
**Extracts specific pages or page ranges into a new standalone PDF document.**  
Pulls the selected pages (e.g. `1,3,5-7`) from a multi-page PDF into a fresh file while preserving original formatting, annotations, and media without altering the source file.

---

### CLI
Extract pages via the PaperPilot CLI using either `extract-pages` or the `extract` alias:

```bash
paperpilot extract-pages \
  --input tests/e2e_fixtures/real/multi_page.pdf \
  --pages 1,3 \
  --output tests/e2e_fixtures/out/penta_e2e/extracted.pdf \
  --json
```

Or using the shorthand subcommand alias:
```bash
paperpilot extract \
  --input tests/e2e_fixtures/real/multi_page.pdf \
  --pages 1-3 \
  --output tests/e2e_fixtures/out/penta_e2e/extracted.pdf \
  --json
```

#### CLI Parameters
- `--input` *(string, required)*: Path to the source PDF document.
- `--pages` *(string, required)*: 1-based comma-separated page indices and/or hyphenated ranges (e.g. `1,3`, `1-3,5`).
- `--output` *(string, required)*: Destination file path for the extracted output PDF.
- `--json` *(flag, optional)*: Emit structured JSON execution result.

---

### MCP JSON-RPC
When integrating via Model Context Protocol (MCP), invoke the tool `pdf_extract_pages`:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_extract_pages",
    "arguments": {
      "input": "tests/e2e_fixtures/real/multi_page.pdf",
      "pages": "1,3",
      "output": "tests/e2e_fixtures/out/penta_e2e/extracted.pdf"
    }
  },
  "id": 1
}
```

#### MCP Schema Parameters
- `input` *(string, required)*: Path to the input PDF file.
- `pages` *(string, required)*: 1-based page numbers or ranges to extract (e.g., `'1,3,5-7'`).
- `output` *(string, required)*: File path for the extracted output PDF.

---

### REST API
Execute extraction operations via PaperPilot Gateway:

#### 1. Dedicated Route (`/api/v1/pdf/pdf_extract_pages`)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/pdf_extract_pages \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/multi_page.pdf",
    "pages": "1,3",
    "output": "tests/e2e_fixtures/out/penta_e2e/extracted.pdf"
  }'
```

#### 2. Universal Tool Route (`/api/v1/pdf/tools/pdf_extract_pages`)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_extract_pages \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/multi_page.pdf",
    "pages": "1,3",
    "output": "tests/e2e_fixtures/out/penta_e2e/extracted.pdf"
  }'
```

#### Response Example
```json
{
  "success": true,
  "request_id": "01c4d98c-6dd0-4029-9cec-1ffa225ca417",
  "message": "Success",
  "output": {
    "isError": false,
    "resultType": "complete",
    "content": [
      {
        "type": "text",
        "text": "{\"success\":true,\"message\":\"Pages extracted successfully.\",\"output_path\":\"tests/e2e_fixtures/out/penta_e2e/extracted.pdf\",\"data\":null}"
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

// Returns a Uint8Array containing only the requested pages
const extractedPdfBytes = WasmPdfEngine.extract_pages(pdfBytes, "1,3,5-7");
```

---

### Known Limitations & Troubleshooting
- **1-Based Indexing:** Page numbers are 1-based (page 1 is the first page of the document). Specifying `0` will trigger an invalid page index error.
- **Out-of-Bounds Pages:** Requesting page numbers greater than the total page count will fail validation.
- **REST Gateway Endpoint Naming:** Note that the dedicated REST endpoint is named `/api/v1/pdf/pdf_extract_pages` (the tool name with prefix) or accessible through `/api/v1/pdf/tools/pdf_extract_pages`.
- **Proxy Environments:** In environments where corporate HTTP proxies are enabled, use `--noproxy '*'` or configure `NO_PROXY=127.0.0.1` when executing curl requests against the gateway.
