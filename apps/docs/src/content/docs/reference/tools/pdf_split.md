---
title: "pdf_split"
description: "Reference, CLI, MCP, and REST API commands for pdf_split."
sidebar:
  order: 45
---

**Status**: ✅ Verified  |  **Format**: PDF (`.pdf`)

### What it does
**Splits a PDF into smaller documents based on page numbers, ranges, cut points, or periodic intervals.**  
Divides a multi-page PDF into multiple discrete documents (e.g., splitting at page 3 partitions pages 1–3 into one document and pages 4–5 into another). Supports cut points, page intervals (`--every`), and full single-page bursts.

---

### CLI
Split documents via the PaperPilot CLI:

#### 1. Split by Cut Points or Page Ranges
```bash
paperpilot split \
  --input tests/e2e_fixtures/real/multi_page.pdf \
  --pages 1,3 \
  --output tests/e2e_fixtures/out/penta_e2e/pdf_split_simple \
  --json
```

#### 2. Split Every N Pages
```bash
paperpilot split \
  --input tests/e2e_fixtures/real/multi_page.pdf \
  --every 2 \
  --output tests/e2e_fixtures/out/penta_e2e/pdf_split_every \
  --json
```

#### CLI Parameters
- `--input` *(string, required)*: Path to the input PDF document.
- `--output` *(string, required)*: Directory destination path where generated split files (`split_1.pdf`, `split_2.pdf`, etc.) will be written.
- `--pages` *(string, optional, default: `""`)*: Comma-separated split boundary points (e.g. `1,3` or `1-2, 3-5`). If empty and `--every` is not specified, defaults to splitting into single pages.
- `--every` *(integer, optional)*: Split the document into chunks of N pages each.
- `--json` *(flag, optional)*: Emit structured JSON execution result.

---

### MCP JSON-RPC
When integrating via Model Context Protocol (MCP), invoke the tool `pdf_split`:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_split",
    "arguments": {
      "input": "tests/e2e_fixtures/real/multi_page.pdf",
      "output_dir": "tests/e2e_fixtures/out/penta_e2e/pdf_split_simple",
      "pages": "1,3"
    }
  },
  "id": 1
}
```

#### Split Periodically via MCP
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_split",
    "arguments": {
      "input": "tests/e2e_fixtures/real/multi_page.pdf",
      "output_dir": "tests/e2e_fixtures/out/penta_e2e/pdf_split_every",
      "every": 2
    }
  },
  "id": 2
}
```

#### MCP Schema Parameters
- `input` *(string, required)*: Source PDF file path.
- `output_dir` *(string, required)*: Directory where split pages will be saved (also accepts `output` as an alias).
- `pages` *(string, optional)*: Comma-separated split boundary points or ranges (also accepts aliases `ranges`, `split_at`, or `split_points`).
- `every` *(integer, optional)*: Split intervals into uniform chunk sizes.

---

### REST API
Execute splitting operations via PaperPilot Gateway:

#### 1. Dedicated Split Route
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/split \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/multi_page.pdf",
    "output_dir": "tests/e2e_fixtures/out/penta_e2e/pdf_split_simple",
    "pages": "1,3"
  }'
```

#### 2. Exact Tool Route (`pdf_split`)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/pdf_split \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/multi_page.pdf",
    "output_dir": "tests/e2e_fixtures/out/penta_e2e/pdf_split_simple",
    "pages": "1,3"
  }'
```

#### Response Example
```json
{
  "success": true,
  "request_id": "73fb17eb-3446-4251-9f6f-b9d5e2ab3b79",
  "message": "Success",
  "output": {
    "isError": false,
    "resultType": "complete",
    "content": [
      {
        "type": "text",
        "text": "{\"success\":true,\"message\":\"PDF split successfully.\",\"output_path\":\"tests/e2e_fixtures/out/penta_e2e/pdf_split_simple\",\"data\":{\"files\":[\"tests/e2e_fixtures/out/penta_e2e/pdf_split_simple/split_1.pdf\",\"tests/e2e_fixtures/out/penta_e2e/pdf_split_simple/split_2.pdf\",\"tests/e2e_fixtures/out/penta_e2e/pdf_split_simple/split_3.pdf\"]}}"
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

// Returns an Array of Uint8Array buffers (one per split chunk)
const splitBuffers = WasmPdfEngine.split(pdfBytes, "1,3");
console.log(`Generated ${splitBuffers.length} PDF documents.`);
```

---

### Known Limitations & Troubleshooting
- **Output Directory Requirement:** In CLI and Gateway REST modes, the target output path represents a directory. If the directory does not exist, the CLI and Gateway will create it automatically, writing numbered files following the pattern `split_1.pdf`, `split_2.pdf`, etc.
- **Split Points vs Page Ranges:** Cut points represent 1-based end indices for each segment. For example, on a 5-page document, passing `--pages 1,3` partitions the file into pages 1, pages 2–3, and pages 4–5.
- **Proxy Environments:** When invoking the REST API locally in environments with corporate proxies configured (`HTTP_PROXY`/`HTTPS_PROXY`), pass `--noproxy '*'` or `NO_PROXY=127.0.0.1` to avoid proxy interception of loopback requests.
