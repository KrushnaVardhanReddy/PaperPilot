---
title: "pdf_hash"
description: "Reference, CLI, MCP, and REST API commands for pdf_hash."
sidebar:
  order: 27
---

**Status**: ✅ Verified  |  **Format**: PDF (`.pdf`) $\to$ Cryptographic Hash Digest / Report

### What it does
**Calculates cryptographic checksums and document integrity hashes of a PDF file.**  
Generates deterministic digital fingerprints (SHA-256) and reports exact byte sizes to verify document integrity, detect tampering across transmission pipelines, and establish a verifiable chain of custody.

- **Tamper Detection**: Instantly exposes bit-level alterations, modifications, or stream corruptions.
- **Audit Reports**: Emits structured JSON or plain text digest files reporting algorithm, filepath, SHA-256 hash, and byte length.
- **Universal Cross-Platform**: Supported natively across CLI, MCP, REST Gateway, and purely offline in WebAssembly (`paperpilot-wasm`).

---

### CLI

`pdf_hash` is invoked via `paperpilot hash`:

```bash
# Compute hash and print SHA-256 with JSON metadata
paperpilot hash \
  --input tests/e2e_fixtures/real/form.pdf \
  --json
```

```bash
# Write formatted checksum report to output file
paperpilot hash \
  --input tests/e2e_fixtures/real/form.pdf \
  --output tests/e2e_fixtures/out/batch16_test/hash_out.txt \
  --json
```

#### CLI Parameters
- `--input` *(string, required)*: Path to input PDF file.
- `--output` *(string, optional)*: Destination path to write formatted text checksum report.
- `--json` *(flag, optional)*: Output structured JSON execution status.

#### Example CLI Output
```
25316f962c77870acaeb5b3317b2e467fa32ebe29e5574bf8c872df542a86cf1
{"success":true,"operation":"Hash","error":null}
```

#### Example Formatted Output File (`hash_out.txt`)
```
Algorithm: SHA-256
File: tests/e2e_fixtures/real/form.pdf
Hash: 25316f962c77870acaeb5b3317b2e467fa32ebe29e5574bf8c872df542a86cf1
Size: 23420 bytes
```

---

### MCP JSON-RPC

Invoke `pdf_hash` via Model Context Protocol:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_hash",
    "arguments": {
      "input": "tests/e2e_fixtures/real/form.pdf"
    }
  },
  "id": 1
}
```

#### MCP Schema Parameters
- `input` *(string, required)*: Path to the input PDF file.
- `output` *(string, optional)*: Optional path to save text report to disk.

#### Example MCP Response
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "content": [
      {
        "type": "text",
        "text": "{\"diff_detected\":null,\"data\":{\"hash\":\"25316f962c77870acaeb5b3317b2e467fa32ebe29e5574bf8c872df542a86cf1\",\"size\":23420},\"message\":\"SHA-256: 25316f962c77870acaeb5b3317b2e467fa32ebe29e5574bf8c872df542a86cf1\",\"output_path\":\"25316f962c77870acaeb5b3317b2e467fa32ebe29e5574bf8c872df542a86cf1\",\"success\":true}"
      }
    ],
    "isError": false
  }
}
```

---

### REST API

Calculate PDF hashes via PaperPilot Gateway:

```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/pdf_hash \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/form.pdf"
  }'
```

#### Example Gateway Response
```json
{
  "message": "Success",
  "output": {
    "content": [
      {
        "text": "{\"data\":{\"hash\":\"25316f962c77870acaeb5b3317b2e467fa32ebe29e5574bf8c872df542a86cf1\",\"size\":23420},\"diff_detected\":null,\"message\":\"SHA-256: 25316f962c77870acaeb5b3317b2e467fa32ebe29e5574bf8c872df542a86cf1\",\"output_path\":\"25316f962c77870acaeb5b3317b2e467fa32ebe29e5574bf8c872df542a86cf1\",\"success\":true}",
        "type": "text"
      }
    ],
    "isError": false,
    "resultType": "complete"
  },
  "request_id": "c3504976-9361-4a81-873d-3f2bbc12de02",
  "success": true
}
```

---

### WASM Support
`pdf_hash` is available in pure WebAssembly environments via the `paperpilot-wasm` package:

```javascript
import { pdf_hash } from "paperpilot-wasm";

// Provide file bytes as Uint8Array
const fileBuffer = await fetch("form.pdf").then(res => res.arrayBuffer());
const hash = pdf_hash(new Uint8Array(fileBuffer));
console.log("SHA-256:", hash);
```

---

### Known Limitations & Troubleshooting
- **Metadata Incremental Updates**: In incremental PDF updates, even small changes (such as viewing timestamps or annotation flag updates saved by PDF readers) will alter the binary byte hash. To verify structural content identity rather than byte identity, use `paperpilot compare` or normalize the PDF before hashing.
- **Hash Algorithm**: The primary cryptographic hashing algorithm is standardized on SHA-256. For legacy algorithms like MD5 or SHA-1, calculate via standard system utilities or custom workflow recipes.
