---
title: "pdf_read_form"
description: "Reference, CLI, MCP, REST API, and WASM specifications for pdf_read_form."
sidebar:
  order: 36
---

**Status**: ✅ Verified  |  **Format**: PDF (`.pdf`) -> JSON (`.json`)

### What it does
**Extracts AcroForm field names and current values from a PDF document.**  
Reads text fields, checkboxes, choices, and hierarchical field names. Exports field state to standard stdout, formatted JSON output files, or structured MCP/REST responses.

---

### CLI
Extract form fields from an AcroForm document using either `paperpilot form read-form` (or alias `read`) or direct command alias `pdf-read-form`:

#### 1. Inspect Form Fields to Terminal (JSON Output)
```bash
paperpilot form read-form tests/e2e_fixtures/real/form.pdf --json
```

Output:
```json
{
  "FullName": "",
  "Email": "",
  "AgreedToTerms": "Off",
  "AccountType": "Enterprise"
}
{"success":true,"operation":"Form","error":null}
```

#### 2. Save Extracted Form Fields to File
```bash
paperpilot form read-form tests/e2e_fixtures/real/form.pdf \
  -o tests/e2e_fixtures/out/batch9_test/read_fields.json
```

#### CLI Parameters
- `<INPUT>` *(string, positional, required)*: Path to input AcroForm PDF document.
- `-o, --output` *(string, optional)*: Destination file path for extracted JSON key-value map.
- `--json` *(flag, optional)*: Emit structured execution status and field mapping to stdout.

---

### MCP JSON-RPC
Invoke `pdf_read_form` via Model Context Protocol:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_read_form",
    "arguments": {
      "input": "tests/e2e_fixtures/real/form.pdf",
      "output": "tests/e2e_fixtures/out/batch9_test/read_fields_mcp.json"
    }
  },
  "id": 1
}
```

#### MCP Output Response
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "content": [
      {
        "type": "text",
        "text": "{\n  \"AccountType\": \"Enterprise\",\n  \"AgreedToTerms\": \"Off\",\n  \"Email\": \"\",\n  \"FullName\": \"\"\n}"
      }
    ],
    "isError": false
  }
}
```

#### MCP Schema Parameters
- `input` *(string, required)*: Path to source PDF document.
- `output` *(string, optional)*: Optional output JSON file path to persist field map.

---

### REST API
Execute via PaperPilot Gateway endpoints:

#### Endpoint
- `POST /api/v1/pdf/pdf_read_form`
- Alternate tool runner route: `POST /api/v1/pdf/tools/pdf_read_form`

#### Request Payload
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/pdf_read_form \
  -H "Content-Type: application/json" \
  -d '{
    "input": "tests/e2e_fixtures/real/form.pdf"
  }'
```

#### REST Response
```json
{
  "success": true,
  "message": "{\n  \"AccountType\": \"Enterprise\",\n  \"AgreedToTerms\": \"Off\",\n  \"Email\": \"\",\n  \"FullName\": \"\"\n}",
  "data": {
    "AccountType": "Enterprise",
    "AgreedToTerms": "Off",
    "Email": "",
    "FullName": ""
  },
  "output_path": null,
  "diff_detected": null
}
```

---

### WASM Support
`pdf_read_form` inspection is available via the browser/WASM document model assertions:

```javascript
import { PdfDocument } from "@paperpilot/wasm";

const doc = await PdfDocument.load(pdfBytes);
const fields = doc.form_fields();
console.log("AcroForm Fields:", fields);
```

---

### Known Limitations & Troubleshooting
- **Non-AcroForm Documents**: PDFs that do not contain an `/AcroForm` catalog dictionary return empty dictionaries `{}` or emit an error indicating no interactive form fields were detected.
- **XFA Forms**: Dynamic XML Forms Architecture (XFA) fields are not supported; only standard static PDF AcroForm dictionaries (`/Fields`) are enumerated.
