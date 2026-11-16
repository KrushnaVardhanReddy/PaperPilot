---
title: "pdf_to_json"
description: "Reference, CLI, MCP, and REST API commands for pdf_to_json."
sidebar:
  order: 51
---

**Status**: ✅ Verified  |  **Format**: PDF (`.pdf`) $\to$ Structured Layout AST (`.json`)

### What it does
**Exports full multimodal PDF layout AST to structured JSON.**  
Extracts text blocks, page arrays, character metrics, structural hierarchy, and document metadata into clean, machine-readable JSON for downstream AI workflows and data pipelines.

- **Document AST & Page Arrays**: Structures pages sequentially with per-page text blocks, character counts, and document-level identifiers.
- **Agent & RAG Pipeline Friendly**: Directly consumed by agentic frameworks, knowledge graphs, and embedding generators without custom regex parsers.
- **MCP Native Integration**: When invoked via MCP, returns parsed JSON directly in the `data` response object for immediate programmatic consumption by AI agents.

---

### CLI
Export PDF structure to JSON using the unified `convert` command:

```bash
paperpilot convert \
  --input tests/e2e_fixtures/real/single_page.pdf \
  --format json \
  --output tests/e2e_fixtures/out/penta_e2e/document.json \
  --json
```

#### CLI Parameters
- `--input` *(string, required)*: Path to source PDF document.
- `--format` *(string, required)*: Target output format (`json`).
- `--output` *(string, optional, default: `"output.json"`)*: Destination path for JSON file.
- `--json` *(flag, optional)*: Emit structured JSON completion status.

#### Example Output (`document.json`)
```json
{
  "document_name": "single_page.pdf",
  "pages": [
    {
      "page_number": 1,
      "text": "PaperPilot Enterprise Solutions \nGlobal Document Technology Division \n...",
      "char_count": 1377
    }
  ]
}
```

---

### MCP JSON-RPC
Invoke `pdf_to_json` through Model Context Protocol:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_to_json",
    "arguments": {
      "input": "tests/e2e_fixtures/real/single_page.pdf",
      "output": "tests/e2e_fixtures/out/penta_e2e/document.json"
    }
  },
  "id": 1
}
```

#### MCP Schema Parameters
- `input` *(string, required)*: The input PDF file path.
- `output` *(string, required)*: The output JSON destination file path.

#### Example MCP Response
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "content": [
      {
        "type": "text",
        "text": "{\"success\":true,\"message\":\"Converted to JSON successfully.\",\"output_path\":\"tests/e2e_fixtures/out/penta_e2e/document.json\",\"data\":{\"document_name\":\"single_page.pdf\",\"pages\":[{\"page_number\":1,\"text\":\"PaperPilot Enterprise Solutions...\",\"char_count\":1377}]}}"
      }
    ],
    "isError": false
  }
}
```

---

### REST API
Export structured PDF JSON via PaperPilot Gateway:

```bash
# Dedicated Gateway Route
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/pdf_to_json \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/document.json"
  }'
```

```bash
# Format Conversion Route
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/convert \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "format": "json",
    "output": "tests/e2e_fixtures/out/penta_e2e/document.json"
  }'
```

```bash
# Universal MCP Tool Route
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_to_json \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/document.json"
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
Direct in-memory buffer JSON AST export is not currently exposed in `paperpilot-wasm`. For browser environments requiring document text, use `extract_text` or dispatch layout extraction requests to the REST Gateway / MCP server.

---

### Known Limitations & Troubleshooting
- **CLI Subcommand Note**: Always use `paperpilot convert --input <INPUT> --format json --output <OUTPUT>`. The command `paperpilot pdf-to-json` is a placeholder that does not write the output file directly; production workflows should always use `paperpilot convert`.
- **Large Document Memory Bounds**: Multi-hundred page documents output complete aggregated page arrays. For streaming or memory-constrained AI chunking, consider `paperpilot pdf-chunk` or `rag-chunks`.
