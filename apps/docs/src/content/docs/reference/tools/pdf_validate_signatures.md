---
title: "pdf_validate_signatures"
description: "Reference, CLI, MCP, and REST API commands for pdf_validate_signatures."
sidebar:
  order: 55
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### What it does
**Validates PKCS#7 / CMS digital signatures embedded in PDF documents.**  
Extracts signing certificates, checks byte range integrity, verifies cryptographic digest matches document content, and reports validity status.

### CLI
```bash
paperpilot pdf-validate-signatures --input tests/e2e_fixtures/real/single_page.pdf --output tests/e2e_fixtures/out/penta_e2e/sig_report.json --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_validate_signatures",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/sig_report.json"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/validate-signatures -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/sig_report.json"}'
```
