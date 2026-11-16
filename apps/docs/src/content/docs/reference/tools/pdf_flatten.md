---
title: "pdf_flatten"
description: "Reference, CLI, MCP, and REST API commands for pdf_flatten."
sidebar:
  order: 26
---

**Status**: ✅ Verified  |  **Format**: PDF (`.pdf`)

### What it does
**Flattens interactive AcroForm form fields and annotations directly into the static page content stream.**  
Paints AcroForm widget appearances into page content streams and strips interactive form fields, preventing further form modification or data tampering. Preserves existing page resources and unrelated graphical elements while rendering form values permanently into the document geometry.

---

### CLI
Flatten interactive elements via PaperPilot CLI:

#### 1. Flatten Form & Annotations
```bash
paperpilot flatten \
  --input tests/e2e_fixtures/form.pdf \
  --output tests/e2e_fixtures/out/flattened_document.pdf \
  --json
```

#### CLI Parameters
- `--input` *(string, required)*: Path to the input PDF containing interactive forms or annotations.
- `--output` *(string, required)*: Path to write the flattened output PDF.
- `--json` *(flag, optional)*: Emit structured JSON execution result.

---

### MCP JSON-RPC
When integrating via Model Context Protocol (MCP), call the tool `pdf_flatten`:

#### Flatten Form Fields via MCP
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_flatten",
    "arguments": {
      "input": "tests/e2e_fixtures/form.pdf",
      "output": "tests/e2e_fixtures/out/flattened_document.pdf"
    }
  },
  "id": 1
}
```

#### MCP Schema Parameters
- `input` *(string, required)*: Path to the PDF to flatten.
- `output` *(string, required)*: Path to save the flattened PDF.

#### Response Example
```json
{
  "success": true,
  "message": "PDF flattened successfully.",
  "output_path": "tests/e2e_fixtures/out/flattened_document.pdf"
}
```

---

### REST API
Execute flattening operations via PaperPilot Gateway:

#### 1. Dedicated Route (`/api/v1/pdf/pdf_flatten`)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/pdf_flatten \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/form.pdf",
    "output": "tests/e2e_fixtures/out/flattened_document.pdf"
  }'
```

#### 2. Tool Route (`/api/v1/pdf/tools/pdf_flatten`)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_flatten \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/form.pdf",
    "output": "tests/e2e_fixtures/out/flattened_document.pdf"
  }'
```

#### Response Example
```json
{
  "success": true,
  "request_id": "73fb17eb-3446-4251-9f6f-b9d5e2ab3b79",
  "message": "PDF flattened successfully.",
  "output": {
    "isError": false,
    "resultType": "complete",
    "content": [
      {
        "type": "text",
        "text": "PDF flattened successfully."
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

// Returns a Uint8Array containing the flattened PDF with interactive fields baked in
const flattenedBytes = WasmPdfEngine.flatten(pdfBytes);
```

---

### Known Limitations & Troubleshooting
- **Cryptographic Signatures:** Flattening form fields and annotations modifies the underlying PDF content stream. If the original document contains digital signatures, flattening invalidates any existing cryptographic signature locks. To preserve digital signatures, do not flatten signed documents, or re-sign after flattening.
- **Hidden Form Widgets:** Hidden widgets with empty appearances are removed cleanly without rendering residual bounding boxes or empty field outlines into the background.
- **Proxy Environments:** In environments with corporate proxies configured (`HTTP_PROXY`/`HTTPS_PROXY`), pass `--noproxy '*'` or `NO_PROXY=127.0.0.1` to avoid proxy interception of loopback requests.
