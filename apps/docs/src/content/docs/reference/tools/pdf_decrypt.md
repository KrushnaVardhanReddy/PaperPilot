---
title: "pdf_decrypt"
description: "Reference, CLI, MCP, and REST API commands for pdf_decrypt."
sidebar:
  order: 19
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### CLI
```bash
paperpilot decrypt --input tests/e2e_fixtures/real/encrypted.pdf --output tests/e2e_fixtures/out/penta_e2e_security/decrypt_cli_Simple.pdf --password testpass --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_decrypt",
  "arguments": {
    "input": "tests/e2e_fixtures/real/encrypted.pdf",
    "password": "testpass",
    "output": "tests/e2e_fixtures/out/penta_e2e/decrypted.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/decrypt -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/encrypted.pdf", "password": "testpass", "output": "tests/e2e_fixtures/out/penta_e2e/decrypted.pdf"}'
```
