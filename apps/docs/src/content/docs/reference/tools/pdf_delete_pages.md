---
title: "pdf_delete_pages"
description: "Reference, CLI, MCP, and REST API commands for pdf_delete_pages."
sidebar:
  order: 20
---

**Status**: ✅ Verified  |  **Format**: PDF (`.pdf`)

### What it does
**Deletes specific pages or page ranges from a PDF document.**  
Prunes unwanted pages (such as blank sheets, cover letters, or draft appendices) from the PDF page tree, updates object references, and cleanly rebuilds the document catalog without corrupting internal cross-reference tables.

---

### CLI
Delete pages via the PaperPilot CLI using either `delete-pages` or the `delete` alias:

```bash
paperpilot delete-pages \
  --input tests/e2e_fixtures/real/multi_page.pdf \
  --pages 2,4 \
  --output tests/e2e_fixtures/out/penta_e2e/deleted.pdf \
  --json
```

Or using the shorthand subcommand alias:
```bash
paperpilot delete \
  --input tests/e2e_fixtures/real/multi_page.pdf \
  --pages 2 \
  --output tests/e2e_fixtures/out/penta_e2e/deleted.pdf \
  --json
```

#### CLI Parameters
- `--input` *(string, required)*: Path to the source PDF document.
- `--pages` *(string, required)*: 1-based comma-separated page numbers or ranges to remove (e.g. `2,4` or `2-3`).
- `--output` *(string, required)*: Destination file path for the modified PDF.
- `--json` *(flag, optional)*: Emit structured JSON execution result.

---

### MCP JSON-RPC
When integrating via Model Context Protocol (MCP), invoke the tool `pdf_delete_pages`:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_delete_pages",
    "arguments": {
      "input": "tests/e2e_fixtures/real/multi_page.pdf",
      "pages": "2,4",
      "output": "tests/e2e_fixtures/out/penta_e2e/deleted.pdf"
    }
  },
  "id": 1
}
```

#### MCP Schema Parameters
- `input` *(string, required)*: Path to the input PDF file.
- `pages` *(string, required)*: 1-based page numbers or ranges to delete (e.g., `'1,3,5-7'`).
- `output` *(string, required)*: File path for the modified output PDF.

---

### REST API
Execute page deletion operations via PaperPilot Gateway:

#### 1. Dedicated Route (`/api/v1/pdf/pdf_delete_pages`)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/pdf_delete_pages \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/multi_page.pdf",
    "pages": "2,4",
    "output": "tests/e2e_fixtures/out/penta_e2e/deleted.pdf"
  }'
```

#### 2. Universal Tool Route (`/api/v1/pdf/tools/pdf_delete_pages`)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_delete_pages \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/multi_page.pdf",
    "pages": "2,4",
    "output": "tests/e2e_fixtures/out/penta_e2e/deleted.pdf"
  }'
```

#### Response Example
```json
{
  "success": true,
  "request_id": "8f9d79fa-c338-4c2d-9d94-23b2654c66db",
  "message": "Success",
  "output": {
    "isError": false,
    "resultType": "complete",
    "content": [
      {
        "type": "text",
        "text": "{\"success\":true,\"message\":\"Pages deleted successfully.\",\"output_path\":\"tests/e2e_fixtures/out/penta_e2e/deleted.pdf\",\"data\":null}"
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

// Returns a Uint8Array with the specified pages removed
const modifiedPdfBytes = WasmPdfEngine.delete_pages(pdfBytes, "2,4");
```

---

### Known Limitations & Troubleshooting
- **Cannot Delete All Pages:** Deleting all pages in a document results in an invalid empty PDF. At least one page must remain in the output document.
- **1-Based Indexing:** Page numbers start at index 1. Requesting deletion of page 0 or invalid non-numeric values will fail.
- **REST Gateway Endpoint Naming:** Note that the dedicated REST endpoint is named `/api/v1/pdf/pdf_delete_pages` (or accessible through `/api/v1/pdf/tools/pdf_delete_pages`).
- **Proxy Environments:** In environments where corporate HTTP proxies are enabled, use `--noproxy '*'` or configure `NO_PROXY=127.0.0.1` when executing curl requests against the gateway.
