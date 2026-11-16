---
title: "pdf_to_pdf_a"
description: "Reference, CLI, MCP, and REST API commands for pdf_to_pdf_a."
sidebar:
  order: 48
---

**Status**: ✅ Verified  |  **Format**: PDF (`.pdf`) $\to$ Archival PDF/A (`.pdf`)

### What it does
**Converts PDF documents into the ISO 19005 archival format (PDF/A-1b).**  
Injects PDF/A conformance metadata, standard OutputIntents, and archival flags into document dictionary headers to ensure permanent, device-independent long-term document preservation and regulatory compliance.

- **ISO 19005 Archival Standard**: Attaches `GTS_PDFXVersion` / `PDF/A-1b` conformance keys to the document `/Info` dictionary.
- **Regulatory & Legal Compliance**: Prepares digital records, legal filings, and municipal archives for compliance audits.
- **Device-Independent Output**: Strips external device dependencies and dynamic actions to guarantee consistent rendering decades into the future.

---

### CLI

Convert a document to PDF/A format using `paperpilot pdf-a`:

```bash
paperpilot pdf-a \
  --input tests/e2e_fixtures/real/single_page.pdf \
  --output tests/e2e_fixtures/out/penta_e2e/document_pdfa.pdf \
  --json
```

*(Aliases supported: `paperpilot to-pdf-a`, `paperpilot pdf-to-pdf-a`, `paperpilot pdf_to_pdf_a`)*

#### CLI Parameters
- `--input` *(string, required)*: Path to source PDF document.
- `--output` *(string, required)*: Destination path for archival PDF/A document.
- `--json` *(flag, optional)*: Emit structured JSON execution result.

#### Example CLI Output
```json
{"success":true,"operation":"PdfA","error":null}
```

---

### MCP JSON-RPC

Invoke `pdf_to_pdf_a` via Model Context Protocol:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_to_pdf_a",
    "arguments": {
      "input": "tests/e2e_fixtures/real/single_page.pdf",
      "output": "tests/e2e_fixtures/out/penta_e2e/document_pdfa.pdf"
    }
  },
  "id": 1
}
```

#### MCP Schema Parameters
- `input` *(string, required)*: Path to the PDF to convert to PDF/A.
- `output` *(string, required)*: Path to save the PDF/A file.

#### Example MCP Response
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "content": [
      {
        "type": "text",
        "text": "{\"success\":true,\"message\":\"PDF successfully converted to PDF/A format.\",\"output_path\":\"tests/e2e_fixtures/out/penta_e2e/document_pdfa.pdf\",\"data\":null}"
      }
    ],
    "isError": false
  }
}
```

---

### REST API

Execute PDF/A conversion via PaperPilot Gateway:

```bash
# Dedicated Gateway Endpoint
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/pdf_to_pdf_a \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/document_pdfa.pdf"
  }'
```

```bash
# Standard Tool API Route
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_to_pdf_a \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/document_pdfa.pdf"
  }'
```

#### Example Gateway Response
```json
{
  "success": true,
  "data": null,
  "error": null
}
```

---

### WASM Support
PDF/A dictionary and metadata stamping operates directly on Lopdf document structures, which is fully compatible with in-memory execution models.

---

### Known Limitations & Troubleshooting
- **Dynamic JavaScript & External Links**: PDF/A standards disallow active scripts and unresolvable external URI actions. If your document contains active scripts, run `paperpilot sanitize` prior to archival conversion.
- **Embedded Fonts**: PDF/A requires all rendered font glyphs to be self-contained in the document. Unembedded system fonts should be normalized prior to strict validation.
