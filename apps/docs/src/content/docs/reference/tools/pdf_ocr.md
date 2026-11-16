---
title: "pdf_ocr"
description: "Reference, CLI, MCP, REST API, and WASM commands for pdf_ocr."
sidebar:
  order: 33
---

**Status**: ✅ Verified  |  **Format**: PDF / Images $\to$ Searchable PDF (`.pdf`) / Text (`.txt`)

### What it does
**Performs local neural optical character recognition (OCR) on scanned documents and raster images.**  
Embeds an invisible searchable text layer behind scanned page bitmaps or extracts recognized text directly into a plain text file, operating 100% offline without transmitting confidential scans to external cloud APIs.

---

### CLI
Perform OCR via the PaperPilot CLI using `ocr`:

#### Generate Searchable PDF
```bash
paperpilot ocr \
  --input tests/e2e_fixtures/real/single_page.pdf \
  --output tests/e2e_fixtures/out/penta_e2e/ocr_searchable.pdf \
  --format pdf \
  --json
```

#### Extract OCR Text Layer to File
```bash
paperpilot ocr \
  --input tests/e2e_fixtures/real/single_page.pdf \
  --output tests/e2e_fixtures/out/penta_e2e/ocr_text.txt \
  --format text \
  --json
```

#### CLI Parameters
- `--input <INPUT>` *(string, required)*: Path to the source scanned PDF or image document.
- `--output <OUTPUT>` *(string, optional)*: Destination file path for the output searchable PDF or plain text.
- `--format <FORMAT>` *(string, optional, default: `pdf`)*: Output format (`pdf` or `text`).
- `--clipboard` *(flag, optional)*: Copy recognized OCR text directly to the system clipboard.
- `--tier <TIER>` *(string, optional)*: Recognition model complexity tier.
- `--json` *(flag, optional)*: Emit structured JSON execution result.

---

### MCP JSON-RPC
When integrating via Model Context Protocol (MCP), invoke the tool `pdf_ocr`:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_ocr",
    "arguments": {
      "input": "tests/e2e_fixtures/real/single_page.pdf",
      "output": "tests/e2e_fixtures/out/penta_e2e/ocr_searchable.pdf"
    }
  },
  "id": 1
}
```

#### MCP Schema Parameters
- `input` *(string, required)*: The input PDF or image file path.
- `output` *(string, required)*: The output PDF file path.

---

### REST API
Execute OCR operations via PaperPilot Gateway:

#### 1. Dedicated Route (`/api/v1/pdf/pdf_ocr`)
```bash
curl -s --noproxy '*' -X POST http://127.0.0.1:7823/api/v1/pdf/pdf_ocr \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/api_ocr.pdf"
  }'
```

#### 2. Universal Tool Route (`/api/v1/pdf/tools/pdf_ocr`)
```bash
curl -s --noproxy '*' -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_ocr \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/api_ocr.txt"
  }'
```

#### Response Example
```json
{
  "success": true,
  "request_id": "8d04bc28-829f-4d59-846e-7c636def845c",
  "message": "Success",
  "output": {
    "isError": false,
    "resultType": "complete",
    "content": [
      {
        "type": "text",
        "text": "{\"success\":true,\"message\":\"OCR operation completed successfully.\",\"output_path\":\"tests/e2e_fixtures/out/penta_e2e/api_ocr.pdf\",\"data\":{\"output_path\":\"tests/e2e_fixtures/out/penta_e2e/api_ocr.pdf\",\"text\":\"\"}}"
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

// Performs OCR and returns the recognized text string
const recognizedText = WasmPdfEngine.ocr(scannedPdfOrImageBytes);
console.log(recognizedText);
```

---

### Known Limitations & Troubleshooting
- **Input Page Resolution**: For best recognition accuracy on scanned documents, scan at 300 DPI. Extremely low resolution ($<150$ DPI) or heavily skewed camera photos may yield reduced character confidence. For rotated scans, run `pdf_auto_rotate` before OCR.
- **MCP Output Parameter**: The MCP schema for `pdf_ocr` requires the `output` parameter. Ensure a target path is supplied in arguments.
- **Pure Vector Documents**: If a document already contains full vector text streams, `pdf_extract_text` provides instant zero-latency extraction without running neural model inference.
