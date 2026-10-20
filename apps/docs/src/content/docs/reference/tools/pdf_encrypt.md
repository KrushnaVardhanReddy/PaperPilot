---
title: "pdf_encrypt"
description: "Reference, CLI, MCP, and REST API commands for pdf_encrypt."
sidebar:
  order: 21
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### What it does
**Secures a PDF document with standard AES-256 password encryption.**  
Sets user opening passwords and owner permission passwords to restrict printing, copying text, or editing.

### CLI
```bash
paperpilot encrypt --input tests/e2e_fixtures/real/single_page.pdf --output tests/e2e_fixtures/out/penta_e2e_security/encrypt_cli_Simple.pdf --user-password testpass --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_encrypt",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "password": "testpass",
    "output": "tests/e2e_fixtures/out/penta_e2e/encrypted.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/encrypt -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "password": "testpass", "output": "tests/e2e_fixtures/out/penta_e2e/encrypted.pdf"}'
```
