---
title: "document_pii_scan"
description: "Reference, CLI, MCP, and REST API commands for document_pii_scan."
sidebar:
  order: 56
---

**Status**: ✅ Verified  |  **Format**: PDF (`.pdf`) -> Structured JSON Findings

### What it does
**Scans document content streams for Personally Identifiable Information (PII) and sensitive data leaks.**  
Heuristically analyzes text layers across pages using validated regex patterns and checksum validation to detect Social Security Numbers (SSN), Credit Card Numbers (CCN), Email Addresses, Phone Numbers, and International Bank Account Numbers (IBAN). Emits structured findings with masked previews, page indexes, and confidence scores.

---

### CLI
Scan documents for PII using `paperpilot pii-scan` (or alias `document-pii-scan`):

#### 1. Full Document PII Scan
```bash
paperpilot pii-scan \
  --input tests/e2e_fixtures/real/text_pii.pdf \
  --output tests/e2e_fixtures/out/penta_e2e/pii_report.json \
  --json
```

#### 2. Targeted Category Scan
Filter for specific PII types using category flags:
```bash
paperpilot pii-scan \
  --input tests/e2e_fixtures/real/text_pii.pdf \
  --output tests/e2e_fixtures/out/penta_e2e/pii_targeted.json \
  --scan-ssn \
  --scan-emails \
  --scan-credit-cards \
  --json
```

#### CLI Parameters
- `--input` *(string, required)*: Path to source PDF document.
- `--output` *(string, optional)*: Path to write the JSON findings report.
- `--scan-ssn` *(flag, optional)*: Explicitly scan for US Social Security Numbers (`\d{3}-\d{2}-\d{4}`).
- `--scan-credit-cards` *(flag, optional)*: Explicitly scan for major credit card numbers (Visa, Mastercard, Amex, Discover).
- `--scan-emails` *(flag, optional)*: Explicitly scan for email addresses.
- `--scan-phones` *(flag, optional)*: Explicitly scan for phone numbers.
- `--scan-iban` *(flag, optional)*: Explicitly scan for International Bank Account Numbers.
- `--json` *(flag, optional)*: Emit structured JSON output to stdout.

---

### MCP JSON-RPC
Invoke PII scanning via Model Context Protocol:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "document_pii_scan",
    "arguments": {
      "input": "tests/e2e_fixtures/real/text_pii.pdf",
      "output": "tests/e2e_fixtures/out/penta_e2e/pii_report.json",
      "scan_ssn": true,
      "scan_emails": true,
      "scan_phones": true
    }
  },
  "id": 1
}
```

#### MCP Schema Parameters
- `input` *(string, required)*: Path to the input PDF file.
- `output` *(string, optional)*: Destination path for the JSON report file.
- `scan_ssn` *(boolean, optional)*: Enable SSN detection.
- `scan_credit_cards` *(boolean, optional)*: Enable credit card detection.
- `scan_emails` *(boolean, optional)*: Enable email detection.
- `scan_phones` *(boolean, optional)*: Enable phone number detection.
- `scan_iban` *(boolean, optional)*: Enable IBAN detection.

---

### REST API
Perform PII inspection via PaperPilot Gateway:

#### Dedicated Endpoint
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/pii-scan \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/text_pii.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/pii_report.json"
  }'
```

#### Universal Tool Route
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/document_pii_scan \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/text_pii.pdf",
    "scan_ssn": true,
    "scan_emails": true
  }'
```

---

### WASM Execution
`document_pii_scan` is fully supported in pure client-side WebAssembly via `paperpilot-wasm`:

```typescript
import { WasmPdfEngine } from "paperpilot-wasm";

const engine = new WasmPdfEngine();
const reportJson = engine.pii_scan(pdfBuffer, JSON.stringify({
  scan_ssn: true,
  scan_emails: true,
  scan_credit_cards: true
}));
const findings = JSON.parse(reportJson);
console.log(`Found ${findings.total_findings} sensitive records.`);
```

---

### Known Limitations & Troubleshooting
- **Scanned Image Documents**: `document_pii_scan` inspects native PDF digital text streams. For scanned or rasterized PDFs, first execute `pdf_ocr` to generate a searchable text layer before running PII scanning.
- **Value Masking**: Reported values are masked by default (`XXX-XX-1234`, `j***e@example.com`) to avoid secondary leaks in logging and audit registries.
