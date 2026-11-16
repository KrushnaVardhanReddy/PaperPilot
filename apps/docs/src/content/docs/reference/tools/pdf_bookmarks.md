---
title: "pdf_bookmarks"
description: "Reference, CLI, MCP, and REST API commands for pdf_bookmarks."
sidebar:
  order: 9
---

**Status**: ✅ Verified  |  **Format**: PDF (`.pdf`) / JSON (`.json`)

### What it does
**Reads, extracts, and parses the document outline and Table of Contents bookmarks hierarchy into structured JSON.**  
Allows inspection of internal document navigation trees, section hierarchies, titles, and target page destinations.

---

### CLI
Extract bookmarks and outlines via PaperPilot CLI:

#### 1. Extract Outline Hierarchy to Stdout / JSON
```bash
paperpilot bookmarks \
  --input tests/e2e_fixtures/bates.pdf \
  --json
```

#### CLI Parameters
- `--input` *(string, required)*: Path to the input PDF document.
- `--json` *(flag, optional)*: Emit structured JSON execution result.

---

### MCP JSON-RPC
When integrating via Model Context Protocol (MCP), call the tool `pdf_bookmarks`:

#### Extract Bookmarks to JSON File
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_bookmarks",
    "arguments": {
      "input": "tests/e2e_fixtures/bates.pdf",
      "output": "tests/e2e_fixtures/out/bookmarks_extracted.json"
    }
  },
  "id": 1
}
```

#### MCP Schema Parameters
- `input` *(string, required)*: Path to the input PDF document.
- `output` *(string, required)*: Destination path for the extracted bookmarks JSON file.

#### MCP Result Example
```json
{
  "success": true,
  "message": "Successfully extracted 0 bookmark(s) to JSON.",
  "data": {
    "bookmarks_extracted": 0,
    "output": "tests/e2e_fixtures/out/bookmarks_extracted.json"
  },
  "output_path": "tests/e2e_fixtures/out/bookmarks_extracted.json"
}
```

---

### REST API
Extract document outline bookmarks via PaperPilot Gateway:

#### 1. Dedicated Route (`/api/v1/pdf/pdf_bookmarks`)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/pdf_bookmarks \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/bates.pdf",
    "output": "tests/e2e_fixtures/out/bookmarks_extracted.json"
  }'
```

#### 2. Tool Route (`/api/v1/pdf/tools/pdf_bookmarks`)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_bookmarks \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/bates.pdf",
    "output": "tests/e2e_fixtures/out/bookmarks_extracted.json"
  }'
```

#### Response Example
```json
{
  "success": true,
  "request_id": "73fb17eb-3446-4251-9f6f-b9d5e2ab3b79",
  "message": "Successfully extracted 0 bookmark(s) to JSON.",
  "output": {
    "isError": false,
    "resultType": "complete",
    "content": [
      {
        "type": "text",
        "text": "{\"bookmarks_extracted\":0,\"output\":\"tests/e2e_fixtures/out/bookmarks_extracted.json\"}"
      }
    ]
  }
}
```

---

### WebAssembly (WASM) & Native Operation
- In native Rust workflows, outline extraction is provided by `paperpilot_pdf::operations::bookmarks::BookmarksOperation`.
- In WASM client runtime environments, use document splitting by outline or table-of-contents extraction via `split_by_outline` or structural parser bindings.

---

### Known Limitations & Troubleshooting
- **Documents Without Outline Roots:** If a PDF document lacks an `/Outlines` dictionary or bookmark catalog entry, `pdf_bookmarks` succeeds cleanly and returns an empty bookmarks list with count `0`.
- **MCP Requires `output`:** In the MCP schema, both `input` and `output` are marked required. When calling via MCP or Gateway REST endpoints, always supply a valid target path for `output` (e.g. `bookmarks.json`).
- **Proxy Environments:** In environments with corporate proxies configured (`HTTP_PROXY`/`HTTPS_PROXY`), pass `--noproxy '*'` or `NO_PROXY=127.0.0.1` to avoid proxy interception of loopback requests.
