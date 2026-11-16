---
title: "pdf_comic_archive"
description: "Reference, CLI, MCP, and REST API commands for pdf_comic_archive."
sidebar:
  order: 57
---

**Status**: ✅ Verified  |  **Format**: PDF (`.pdf`) or Images (`.png`, `.jpg`, `.jpeg`, `.webp`) -> Comic Book Archive (`.cbz`)

### What it does
**Converts PDF documents or folders of images into Comic Book Archive (.cbz) packages.**  
Rasterizes each PDF page into high-fidelity sequential PNG images (`page_001.png`, `page_002.png`, etc.) or packs directories of illustration scans using natural sort order (`natural_cmp`) into a standardized, zero-compression ZIP container compatible with all major comic and e-reader applications.

---

### CLI
Convert PDFs or image directories to comic archives using `paperpilot pdf-comic-archive` (or aliases `comic-archive`, `pdf_comic_archive`):

#### 1. Convert PDF to Comic Book Archive (`.cbz`)
```bash
paperpilot pdf-comic-archive \
  --input tests/e2e_fixtures/real/single_page.pdf \
  --output tests/e2e_fixtures/out/penta_e2e/comic.cbz \
  --json
```

#### 2. Convert Directory of Image Scans to Comic Book Archive
```bash
paperpilot pdf-comic-archive \
  --input tests/e2e_fixtures/real/ \
  --output tests/e2e_fixtures/out/penta_e2e/manga.cbz \
  --json
```

#### CLI Parameters
- `--input` *(string, required)*: Path to source PDF document, single image file, or directory containing scanned images (`.png`, `.jpg`, `.jpeg`, `.webp`).
- `--output` *(string, required)*: Path to write the output comic archive (`.cbz`).
- `--json` *(flag, optional)*: Emit structured execution status in JSON format.

---

### MCP JSON-RPC
Invoke comic archive creation via Model Context Protocol:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_comic_archive",
    "arguments": {
      "input": "tests/e2e_fixtures/real/single_page.pdf",
      "output": "tests/e2e_fixtures/out/penta_e2e/comic.cbz"
    }
  },
  "id": 1
}
```

#### MCP Schema Parameters
- `input` *(string, required)*: Source PDF or image directory path.
- `output` *(string, required)*: Destination comic archive path (`.cbz`).

---

### REST API
Generate comic archives via PaperPilot Gateway:

#### Dedicated Endpoint
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/pdf_comic_archive \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/comic.cbz"
  }'
```

#### Universal Tool Route
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_comic_archive \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/comic.cbz"
  }'
```

---

### WASM Execution
`pdf_comic_archive` is supported directly in client-side WebAssembly via `paperpilot-wasm`:

```typescript
import { WasmPdfEngine } from "paperpilot-wasm";

const engine = new WasmPdfEngine();
const options = JSON.stringify({ dpi: 150.0 });
const cbzBuffer = engine.pdf_comic_archive(pdfBytes, options);
console.log(`Generated CBZ package: ${cbzBuffer.byteLength} bytes`);
```

---

### Known Limitations & Troubleshooting
- **Archive Format Standard**: In PaperPilot, comic archives are assembled using `.cbz` (ZIP container containing standard images). While CBR is an archive alias, modern e-readers and comic readers universally support the open ZIP standard `.cbz`.
- **Directionality**: PDF pages are rendered directly into sequential images. Converting back from `.cbz` to `.pdf` should be done using `images-to-pdf` with the extracted image stream.
- **Natural Ordering**: Directory ingestion employs natural alphanumeric sort order (`chapter1_page2.png` sorts before `chapter1_page10.png`) to avoid page interleaving.
