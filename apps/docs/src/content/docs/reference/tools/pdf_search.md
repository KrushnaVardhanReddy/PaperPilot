---
title: "pdf_search"
description: "Reference, CLI, MCP, and REST API commands for pdf_search."
sidebar:
  order: 43
---

**Status**: ✅ Verified  |  **Format**: PDF $\to$ JSON

### What it does
**Searches for a text query inside a PDF document or a folder of PDFs and returns matching 1-based page numbers.**  
Full-text token search evaluates decoded page streams using case-insensitive normalized matching. When pointing to a directory, `pdf_search` scans all `.pdf` files in that folder and aggregates matching pages.

### CLI
```bash
# Basic keyword search on a PDF file
paperpilot search --input tests/e2e_fixtures/real/single_page.pdf --query "Single"

# Search with structured JSON output and write to file
paperpilot search \
  --input tests/e2e_fixtures/real/single_page.pdf \
  --query "Single" \
  --output /tmp/search_results.json \
  --json
```

**CLI Output (`/tmp/search_results.json`):**
```json
{
  "matches": [
    1
  ],
  "query": "Single"
}
```

### MCP JSON-RPC
Tool definition: `pdf_search` in `paperpilot-mcp/src/server.rs`.
```json
{
  "name": "pdf_search",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "query": "Single"
  }
}
```

**MCP Response:**
```json
{
  "success": true,
  "message": "Matches found on pages: [1]",
  "data": {
    "matches": [
      1
    ]
  },
  "output_path": null
}
```

### REST API
Endpoint: `POST /api/v1/pdf/tools/pdf_search` or fallback via `POST /api/v1/pdf/tools/search` (Local Gateway default: port 7823).
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_search \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "query": "Single"
  }'
```

**REST API Response:**
```json
{
  "success": true,
  "message": "Matches found on pages: [1]",
  "data": {
    "matches": [
      1
    ]
  },
  "output_path": null
}
```

### Parameter Reference
| Option | Type | Required | Description |
|---|---|---|---|
| `input` | `string` | Yes | Path to the input PDF file or directory of PDFs. |
| `query` | `string` | Yes | Case-insensitive text string to search for across pages. |
| `output` | `string` | No | Optional destination path to write search matches JSON. |
| `dir` | `string` | No | Optional directory path when scanning a folder of PDFs. |

### Known Limitations & Troubleshooting
- **Spatial Bounding Boxes**: `pdf_search` returns an array of matching 1-based page indices (`matches: [1, 3]`). To retrieve exact word coordinates or regex patterns with context strings, use `pdf_extract_patterns` instead.
- **Corrupted XREF Tables**: Scanned or malformed PDFs with missing xref tables will log fallback reconstruction warnings. Run `pdf-repair` first if text extraction fails.
- **WASM Support**: Full-text token search is available in native CLI, MCP, and Gateway; standalone WASM builds provide `PdfInspector.contains_text(text)` and `PdfInspector.get_text(page)` via `paperpilot-wasm`.
