---
title: "pdf_extract_text"
description: "Reference, CLI, MCP, REST API, and WASM commands for pdf_extract_text."
sidebar:
  order: 24
---

**Status**: ✅ Verified  |  **Format**: PDF (`.pdf`) $\to$ Plain Text (`.txt`)

### What it does
**Extracts all readable textual content from a PDF document with layout and reading-order awareness.**  
Decodes ToUnicode font mappings and content streams in visual reading order, dumping pure text without formatting or headers into a UTF-8 `.txt` file or structured JSON response.

---

### CLI
Extract plain text via the PaperPilot CLI using `extract-text`:

```bash
paperpilot extract-text \
  --input tests/e2e_fixtures/real/single_page.pdf \
  --output tests/e2e_fixtures/out/penta_e2e/extracted_text.txt \
  --json
```

#### CLI Parameters
- `--input <INPUT>` *(string, required)*: Path to the input PDF document.
- `--output <OUTPUT>` *(string, required)*: Path to the destination `.txt` file.
- `--format <FORMAT>` *(string, optional, default: `text`)*: Output format (`text`).
- `--json` *(flag, optional)*: Emit structured JSON execution result.

---

### MCP JSON-RPC
When integrating via Model Context Protocol (MCP), invoke the tool `pdf_extract_text`:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_extract_text",
    "arguments": {
      "input": "tests/e2e_fixtures/real/single_page.pdf",
      "output": "tests/e2e_fixtures/out/penta_e2e/extracted_text.txt",
      "pages": "all"
    }
  },
  "id": 1
}
```

#### MCP Schema Parameters
- `input` *(string, required)*: The input PDF file path.
- `output` *(string, required)*: The output text file path.
- `pages` *(string, optional)*: Pages to extract text from, e.g., `'1,3,5-7'` or `'all'`.

---

### REST API
Execute text extraction via PaperPilot Gateway:

#### 1. Direct Route (`/api/v1/pdf/extract-text`)
```bash
curl -s --noproxy '*' -X POST http://127.0.0.1:7823/api/v1/pdf/extract-text \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf"
  }'
```

#### 2. Canonical Tool Route (`/api/v1/pdf/pdf_extract_text`)
```bash
curl -s --noproxy '*' -X POST http://127.0.0.1:7823/api/v1/pdf/pdf_extract_text \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/extracted_text.txt"
  }'
```

#### Response Example
```json
{
  "success": true,
  "request_id": "9c235a99-196b-4d89-81ba-b4a6a3adf036",
  "message": "Success",
  "output": {
    "isError": false,
    "resultType": "complete",
    "content": [
      {
        "type": "text",
        "text": "{\"success\":true,\"message\":\"Extracted text successfully.\",\"output_path\":null,\"data\":{\"text\":\"# PaperPilot Enterprise Solutions\\n\\nGlobal Document Technology Division\\n...\"}}"
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

// Returns extracted text as a UTF-8 string
const extractedText = WasmPdfEngine.extract_text(pdfBytes);
console.log(extractedText);
```

---

### Known Limitations & Troubleshooting
- **CLI Requires `--output`**: Unlike the REST API where `output` is optional (returning text directly in the payload), the CLI parser mandates `--output <OUTPUT>`.
- **Image-Only & Scanned Documents**: Documents lacking a digital text layer (e.g., pure scans or rasterized artwork) return empty strings. For scanned files, run [`pdf_ocr`](/reference/tools/pdf_ocr) to generate a searchable text layer.
- **Proxy Environments**: When invoking the REST API in environments with HTTP proxies configured, add `--noproxy '*'` or configure `NO_PROXY=127.0.0.1` to prevent `502 Bad Gateway` or connection rejection errors.
