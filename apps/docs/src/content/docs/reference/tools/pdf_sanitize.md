---
title: "pdf_sanitize"
description: "Reference, CLI, MCP, and REST API commands for pdf_sanitize threat neutralization and metadata stripping."
sidebar:
  order: 57
---

**Status**: ✅ Verified  |  **Format**: PDF $\to$ Sanitized PDF

### What it does
**Performs deep structural sanitization and threat neutralization on PDF files.**  
Strips sensitive document metadata, removes embedded file attachments, neutralizes JavaScript actions and launch triggers, deletes embedded thumbnails, and cleans private application data.

- **Metadata Eradication**: Clears standard `/Info` dictionary entries (`Author`, `Creator`, `Producer`, `Title`, `Subject`) and strips embedded XMP XML metadata packets.
- **Active Code Neutralization**: Detects and purges `/JavaScript`, `/JS`, `/Launch`, `/EmbeddedFiles`, and automated `/OpenAction` triggers that can execute arbitrary payloads.
- **Attachment Extraction/Removal**: Strips embedded binary files from the document Names dictionary (`/Names -> /EmbeddedFiles`).
- **Private Data Purging**: Deletes `/PieceInfo` and vendor-specific application data (e.g. Adobe Illustrator / Photoshop private editing layers).
- **Embedded Thumbnail Scrubbing**: Deletes uncompressed and compressed image thumbnails stored inside page object dictionaries.

---

### CLI
`pdf_sanitize` is invoked via `paperpilot sanitize`:

```bash
# Deep sanitization stripping metadata, scripts, attachments, and thumbnails
paperpilot sanitize \
  --input tests/e2e_fixtures/real/multi_page.pdf \
  --output tests/e2e_fixtures/out/penta_e2e/sanitized.pdf \
  --json

# Selective sanitization with specific options
paperpilot sanitize \
  --input tests/e2e_fixtures/real/multi_page.pdf \
  --output tests/e2e_fixtures/out/penta_e2e/sanitized.pdf \
  --remove-metadata \
  --remove-attachments \
  --remove-scripts \
  --json
```

#### CLI Parameters
- `--input` *(string, required)*: Path to the input PDF file to sanitize.
- `--output` *(string, required)*: Destination path for the sanitized PDF output.
- `--remove-metadata` *(flag, optional)*: Explicitly strip document metadata (defaults to true).
- `--remove-attachments` *(flag, optional)*: Explicitly delete embedded portfolio and attachment streams (defaults to true).
- `--remove-scripts` *(flag, optional)*: Explicitly delete embedded JavaScript and executable action triggers (defaults to true).
- `--json` *(flag, optional)*: Output execution status as JSON to stdout.

---

### MCP JSON-RPC
Invoke `pdf_sanitize` via Model Context Protocol:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_sanitize",
    "arguments": {
      "input": "tests/e2e_fixtures/real/multi_page.pdf",
      "output": "tests/e2e_fixtures/out/penta_e2e/sanitized.pdf",
      "remove_metadata": true,
      "remove_attachments": true,
      "remove_scripts": true
    }
  },
  "id": 1
}
```

#### MCP Schema Parameters
- `input` *(string, required)*: Path to input PDF document.
- `output` *(string, required)*: Destination path for the sanitized output PDF.
- `remove_metadata` *(boolean, optional, default: true)*: Remove document info and XMP metadata.
- `remove_attachments` *(boolean, optional, default: true)*: Remove embedded attachments.
- `remove_scripts` *(boolean, optional, default: true)*: Remove embedded JavaScript and launch actions.

---

### REST API
Execute document sanitization via PaperPilot Gateway:

```bash
# Dedicated Route
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/pdf_sanitize \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/multi_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/sanitized.pdf",
    "remove_metadata": true,
    "remove_attachments": true,
    "remove_scripts": true
  }'
```

```bash
# Alternative Short Route
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/sanitize \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/multi_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/sanitized.pdf"
  }'
```

```bash
# Universal Tool Route
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_sanitize \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/multi_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/sanitized.pdf"
  }'
```

---

### WebAssembly (WASM)
In browser and client runtimes via `@paperpilot/wasm`:

```typescript
import { sanitize } from "@paperpilot/wasm";

const sanitizedPdfBytes = sanitize(
  inputPdfBytes,
  JSON.stringify({
    remove_metadata: true,
    remove_attachments: true,
    remove_scripts: true,
    remove_thumbnails: true,
    remove_private_data: true
  })
);
```

---

### Return Structure & Output Schema
When invoked via MCP or Gateway REST API:

```json
{
  "success": true,
  "message": "PDF sanitized successfully.",
  "output_path": "tests/e2e_fixtures/out/penta_e2e/sanitized.pdf",
  "data": {
    "attachments_removed": 0,
    "metadata_stripped": true,
    "output_bytes": 1680,
    "scripts_removed": 0,
    "thumbnails_removed": 0
  }
}
```

When invoked via CLI:
```text
Sanitization complete:
  Metadata stripped: true
  Attachments removed: 0
  Scripts neutralized: 0
  Thumbnails removed: 0
  Output size: 1680 bytes
```

```json
{
  "success": true,
  "operation": "Sanitize",
  "error": null
}
```

---

### Known Limitations & Troubleshooting
- **Interactive Forms**: Active JavaScript inside form calculations is neutralized during sanitization. If you require interactive AcroForm calculations to remain functional, use [`pdf_fill_form`](/reference/tools/pdf_fill_form/) without running a script-stripping pass.
- **Digital Signatures**: Stripping metadata or objects from an already signed PDF invalidates existing cryptographic signatures. To verify or strip signatures intentionally, use [`pdf_validate_signatures`](/reference/tools/pdf_validate_signatures/) or [`pdf_remove_signatures`](/reference/tools/pdf_remove_signatures/).
