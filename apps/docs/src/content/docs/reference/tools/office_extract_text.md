---
title: "office_extract_text"
description: "Reference, CLI, MCP, and REST API commands for office_extract_text."
sidebar:
  order: 4
---

**Status**: ✅ Verified  |  **Format**: Office OpenXML / Binary (`.docx`, `.xlsx`, `.pptx`) -> Plain Text (`.txt`)

### What it does
**Extracts raw, unformatted text strings from Microsoft Office documents (.docx, .xlsx, .pptx).**  
Bypasses visual styles, runs, formulas, and slide transitions to dump pure text contents for high-speed indexing, grep search, and LLM embedding/RAG ingest pipelines.

---

### CLI
Extract plain text from any supported Office document using `paperpilot office extract` (or alias `office extract-text`):

#### 1. Extract from Spreadsheet (`.xlsx`)
```bash
paperpilot office extract \
  --input tests/e2e_fixtures/real/test.xlsx \
  --output tests/e2e_fixtures/out/penta_e2e/test_xlsx_extract.txt \
  --json
```

#### 2. Extract from Word Document (`.docx`)
```bash
paperpilot office extract \
  --input tests/e2e_fixtures/real/test.docx \
  --output tests/e2e_fixtures/out/penta_e2e/test_docx_extract.txt \
  --json
```

#### 3. Extract from Presentation (`.pptx`)
```bash
paperpilot office extract \
  --input tests/e2e_fixtures/out/all50_audit_1794659040/converted.pptx \
  --output tests/e2e_fixtures/out/penta_e2e/test_pptx_extract.txt \
  --json
```

#### CLI Parameters
- `--input` *(string, required)*: Path to the source Office document (`.docx`, `.xlsx`, `.pptx`).
- `--output` *(string, required)*: Path to the destination plain text file (`.txt`).
- `--json` *(flag, optional)*: Output structured execution status and diagnostics in JSON format.

---

### MCP JSON-RPC
Invoke `office_extract_text` via Model Context Protocol:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "office_extract_text",
    "arguments": {
      "input": "tests/e2e_fixtures/real/test.xlsx",
      "output": "tests/e2e_fixtures/out/penta_e2e/office_extracted.txt"
    }
  },
  "id": 1
}
```

#### MCP Schema Parameters
- `input` *(string, required)*: Path to the input Office file (`.docx`, `.xlsx`, `.pptx`).
- `output` *(string, optional)*: Destination text file path. When provided, dumps output to disk.

---

### REST API
Execute text extraction via PaperPilot Gateway:

#### Dedicated Endpoint
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/office-extract-text \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/test.xlsx",
    "output": "tests/e2e_fixtures/out/penta_e2e/office_extracted.txt"
  }'
```

#### Universal Tool Route
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/office_extract_text \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/test.docx",
    "output": "tests/e2e_fixtures/out/penta_e2e/docx_extracted.txt"
  }'
```

---

### Known Limitations & Troubleshooting
- **Legacy Binary Formats (.doc, .xls, .ppt)**: `office_extract_text` targets OOXML structures. For legacy 97-2003 binary formats (`.doc`, `.xls`), convert first using `office_modernize_legacy` before text extraction.
- **Malformed Archives**: Truncated ZIP archives or missing OOXML relationship parts (such as missing `_rels/.rels` or `officeDocument` relations) fail with standard archive validation errors. Ensure files are intact OOXML packages.
- **WASM Support**: Office document extraction relies on filesystem-level OOXML package inspection and is supported via CLI, MCP, and Gateway REST API.
