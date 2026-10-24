---
title: Validate Signatures
description: Cryptographically verify digital signatures, certificate chains, and pre-sign baseline integrity.
---

# Validate Signatures

Cryptographically verify digital signatures, certificate chains, and document integrity. Supports dual-layer audit by checking against a baseline document for pre-signature structural/semantic alterations (Spec 101).

## Interface Invocations

### CLI
```bash
paperpilot pdf-validate-signatures --input signed.pdf --output report.json
# With baseline verification
paperpilot pdf-validate-signatures --input signed.pdf --output report.json --baseline original.pdf
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "id": "1",
  "method": "callTool",
  "params": {
    "name": "pdf_validate_signatures",
    "arguments": {
      "input": "signed.pdf",
      "output": "report.json",
      "baseline": true
    }
  }
}
```

### Gateway REST API
```bash
curl -X POST http://localhost:7823/api/v1/pdf/validate-signatures \\
  -H "Content-Type: application/json" \\
  -d '{
    "input": "signed.pdf",
    "output": "report.json",
    "baseline": true
  }'
```
