---
title: "pdf_metadata"
description: "Reference, CLI, MCP, and REST API commands for pdf_metadata."
sidebar:
  order: 32
---

**Status**: ✅ Verified  |  **Format**: PDF (`.pdf`) / JSON (`.json`)

### What it does
**Inspects, reads, updates, or edits document metadata fields (Title, Author, Subject, Keywords).**  
Allows extracting document information properties or updating metadata tags into output PDF files for compliance, indexing, and publication standards.

---

### CLI
Manipulate metadata using the PaperPilot CLI:

#### 1. Update Document Metadata
```bash
paperpilot metadata \
  --input tests/e2e_fixtures/single_page.pdf \
  --title "Annual Financial Report" \
  --author "PaperPilot Team" \
  --subject "Quarterly & Annual Review" \
  --keywords "finance,audit,report,2026" \
  --output tests/e2e_fixtures/out/metadata_updated.pdf \
  --json
```

#### CLI Parameters
- `--input` *(string, required)*: Path to the input PDF file.
- `--output` *(string, required)*: Path to write the output PDF containing updated metadata tags.
- `--title` *(string, optional)*: Document title metadata string.
- `--author` *(string, optional)*: Author / creator metadata string.
- `--subject` *(string, optional)*: Subject metadata description.
- `--keywords` *(string, optional)*: Comma-separated or space-delimited keywords.
- `--json` *(flag, optional)*: Emit structured JSON execution result.

---

### MCP JSON-RPC
When integrating via Model Context Protocol (MCP), call the tool `pdf_metadata`:

#### 1. Read Document Metadata
When `--output` is omitted, `pdf_metadata` reads and returns the existing metadata attributes:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_metadata",
    "arguments": {
      "input": "tests/e2e_fixtures/single_page.pdf"
    }
  },
  "id": 1
}
```

#### 2. Write / Update Document Metadata
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_metadata",
    "arguments": {
      "input": "tests/e2e_fixtures/single_page.pdf",
      "output": "tests/e2e_fixtures/out/metadata_updated.pdf",
      "title": "Annual Financial Report",
      "author": "PaperPilot Team",
      "subject": "Quarterly & Annual Review",
      "keywords": "finance,audit,report,2026"
    }
  },
  "id": 2
}
```

#### MCP Schema Parameters
- `input` *(string, required)*: Path to the input PDF document.
- `output` *(string, optional)*: Path where the updated PDF will be saved.
- `title` *(string, optional)*: New title.
- `author` *(string, optional)*: New author.
- `subject` *(string, optional)*: New subject.
- `keywords` *(string, optional)*: New keywords.

---

### REST API
Execute metadata inspections and modifications via PaperPilot Gateway:

#### 1. Tool Endpoint (`/api/v1/pdf/tools/pdf_metadata`)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_metadata \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/single_page.pdf",
    "output": "tests/e2e_fixtures/out/metadata_updated.pdf",
    "title": "Annual Financial Report",
    "author": "PaperPilot Team"
  }'
```

#### 2. MCP Exec Generic Gateway Endpoint
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/mcp-exec \
  -H 'Content-Type: application/json' \
  -d '{
    "tool": "pdf_metadata",
    "arguments": {
      "input": "tests/e2e_fixtures/single_page.pdf"
    }
  }'
```

#### Response Example
```json
{
  "success": true,
  "request_id": "73fb17eb-3446-4251-9f6f-b9d5e2ab3b79",
  "message": "Metadata updated successfully.",
  "output": {
    "isError": false,
    "resultType": "complete",
    "content": [
      {
        "type": "text",
        "text": "Metadata updated successfully."
      }
    ]
  }
}
```

---

### WebAssembly (WASM)
In browser or worker contexts using `@paperpilot/wasm`:

```javascript
import { WasmPdfEngine } from 'paperpilot-wasm';

// Returns a Uint8Array with new metadata written
const updatedPdfBytes = WasmPdfEngine.set_metadata(
  pdfBytes,
  "Annual Financial Report", // title
  "PaperPilot Team",          // author
  "Quarterly & Annual Review",// subject
  "finance,audit,report,2026" // keywords
);
```

---

### Known Limitations & Troubleshooting
- **CLI Requires `--output`:** The command line CLI requires `--output <PATH>` to write the modified PDF. For CLI-based metadata inspection without writing a file, run `paperpilot bookmarks` or examine the document via MCP / Gateway REST endpoints (`/api/v1/pdf/info` or `pdf_metadata` without output).
- **Metadata Sanitization vs Editing:** To strip all metadata, embedded scripts, and hidden identifiers for redaction / privacy compliance, use `paperpilot sanitize` or the `document_pii_scan` tool instead of empty metadata strings.
- **Proxy Environments:** In environments with corporate proxies configured (`HTTP_PROXY`/`HTTPS_PROXY`), pass `--noproxy '*'` or `NO_PROXY=127.0.0.1` to avoid proxy interception of loopback requests.
