---
title: "pdf_timestamp"
description: "Reference, CLI, MCP, REST API, and WASM specifications for pdf_timestamp."
sidebar:
  order: 50
---

**Status**: ✅ Verified  |  **Format**: PDF (`.pdf`)

### What it does
**Applies RFC 3161 cryptographically verified TSA timestamp tokens.**  
Affixes immutable timestamp tokens from trusted Time Stamping Authorities (TSA) or offline verifiable timestamp tokens directly into the PDF document trailer and security structures to establish non-repudiation of document state.

---

### CLI
Apply cryptographic timestamp tokens from the command line:

#### 1. Standard TSA Timestamping
```bash
paperpilot pdf-timestamp \
  --input tests/e2e_fixtures/single_page.pdf \
  --output tests/e2e_fixtures/out/batch18/timestamped.pdf \
  --json
```

#### 2. Specifying a Remote TSA Server URL
```bash
paperpilot pdf-timestamp \
  --input legal_document.pdf \
  --output legal_document_stamped.pdf \
  --server-url "http://timestamp.digicert.com"
```

#### CLI Parameters
- `--input` *(string, required)*: Path to source PDF document.
- `--output` *(string, required)*: Path to write the timestamp-affixed PDF.
- `--server-url` *(string, optional)*: URL of the RFC 3161 compliant Time Stamping Authority server.
- `--json` *(flag, optional)*: Emit structured JSON status report (`{"success":true,"operation":"PdfTimestamp","error":null}`).
- `--log-format` *(string, optional)*: Output format for logs (`text`, `json`, `compact`, `pretty`).
- `--log-level` *(string, optional)*: Log level verbosity (`trace`, `debug`, `info`, `warn`, `error`).
- `--webhook` *(string, optional)*: Webhook URL to deliver completion results.

---

### MCP JSON-RPC
Invoke `pdf_timestamp` through Model Context Protocol (MCP) clients:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_timestamp",
    "arguments": {
      "input": "tests/e2e_fixtures/single_page.pdf",
      "output": "tests/e2e_fixtures/out/batch18/timestamped_mcp.pdf"
    }
  },
  "id": 1
}
```

#### MCP Output Response
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "content": [
      {
        "type": "text",
        "text": "{\"success\":true,\"message\":\"Cryptographic Timestamp TSA executed successfully\",\"output_path\":\"tests/e2e_fixtures/out/batch18/timestamped_mcp.pdf\",\"data\":{\"output_path\":\"tests/e2e_fixtures/out/batch18/timestamped_mcp.pdf\"}}"
      }
    ],
    "isError": false
  }
}
```

#### MCP Schema Parameters
- `input` *(string, required)*: Path to the input PDF file.
- `output` *(string, required)*: Path where the stamped PDF will be saved.

---

### REST API
Affix timestamps via PaperPilot Gateway endpoints:

#### Dedicated Endpoint (`/api/v1/pdf/timestamp` or `/api/v1/pdf/pdf_timestamp`)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/timestamp \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/single_page.pdf",
    "output": "tests/e2e_fixtures/out/batch18/timestamped_api.pdf"
  }'
```

#### Universal Tool Route
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_timestamp \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/single_page.pdf",
    "output": "tests/e2e_fixtures/out/batch18/timestamped_tool.pdf"
  }'
```

---

### WASM (Web & Edge)
Apply timestamp tokens in pure client-side or browser runtimes using `paperpilot-wasm`:

```typescript
import { WasmPdfEngine } from "paperpilot-wasm";

// Options parameter (serializable options object)
const options = JSON.stringify({});

const timestampedBytes: Uint8Array = WasmPdfEngine.pdf_timestamp(
  inputBytes,
  options
);
```

---

### Known Limitations & Troubleshooting
- **Offline Fallback**: In environments without active Internet connectivity or when no `--server-url` is supplied, PaperPilot applies an internal RFC 3161 timestamp trailer token to ensure workflow continuity and offline determinism.
- **Incremental Updates**: Timestamps are affixed cleanly to the document trailer without invalidating prior digital signatures or altering visual content streams.
