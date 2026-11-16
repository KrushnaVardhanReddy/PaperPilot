---
title: "pdf_fill_form"
description: "Reference, CLI, MCP, REST API, and WASM specifications for pdf_fill_form."
sidebar:
  order: 25
---

**Status**: ✅ Verified  |  **Format**: PDF (`.pdf`) + JSON (`.json`) -> PDF (`.pdf`)

### What it does
**Fills interactive AcroForm fields (text fields, checkboxes, choices) from a JSON dataset.**  
Updates form field values in place, sets field values (`/V`), updates widget appearances (`/AP`), and maintains document integrity.

For bulk batch mail-merge across multiple rows of CSV or JSON, see [`cli_fill_batch`](/reference/tools/cli_fill_batch/) or [`batch_fill`](/reference/tools/batch_fill/).

---

### CLI
Populate form fields using either `paperpilot pdf-fill-form` or `paperpilot form fill-form`:

#### 1. Direct CLI Command (`pdf-fill-form`)
```bash
paperpilot pdf-fill-form \
  --input tests/e2e_fixtures/real/form.pdf \
  --data tests/e2e_fixtures/real/form_data.json \
  --output tests/e2e_fixtures/out/batch9_test/form_filled.pdf \
  --json
```

Output:
```json
{"success":true,"operation":"PdfFillForm","error":null}
```

#### 2. Subcommand Interface (`form fill-form`)
```bash
paperpilot form fill-form tests/e2e_fixtures/real/form.pdf \
  --data tests/e2e_fixtures/real/form_data.json \
  --output tests/e2e_fixtures/out/batch9_test/form_filled.pdf \
  --json
```

#### CLI Parameters
- `--input` *(string, required in `pdf-fill-form`, positional in `form fill-form`)*: Source PDF containing AcroForm widgets.
- `--data` *(string, required)*: Path to JSON file containing key-value mappings.
- `-o, --output` *(string, required)*: Destination path for the filled PDF.
- `--json` *(flag, optional)*: Emit structured JSON status report.

---

### MCP JSON-RPC
Invoke `pdf_fill_form` across Model Context Protocol clients. Accepts either an inline JSON object or file path via `values` (or `data`):

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_fill_form",
    "arguments": {
      "input": "tests/e2e_fixtures/real/form.pdf",
      "values": {
        "FullName": "Jane Doe",
        "Email": "jane.doe@enterprise.corp",
        "AgreedToTerms": "Yes",
        "AccountType": "Enterprise"
      },
      "output": "tests/e2e_fixtures/out/batch9_test/mcp_form_filled.pdf"
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
        "text": "{\"success\":true,\"message\":\"Form filled successfully.\",\"output_path\":\"tests/e2e_fixtures/out/batch9_test/mcp_form_filled.pdf\",\"data\":null}"
      }
    ],
    "isError": false
  }
}
```

#### MCP Schema Parameters
- `input` *(string, required)*: Path to input PDF file.
- `values` *(object or string, required)*: JSON object of field mappings or path to JSON file. `data` is accepted as an alias.
- `output` *(string, required)*: Output PDF path.

---

### REST API
Execute form filling via the REST gateway:

#### Endpoint
- `POST /api/v1/pdf/pdf_fill_form`
- Alternate tool runner route: `POST /api/v1/pdf/tools/pdf_fill_form`

#### Request Payload
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/pdf_fill_form \
  -H "Content-Type: application/json" \
  -d '{
    "input": "tests/e2e_fixtures/real/form.pdf",
    "values": {
      "FullName": "Jane Doe",
      "Email": "jane.doe@enterprise.corp",
      "AgreedToTerms": "Yes",
      "AccountType": "Enterprise"
    },
    "output": "tests/e2e_fixtures/out/batch9_test/api_form_filled.pdf"
  }'
```

#### REST Response
```json
{
  "success": true,
  "message": "Form filled successfully.",
  "data": null,
  "output_path": "tests/e2e_fixtures/out/batch9_test/api_form_filled.pdf",
  "diff_detected": null
}
```

---

### WASM Support
In-browser WASM builds provide document structure inspection and form field extraction. Interactive in-memory single-document filling across browser byte buffers can be flattened or integrated via the WASM core pipeline.

---

### Known Limitations & Troubleshooting
- **Missing Field Keys**: Values passed with keys not matching existing `/T` form field names are safely ignored.
- **Checkbox Export Values**: Checkboxes require state values corresponding to their appearance states (e.g. `"Yes"`, `"On"`, `"1"`, or `"Off"`). Passing invalid strings may leave the checkbox unchecked.
- **Radio and Pushbutton Widgets**: Complex radio button groups and signature pushbuttons are not filled by text mapping and require dedicated signing or choice handling.
