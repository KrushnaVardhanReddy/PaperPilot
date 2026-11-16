---
title: "pdf_reorder_pages"
description: "Reference, CLI, MCP, and REST API commands for pdf_reorder_pages."
sidebar:
  order: 40
---

**Status**: ✅ Verified  |  **Format**: PDF (`.pdf`)

### What it does
**Rearranges the sequence of pages in a PDF according to a custom specified order.**  
Restructures the document page tree hierarchy to match the requested permutation (e.g. `5,4,3,2,1` or `2,1,3,4,5`), enabling scan correction, page inversion, and presentation slide reorganizing.

---

### CLI
Reorder document pages via the PaperPilot CLI using either `reorder-pages` or the `reorder` alias:

```bash
paperpilot reorder-pages \
  --input tests/e2e_fixtures/real/multi_page.pdf \
  --order 5,4,3,2,1 \
  --output tests/e2e_fixtures/out/penta_e2e/reordered.pdf \
  --json
```

Or using the shorthand subcommand alias:
```bash
paperpilot reorder \
  --input tests/e2e_fixtures/real/multi_page.pdf \
  --order 2,1,3,4,5 \
  --output tests/e2e_fixtures/out/penta_e2e/reordered.pdf \
  --json
```

#### CLI Parameters
- `--input` *(string, required)*: Path to the source PDF document.
- `--order` *(string, required)*: Comma-separated 1-based page order. In strict CLI mode, the sequence must list every page in the document (length matching total page count).
- `--output` *(string, required)*: Destination file path for the reordered output PDF.
- `--json` *(flag, optional)*: Emit structured JSON execution result.

---

### MCP JSON-RPC
When integrating via Model Context Protocol (MCP), invoke the tool `pdf_reorder_pages`:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_reorder_pages",
    "arguments": {
      "input": "tests/e2e_fixtures/real/multi_page.pdf",
      "order": "2,1,3,4,5",
      "output": "tests/e2e_fixtures/out/penta_e2e/reordered.pdf"
    }
  },
  "id": 1
}
```

#### MCP Schema Parameters
- `input` *(string, required)*: Path to the input PDF file.
- `order` *(string, required)*: The new 1-based page order, e.g., `'3,2,1'` (also accepts alias `page_order`).
- `output` *(string, required)*: File path for the reordered output PDF.

---

### REST API
Execute reordering operations via PaperPilot Gateway:

#### 1. Dedicated Route (`/api/v1/pdf/pdf_reorder_pages`)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/pdf_reorder_pages \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/multi_page.pdf",
    "order": "2,1,3,4,5",
    "output": "tests/e2e_fixtures/out/penta_e2e/reordered.pdf"
  }'
```

#### 2. Universal Tool Route (`/api/v1/pdf/tools/pdf_reorder_pages`)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_reorder_pages \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/multi_page.pdf",
    "order": "2,1,3,4,5",
    "output": "tests/e2e_fixtures/out/penta_e2e/reordered.pdf"
  }'
```

#### Response Example
```json
{
  "success": true,
  "request_id": "585bc3b6-cf8f-474b-9b5e-0b51ae0b5cff",
  "message": "Success",
  "output": {
    "isError": false,
    "resultType": "complete",
    "content": [
      {
        "type": "text",
        "text": "{\"success\":true,\"message\":\"Pages reordered successfully.\",\"output_path\":\"tests/e2e_fixtures/out/penta_e2e/reordered.pdf\",\"data\":null}"
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

// Reorders pages according to an array of 1-based page numbers
const reorderedPdfBytes = WasmPdfEngine.reorder_pages(pdfBytes, new Uint32Array([2, 1, 3, 4, 5]));
```

---

### Known Limitations & Troubleshooting
- **Strict Page Count Permutation:** In the underlying engine and CLI, `order` must be a complete permutation matching the exact page count of the document. For instance, on a 5-page document, passing `--order 3,2,1` will fail with `Parse error: New order length (3) must match document page count (5)`. Ensure all pages (e.g. `3,2,1,4,5`) are explicitly positioned.
- **Gateway Auto-Completion:** In PaperPilot Gateway (`/api/v1/pdf/pdf_reorder_pages`), if a partial page order is provided (e.g. `2,1`), the gateway handler automatically appends the remaining unmentioned pages in their original sequential order before dispatching to the engine.
- **Duplicate Page Numbers:** Page indices in the reorder list must be unique and within bounds (`1 <= page <= total_pages`). Duplicating a page in the permutation is not supported.
- **Proxy Environments:** In environments where corporate HTTP proxies are enabled, use `--noproxy '*'` or configure `NO_PROXY=127.0.0.1` when executing curl requests against the gateway.
