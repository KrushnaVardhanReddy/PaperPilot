---
title: "pdf_to_markdown"
description: "Reference, CLI, MCP, and REST API commands for pdf_to_markdown."
sidebar:
  order: 52
---

**Status**: ✅ Verified  |  **Format**: PDF (`.pdf`) $\to$ Markdown (`.md`)

### What it does
**Decompiles PDF documents into clean, LLM-ready Markdown.**  
Extracts semantic headings, page divisions, bullet lists, inline formatting, code blocks, tables, and text sections into high-fidelity Markdown text.

- **LLM Context Optimization**: Removes page artifacts, repetitive running headers, and noise, outputting structured Markdown suitable for RAG and agent context windows.
- **Page Anchor Preservation**: Identifies page boundaries and inserts logical separators (`# Page 1`, `# Page 2`) for auditability and document referencing.
- **Header & List Detection**: Infers semantic heading levels (`#`, `##`, `###`, `####`) based on font size differentials and typographic weights.

---

### CLI
Convert PDF documents to Markdown using the unified `convert` command:

```bash
paperpilot convert \
  --input tests/e2e_fixtures/real/single_page.pdf \
  --format markdown \
  --output tests/e2e_fixtures/out/penta_e2e/document.md \
  --json
```

#### CLI Parameters
- `--input` *(string, required)*: Path to source PDF document.
- `--format` *(string, required)*: Target output format (`markdown` or `md`).
- `--output` *(string, optional, default: `"output.md"`)*: Destination path for the Markdown file.
- `--json` *(flag, optional)*: Emit structured JSON completion status.

---

### MCP JSON-RPC
Invoke `pdf_to_markdown` through Model Context Protocol:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_to_markdown",
    "arguments": {
      "input": "tests/e2e_fixtures/real/single_page.pdf",
      "output": "tests/e2e_fixtures/out/penta_e2e/document.md"
    }
  },
  "id": 1
}
```

#### MCP Schema Parameters
- `input` *(string, required)*: The input PDF file path.
- `output` *(string, required)*: The output Markdown destination file path.

#### Example MCP Response
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "content": [
      {
        "type": "text",
        "text": "{\"success\":true,\"message\":\"Converted to Markdown successfully.\",\"output_path\":\"tests/e2e_fixtures/out/penta_e2e/document.md\",\"data\":null}"
      }
    ],
    "isError": false
  }
}
```

---

### REST API
Execute Markdown conversions via PaperPilot Gateway:

```bash
# Dedicated Gateway Route
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/pdf_to_markdown \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/document.md"
  }'
```

```bash
# Format Conversion Route
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/convert \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "format": "markdown",
    "output": "tests/e2e_fixtures/out/penta_e2e/document.md"
  }'
```

```bash
# Universal MCP Tool Route
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_to_markdown \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/document.md"
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
In-browser WASM compilation for `pdf_to_markdown` is planned for future releases. For client-side text retrieval in browser WASM, use `extract_text` or dispatch Markdown conversions to the REST Gateway / MCP server.

---

### Known Limitations & Troubleshooting
- **CLI Subcommand Note**: Always use `paperpilot convert --input <INPUT> --format markdown --output <OUTPUT>`. The command `paperpilot pdf-to-markdown` is a placeholder that does not write the output file directly; production workflows should always use `paperpilot convert`.
- **Complex Multi-Column Flows**: In documents with complex asymmetric column layouts or marginal notes, text extraction reconstructs flow top-to-bottom. For fine-grained semantic chunking, pair with `paperpilot rag-chunks` or `pdf_chunk`.
