---
title: "pdf_timestamp"
description: "Reference, CLI, MCP, and REST API commands for pdf_timestamp."
sidebar:
  order: 1
---

**Status**: ✅ Verified  |  **Format**: PDF / PDF

### What it does
**Cryptographically timestamp PDFs.**
This tool applies an RFC 3161 Time-Stamp Protocol token to a PDF document to cryptographically verify its time of existence.

### CLI
```bash
paperpilot security timestamp --input tests/e2e_fixtures/real/test.pdf --output tests/e2e_fixtures/out/penta_e2e/pdf_timestamp_result.pdf --tsa-url http://tsa.example.com
```

### MCP JSON-RPC
```json
{
  "name": "pdf_timestamp",
  "arguments": {
    "input": "tests/e2e_fixtures/real/test.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/pdf_timestamp_result.pdf",
    "tsa_url": "http://tsa.example.com"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/timestamp \
  -H 'Content-Type: application/json' \
  -d '{"input": "tests/e2e_fixtures/real/test.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/pdf_timestamp_result.pdf", "tsa_url": "http://tsa.example.com"}'
```
