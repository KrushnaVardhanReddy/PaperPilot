---
title: "pdf_classify_type"
description: "Reference, CLI, MCP, and REST API commands for pdf_classify_type."
sidebar:
  order: 11
---

**Status**: ✅ Verified  |  **Format**: PDF $\to$ JSON

### What it does
**Classifies a PDF document into standard taxonomy types using structural and typographical layout heuristics.**  
`pdf_classify_type` inspects typography hierarchies, keywords, metadata tags, and tabular layouts to categorize the document (such as `Document Specification`, `Invoice`, `Academic Paper`, `Contract`, `Presentation`, or `Form`) and outputs a classification score with confidence levels.

### CLI
```bash
# Classify document type and print JSON output to stdout
paperpilot classify-type --input tests/e2e_fixtures/real/single_page.pdf --json

# Save classification report to JSON file
paperpilot classify-type \
  --input tests/e2e_fixtures/real/single_page.pdf \
  --output /tmp/classify.json
```

**CLI Output (`/tmp/classify.json`):**
```json
{
  "type": "Document Specification",
  "confidence": 0.5555556
}
```

### MCP JSON-RPC
Tool definition: `pdf_classify_type` in `paperpilot-mcp/src/server.rs`.
```json
{
  "name": "pdf_classify_type",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "/tmp/classify.json"
  }
}
```

**MCP Response:**
```json
{
  "success": true,
  "message": "{\"type\":\"Document Specification\",\"confidence\":0.5555556}",
  "data": {
    "type": "Document Specification",
    "confidence": 0.5555555820465088
  },
  "output_path": "/tmp/classify.json"
}
```

### REST API
Dedicated routes: `POST /api/v1/pdf/pdf_classify_type` or `POST /api/v1/pdf/tools/pdf_classify_type` (Local Gateway default: port 7823).
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/pdf_classify_type \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf"
  }'
```

**REST API Response:**
```json
{
  "success": true,
  "message": "{\"type\":\"Document Specification\",\"confidence\":0.5555556}",
  "data": {
    "type": "Document Specification",
    "confidence": 0.5555555820465088
  },
  "output_path": null
}
```

### Parameter Reference
| Option | Type | Required | Description |
|---|---|---|---|
| `input` | `string` | Yes | Path to the PDF document to classify. |
| `output` | `string` | No | Optional path to write the JSON classification report. |

### Taxonomy Classifications
- `Document Specification`: Technical whitepapers, engineering specifications, or product documentation.
- `Invoice`: Financial invoices with tabular itemization, currency tokens, and billing addresses.
- `Contract`: Legal agreements with numbered clauses, recital paragraphs, and signature blocks.
- `Academic Paper`: Two-column research papers with abstracts, citations, and DOI references.
- `Presentation`: Landscape-oriented slide deck exports with large title font-to-body ratios.
- `Form`: Documents featuring interactive AcroForm widget annotations or fillable box grids.

### Known Limitations & Troubleshooting
- **Scanned Documents**: Heuristic classification relies on extractable digital text. For rasterized or scanned documents without OCR, run `paperpilot ocr` prior to classification.
- **WASM Support**: Native heuristic classification runs in CLI, Gateway, and MCP. Under pure WASM environments, combine `extract_text()` with client-side pattern matching.
