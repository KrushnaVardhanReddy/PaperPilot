---
title: "json_to_pdf"
description: "Reference, CLI, MCP, and REST API commands for json_to_pdf."
sidebar:
  order: 3
---

**Status**: ✅ Verified  |  **Format**: Structured JSON (`.json`) $\to$ Publication PDF (`.pdf`)

### What it does
**Synthesizes publication-ready PDF documents from structured JSON document definitions.**  
Transforms structured layout trees, text blocks, table elements, styling metadata, and embedded images from JSON directly into clean, rendered PDF documents without requiring a web browser engine.

- **Deterministic Layout Engine**: Compiles JSON document specifications into exact PDF page geometries, standard fonts, and coordinates.
- **Embedded Visual Elements**: Supports rendering structured metadata, headers, paragraphs, and embedded base64 graphics.
- **Universal Cross-Platform Support**: Supported natively in CLI, MCP, REST Gateway, and directly in WebAssembly (`paperpilot-wasm`).

---

### CLI

`json_to_pdf` is invoked via `paperpilot json-to-pdf` or the unified `convert` command:

```bash
# Direct json-to-pdf command
paperpilot json-to-pdf \
  --input tests/e2e_fixtures/real/json_doc.json \
  --output tests/e2e_fixtures/out/batch16_test/json_out.pdf \
  --json
```

```bash
# Unified convert command
paperpilot convert \
  --input tests/e2e_fixtures/real/json_doc.json \
  --format pdf \
  --output tests/e2e_fixtures/out/batch16_test/json_out.pdf \
  --json
```

#### CLI Parameters
- `--input` *(string, required)*: Path to structured JSON document specification file.
- `--output` *(string, required)*: Destination path for generated PDF document.
- `--json` *(flag, optional)*: Output structured JSON result.

#### Example CLI Output
```json
{"success":true,"operation":"JsonToPdf","error":null}
```

---

### MCP JSON-RPC

Invoke `json_to_pdf` via Model Context Protocol:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "json_to_pdf",
    "arguments": {
      "input": "tests/e2e_fixtures/real/json_doc.json",
      "output": "tests/e2e_fixtures/out/batch16_test/mcp_json.pdf"
    }
  },
  "id": 1
}
```

#### MCP Schema Parameters
- `input` *(string, required)*: Path to input JSON document file.
- `output` *(string, required)*: Path to save the compiled PDF document.
- `preset` *(string, optional)*: Visual theme preset (`github`, `elegant`, `minimal`, `branded`, `compact`).
- `custom_css` *(string, optional)*: Custom CSS rules to apply.
- `page_size` *(string, optional)*: Page size format (e.g. `A4`, `Letter`).
- `orientation` *(string, optional)*: Page orientation (`portrait` or `landscape`).

#### Example MCP Response
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "content": [
      {
        "type": "text",
        "text": "{\"success\":true,\"message\":\"JSON synthesized to PDF successfully.\",\"output_path\":\"tests/e2e_fixtures/out/batch16_test/mcp_json.pdf\",\"data\":null}"
      }
    ],
    "isError": false
  }
}
```

---

### REST API

Synthesize PDFs from JSON via PaperPilot Gateway:

```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/json_to_pdf \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/json_doc.json",
    "output": "tests/e2e_fixtures/out/batch16_test/rest_json.pdf"
  }'
```

#### Example Gateway Response
```json
{
  "message": "Success",
  "output": {
    "content": [
      {
        "text": "{\"success\":true,\"message\":\"JSON synthesized to PDF successfully.\",\"output_path\":\"tests/e2e_fixtures/out/batch16_test/rest_json.pdf\",\"data\":null}",
        "type": "text"
      }
    ],
    "isError": false,
    "resultType": "complete"
  },
  "request_id": "c2434a0b-5dd6-4ece-bb8a-47b933bdceb7",
  "success": true
}
```

---

### WASM Support
`json_to_pdf` is fully supported in WebAssembly environments via the `paperpilot-wasm` package for pure in-browser execution:

```javascript
import { json_to_pdf } from "paperpilot-wasm";

const jsonDocumentString = JSON.stringify({
  title: "Quarterly Report",
  pages: [
    { elements: [{ type: "heading", text: "Executive Summary" }, { type: "paragraph", text: "All quarterly KPIs met." }] }
  ]
});

// Returns Uint8Array of compiled PDF binary
const pdfBytes = json_to_pdf(jsonDocumentString);
```

---

### Known Limitations & Troubleshooting
- **Input Schema Format**: Input JSON must conform to valid PaperPilot document AST format containing structural blocks (`pages`, `elements`, `title`, or structured content nodes). Raw unstructured key-value objects should first be mapped to structured document nodes.
- **Embedded Fonts**: Standard PDF Type 1 fonts (Helvetica, Times, Courier) are used by default. External true-type fonts must be embedded as base64 in font definition objects.
