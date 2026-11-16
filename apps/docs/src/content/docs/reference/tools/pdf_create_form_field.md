---
title: "pdf_create_form_field"
description: "Reference, CLI, MCP, REST API, and WASM specifications for pdf_create_form_field."
sidebar:
  order: 17
---

**Status**: ✅ Verified  |  **Format**: PDF (`.pdf`) -> PDF (`.pdf`)

### What it does
**Appends interactive AcroForm widget annotations (text fields or checkboxes) to a target PDF page.**  
Adds form field entries into the document Catalog `/AcroForm` `/Fields` array and attaches widget dictionaries (`/Subtype /Widget`) to the designated page's `/Annots` array with specified bounding box coordinates (`rect`).

---

### CLI
Add form fields using either `paperpilot pdf-create-form-field` or `paperpilot form create-form-field` (alias `add-field`):

#### 1. Add Text Field with Direct Command
```bash
paperpilot pdf-create-form-field \
  --input tests/e2e_fixtures/real/single_page.pdf \
  --name "SignatureDate" \
  --type text \
  --page 1 \
  --rect "100,400,300,430" \
  --output tests/e2e_fixtures/out/batch9_test/form_created_date.pdf \
  --json
```

Output:
```json
{"success":true,"operation":"PdfCreateFormField","error":null}
```

#### 2. Add Field with Subcommand Interface
```bash
paperpilot form create-form-field tests/e2e_fixtures/real/single_page.pdf \
  --name "Notes" \
  --rect "100,500,400,550" \
  --output tests/e2e_fixtures/out/batch9_test/form_created_notes.pdf \
  --json
```

#### CLI Parameters
- `--input` *(string, required in `pdf-create-form-field`, positional in `form create-form-field`)*: Target PDF file.
- `--name` *(string, required)*: Unique identifier name for the form field (`/T`).
- `--rect` *(string, required)*: Bounding rectangle coordinates in PDF points formatted as `"llx,lly,urx,ury"` (lower-left x, lower-left y, upper-right x, upper-right y).
- `--type` *(string, optional, default: `"text"`)*: Type of form field (`"text"` or `"checkbox"`).
- `--page` *(integer, optional, default: `1`)*: 1-indexed target page number.
- `--label` *(string, optional)*: Optional alternate label for accessibility (`/TU`).
- `-o, --output` *(string, required)*: Destination path for the modified PDF.
- `--json` *(flag, optional)*: Output execution result in JSON format.

---

### MCP JSON-RPC
Invoke `pdf_create_form_field` across MCP clients:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_create_form_field",
    "arguments": {
      "input": "tests/e2e_fixtures/real/single_page.pdf",
      "field_name": "ApplicantName",
      "field_type": "text",
      "page": 1,
      "rect": [100, 500, 350, 530],
      "output": "tests/e2e_fixtures/out/batch9_test/mcp_created_field.pdf"
    }
  },
  "id": 1
}
```

*(Note: MCP also accepts individual coordinate arguments `x`, `y`, `width`, `height` in place of `rect`)*

#### MCP Output Response
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "content": [
      {
        "type": "text",
        "text": "{\"success\":true,\"message\":\"Form field created successfully.\",\"output_path\":\"tests/e2e_fixtures/out/batch9_test/mcp_created_field.pdf\",\"data\":null}"
      }
    ],
    "isError": false
  }
}
```

#### MCP Schema Parameters
- `input` *(string, required)*: Path to input PDF file.
- `field_name` *(string, required)*: Field name (`name` is accepted as an alias).
- `field_type` *(string, required)*: Field type (`"text"`, `"checkbox"`).
- `page` *(integer, optional, default: `1`)*: Page number to attach widget.
- `rect` *(array of 4 numbers or comma-separated string, optional)*: `[llx, lly, urx, ury]`.
- `x`, `y`, `width`, `height` *(number, optional)*: Explicit dimensions if `rect` array is omitted.
- `output` *(string, required)*: Path to write output PDF.

---

### REST API
Execute field creation via PaperPilot Gateway:

#### Endpoint
- `POST /api/v1/pdf/pdf_create_form_field`
- Alternate tool runner route: `POST /api/v1/pdf/tools/pdf_create_form_field`

#### Request Payload
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/pdf_create_form_field \
  -H "Content-Type: application/json" \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "field_name": "SupervisorSign",
    "field_type": "text",
    "page": 1,
    "rect": [100, 200, 300, 230],
    "output": "tests/e2e_fixtures/out/batch9_test/api_created_field.pdf"
  }'
```

#### REST Response
```json
{
  "success": true,
  "message": "Form field created successfully.",
  "data": null,
  "output_path": "tests/e2e_fixtures/out/batch9_test/api_created_field.pdf",
  "diff_detected": null
}
```

---

### WASM Support
In WebAssembly environments, inspect created fields or examine existing field geometry using `PdfDocument.load(bytes).form_fields()`.

---

### Known Limitations & Troubleshooting
- **Coordinate System**: PDF coordinates originate at the lower-left corner `(0, 0)`. Ensure `urx > llx` and `ury > lly`.
- **Page Bounds**: Specifying a page number greater than total page count results in a page lookup error.
- **Duplicate Names**: Creating multiple fields with identical names links them into a single terminal field with multiple widget representations. Use unique names for distinct form fields.
