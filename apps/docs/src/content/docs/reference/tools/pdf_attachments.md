---
title: "pdf_attachments"
description: "Reference, CLI, MCP, and REST API commands for pdf_attachments."
sidebar:
  order: 53
---

**Status**: ✅ Verified  |  **Format**: PDF (`.pdf`) / Data (`.json` / files)

### What it does
**Inspects, extracts, adds, or removes embedded file attachments in PDF documents and portfolios.**  
Allows users to manage encapsulated attachments within PDF documents. A PDF portfolio or container can store arbitrary file types (e.g. CSVs, raw images, spreadsheets, reports) encapsulated securely inside its `/EmbeddedFiles` NameTree. Supports four core lifecycle actions: `list`, `add`, `extract`, and `remove`.

---

### CLI
Manage attachments via the PaperPilot CLI:

#### 1. List Attachments in PDF
```bash
paperpilot pdf-attachments \
  --input tests/e2e_fixtures/single_page.pdf \
  --action list \
  --output tests/e2e_fixtures/out/attachments_report.json
```

#### 2. Embed / Add an Attachment
```bash
paperpilot pdf-attachments \
  --input tests/e2e_fixtures/single_page.pdf \
  --action add \
  --file-name data.txt \
  --file tests/e2e_fixtures/extracted_text.txt \
  --output tests/e2e_fixtures/out/with_attachment.pdf
```

#### 3. Extract an Embedded Attachment
```bash
paperpilot pdf-attachments \
  --input tests/e2e_fixtures/out/with_attachment.pdf \
  --action extract \
  --file-name data.txt \
  --output tests/e2e_fixtures/out/extracted_data.txt
```

#### 4. Remove an Embedded Attachment
```bash
paperpilot pdf-attachments \
  --input tests/e2e_fixtures/out/with_attachment.pdf \
  --action remove \
  --file-name data.txt \
  --output tests/e2e_fixtures/out/attachment_removed.pdf
```

#### CLI Parameters
- `--input` *(string, required)*: Path to the target PDF file.
- `--output` *(string, required)*: Path for the generated output (PDF file for `add`/`remove`, extracted file for `extract`, or JSON report for `list`).
- `--action` *(string, optional, default: `"list"`)*: Lifecycle action: `list`, `add`, `extract`, or `remove`.
- `--file-name` *(string, optional)*: Name of the attachment to add, extract, or delete.
- `--file` *(string, optional)*: Local file path containing the data payload to embed when `--action add` is used.
- `--json` *(flag, optional)*: Emit structured JSON execution result.

---

### MCP JSON-RPC
When integrating via Model Context Protocol (MCP), call the tool `pdf_attachments`:

#### 1. Embed Attachment via Base64 Payload
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_attachments",
    "arguments": {
      "input": "tests/e2e_fixtures/single_page.pdf",
      "output": "tests/e2e_fixtures/out/with_attachment.pdf",
      "action": "add",
      "file_name": "report.csv",
      "description": "Quarterly metrics",
      "file_data_base64": "aWQsbW9udGgsc2FsZXMKMSxKYW4sMTI1MDAKMixGZWIsMTQyMDA="
    }
  },
  "id": 1
}
```

#### 2. List Portfolio Attachments
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_attachments",
    "arguments": {
      "input": "tests/e2e_fixtures/out/with_attachment.pdf",
      "output": "tests/e2e_fixtures/out/attachments_list.json",
      "action": "list"
    }
  },
  "id": 2
}
```

#### MCP Schema Parameters
- `input` *(string, required)*: Source PDF document path.
- `output` *(string, required)*: Destination file path.
- `action` *(string, optional, default: `"list"`)*: Action to execute (`list`, `add`, `extract`, `remove`).
- `file_name` *(string, optional)*: Name of the attachment.
- `description` *(string, optional)*: Optional descriptive label for the attachment.
- `file_data_base64` *(string, optional)*: Base64-encoded raw byte payload for `add` operations.

---

### REST API
Execute attachment management via PaperPilot Gateway:

#### 1. Dedicated Route (`/api/v1/pdf/attachments`)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/attachments \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/single_page.pdf",
    "output": "tests/e2e_fixtures/out/attachments_report.json",
    "action": "list"
  }'
```

#### 2. Tool Route (`/api/v1/pdf/tools/pdf_attachments`)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_attachments \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/single_page.pdf",
    "output": "tests/e2e_fixtures/out/attachments_report.json",
    "action": "list"
  }'
```

---

### WebAssembly (WASM)
In browser or worker contexts using `@paperpilot/wasm`:

```javascript
import { WasmPdfEngine } from 'paperpilot-wasm';

const options = JSON.stringify({
  action: "list"
});

// Returns an ArrayBuffer / Uint8Array with the operation result
const resultBytes = WasmPdfEngine.pdf_attachments(pdfBytes, options);
const report = JSON.parse(new TextDecoder().decode(resultBytes));
console.log(`Found ${report.total_count} attachments.`);
```

---

### Known Limitations & Troubleshooting
- **Base64 Payload for MCP / REST Additions:** When adding files over MCP or REST JSON, supply the file contents via `file_data_base64`. For CLI usage, provide the file path directly using `--file <PATH>`.
- **Filename Matching on Extraction & Removal:** Attachment extraction and removal are matched by exact file name as stored in the PDF NameTree. If the file name does not match any entry, an error is returned.
- **Deep Sanitization:** If your goal is to permanently remove all attachments, embedded files, and scripts across an entire PDF for security sanitization, consider using `paperpilot sanitize` or the `document_pii_scan` tool.
- **Proxy Environments:** In environments with corporate proxies configured (`HTTP_PROXY`/`HTTPS_PROXY`), pass `--noproxy '*'` or `NO_PROXY=127.0.0.1` to avoid proxy interception of loopback requests.
