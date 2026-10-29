---
title: "pdf_validate_signatures"
description: "Reference, CLI, MCP, and REST API commands for pdf_validate_signatures."
sidebar:
  order: 57
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### What it does
**Cryptographically verifies digital signatures, certificate chains, and document integrity.**  
Supports dual-layer audit by checking certificate validity and verifying against a baseline document for pre-signature alterations.

### CLI
```bash
paperpilot pdf-validate-signatures --input signed.pdf --output report.json --baseline original.pdf
```

### MCP JSON-RPC
```json
{
  "name": "pdf_validate_signatures",
  "arguments": {
    "input": "signed.pdf",
    "output": "report.json",
    "baseline": "original.pdf"
  }
}
```

### REST API
```bash
curl -X POST http://localhost:7823/api/v1/pdf/validate-signatures -H "Content-Type: application/json" -d '{"input": "signed.pdf", "output": "report.json", "baseline": "original.pdf"}'
```
