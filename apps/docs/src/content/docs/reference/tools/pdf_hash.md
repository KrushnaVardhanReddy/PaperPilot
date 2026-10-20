---
title: "pdf_hash"
description: "Reference, CLI, MCP, and REST API commands for pdf_hash."
sidebar:
  order: 27
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### What it does
**Calculates cryptographic checksums (MD5, SHA-256) of a PDF document.**  
Produces a unique digital fingerprint to verify document integrity, detect tampering, and validate chain of custody.

### CLI
```bash
paperpilot hash --input tests/e2e_fixtures/real/missing.pdf --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_hash",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/hash -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf"}'
```
