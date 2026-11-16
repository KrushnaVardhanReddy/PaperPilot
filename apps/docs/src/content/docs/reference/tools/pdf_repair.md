---
title: "pdf_repair"
description: "Reference, CLI, MCP, and REST API commands for pdf_repair cross-reference table and stream recovery."
sidebar:
  order: 41
---

**Status**: ✅ Verified  |  **Format**: PDF $\to$ Repaired PDF

### What it does
**Rebuilds corrupted, truncated, or unreadable PDF files.**  
Scans raw file streams, recovers damaged cross-reference tables (`xref`), reconstructs broken object trailers, and salvages readable page structures.

- **Cross-Reference Reconstruction**: Scans the binary document stream sequentially to locate all indirect object definitions (`obj ... endobj`) and synthesizes a compliant cross-reference table.
- **Trailer Rebuilding**: Regenerates missing or truncated `/Trailer` dictionaries and re-establishes pointers to the document `/Root` and `/Info` objects.
- **Orphan Object Pruning**: Drops unreferenced or malformed byte sequences while preserving valid text, vector, and image content streams.
- **Fail-Safe Sanitization**: Rewrites the salvaged objects into a clean, standard-compliant PDF file readable by standard PDF viewers.

---

### CLI
`pdf_repair` is invoked via `paperpilot repair` or `paperpilot pdf-repair`:

```bash
# Repair damaged PDF and output to recovered target file
paperpilot pdf-repair \
  --input tests/e2e_fixtures/real/single_page.pdf \
  --output tests/e2e_fixtures/out/penta_e2e/repaired.pdf \
  --json

# Alternative CLI alias syntax
paperpilot repair \
  --input tests/e2e_fixtures/real/single_page.pdf \
  --output tests/e2e_fixtures/out/penta_e2e/repaired.pdf \
  --json
```

#### CLI Parameters
- `--input` *(string, required)*: Path to the corrupted or malformed input PDF.
- `--output` *(string, required)*: Destination path for the repaired and rewritten PDF.
- `--json` *(flag, optional)*: Emit JSON execution status to stdout.

---

### MCP JSON-RPC
Invoke `pdf_repair` via Model Context Protocol:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_repair",
    "arguments": {
      "input": "tests/e2e_fixtures/real/single_page.pdf",
      "output": "tests/e2e_fixtures/out/penta_e2e/repaired.pdf"
    }
  },
  "id": 1
}
```

#### MCP Schema Parameters
- `input` *(string, required)*: Path to the PDF to repair.
- `output` *(string, required)*: Path to save the repaired PDF.

---

### REST API
Reconstruct damaged documents via PaperPilot Gateway:

```bash
# Dedicated Route
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/pdf_repair \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/repaired.pdf"
  }'
```

```bash
# Universal Tool Route
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_repair \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/repaired.pdf"
  }'
```

---

### Return Structure & Output Schema
When invoked via MCP or Gateway REST API:

```json
{
  "success": true,
  "message": "PDF repaired successfully.",
  "output_path": "tests/e2e_fixtures/out/penta_e2e/repaired.pdf",
  "data": null
}
```

When invoked via CLI with `--json`:
```json
{
  "success": true,
  "operation": "Repair",
  "error": null
}
```

---

### Known Limitations & Troubleshooting
- **Catastrophic Byte Loss**: Files where the document catalog (`/Root`) or page dictionary nodes are entirely missing or overwritten cannot be reconstructed without heuristic page recovery.
- **Encrypted Corrupted PDFs**: If encryption dictionary keys or initialization vectors are damaged, repair cannot recover encrypted streams without prior decryption.
- **REST Route Naming**: The REST API exposes `pdf_repair` under `/api/v1/pdf/pdf_repair` and `/api/v1/pdf/tools/pdf_repair`. Legacy `/api/v1/pdf/repair` is not a direct route; always use `/api/v1/pdf/pdf_repair`.
- **WASM Support**: Native binary stream repair is supported via CLI, MCP, and REST API. For web browser client runtimes, use the REST gateway service.
