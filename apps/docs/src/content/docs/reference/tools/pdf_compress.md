---
title: "pdf_compress"
description: "Reference, CLI, MCP, REST API, and WASM specifications for pdf_compress."
sidebar:
  order: 13
---

**Status**: ✅ Verified  |  **Format**: PDF (`.pdf`)

### What it does
**Reduces PDF file size through intelligent image resampling and stream compression.**  
Re-encodes embedded high-resolution raster images, applies standard Flate/Zlib compression to uncompressed content streams, and cleans unreferenced objects to produce lightweight, web- and email-friendly documents.

---

### CLI
Optimize PDF document file sizes from the command line:

#### 1. Preset Quality Optimization
```bash
paperpilot compress \
  --input tests/e2e_fixtures/real/single_page.pdf \
  --quality medium \
  --output tests/e2e_fixtures/out/batch3_test/compressed.pdf \
  --json
```

#### 2. Integer Quality Level (1–100)
```bash
paperpilot compress \
  --input tests/e2e_fixtures/real/sample_photo_doc.pdf \
  --quality 50 \
  --output tests/e2e_fixtures/out/batch3_test/compressed_q50.pdf \
  --json
```

#### CLI Parameters
- `--input` *(string, required)*: Path to source PDF document.
- `--quality` *(string, required)*: Compression level preset (`low`, `screen`, `medium`, `ebook`, `high`, `prepress`) or integer JPEG quality from `1` to `100`.
- `--output` *(string, required)*: Path to write the compressed PDF.
- `--json` *(flag, optional)*: Emit execution status in JSON format.

---

### MCP JSON-RPC
Invoke `pdf_compress` through standard Model Context Protocol (MCP) clients:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_compress",
    "arguments": {
      "input": "tests/e2e_fixtures/real/single_page.pdf",
      "quality": "medium",
      "output": "tests/e2e_fixtures/out/batch3_test/mcp_compressed.pdf"
    }
  },
  "id": 1
}
```

#### MCP Output Response
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "content": [
      {
        "type": "text",
        "text": "{\"success\":true,\"message\":\"PDF compressed successfully.\",\"output_path\":\"tests/e2e_fixtures/out/batch3_test/mcp_compressed.pdf\",\"data\":null}"
      }
    ],
    "isError": false
  }
}
```

#### MCP Schema Parameters
- `input` *(string, required)*: The input PDF file path.
- `output` *(string, required)*: The output PDF file path.
- `quality` *(string, optional, default: `"medium"`)*: Compression quality preset (`"low"`, `"medium"`, `"high"`) or integer `1`-`100`.

---

### REST API
Execute PDF compression via PaperPilot Gateway endpoints:

#### Dedicated Endpoint
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/compress \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "quality": "medium",
    "output": "tests/e2e_fixtures/out/batch3_test/api_compressed.pdf"
  }'
```

#### Universal Tool Route
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_compress \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "quality": "50",
    "output": "tests/e2e_fixtures/out/batch3_test/api_compressed.pdf"
  }'
```

---

### WASM (Web & Edge)
Run client-side compression in browser runtimes or Cloudflare Workers via `paperpilot-wasm`:

```typescript
import { WasmPdfEngine } from "paperpilot-wasm";

// Compress PDF buffer in memory
const compressedBytes: Uint8Array = WasmPdfEngine.compress(
  inputBytes,   // Uint8Array of input PDF
  "medium"      // Optional quality string: "low" | "medium" | "high" | "1-100"
);
```

---

### Known Limitations & Troubleshooting
- **Pre-Compressed & Flat Vector Files**: PDFs containing only text vectors and lossless streams already compressed with Flate/JBIG2 may see negligible reduction in file size or minor overhead changes due to dictionary reserialization. For lossless stream deduplication without image recompression, consider `pdf_dedup_streams`.
- **Encrypted Documents**: Compression cannot inspect or recompress object streams in password-protected files. Decrypt the document with `pdf_decrypt` prior to compression.
