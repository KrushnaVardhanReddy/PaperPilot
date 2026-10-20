---
title: "pdf_sign"
description: "Reference, CLI, MCP, and REST API commands for pdf_sign."
sidebar:
  order: 44
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### What it does
**Applies cryptographic digital signatures and signature stamps to a PDF.**  
Embeds PKCS#7 / CMS digital signatures with X.509 certificates to verify signer identity and document authenticity.

### CLI
```bash
paperpilot sign --input tests/e2e_fixtures/real/single_page.pdf --output tests/e2e_fixtures/out/penta_e2e_security/sign_cli_Simple.pdf --cert tests/e2e_fixtures/real/cert.pem --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_sign",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/signed.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/sign -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/signed.pdf"}'
```
