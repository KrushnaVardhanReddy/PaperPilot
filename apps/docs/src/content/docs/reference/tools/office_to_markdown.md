---
title: "office_to_markdown"
description: "Reference, CLI, MCP, and REST API commands for office_to_markdown."
sidebar:
  order: 6
---

**Status**: ✅ Verified  |  **Format**: Office OpenXML (`.docx`, `.xlsx`, `.pptx`) -> Markdown (`.md`)

### What it does
**Converts Microsoft Word documents, Excel spreadsheets, and PowerPoint slide decks into structured Markdown.**  
Preserves heading hierarchies, bullet points, bold/italic formatting, and data tables with standard Markdown syntax (`| col | col |`), optimized for LLM RAG context ingestion.

---

### CLI
Convert Office files to Markdown using `paperpilot office to-markdown` (or alias `to-md`):

#### 1. Convert Spreadsheet (`.xlsx`) to Markdown Tables
```bash
paperpilot office to-markdown \
  --input tests/e2e_fixtures/real/test.xlsx \
  --output tests/e2e_fixtures/out/penta_e2e/doc_sheet.md \
  --json
```

#### 2. Convert Word Document (`.docx`) to Structured Markdown
```bash
paperpilot office to-markdown \
  --input tests/e2e_fixtures/real/test.docx \
  --output tests/e2e_fixtures/out/penta_e2e/doc_word.md \
  --json
```

#### CLI Parameters
- `--input` *(string, required)*: Path to source Office document (`.docx`, `.xlsx`, `.pptx`).
- `--output` *(string, required)*: Path to write the resulting Markdown file (`.md`).
- `--json` *(flag, optional)*: Emit structured JSON execution receipt.

---

### MCP JSON-RPC
Convert documents using Model Context Protocol:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "office_to_markdown",
    "arguments": {
      "input": "tests/e2e_fixtures/real/test.xlsx",
      "output": "tests/e2e_fixtures/out/penta_e2e/output.md"
    }
  },
  "id": 1
}
```

#### MCP Schema Parameters
- `input` *(string, required)*: Input file path (`.docx`, `.xlsx`, `.pptx`).
- `output` *(string, required)*: Output markdown file destination path (`.md`).

---

### REST API
Execute markdown conversion via PaperPilot Gateway:

#### Dedicated Endpoint
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/office-to-markdown \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/test.xlsx",
    "output": "tests/e2e_fixtures/out/penta_e2e/output.md"
  }'
```

#### Universal Tool Route
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/office_to_markdown \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/test.docx",
    "output": "tests/e2e_fixtures/out/penta_e2e/output_word.md"
  }'
```

---

### Known Limitations & Troubleshooting
- **Spreadsheet Multi-Sheet Layout**: Multi-sheet workbooks generate secondary headers (`## SheetName`) followed by individual Markdown tables for each non-empty worksheet.
- **Embedded Media & Drawing Objects**: Vector drawings, SmartArt, and embedded chart canvas objects are omitted in plain Markdown representation; text runs within text shapes are preserved.
- **WASM Support**: Office OpenXML parsing is native Rust and served via CLI, MCP, and Gateway REST API.
