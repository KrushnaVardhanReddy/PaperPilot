---
title: "pdf_linearize"
description: "Reference, CLI, MCP, and REST API commands for pdf_linearize Fast Web View byte-serving optimization."
sidebar:
  order: 30
---

**Status**: ✅ Verified  |  **Format**: PDF $\to$ Linearized PDF

### What it does
**Optimizes a PDF for Fast Web View (byte-serving linearization).**  
Restructures internal cross-reference tables and streams so web browsers and cloud document viewers can instantly display Page 1 before the entire file finishes downloading.

- **Fast Web View Preparation**: Re-orders object streams such that all first-page resources (page dictionary, content stream, fonts, and images) reside at the head of the file.
- **Linearization Dictionary Generation**: Synthesizes the required `/Linearized` catalog dictionary at the beginning of the file, specifying file length, primary hint stream offset, and first-page object numbers.
- **Primary Hint Streams**: Generates navigation hint tables that enable web servers supporting HTTP byte ranges (`Range: bytes=...`) to stream individual pages on demand.
- **Improved Streaming Latency**: Eliminates reader freezing when opening 100+ page manuals, books, or technical specifications over network connections.

---

### CLI
`pdf_linearize` is invoked via `paperpilot linearize` or `paperpilot pdf-linearize`:

```bash
# Linearize PDF for Fast Web View
paperpilot pdf-linearize \
  --input tests/e2e_fixtures/real/single_page.pdf \
  --output tests/e2e_fixtures/out/penta_e2e/linearized.pdf \
  --json

# Alternative CLI alias syntax
paperpilot linearize \
  --input tests/e2e_fixtures/real/single_page.pdf \
  --output tests/e2e_fixtures/out/penta_e2e/linearized.pdf \
  --json
```

#### CLI Parameters
- `--input` *(string, required)*: Path to the input PDF file to linearize.
- `--output` *(string, required)*: Destination path for the linearized output PDF.
- `--json` *(flag, optional)*: Emit JSON execution status to stdout.

---

### MCP JSON-RPC
Invoke `pdf_linearize` via Model Context Protocol:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_linearize",
    "arguments": {
      "input": "tests/e2e_fixtures/real/single_page.pdf",
      "output": "tests/e2e_fixtures/out/penta_e2e/linearized.pdf"
    }
  },
  "id": 1
}
```

#### MCP Schema Parameters
- `input` *(string, required)*: Path to the PDF to linearize.
- `output` *(string, required)*: Path to save the linearized PDF.

---

### REST API
Execute Fast Web View linearization via PaperPilot Gateway:

```bash
# Dedicated Route
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/pdf_linearize \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/linearized.pdf"
  }'
```

```bash
# Universal Tool Route
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_linearize \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/linearized.pdf"
  }'
```

---

### Return Structure & Output Schema
When invoked via MCP or Gateway REST API:

```json
{
  "success": true,
  "message": "PDF linearized successfully.",
  "output_path": "tests/e2e_fixtures/out/penta_e2e/linearized.pdf",
  "data": null
}
```

When invoked via CLI with `--json`:
```json
{
  "success": true,
  "operation": "Linearize",
  "error": null
}
```

---

### Known Limitations & Troubleshooting
- **REST Route Naming**: The REST API exposes `pdf_linearize` under `/api/v1/pdf/pdf_linearize` and `/api/v1/pdf/tools/pdf_linearize`. The shortened endpoint `/api/v1/pdf/linearize` returns 404; always use `/api/v1/pdf/pdf_linearize`.
- **Existing Digital Signatures**: Restructuring internal stream ordering and object offsets invalidates pre-existing digital signatures. Always linearize files prior to cryptographic signing.
- **WASM Support**: Full stream offset linearization is supported via CLI, MCP, and REST API. For browser web runtimes, use the REST gateway.
