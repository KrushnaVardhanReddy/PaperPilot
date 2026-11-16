---
title: "pdf_dedup_streams"
description: "Reference, CLI, MCP, and REST API commands for pdf_dedup_streams lossless object stream deduplication."
sidebar:
  order: 58
---

**Status**: ✅ Verified  |  **Format**: PDF $\to$ Deduplicated PDF

### What it does
**Performs lossless indirect object stream deduplication and CMYK-safe document compression.**  
Leverages cryptographic hashing (SHA-256) to identify byte-identical streams across the entire document structure, ensuring zero false positives. Duplicate streams are merged into a single canonical indirect object, reducing file size significantly without sacrificing any data.

- **Cryptographic Stream Hashing**: Computes SHA-256 digests of all uncompressed and compressed object streams (fonts, vector forms, raster images, content streams).
- **Canonical Object Consolidation**: Rewires all indirect references across page dictionaries to point to the canonical stream object and eliminates redundant clones.
- **CMYK Color Space Guard**: Preserves original prepress and print quality profiles (`/DeviceCMYK` or `/ICCBased`) during compression workflows to prevent irreversible color-space distortion.
- **Lossless Size Reduction**: Ideal for compiled documents, slide decks, repetitive catalog pages, and multi-page reports with repeated header/footer art or embedded logos.

---

### CLI
`pdf_dedup_streams` is invoked via `paperpilot pdf-dedup-streams`:

```bash
# Deduplicate redundant streams and save to optimized target file
paperpilot pdf-dedup-streams \
  --input tests/e2e_fixtures/real/single_page.pdf \
  --output tests/e2e_fixtures/out/penta_e2e/deduped.pdf \
  --json
```

#### CLI Parameters
- `--input` *(string, required)*: Path to the input PDF file.
- `--output` *(string, required)*: Destination path for the deduplicated PDF.
- `--json` *(flag, optional)*: Emit JSON execution status to stdout.

---

### MCP JSON-RPC
Invoke `pdf_dedup_streams` via Model Context Protocol:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_dedup_streams",
    "arguments": {
      "input": "tests/e2e_fixtures/real/single_page.pdf",
      "output": "tests/e2e_fixtures/out/penta_e2e/deduped.pdf"
    }
  },
  "id": 1
}
```

#### MCP Schema Parameters
- `input` *(string, required)*: Path to the input PDF file.
- `output` *(string, required)*: Path to save the deduplicated PDF.

---

### REST API
Execute stream deduplication via PaperPilot Gateway:

```bash
# Dedicated Route
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/pdf_dedup_streams \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/deduped.pdf"
  }'
```

```bash
# Alternative Hyphenated Route
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/dedup-streams \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/deduped.pdf"
  }'
```

```bash
# Universal Tool Route
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_dedup_streams \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/deduped.pdf"
  }'
```

---

### WebAssembly (WASM)
In browser and client runtimes via `@paperpilot/wasm`:

```typescript
import { pdf_dedup_streams } from "@paperpilot/wasm";

const deduplicatedPdfBytes = pdf_dedup_streams(
  inputPdfBytes,
  JSON.stringify({})
);
```

---

### Return Structure & Output Schema
When invoked via MCP or Gateway REST API:

```json
{
  "success": true,
  "message": "Lossless Stream Deduplication executed successfully",
  "output_path": "tests/e2e_fixtures/out/penta_e2e/deduped.pdf",
  "data": {
    "output_path": "tests/e2e_fixtures/out/penta_e2e/deduped.pdf"
  }
}
```

When invoked via CLI:
```text
PdfDedupStreams complete.
```

```json
{
  "success": true,
  "operation": "PdfDedupStreams",
  "error": null
}
```

---

### Known Limitations & Troubleshooting
- **CLI Subcommand Name**: The CLI subcommand is `pdf-dedup-streams`. Do not use `dedup-streams` directly without the `pdf-` prefix on the CLI.
- **Documents Without Redundant Streams**: If a document contains unique assets (such as single-page scans or variable raster images), deduplication will complete successfully with an identical byte structure.
- **Signed Documents**: Re-indexing indirect object streams alters cross-reference byte offsets and will invalidate existing digital signatures. Run deduplication before applying signatures.
