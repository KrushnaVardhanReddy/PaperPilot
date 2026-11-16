---
title: "pdf_validate_signatures"
description: "Reference, CLI, MCP, and REST API commands for pdf_validate_signatures."
sidebar:
  order: 57
---

**Status**: ✅ Verified  |  **Format**: PDF $\to$ JSON

### What it does
**Cryptographically verifies digital signatures, certificate chains, and document integrity.**  
Inspects PDF signature dictionaries (`/ByteRange`, `/SubFilter`, `/Contents`, `/Cert`) to validate signatures and assess whether document contents were altered post-signing.

- **Dual-Layer Forensic Audit**: Verifies PKCS#7 / CMS cryptographic byte ranges and compares content streams against an optional baseline reference PDF (`--baseline`).
- **Tamper Forensics**: Identifies exact page numbers where text or drawing streams were altered after a signature was placed.
- **Detailed JSON Reporting**: Outputs comprehensive reports containing field names, signer identity, signing timestamp, validity status, and incremental update diagnostics.

### CLI
```bash
# Validate signatures in a signed document
paperpilot pdf-validate-signatures \
  --input tests/e2e_fixtures/signed.pdf \
  --output report.json \
  --json

# Forensic audit comparing a signed document against a pre-signed baseline reference
paperpilot pdf-validate-signatures \
  --input tests/e2e_fixtures/signed_unsigned.pdf \
  --baseline tests/e2e_fixtures/unsigned.pdf \
  --output audit_report.json \
  --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_validate_signatures",
  "arguments": {
    "input": "tests/e2e_fixtures/signed_unsigned.pdf",
    "output": "audit_report.json",
    "baseline": true
  }
}
```

### REST API
```bash
# Dedicated validation endpoint with baseline comparison
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/validate-signatures \
  -H 'Content-Type: application/json' \
  -d '{"input": "tests/e2e_fixtures/signed_unsigned.pdf", "output": "audit_report.json", "baseline": "tests/e2e_fixtures/unsigned.pdf"}'

# Dedicated endpoint (using tool name alias)
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/pdf_validate_signatures \
  -H 'Content-Type: application/json' \
  -d '{"input": "tests/e2e_fixtures/signed_unsigned.pdf", "output": "audit_report.json"}'
```

### WebAssembly (WASM)
In browser and client runtimes via `@paperpilot/wasm`:
```typescript
import { pdf_validate_signatures } from "@paperpilot/wasm";

const reportBytes = pdf_validate_signatures(
  signedPdfBytes,
  JSON.stringify({ baseline: false })
);
const report = JSON.parse(new TextDecoder().decode(reportBytes));
console.log(report.all_valid);
```

### Parameter Reference
| Option | CLI Flag | MCP Field | REST Field | Type | Default | Description |
|---|---|---|---|---|---|---|
| Input File | `--input` | `input` | `input` | `string` | *(required)* | Path to the PDF document to validate. |
| Output File | `--output` | `output` | `output` | `string` | *(required)* | Destination path for the JSON validation report. |
| Baseline Document | `--baseline` | `baseline` | `baseline` | `string` / `boolean` | `null` / `false` | Path to pre-sign reference PDF (CLI/REST) or boolean flag (MCP) for tamper detection. |

### Return Structure & Output Schema
```json
{
  "total_signatures": 1,
  "valid_signatures": 1,
  "all_valid": true,
  "signatures": [
    {
      "field_name": "Signature1",
      "signer_name": "John Doe",
      "organization": "Acme Corp",
      "signing_time": "D:20261116120000Z",
      "sub_filter": "adbe.pkcs7.detached",
      "byte_range": [0, 84320, 85600, 1024],
      "is_valid": true,
      "is_tampered": false,
      "error_message": null
    }
  ],
  "baseline_report": {
    "baseline_checked": true,
    "baseline_match": true,
    "verdict": "UNTOUCHED_JUST_SIGNED",
    "altered_pages": [],
    "diff_summary": []
  }
}
```

### Troubleshooting & Known Limitations
- **Unsigned Documents**: Documents without digital signature fields return `total_signatures: 0` and `all_valid: true`.
- **Pre-Signature Tampering**: If content streams on page $N$ differ from the baseline file, `baseline_report.verdict` returns `"CONTENT_ALTERED_PRIOR_TO_SIGNATURE"` with altered page indices listed under `altered_pages`.
