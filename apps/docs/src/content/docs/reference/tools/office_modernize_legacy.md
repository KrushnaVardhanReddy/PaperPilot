---
title: "office_modernize_legacy"
description: "Reference, CLI, MCP, and REST API commands for office_modernize_legacy."
sidebar:
  order: 5
---

**Status**: ✅ Verified  |  **Format**: Legacy Binary Office (`.doc`, `.xls`, `.ppt`) -> Modern OpenXML (`.docx`, `.xlsx`, `.pptx`)

### What it does
**Upgrades obsolete legacy binary Microsoft Office files (Word 97-2003 `.doc`, Excel 97-2003 `.xls`, PowerPoint 97-2003 `.ppt`) to modern OpenXML formats (`.docx`, `.xlsx`, `.pptx`).**  
Parses CFBF (Compound File Binary Format) structures, extracts record streams and BIFF cell matrices, and rebuilds standard ECMA-376 OpenXML package containers.

---

### CLI
Modernize legacy binary documents using `paperpilot office modernize` (or alias `modernize-legacy`):

#### 1. Modernize Excel Spreadsheet (`.xls` to `.xlsx`)
```bash
paperpilot office modernize \
  --input tests/e2e_fixtures/real/test.xls \
  --output tests/e2e_fixtures/out/penta_e2e/modernized.xlsx \
  --json
```

#### CLI Parameters
- `--input` *(string, required)*: Path to legacy binary document (`.xls`, `.doc`, `.ppt`).
- `--output` *(string, required)*: Path to modernized OpenXML file (`.xlsx`, `.docx`, `.pptx`).
- `--json` *(flag, optional)*: Emit execution status and timing metrics in JSON.

---

### MCP JSON-RPC
Invoke modernization via Model Context Protocol:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "office_modernize_legacy",
    "arguments": {
      "input": "tests/e2e_fixtures/real/test.xls",
      "output": "tests/e2e_fixtures/out/penta_e2e/modernized.xlsx"
    }
  },
  "id": 1
}
```

#### MCP Schema Parameters
- `input` *(string, required)*: Path to the input legacy file.
- `output` *(string, required)*: Destination path for the converted modern file.

---

### REST API
Execute legacy modernization via PaperPilot Gateway:

#### Dedicated Endpoint
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/office/convert \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/test.xls",
    "output": "tests/e2e_fixtures/out/penta_e2e/modernized.xlsx"
  }'
```

#### Universal Tool Route
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/office_modernize_legacy \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/test.xls",
    "output": "tests/e2e_fixtures/out/penta_e2e/modernized.xlsx"
  }'
```

---

### Known Limitations & Troubleshooting
- **Extension Matching**: Target modern output extension must match the legacy file type (`.xls` -> `.xlsx`, `.doc` -> `.docx`, `.ppt` -> `.pptx`).
- **VBA Macro Stripping**: Binary VBA macro streams (`_VBA_PROJECT`) are safely stripped during modernization to eliminate security risks, producing clean OpenXML packages without macros.
- **WASM Support**: Compound binary file parsing is implemented for native architectures and exposed via CLI, MCP, and Gateway REST API.
