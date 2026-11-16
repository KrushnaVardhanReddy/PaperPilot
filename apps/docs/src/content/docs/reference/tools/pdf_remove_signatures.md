---
title: "pdf_remove_signatures"
description: "Reference, CLI, MCP, and REST API commands for pdf_remove_signatures."
sidebar:
  order: 54
---

**Status**: ✅ Verified  |  **Format**: PDF (`.pdf`)

### What it does
**Strips digital signatures, signature field appearances, cryptographic lock dictionaries, and signature widget annotations from a PDF.**  
Removes digital signature cryptographic objects while preserving document tree structure, remaining interactive form fields, page content, and visual fidelity.

---

### CLI
Remove digital signatures using the `pdf-remove-signatures` CLI subcommand:

```bash
paperpilot pdf-remove-signatures \
  --input tests/e2e_fixtures/signed.pdf \
  --output tests/e2e_fixtures/out/batch14_test/cli_remove_signatures.pdf \
  --json
```

#### CLI Parameters
- `--input` *(string, required)*: Path to the digitally signed PDF file.
- `--output` *(string, required)*: Destination file path for the signature-stripped PDF.
- `--json` *(flag, optional)*: Emit structured JSON execution result.

---

### MCP JSON-RPC
When integrating via Model Context Protocol (MCP), invoke the tool `pdf_remove_signatures`:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_remove_signatures",
    "arguments": {
      "input": "tests/e2e_fixtures/signed.pdf",
      "output": "tests/e2e_fixtures/out/batch14_test/mcp_remove_signatures.pdf"
    }
  },
  "id": 1
}
```

#### MCP Schema Parameters
- `input` *(string, required)*: Path to the input signed PDF document.
- `output` *(string, required)*: Output file path for the unsigned PDF document.

---

### REST API
Strip digital signatures using PaperPilot Gateway:

#### 1. Dedicated Route (`/api/v1/pdf/remove-signatures` or `/api/v1/pdf/pdf_remove_signatures`)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/remove-signatures \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/signed.pdf",
    "output": "tests/e2e_fixtures/out/batch14_test/rest_remove_signatures.pdf"
  }'
```

#### Response Example
```json
{
  "success": true,
  "request_id": "bed134b0-bbc3-4363-ab70-ad70aa9dac82",
  "message": "Success",
  "output": {
    "isError": false,
    "resultType": "complete",
    "content": [
      {
        "type": "text",
        "text": "{\"success\":true,\"message\":\"Remove Digital Signatures executed successfully\",\"output_path\":\"tests/e2e_fixtures/out/batch14_test/rest_remove_signatures.pdf\",\"data\":{\"output_path\":\"tests/e2e_fixtures/out/batch14_test/rest_remove_signatures.pdf\"}}"
      }
    ]
  }
}
```

---

### WebAssembly (WASM)
In browser or worker contexts using `paperpilot-wasm`:

```javascript
import { WasmPdfEngine } from 'paperpilot-wasm';

// Options JSON (empty object for defaults)
const optionsJson = JSON.stringify({});
const strippedPdfBytes = WasmPdfEngine.pdf_remove_signatures(signedPdfBytes, optionsJson);
```

---

### Known Limitations & Troubleshooting
- **Cryptographic Invalidation:** Stripping signature dictionary objects intentionally removes cryptographic validity and tamper-evident seals. The resulting document is unsigned and open for editing.
- **Unsigned Documents:** If executed on a document that contains no signature dictionaries, the operation succeeds safely and produces a clean output copy.
