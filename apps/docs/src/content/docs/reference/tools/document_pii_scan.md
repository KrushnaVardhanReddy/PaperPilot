---
title: "document_pii_scan"
description: "Reference, CLI, MCP, and REST API commands for document_pii_scan."
sidebar:
  order: 56
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### What it does
**Scans document streams for personally identifiable information (PII) and data leaks.**  
Identifies Social Security numbers, credit card numbers, email addresses, phone numbers, and credentials with offset and confidence tracking.

### CLI
```bash
paperpilot pii-scan --input tests/e2e_fixtures/real/single_page.pdf --output tests/e2e_fixtures/out/penta_e2e/pii_report.json --json
```

### MCP JSON-RPC
```json
{
  "name": "document_pii_scan",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/pii_report.json"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/document-pii-scan -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/pii_report.json"}'
```
