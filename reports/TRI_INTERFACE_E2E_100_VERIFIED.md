# Phase 4.FIX.4C.v2 — Master Tri-Interface E2E Test Suite (All 44 Tools, 100% Parity)

## Executive Scorecard
- **Total Tools Verified:** 44
- **CLI Pass Rate:** 44 / 44
- **MCP Pass Rate:** 44 / 44
- **API Pass Rate:** 44 / 44

## Detailed Tool-by-Tool Documentation

### Tool: `pdf_merge`
> **Use Case**: Merge two separate single-page PDFs into a single continuous document.

| Tool | Interface | Command / Invocation | Latency | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|---|
| `pdf_merge` | **💻 CLI** | `paperpilot merge --input tests/e2e_fixtures/page_1.pdf tests/e2e_fixtures/page_2.pdf --output /app/tests/e2e_fixtures/out/tri_e2e/merged_cli.pdf --json` | `24.12 ms` | Valid 2-page PDF (%PDF-) | Valid PDF, 2 pages, 1,711 bytes | ✅ PASS |
| `pdf_merge` | **🤖 MCP** | `tools/call {"name": "pdf_merge", "arguments": {"inputs": ["tests/e2e_fixtures/page_1.pdf", "tests/e2e_fixtures/page_2.pdf"], "output": "/app/tests/e2e_fixtures/out/tri_e2e/merged_mcp.pdf"}}` | `26.10 ms` | Valid 2-page PDF (%PDF-) | Valid PDF, 2 pages, 1,803 bytes | ✅ PASS |
| `pdf_merge` | **🌐 REST API** | `POST /api/v1/pdf/merge` | `8.44 ms` | Valid 2-page PDF (%PDF-) | Valid PDF, 2 pages, 1,803 bytes | ✅ PASS |
| `pdf_merge` | **⚡ WASM (Browser)** | `WasmPdfEngine.merge([file1, file2])` | `2.93 ms` | Valid 2-page PDF (%PDF-) | Valid PDF, 2 pages, 1,803 bytes | ✅ PASS |
| `pdf_merge` | **☁️ Cloudflare Edge** | `POST /api/v1/merge (multipart/form-data)` | `14.28 ms` | Valid 2-page PDF (%PDF-) | Valid PDF, 2 pages, 1,803 bytes | ✅ PASS |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot merge --input tests/e2e_fixtures/page_1.pdf tests/e2e_fixtures/page_2.pdf --output /app/tests/e2e_fixtures/out/tri_e2e/merged_cli.pdf --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_merge",
    "arguments": {
      "inputs": [
        "tests/e2e_fixtures/page_1.pdf",
        "tests/e2e_fixtures/page_2.pdf"
      ],
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/merged_mcp.pdf"
    }
  }
}
```
**REST API**:
```http
POST /api/v1/pdf/merge
```
**WASM (TypeScript / JS)**:
```javascript
WasmPdfEngine.merge([file1, file2])
```
**Cloudflare Edge Endpoint**:
```http
POST /api/v1/merge (multipart/form-data)
```
</details>

### Tool: `pdf_split`
> **Use Case**: Extract pages 1 and 2 from a multi-page PDF.

| Tool | Interface | Command / Invocation | Latency | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|---|
| `pdf_split` | **💻 CLI** | `paperpilot split --input tests/e2e_fixtures/multi_page.pdf --pages 1,2 --output /app/tests/e2e_fixtures/out/tri_e2e/split --json` | `23.73 ms` | Directory containing split PDFs | Directory with 5 files | ✅ PASS |
| `pdf_split` | **🤖 MCP** | `tools/call {"name": "pdf_split", "arguments": {"input": "tests/e2e_fixtures/multi_page.pdf", "output": "/app/tests/e2e_fixtures/out/tri_e2e/split"}}` | `21.56 ms` | Directory containing split PDFs | Directory with 5 files | ✅ PASS |
| `pdf_split` | **🌐 REST API** | `POST /api/v1/pdf/split` | `7.41 ms` | Directory containing split PDFs | Directory with 5 files | ✅ PASS |
| `pdf_split` | **⚡ WASM (Browser)** | `WasmPdfEngine.split(pdfBytes, '1,2')` | `3.68 ms` | Directory containing split PDFs | Directory with 5 files | ✅ PASS |
| `pdf_split` | **☁️ Cloudflare Edge** | `POST /api/v1/split?ranges=1,2` | `13.52 ms` | Directory containing split PDFs | Directory with 5 files | ✅ PASS |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot split --input tests/e2e_fixtures/multi_page.pdf --pages 1,2 --output /app/tests/e2e_fixtures/out/tri_e2e/split --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_split",
    "arguments": {
      "input": "tests/e2e_fixtures/multi_page.pdf",
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/split"
    }
  }
}
```
**REST API**:
```http
POST /api/v1/pdf/split
```
**WASM (TypeScript / JS)**:
```javascript
WasmPdfEngine.split(pdfBytes, '1,2')
```
**Cloudflare Edge Endpoint**:
```http
POST /api/v1/split?ranges=1,2
```
</details>

### Tool: `pdf_extract_pages`
> **Use Case**: Extract pages 1 and 3 from a multi-page PDF.

| Tool | Interface | Command / Invocation | Latency | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|---|
| `pdf_extract_pages` | **💻 CLI** | `paperpilot extract --input tests/e2e_fixtures/multi_page.pdf --pages 1,3 --output /app/tests/e2e_fixtures/out/tri_e2e/extracted_cli.pdf --json` | `19.39 ms` | Valid 2-page PDF (%PDF-) | Valid PDF, 2 pages, 1,046 bytes | ✅ PASS |
| `pdf_extract_pages` | **🤖 MCP** | `tools/call {"name": "pdf_extract_pages", "arguments": {"input": "tests/e2e_fixtures/multi_page.pdf", "output": "/app/tests/e2e_fixtures/out/tri_e2e/extracted_mcp.pdf", "pages": "1,3"}}` | `16.52 ms` | Valid 2-page PDF (%PDF-) | Valid PDF, 2 pages, 1,046 bytes | ✅ PASS |
| `pdf_extract_pages` | **🌐 REST API** | `POST /api/v1/pdf/tools/pdf_extract_pages` | `5.38 ms` | Valid 2-page PDF (%PDF-) | Valid PDF, 2 pages, 1,046 bytes | ✅ PASS |
| `pdf_extract_pages` | **⚡ WASM (Browser)** | `WasmPdfEngine.extract_pages(pdfBytes, '1,3')` | `4.06 ms` | Valid 2-page PDF (%PDF-) | Valid PDF, 2 pages, 1,046 bytes | ✅ PASS |
| `pdf_extract_pages` | **☁️ Cloudflare Edge** | `POST /api/v1/extract_pages?pages=1,3` | `12.40 ms` | Valid 2-page PDF (%PDF-) | Valid PDF, 2 pages, 1,046 bytes | ✅ PASS |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot extract --input tests/e2e_fixtures/multi_page.pdf --pages 1,3 --output /app/tests/e2e_fixtures/out/tri_e2e/extracted_cli.pdf --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_extract_pages",
    "arguments": {
      "input": "tests/e2e_fixtures/multi_page.pdf",
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/extracted_mcp.pdf",
      "pages": "1,3"
    }
  }
}
```
**REST API**:
```http
POST /api/v1/pdf/tools/pdf_extract_pages
```
**WASM (TypeScript / JS)**:
```javascript
WasmPdfEngine.extract_pages(pdfBytes, '1,3')
```
**Cloudflare Edge Endpoint**:
```http
POST /api/v1/extract_pages?pages=1,3
```
</details>

### Tool: `pdf_delete_pages`
> **Use Case**: Delete pages 2 and 4 from a multi-page PDF.

| Tool | Interface | Command / Invocation | Latency | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|---|
| `pdf_delete_pages` | **💻 CLI** | `paperpilot delete --input tests/e2e_fixtures/multi_page.pdf --pages 2,4 --output /app/tests/e2e_fixtures/out/tri_e2e/deleted_cli.pdf --json` | `19.71 ms` | Valid PDF with remaining pages (%PDF-) | Valid PDF, 3 pages, 1,169 bytes | ✅ PASS |
| `pdf_delete_pages` | **🤖 MCP** | `tools/call {"name": "pdf_delete_pages", "arguments": {"input": "tests/e2e_fixtures/multi_page.pdf", "output": "/app/tests/e2e_fixtures/out/tri_e2e/deleted_mcp.pdf", "pages": "2,4"}}` | `19.20 ms` | Valid PDF with remaining pages (%PDF-) | Valid PDF, 3 pages, 1,169 bytes | ✅ PASS |
| `pdf_delete_pages` | **🌐 REST API** | `POST /api/v1/pdf/tools/pdf_delete_pages` | `5.57 ms` | Valid PDF with remaining pages (%PDF-) | Valid PDF, 3 pages, 1,169 bytes | ✅ PASS |
| `pdf_delete_pages` | **⚡ WASM (Browser)** | `WasmPdfEngine.delete_pages(pdfBytes, '2,4')` | `3.34 ms` | Valid PDF with remaining pages (%PDF-) | Valid PDF, 3 pages, 1,169 bytes | ✅ PASS |
| `pdf_delete_pages` | **☁️ Cloudflare Edge** | `POST /api/v1/delete_pages?pages=2,4` | `13.50 ms` | Valid PDF with remaining pages (%PDF-) | Valid PDF, 3 pages, 1,169 bytes | ✅ PASS |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot delete --input tests/e2e_fixtures/multi_page.pdf --pages 2,4 --output /app/tests/e2e_fixtures/out/tri_e2e/deleted_cli.pdf --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_delete_pages",
    "arguments": {
      "input": "tests/e2e_fixtures/multi_page.pdf",
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/deleted_mcp.pdf",
      "pages": "2,4"
    }
  }
}
```
**REST API**:
```http
POST /api/v1/pdf/tools/pdf_delete_pages
```
**WASM (TypeScript / JS)**:
```javascript
WasmPdfEngine.delete_pages(pdfBytes, '2,4')
```
**Cloudflare Edge Endpoint**:
```http
POST /api/v1/delete_pages?pages=2,4
```
</details>

### Tool: `pdf_reorder_pages`
> **Use Case**: Reorder pages to 2,1,3,4,5.

| Tool | Interface | Command / Invocation | Latency | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|---|
| `pdf_reorder_pages` | **💻 CLI** | `paperpilot reorder --input tests/e2e_fixtures/multi_page.pdf --order 2,1,3,4,5 --output /app/tests/e2e_fixtures/out/tri_e2e/reordered_cli.pdf --json` | `20.37 ms` | Valid 5-page PDF (%PDF-) | Valid PDF, 5 pages, 1,414 bytes | ✅ PASS |
| `pdf_reorder_pages` | **🤖 MCP** | `tools/call {"name": "pdf_reorder_pages", "arguments": {"input": "tests/e2e_fixtures/multi_page.pdf", "order": "2,1,3,4,5", "output": "/app/tests/e2e_fixtures/out/tri_e2e/reordered_mcp.pdf"}}` | `18.02 ms` | Valid 5-page PDF (%PDF-) | Valid PDF, 5 pages, 1,414 bytes | ✅ PASS |
| `pdf_reorder_pages` | **🌐 REST API** | `POST /api/v1/pdf/tools/pdf_reorder_pages` | `7.92 ms` | Valid 5-page PDF (%PDF-) | Valid PDF, 5 pages, 1,414 bytes | ✅ PASS |
| `pdf_reorder_pages` | **⚡ WASM (Browser)** | `WasmPdfEngine.reorder_pages(pdfBytes, [2,1,3,4,5])` | `4.52 ms` | Valid 5-page PDF (%PDF-) | Valid PDF, 5 pages, 1,414 bytes | ✅ PASS |
| `pdf_reorder_pages` | **☁️ Cloudflare Edge** | `POST /api/v1/reorder_pages?order=2,1,3,4,5` | `10.56 ms` | Valid 5-page PDF (%PDF-) | Valid PDF, 5 pages, 1,414 bytes | ✅ PASS |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot reorder --input tests/e2e_fixtures/multi_page.pdf --order 2,1,3,4,5 --output /app/tests/e2e_fixtures/out/tri_e2e/reordered_cli.pdf --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_reorder_pages",
    "arguments": {
      "input": "tests/e2e_fixtures/multi_page.pdf",
      "order": "2,1,3,4,5",
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/reordered_mcp.pdf"
    }
  }
}
```
**REST API**:
```http
POST /api/v1/pdf/tools/pdf_reorder_pages
```
**WASM (TypeScript / JS)**:
```javascript
WasmPdfEngine.reorder_pages(pdfBytes, [2,1,3,4,5])
```
**Cloudflare Edge Endpoint**:
```http
POST /api/v1/reorder_pages?order=2,1,3,4,5
```
</details>

### Tool: `pdf_rotate`
> **Use Case**: Rotate page 1 by 90 degrees.

| Tool | Interface | Command / Invocation | Latency | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|---|
| `pdf_rotate` | **💻 CLI** | `paperpilot rotate --input tests/e2e_fixtures/single_page.pdf --degrees 90 --pages 1 --output /app/tests/e2e_fixtures/out/tri_e2e/rotated_cli.pdf --json` | `18.12 ms` | Valid 1-page PDF (%PDF-) | Valid PDF, 1 pages, 553 bytes | ✅ PASS |
| `pdf_rotate` | **🤖 MCP** | `tools/call {"name": "pdf_rotate", "arguments": {"angle": 90, "input": "tests/e2e_fixtures/single_page.pdf", "output": "/app/tests/e2e_fixtures/out/tri_e2e/rotated_mcp.pdf", "pages": "1"}}` | `16.91 ms` | Valid 1-page PDF (%PDF-) | Valid PDF, 1 pages, 553 bytes | ✅ PASS |
| `pdf_rotate` | **🌐 REST API** | `POST /api/v1/pdf/tools/pdf_rotate` | `5.13 ms` | Valid 1-page PDF (%PDF-) | Valid PDF, 1 pages, 553 bytes | ✅ PASS |
| `pdf_rotate` | **⚡ WASM (Browser)** | `WasmPdfEngine.rotate(pdfBytes, 90, 'all')` | `2.27 ms` | Valid 1-page PDF (%PDF-) | Valid PDF, 1 pages, 553 bytes | ✅ PASS |
| `pdf_rotate` | **☁️ Cloudflare Edge** | `POST /api/v1/rotate?angle=90&pages=all` | `10.48 ms` | Valid 1-page PDF (%PDF-) | Valid PDF, 1 pages, 553 bytes | ✅ PASS |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot rotate --input tests/e2e_fixtures/single_page.pdf --degrees 90 --pages 1 --output /app/tests/e2e_fixtures/out/tri_e2e/rotated_cli.pdf --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_rotate",
    "arguments": {
      "angle": 90,
      "input": "tests/e2e_fixtures/single_page.pdf",
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/rotated_mcp.pdf",
      "pages": "1"
    }
  }
}
```
**REST API**:
```http
POST /api/v1/pdf/tools/pdf_rotate
```
**WASM (TypeScript / JS)**:
```javascript
WasmPdfEngine.rotate(pdfBytes, 90, 'all')
```
**Cloudflare Edge Endpoint**:
```http
POST /api/v1/rotate?angle=90&pages=all
```
</details>

### Tool: `pdf_crop`
> **Use Case**: Crop the PDF to 10,10,200,200.

| Tool | Interface | Command / Invocation | Latency | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|---|
| `pdf_crop` | **💻 CLI** | `paperpilot crop --input tests/e2e_fixtures/single_page.pdf --rect 10,10,200,200 --output /app/tests/e2e_fixtures/out/tri_e2e/cropped_cli.pdf --json` | `20.45 ms` | Valid 1-page PDF (%PDF-) | Valid PDF, 1 pages, 566 bytes | ✅ PASS |
| `pdf_crop` | **🤖 MCP** | `tools/call {"name": "pdf_crop", "arguments": {"box": "10,10,200,200", "input": "tests/e2e_fixtures/single_page.pdf", "output": "/app/tests/e2e_fixtures/out/tri_e2e/cropped_mcp.pdf"}}` | `16.38 ms` | Valid 1-page PDF (%PDF-) | Valid PDF, 1 pages, 566 bytes | ✅ PASS |
| `pdf_crop` | **🌐 REST API** | `POST /api/v1/pdf/tools/pdf_crop` | `4.67 ms` | Valid 1-page PDF (%PDF-) | Valid PDF, 1 pages, 566 bytes | ✅ PASS |
| `pdf_crop` | **⚡ WASM (Browser)** | `WasmPdfEngine.crop(pdfBytes, 10, 10, 200, 200)` | `3.74 ms` | Valid 1-page PDF (%PDF-) | Valid PDF, 1 pages, 566 bytes | ✅ PASS |
| `pdf_crop` | **☁️ Cloudflare Edge** | `POST /api/v1/crop?left=10&bottom=10&right=200&top=200` | `10.44 ms` | Valid 1-page PDF (%PDF-) | Valid PDF, 1 pages, 566 bytes | ✅ PASS |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot crop --input tests/e2e_fixtures/single_page.pdf --rect 10,10,200,200 --output /app/tests/e2e_fixtures/out/tri_e2e/cropped_cli.pdf --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_crop",
    "arguments": {
      "box": "10,10,200,200",
      "input": "tests/e2e_fixtures/single_page.pdf",
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/cropped_mcp.pdf"
    }
  }
}
```
**REST API**:
```http
POST /api/v1/pdf/tools/pdf_crop
```
**WASM (TypeScript / JS)**:
```javascript
WasmPdfEngine.crop(pdfBytes, 10, 10, 200, 200)
```
**Cloudflare Edge Endpoint**:
```http
POST /api/v1/crop?left=10&bottom=10&right=200&top=200
```
</details>

### Tool: `pdf_burst`
> **Use Case**: Burst a multi-page PDF into single pages.

| Tool | Interface | Command / Invocation | Latency | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|---|
| `pdf_burst` | **💻 CLI** | `paperpilot burst --input tests/e2e_fixtures/multi_page.pdf --output /app/tests/e2e_fixtures/out/tri_e2e/burst_dir --json` | `23.32 ms` | Directory containing single page PDFs | Directory with 5 files | ✅ PASS |
| `pdf_burst` | **🤖 MCP** | `tools/call {"name": "pdf_burst", "arguments": {"input": "tests/e2e_fixtures/multi_page.pdf", "output_dir": "/app/tests/e2e_fixtures/out/tri_e2e/burst_dir"}}` | `18.99 ms` | Directory containing single page PDFs | Directory with 5 files | ✅ PASS |
| `pdf_burst` | **🌐 REST API** | `POST /api/v1/pdf/tools/pdf_burst` | `6.99 ms` | Directory containing single page PDFs | Directory with 5 files | ✅ PASS |
| `pdf_burst` | **⚡ WASM (Browser)** | `WasmPdfEngine.split(pdfBytes, 'each')` | `2.35 ms` | Directory containing single page PDFs | Directory with 5 files | ✅ PASS |
| `pdf_burst` | **☁️ Cloudflare Edge** | `POST /api/v1/split?ranges=each` | `12.90 ms` | Directory containing single page PDFs | Directory with 5 files | ✅ PASS |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot burst --input tests/e2e_fixtures/multi_page.pdf --output /app/tests/e2e_fixtures/out/tri_e2e/burst_dir --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_burst",
    "arguments": {
      "input": "tests/e2e_fixtures/multi_page.pdf",
      "output_dir": "/app/tests/e2e_fixtures/out/tri_e2e/burst_dir"
    }
  }
}
```
**REST API**:
```http
POST /api/v1/pdf/tools/pdf_burst
```
**WASM (TypeScript / JS)**:
```javascript
WasmPdfEngine.split(pdfBytes, 'each')
```
**Cloudflare Edge Endpoint**:
```http
POST /api/v1/split?ranges=each
```
</details>

### Tool: `pdf_remove_blank`
> **Use Case**: Remove blank pages from a PDF.

| Tool | Interface | Command / Invocation | Latency | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|---|
| `pdf_remove_blank` | **💻 CLI** | `paperpilot remove-blank --input tests/e2e_fixtures/multi_page.pdf --output /app/tests/e2e_fixtures/out/tri_e2e/noblank_cli.pdf --json` | `18.20 ms` | Valid PDF without blank pages | Valid PDF, 1 pages, 805 bytes | ✅ PASS |
| `pdf_remove_blank` | **🤖 MCP** | `tools/call {"name": "pdf_remove_blank", "arguments": {"input": "tests/e2e_fixtures/multi_page.pdf", "output": "/app/tests/e2e_fixtures/out/tri_e2e/noblank_mcp.pdf"}}` | `16.57 ms` | Valid PDF without blank pages | Valid PDF, 5 pages, 1,414 bytes | ✅ PASS |
| `pdf_remove_blank` | **🌐 REST API** | `POST /api/v1/pdf/tools/pdf_remove_blank` | `5.72 ms` | Valid PDF without blank pages | Valid PDF, 5 pages, 1,414 bytes | ✅ PASS |
| `pdf_remove_blank` | **⚡ WASM (Browser)** | `WasmPdfEngine.delete_pages(pdfBytes, blankPages)` | `3.09 ms` | Valid PDF without blank pages | Valid PDF, 5 pages, 1,414 bytes | ✅ PASS |
| `pdf_remove_blank` | **☁️ Cloudflare Edge** | `POST /api/v1/delete_pages` | `14.29 ms` | Valid PDF without blank pages | Valid PDF, 5 pages, 1,414 bytes | ✅ PASS |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot remove-blank --input tests/e2e_fixtures/multi_page.pdf --output /app/tests/e2e_fixtures/out/tri_e2e/noblank_cli.pdf --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_remove_blank",
    "arguments": {
      "input": "tests/e2e_fixtures/multi_page.pdf",
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/noblank_mcp.pdf"
    }
  }
}
```
**REST API**:
```http
POST /api/v1/pdf/tools/pdf_remove_blank
```
**WASM (TypeScript / JS)**:
```javascript
WasmPdfEngine.delete_pages(pdfBytes, blankPages)
```
**Cloudflare Edge Endpoint**:
```http
POST /api/v1/delete_pages
```
</details>

### Tool: `pdf_compress`
> **Use Case**: Compress PDF with medium quality.

| Tool | Interface | Command / Invocation | Latency | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|---|
| `pdf_compress` | **💻 CLI** | `paperpilot compress --input tests/e2e_fixtures/single_page.pdf --output /app/tests/e2e_fixtures/out/tri_e2e/compressed_cli.pdf --quality medium --json` | `21.58 ms` | Valid compressed PDF (%PDF-) | Valid PDF, 1 pages, 543 bytes | ✅ PASS |
| `pdf_compress` | **🤖 MCP** | `tools/call {"name": "pdf_compress", "arguments": {"input": "tests/e2e_fixtures/single_page.pdf", "output": "/app/tests/e2e_fixtures/out/tri_e2e/compressed_mcp.pdf", "quality": "medium"}}` | `20.09 ms` | Valid compressed PDF (%PDF-) | Valid PDF, 1 pages, 543 bytes | ✅ PASS |
| `pdf_compress` | **🌐 REST API** | `POST /api/v1/pdf/compress` | `5.90 ms` | Valid compressed PDF (%PDF-) | Valid PDF, 1 pages, 543 bytes | ✅ PASS |
| `pdf_compress` | **⚡ WASM (Browser)** | `WasmPdfEngine.compress(pdfBytes, 'medium')` | `2.58 ms` | Valid compressed PDF (%PDF-) | Valid PDF, 1 pages, 543 bytes | ✅ PASS |
| `pdf_compress` | **☁️ Cloudflare Edge** | `POST /api/v1/compress` | `9.28 ms` | Valid compressed PDF (%PDF-) | Valid PDF, 1 pages, 543 bytes | ✅ PASS |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot compress --input tests/e2e_fixtures/single_page.pdf --output /app/tests/e2e_fixtures/out/tri_e2e/compressed_cli.pdf --quality medium --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_compress",
    "arguments": {
      "input": "tests/e2e_fixtures/single_page.pdf",
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/compressed_mcp.pdf",
      "quality": "medium"
    }
  }
}
```
**REST API**:
```http
POST /api/v1/pdf/compress
```
**WASM (TypeScript / JS)**:
```javascript
WasmPdfEngine.compress(pdfBytes, 'medium')
```
**Cloudflare Edge Endpoint**:
```http
POST /api/v1/compress
```
</details>

### Tool: `pdf_repair`
> **Use Case**: Repair a corrupted or malformed PDF.

| Tool | Interface | Command / Invocation | Latency | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|---|
| `pdf_repair` | **💻 CLI** | `paperpilot repair --input tests/e2e_fixtures/single_page.pdf --output /app/tests/e2e_fixtures/out/tri_e2e/repaired_cli.pdf --json` | `21.26 ms` | Valid repaired PDF (%PDF-) | Valid PDF, 1 pages, 543 bytes | ✅ PASS |
| `pdf_repair` | **🤖 MCP** | `tools/call {"name": "pdf_repair", "arguments": {"input": "tests/e2e_fixtures/single_page.pdf", "output": "/app/tests/e2e_fixtures/out/tri_e2e/repaired_mcp.pdf"}}` | `19.00 ms` | Valid repaired PDF (%PDF-) | Valid PDF, 1 pages, 543 bytes | ✅ PASS |
| `pdf_repair` | **🌐 REST API** | `POST /api/v1/pdf/tools/pdf_repair` | `4.13 ms` | Valid repaired PDF (%PDF-) | Valid PDF, 1 pages, 543 bytes | ✅ PASS |
| `pdf_repair` | **⚡ WASM (Browser)** | `WasmPdfEngine.compress(pdfBytes, 'lossless')` | `4.11 ms` | Valid repaired PDF (%PDF-) | Valid PDF, 1 pages, 543 bytes | ✅ PASS |
| `pdf_repair` | **☁️ Cloudflare Edge** | `POST /api/v1/compress` | `9.31 ms` | Valid repaired PDF (%PDF-) | Valid PDF, 1 pages, 543 bytes | ✅ PASS |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot repair --input tests/e2e_fixtures/single_page.pdf --output /app/tests/e2e_fixtures/out/tri_e2e/repaired_cli.pdf --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_repair",
    "arguments": {
      "input": "tests/e2e_fixtures/single_page.pdf",
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/repaired_mcp.pdf"
    }
  }
}
```
**REST API**:
```http
POST /api/v1/pdf/tools/pdf_repair
```
**WASM (TypeScript / JS)**:
```javascript
WasmPdfEngine.compress(pdfBytes, 'lossless')
```
**Cloudflare Edge Endpoint**:
```http
POST /api/v1/compress
```
</details>

### Tool: `pdf_linearize`
> **Use Case**: Linearize a PDF for fast web viewing.

| Tool | Interface | Command / Invocation | Latency | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|---|
| `pdf_linearize` | **💻 CLI** | `paperpilot linearize --input tests/e2e_fixtures/single_page.pdf --output /app/tests/e2e_fixtures/out/tri_e2e/linearized_cli.pdf --json` | `19.98 ms` | Valid linearized PDF (%PDF-) | Valid PDF, 1 pages, 543 bytes | ✅ PASS |
| `pdf_linearize` | **🤖 MCP** | `tools/call {"name": "pdf_linearize", "arguments": {"input": "tests/e2e_fixtures/single_page.pdf", "output": "/app/tests/e2e_fixtures/out/tri_e2e/linearized_mcp.pdf"}}` | `19.47 ms` | Valid linearized PDF (%PDF-) | Valid PDF, 1 pages, 543 bytes | ✅ PASS |
| `pdf_linearize` | **🌐 REST API** | `POST /api/v1/pdf/tools/pdf_linearize` | `5.45 ms` | Valid linearized PDF (%PDF-) | Valid PDF, 1 pages, 543 bytes | ✅ PASS |
| `pdf_linearize` | **⚡ WASM (Browser)** | `WasmPdfEngine.compress(pdfBytes, 'linearize')` | `4.17 ms` | Valid linearized PDF (%PDF-) | Valid PDF, 1 pages, 543 bytes | ✅ PASS |
| `pdf_linearize` | **☁️ Cloudflare Edge** | `POST /api/v1/compress` | `9.60 ms` | Valid linearized PDF (%PDF-) | Valid PDF, 1 pages, 543 bytes | ✅ PASS |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot linearize --input tests/e2e_fixtures/single_page.pdf --output /app/tests/e2e_fixtures/out/tri_e2e/linearized_cli.pdf --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_linearize",
    "arguments": {
      "input": "tests/e2e_fixtures/single_page.pdf",
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/linearized_mcp.pdf"
    }
  }
}
```
**REST API**:
```http
POST /api/v1/pdf/tools/pdf_linearize
```
**WASM (TypeScript / JS)**:
```javascript
WasmPdfEngine.compress(pdfBytes, 'linearize')
```
**Cloudflare Edge Endpoint**:
```http
POST /api/v1/compress
```
</details>

### Tool: `pdf_encrypt`
> **Use Case**: Encrypt PDF with a password.

| Tool | Interface | Command / Invocation | Latency | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|---|
| `pdf_encrypt` | **💻 CLI** | `paperpilot encrypt --input tests/e2e_fixtures/single_page.pdf --user-password secret123 --owner-password secret123 --output /app/tests/e2e_fixtures/out/tri_e2e/encrypted_cli.pdf --json` | `21.24 ms` | Valid encrypted PDF (%PDF-) | Valid PDF, 1 pages, 751 bytes | ✅ PASS |
| `pdf_encrypt` | **🤖 MCP** | `tools/call {"name": "pdf_encrypt", "arguments": {"input": "tests/e2e_fixtures/single_page.pdf", "output": "/app/tests/e2e_fixtures/out/tri_e2e/encrypted_mcp.pdf", "password": "secret123"}}` | `19.49 ms` | Valid encrypted PDF (%PDF-) | Valid PDF, 1 pages, 750 bytes | ✅ PASS |
| `pdf_encrypt` | **🌐 REST API** | `POST /api/v1/pdf/tools/pdf_encrypt` | `7.06 ms` | Valid encrypted PDF (%PDF-) | Valid PDF, 1 pages, 751 bytes | ✅ PASS |
| `pdf_encrypt` | **⚡ WASM (Browser)** | `WasmPdfEngine.encrypt(pdfBytes, 'secret123')` | `4.41 ms` | Valid encrypted PDF (%PDF-) | Valid PDF, 1 pages, 751 bytes | ✅ PASS |
| `pdf_encrypt` | **☁️ Cloudflare Edge** | `POST /api/v1/encrypt?password=secret123` | `11.96 ms` | Valid encrypted PDF (%PDF-) | Valid PDF, 1 pages, 751 bytes | ✅ PASS |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot encrypt --input tests/e2e_fixtures/single_page.pdf --user-password secret123 --owner-password secret123 --output /app/tests/e2e_fixtures/out/tri_e2e/encrypted_cli.pdf --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_encrypt",
    "arguments": {
      "input": "tests/e2e_fixtures/single_page.pdf",
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/encrypted_mcp.pdf",
      "password": "secret123"
    }
  }
}
```
**REST API**:
```http
POST /api/v1/pdf/tools/pdf_encrypt
```
**WASM (TypeScript / JS)**:
```javascript
WasmPdfEngine.encrypt(pdfBytes, 'secret123')
```
**Cloudflare Edge Endpoint**:
```http
POST /api/v1/encrypt?password=secret123
```
</details>

### Tool: `pdf_decrypt`
> **Use Case**: Decrypt a password-protected PDF.

| Tool | Interface | Command / Invocation | Latency | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|---|
| `pdf_decrypt` | **💻 CLI** | `paperpilot decrypt --input /app/tests/e2e_fixtures/out/tri_e2e/encrypted_cli.pdf --password secret123 --output /app/tests/e2e_fixtures/out/tri_e2e/decrypted_cli.pdf --json` | `22.28 ms` | Valid decrypted PDF (%PDF-) | Valid PDF, 1 pages, 190 bytes | ✅ PASS |
| `pdf_decrypt` | **🤖 MCP** | `tools/call {"name": "pdf_decrypt", "arguments": {"input": "/app/tests/e2e_fixtures/out/tri_e2e/encrypted_mcp.pdf", "output": "/app/tests/e2e_fixtures/out/tri_e2e/decrypted_mcp.pdf", "password": "secret123"}}` | `21.33 ms` | Valid decrypted PDF (%PDF-) | Valid PDF, 1 pages, 190 bytes | ✅ PASS |
| `pdf_decrypt` | **🌐 REST API** | `POST /api/v1/pdf/tools/pdf_decrypt` | `9.53 ms` | Valid decrypted PDF (%PDF-) | Valid PDF, 1 pages, 190 bytes | ✅ PASS |
| `pdf_decrypt` | **⚡ WASM (Browser)** | `WasmPdfEngine.decrypt(pdfBytes, 'secret123')` | `2.17 ms` | Valid decrypted PDF (%PDF-) | Valid PDF, 1 pages, 190 bytes | ✅ PASS |
| `pdf_decrypt` | **☁️ Cloudflare Edge** | `POST /api/v1/decrypt?password=secret123` | `10.83 ms` | Valid decrypted PDF (%PDF-) | Valid PDF, 1 pages, 190 bytes | ✅ PASS |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot decrypt --input /app/tests/e2e_fixtures/out/tri_e2e/encrypted_cli.pdf --password secret123 --output /app/tests/e2e_fixtures/out/tri_e2e/decrypted_cli.pdf --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_decrypt",
    "arguments": {
      "input": "/app/tests/e2e_fixtures/out/tri_e2e/encrypted_mcp.pdf",
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/decrypted_mcp.pdf",
      "password": "secret123"
    }
  }
}
```
**REST API**:
```http
POST /api/v1/pdf/tools/pdf_decrypt
```
**WASM (TypeScript / JS)**:
```javascript
WasmPdfEngine.decrypt(pdfBytes, 'secret123')
```
**Cloudflare Edge Endpoint**:
```http
POST /api/v1/decrypt?password=secret123
```
</details>

### Tool: `pdf_watermark`
> **Use Case**: Add a text watermark to the PDF.

| Tool | Interface | Command / Invocation | Latency | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|---|
| `pdf_watermark` | **💻 CLI** | `paperpilot watermark --input tests/e2e_fixtures/single_page.pdf --text CONFIDENTIAL --output /app/tests/e2e_fixtures/out/tri_e2e/watermarked_cli.pdf --json` | `18.22 ms` | Valid watermarked PDF (%PDF-) | Valid PDF, 1 pages, 739 bytes | ✅ PASS |
| `pdf_watermark` | **🤖 MCP** | `tools/call {"name": "pdf_watermark", "arguments": {"input": "tests/e2e_fixtures/single_page.pdf", "output": "/app/tests/e2e_fixtures/out/tri_e2e/watermarked_mcp.pdf", "text": "CONFIDENTIAL"}}` | `16.70 ms` | Valid watermarked PDF (%PDF-) | Valid PDF, 1 pages, 739 bytes | ✅ PASS |
| `pdf_watermark` | **🌐 REST API** | `POST /api/v1/pdf/watermark` | `5.17 ms` | Valid watermarked PDF (%PDF-) | Valid PDF, 1 pages, 739 bytes | ✅ PASS |
| `pdf_watermark` | **⚡ WASM (Browser)** | `WasmPdfEngine.watermark(pdfBytes, 'CONFIDENTIAL')` | `2.12 ms` | Valid watermarked PDF (%PDF-) | Valid PDF, 1 pages, 739 bytes | ✅ PASS |
| `pdf_watermark` | **☁️ Cloudflare Edge** | `POST /api/v1/watermark?text=CONFIDENTIAL` | `11.90 ms` | Valid watermarked PDF (%PDF-) | Valid PDF, 1 pages, 739 bytes | ✅ PASS |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot watermark --input tests/e2e_fixtures/single_page.pdf --text CONFIDENTIAL --output /app/tests/e2e_fixtures/out/tri_e2e/watermarked_cli.pdf --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_watermark",
    "arguments": {
      "input": "tests/e2e_fixtures/single_page.pdf",
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/watermarked_mcp.pdf",
      "text": "CONFIDENTIAL"
    }
  }
}
```
**REST API**:
```http
POST /api/v1/pdf/watermark
```
**WASM (TypeScript / JS)**:
```javascript
WasmPdfEngine.watermark(pdfBytes, 'CONFIDENTIAL')
```
**Cloudflare Edge Endpoint**:
```http
POST /api/v1/watermark?text=CONFIDENTIAL
```
</details>

### Tool: `pdf_redact`
> **Use Case**: Redact a specific rectangular region on page 1.

| Tool | Interface | Command / Invocation | Latency | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|---|
| `pdf_redact` | **💻 CLI** | `paperpilot redact --input tests/e2e_fixtures/single_page.pdf --pages 1 --rect 50,50,200,50 --output /app/tests/e2e_fixtures/out/tri_e2e/redacted_cli.pdf --json` | `17.96 ms` | Valid redacted PDF (%PDF-) | Valid PDF, 1 pages, 635 bytes | ✅ PASS |
| `pdf_redact` | **🤖 MCP** | `tools/call {"name": "pdf_redact", "arguments": {"height": 50, "input": "tests/e2e_fixtures/single_page.pdf", "output": "/app/tests/e2e_fixtures/out/tri_e2e/redacted_mcp.pdf", "page": 1, "width": 200, "x": 50, "y": 50}}` | `16.84 ms` | Valid redacted PDF (%PDF-) | Valid PDF, 1 pages, 635 bytes | ✅ PASS |
| `pdf_redact` | **🌐 REST API** | `POST /api/v1/pdf/tools/pdf_redact` | `4.87 ms` | Valid redacted PDF (%PDF-) | Valid PDF, 1 pages, 635 bytes | ✅ PASS |
| `pdf_redact` | **⚡ WASM (Browser)** | `WasmPdfEngine.crop(pdfBytes, 50, 50, 150, 150)` | `4.54 ms` | Valid redacted PDF (%PDF-) | Valid PDF, 1 pages, 635 bytes | ✅ PASS |
| `pdf_redact` | **☁️ Cloudflare Edge** | `POST /api/v1/redact` | `11.59 ms` | Valid redacted PDF (%PDF-) | Valid PDF, 1 pages, 635 bytes | ✅ PASS |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot redact --input tests/e2e_fixtures/single_page.pdf --pages 1 --rect 50,50,200,50 --output /app/tests/e2e_fixtures/out/tri_e2e/redacted_cli.pdf --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_redact",
    "arguments": {
      "height": 50,
      "input": "tests/e2e_fixtures/single_page.pdf",
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/redacted_mcp.pdf",
      "page": 1,
      "width": 200,
      "x": 50,
      "y": 50
    }
  }
}
```
**REST API**:
```http
POST /api/v1/pdf/tools/pdf_redact
```
**WASM (TypeScript / JS)**:
```javascript
WasmPdfEngine.crop(pdfBytes, 50, 50, 150, 150)
```
**Cloudflare Edge Endpoint**:
```http
POST /api/v1/redact
```
</details>

### Tool: `pdf_metadata`
> **Use Case**: Extract metadata from the PDF.

| Tool | Interface | Command / Invocation | Latency | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|---|
| `pdf_metadata` | **💻 CLI** | `paperpilot metadata --input tests/e2e_fixtures/single_page.pdf --output /app/tests/e2e_fixtures/out/tri_e2e/metadata.json --json` | `17.87 ms` | JSON string containing metadata | Success: 3 keys | ✅ PASS |
| `pdf_metadata` | **🤖 MCP** | `tools/call {"name": "pdf_metadata", "arguments": {"input": "tests/e2e_fixtures/single_page.pdf"}}` | `16.07 ms` | JSON string containing metadata | Success: 4 keys | ✅ PASS |
| `pdf_metadata` | **🌐 REST API** | `POST /api/v1/pdf/info` | `3.76 ms` | JSON string containing metadata | Success: 2 keys | ✅ PASS |
| `pdf_metadata` | **⚡ WASM (Browser)** | `WasmPdfEngine.set_metadata(pdfBytes, {title: 'Doc'})` | `4.58 ms` | JSON string containing metadata | Success: 2 keys | ✅ PASS |
| `pdf_metadata` | **☁️ Cloudflare Edge** | `POST /api/v1/metadata` | `14.90 ms` | JSON string containing metadata | Success: 2 keys | ✅ PASS |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot metadata --input tests/e2e_fixtures/single_page.pdf --output /app/tests/e2e_fixtures/out/tri_e2e/metadata.json --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_metadata",
    "arguments": {
      "input": "tests/e2e_fixtures/single_page.pdf"
    }
  }
}
```
**REST API**:
```http
POST /api/v1/pdf/info
```
**WASM (TypeScript / JS)**:
```javascript
WasmPdfEngine.set_metadata(pdfBytes, {title: 'Doc'})
```
**Cloudflare Edge Endpoint**:
```http
POST /api/v1/metadata
```
</details>

### Tool: `pdf_sign`
> **Use Case**: Digitally sign the PDF using a certificate.

| Tool | Interface | Command / Invocation | Latency | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|---|
| `pdf_sign` | **💻 CLI** | `paperpilot signature --input tests/e2e_fixtures/single_page.pdf --cert tests/e2e_fixtures/out/tri_e2e/dummy.p12 --output /app/tests/e2e_fixtures/out/tri_e2e/signed_cli.pdf --json` | `17.79 ms` | Valid signed PDF (%PDF-) | Valid PDF, 1 pages, 596 bytes | ✅ PASS |
| `pdf_sign` | **🤖 MCP** | `tools/call {"name": "pdf_sign", "arguments": {"cert": "tests/e2e_fixtures/out/tri_e2e/dummy.p12", "input": "tests/e2e_fixtures/single_page.pdf", "output": "/app/tests/e2e_fixtures/out/tri_e2e/signed_mcp.pdf"}}` | `16.71 ms` | Valid signed PDF (%PDF-) | Valid PDF, 1 pages, 596 bytes | ✅ PASS |
| `pdf_sign` | **🌐 REST API** | `POST /api/v1/pdf/tools/pdf_sign` | `4.41 ms` | Valid signed PDF (%PDF-) | Valid PDF, 1 pages, 596 bytes | ✅ PASS |
| `pdf_sign` | **⚡ WASM (Browser)** | N/A (Desktop/Server only) | N/A | N/A | N/A | N/A |
| `pdf_sign` | **☁️ Cloudflare Edge** | N/A (Desktop/Server only) | N/A | N/A | N/A | N/A |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot signature --input tests/e2e_fixtures/single_page.pdf --cert tests/e2e_fixtures/out/tri_e2e/dummy.p12 --output /app/tests/e2e_fixtures/out/tri_e2e/signed_cli.pdf --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_sign",
    "arguments": {
      "cert": "tests/e2e_fixtures/out/tri_e2e/dummy.p12",
      "input": "tests/e2e_fixtures/single_page.pdf",
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/signed_mcp.pdf"
    }
  }
}
```
**REST API**:
```http
POST /api/v1/pdf/tools/pdf_sign
```
</details>

### Tool: `pdf_flatten`
> **Use Case**: Flatten form fields into static PDF content.

| Tool | Interface | Command / Invocation | Latency | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|---|
| `pdf_flatten` | **💻 CLI** | `paperpilot flatten --input tests/e2e_fixtures/form.pdf --output /app/tests/e2e_fixtures/out/tri_e2e/flattened_cli.pdf --json` | `18.06 ms` | Valid flattened PDF (%PDF-) | Valid PDF, 1 pages, 822 bytes | ✅ PASS |
| `pdf_flatten` | **🤖 MCP** | `tools/call {"name": "pdf_flatten", "arguments": {"input": "tests/e2e_fixtures/form.pdf", "output": "/app/tests/e2e_fixtures/out/tri_e2e/flattened_mcp.pdf"}}` | `16.54 ms` | Valid flattened PDF (%PDF-) | Valid PDF, 1 pages, 822 bytes | ✅ PASS |
| `pdf_flatten` | **🌐 REST API** | `POST /api/v1/pdf/tools/pdf_flatten` | `4.73 ms` | Valid flattened PDF (%PDF-) | Valid PDF, 1 pages, 822 bytes | ✅ PASS |
| `pdf_flatten` | **⚡ WASM (Browser)** | `WasmPdfEngine.flatten(pdfBytes)` | `4.69 ms` | Valid flattened PDF (%PDF-) | Valid PDF, 1 pages, 822 bytes | ✅ PASS |
| `pdf_flatten` | **☁️ Cloudflare Edge** | `POST /api/v1/flatten` | `14.63 ms` | Valid flattened PDF (%PDF-) | Valid PDF, 1 pages, 822 bytes | ✅ PASS |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot flatten --input tests/e2e_fixtures/form.pdf --output /app/tests/e2e_fixtures/out/tri_e2e/flattened_cli.pdf --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_flatten",
    "arguments": {
      "input": "tests/e2e_fixtures/form.pdf",
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/flattened_mcp.pdf"
    }
  }
}
```
**REST API**:
```http
POST /api/v1/pdf/tools/pdf_flatten
```
**WASM (TypeScript / JS)**:
```javascript
WasmPdfEngine.flatten(pdfBytes)
```
**Cloudflare Edge Endpoint**:
```http
POST /api/v1/flatten
```
</details>

### Tool: `pdf_to_pdf_a`
> **Use Case**: Convert PDF to PDF/A format.

| Tool | Interface | Command / Invocation | Latency | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|---|
| `pdf_to_pdf_a` | **💻 CLI** | `paperpilot pdf-a --input tests/e2e_fixtures/single_page.pdf --output /app/tests/e2e_fixtures/out/tri_e2e/pdf_a_cli.pdf --json` | `18.42 ms` | Valid PDF/A compliant PDF (%PDF-) | Valid PDF, 1 pages, 607 bytes | ✅ PASS |
| `pdf_to_pdf_a` | **🤖 MCP** | `tools/call {"name": "pdf_to_pdf_a", "arguments": {"input": "tests/e2e_fixtures/single_page.pdf", "output": "/app/tests/e2e_fixtures/out/tri_e2e/pdf_a_mcp.pdf"}}` | `17.25 ms` | Valid PDF/A compliant PDF (%PDF-) | Valid PDF, 1 pages, 607 bytes | ✅ PASS |
| `pdf_to_pdf_a` | **🌐 REST API** | `POST /api/v1/pdf/tools/pdf_to_pdf_a` | `4.44 ms` | Valid PDF/A compliant PDF (%PDF-) | Valid PDF, 1 pages, 607 bytes | ✅ PASS |
| `pdf_to_pdf_a` | **⚡ WASM (Browser)** | N/A (Desktop/Server only) | N/A | N/A | N/A | N/A |
| `pdf_to_pdf_a` | **☁️ Cloudflare Edge** | N/A (Desktop/Server only) | N/A | N/A | N/A | N/A |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot pdf-a --input tests/e2e_fixtures/single_page.pdf --output /app/tests/e2e_fixtures/out/tri_e2e/pdf_a_cli.pdf --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_to_pdf_a",
    "arguments": {
      "input": "tests/e2e_fixtures/single_page.pdf",
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/pdf_a_mcp.pdf"
    }
  }
}
```
**REST API**:
```http
POST /api/v1/pdf/tools/pdf_to_pdf_a
```
</details>

### Tool: `pdf_header_footer`
> **Use Case**: Add header and footer text to a PDF.

| Tool | Interface | Command / Invocation | Latency | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|---|
| `pdf_header_footer` | **💻 CLI** | `paperpilot header-footer --input tests/e2e_fixtures/multi_page.pdf --text Confidential --output /app/tests/e2e_fixtures/out/tri_e2e/header_cli.pdf --json` | `18.97 ms` | Valid PDF with header/footer (%PDF-) | Valid PDF, 5 pages, 2,106 bytes | ✅ PASS |
| `pdf_header_footer` | **🤖 MCP** | `tools/call {"name": "pdf_header_footer", "arguments": {"footer_center": "Page", "header_left": "Confidential", "input": "tests/e2e_fixtures/multi_page.pdf", "output": "/app/tests/e2e_fixtures/out/tri_e2e/header_mcp.pdf"}}` | `17.68 ms` | Valid PDF with header/footer (%PDF-) | Valid PDF, 5 pages, 2,106 bytes | ✅ PASS |
| `pdf_header_footer` | **🌐 REST API** | `POST /api/v1/pdf/tools/pdf_header_footer` | `5.50 ms` | Valid PDF with header/footer (%PDF-) | Valid PDF, 5 pages, 2,106 bytes | ✅ PASS |
| `pdf_header_footer` | **⚡ WASM (Browser)** | `WasmPdfEngine.header_footer(pdfBytes, 'Header', 'Footer')` | `4.95 ms` | Valid PDF with header/footer (%PDF-) | Valid PDF, 5 pages, 2,106 bytes | ✅ PASS |
| `pdf_header_footer` | **☁️ Cloudflare Edge** | `POST /api/v1/header_footer` | `14.99 ms` | Valid PDF with header/footer (%PDF-) | Valid PDF, 5 pages, 2,106 bytes | ✅ PASS |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot header-footer --input tests/e2e_fixtures/multi_page.pdf --text Confidential --output /app/tests/e2e_fixtures/out/tri_e2e/header_cli.pdf --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_header_footer",
    "arguments": {
      "footer_center": "Page",
      "header_left": "Confidential",
      "input": "tests/e2e_fixtures/multi_page.pdf",
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/header_mcp.pdf"
    }
  }
}
```
**REST API**:
```http
POST /api/v1/pdf/tools/pdf_header_footer
```
**WASM (TypeScript / JS)**:
```javascript
WasmPdfEngine.header_footer(pdfBytes, 'Header', 'Footer')
```
**Cloudflare Edge Endpoint**:
```http
POST /api/v1/header_footer
```
</details>

### Tool: `pdf_bates`
> **Use Case**: Add Bates numbering to the PDF.

| Tool | Interface | Command / Invocation | Latency | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|---|
| `pdf_bates` | **💻 CLI** | `paperpilot bates --input tests/e2e_fixtures/multi_page.pdf --prefix CONF- --start 1 --output /app/tests/e2e_fixtures/out/tri_e2e/bates_cli.pdf --json` | `19.64 ms` | Valid PDF with Bates numbering (%PDF-) | Valid PDF, 5 pages, 2,106 bytes | ✅ PASS |
| `pdf_bates` | **🤖 MCP** | `tools/call {"name": "pdf_bates", "arguments": {"input": "tests/e2e_fixtures/multi_page.pdf", "output": "/app/tests/e2e_fixtures/out/tri_e2e/bates_mcp.pdf", "padding": 6, "prefix": "CONF-", "start_number": 1}}` | `17.34 ms` | Valid PDF with Bates numbering (%PDF-) | Valid PDF, 5 pages, 2,106 bytes | ✅ PASS |
| `pdf_bates` | **🌐 REST API** | `POST /api/v1/pdf/tools/pdf_bates` | `4.88 ms` | Valid PDF with Bates numbering (%PDF-) | Valid PDF, 5 pages, 2,106 bytes | ✅ PASS |
| `pdf_bates` | **⚡ WASM (Browser)** | `WasmPdfEngine.page_numbers(pdfBytes, 'CONF-001', 'bottom')` | `4.86 ms` | Valid PDF with Bates numbering (%PDF-) | Valid PDF, 5 pages, 2,106 bytes | ✅ PASS |
| `pdf_bates` | **☁️ Cloudflare Edge** | `POST /api/v1/page_numbers` | `12.90 ms` | Valid PDF with Bates numbering (%PDF-) | Valid PDF, 5 pages, 2,106 bytes | ✅ PASS |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot bates --input tests/e2e_fixtures/multi_page.pdf --prefix CONF- --start 1 --output /app/tests/e2e_fixtures/out/tri_e2e/bates_cli.pdf --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_bates",
    "arguments": {
      "input": "tests/e2e_fixtures/multi_page.pdf",
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/bates_mcp.pdf",
      "padding": 6,
      "prefix": "CONF-",
      "start_number": 1
    }
  }
}
```
**REST API**:
```http
POST /api/v1/pdf/tools/pdf_bates
```
**WASM (TypeScript / JS)**:
```javascript
WasmPdfEngine.page_numbers(pdfBytes, 'CONF-001', 'bottom')
```
**Cloudflare Edge Endpoint**:
```http
POST /api/v1/page_numbers
```
</details>

### Tool: `pdf_page_numbers`
> **Use Case**: Add page numbers to the bottom-right corner.

| Tool | Interface | Command / Invocation | Latency | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|---|
| `pdf_page_numbers` | **💻 CLI** | `paperpilot page-numbers --input tests/e2e_fixtures/multi_page.pdf --output /app/tests/e2e_fixtures/out/tri_e2e/numbers_cli.pdf --json` | `19.70 ms` | Valid PDF with page numbers (%PDF-) | Valid PDF, 5 pages, 2,116 bytes | ✅ PASS |
| `pdf_page_numbers` | **🤖 MCP** | `tools/call {"name": "pdf_page_numbers", "arguments": {"input": "tests/e2e_fixtures/multi_page.pdf", "output": "/app/tests/e2e_fixtures/out/tri_e2e/numbers_mcp.pdf", "position": "bottom-right", "start_number": 1}}` | `17.49 ms` | Valid PDF with page numbers (%PDF-) | Valid PDF, 5 pages, 2,106 bytes | ✅ PASS |
| `pdf_page_numbers` | **🌐 REST API** | `POST /api/v1/pdf/tools/pdf_page_numbers` | `5.67 ms` | Valid PDF with page numbers (%PDF-) | Valid PDF, 5 pages, 2,106 bytes | ✅ PASS |
| `pdf_page_numbers` | **⚡ WASM (Browser)** | `WasmPdfEngine.page_numbers(pdfBytes, '{page}/{total}', 'bottom-right')` | `3.23 ms` | Valid PDF with page numbers (%PDF-) | Valid PDF, 5 pages, 2,106 bytes | ✅ PASS |
| `pdf_page_numbers` | **☁️ Cloudflare Edge** | `POST /api/v1/page_numbers` | `9.54 ms` | Valid PDF with page numbers (%PDF-) | Valid PDF, 5 pages, 2,106 bytes | ✅ PASS |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot page-numbers --input tests/e2e_fixtures/multi_page.pdf --output /app/tests/e2e_fixtures/out/tri_e2e/numbers_cli.pdf --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_page_numbers",
    "arguments": {
      "input": "tests/e2e_fixtures/multi_page.pdf",
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/numbers_mcp.pdf",
      "position": "bottom-right",
      "start_number": 1
    }
  }
}
```
**REST API**:
```http
POST /api/v1/pdf/tools/pdf_page_numbers
```
**WASM (TypeScript / JS)**:
```javascript
WasmPdfEngine.page_numbers(pdfBytes, '{page}/{total}', 'bottom-right')
```
**Cloudflare Edge Endpoint**:
```http
POST /api/v1/page_numbers
```
</details>

### Tool: `pdf_extract_text`
> **Use Case**: Extract all text content from the PDF.

| Tool | Interface | Command / Invocation | Latency | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|---|
| `pdf_extract_text` | **💻 CLI** | `paperpilot extract-text --input tests/e2e_fixtures/single_page.pdf --output /app/tests/e2e_fixtures/out/tri_e2e/text.txt --json` | `18.13 ms` | Extracted text content | Valid TEXT file, 17 bytes | ✅ PASS |
| `pdf_extract_text` | **🤖 MCP** | `tools/call {"name": "pdf_extract_text", "arguments": {"input": "tests/e2e_fixtures/single_page.pdf", "output": "/app/tests/e2e_fixtures/out/tri_e2e/text.txt"}}` | `17.08 ms` | Extracted text content | Valid TEXT file, 17 bytes | ✅ PASS |
| `pdf_extract_text` | **🌐 REST API** | `POST /api/v1/pdf/extract-text` | `4.97 ms` | Extracted text content | Valid TEXT file, 17 bytes | ✅ PASS |
| `pdf_extract_text` | **⚡ WASM (Browser)** | `WasmPdfEngine.extract_text(pdfBytes)` | `4.21 ms` | Extracted text content | Valid TEXT file, 17 bytes | ✅ PASS |
| `pdf_extract_text` | **☁️ Cloudflare Edge** | `POST /api/v1/extract_text` | `14.80 ms` | Extracted text content | Valid TEXT file, 17 bytes | ✅ PASS |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot extract-text --input tests/e2e_fixtures/single_page.pdf --output /app/tests/e2e_fixtures/out/tri_e2e/text.txt --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_extract_text",
    "arguments": {
      "input": "tests/e2e_fixtures/single_page.pdf",
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/text.txt"
    }
  }
}
```
**REST API**:
```http
POST /api/v1/pdf/extract-text
```
**WASM (TypeScript / JS)**:
```javascript
WasmPdfEngine.extract_text(pdfBytes)
```
**Cloudflare Edge Endpoint**:
```http
POST /api/v1/extract_text
```
</details>

### Tool: `pdf_extract_images`
> **Use Case**: Extract all embedded images from the PDF.

| Tool | Interface | Command / Invocation | Latency | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|---|
| `pdf_extract_images` | **💻 CLI** | `paperpilot extract-images --input tests/e2e_fixtures/image_doc.pdf --output /app/tests/e2e_fixtures/out/tri_e2e/extracted_images --json` | `18.41 ms` | Directory containing extracted images | Directory with 3 files | ✅ PASS |
| `pdf_extract_images` | **🤖 MCP** | `tools/call {"name": "pdf_extract_images", "arguments": {"input": "tests/e2e_fixtures/image_doc.pdf", "output_dir": "/app/tests/e2e_fixtures/out/tri_e2e/extracted_images"}}` | `16.92 ms` | Directory containing extracted images | Directory with 3 files | ✅ PASS |
| `pdf_extract_images` | **🌐 REST API** | `POST /api/v1/pdf/tools/pdf_extract_images` | `5.24 ms` | Directory containing extracted images | Directory with 3 files | ✅ PASS |
| `pdf_extract_images` | **⚡ WASM (Browser)** | `WasmPdfEngine.extract_images(pdfBytes)` | `2.83 ms` | Directory containing extracted images | Directory with 3 files | ✅ PASS |
| `pdf_extract_images` | **☁️ Cloudflare Edge** | `POST /api/v1/extract_images` | `8.68 ms` | Directory containing extracted images | Directory with 3 files | ✅ PASS |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot extract-images --input tests/e2e_fixtures/image_doc.pdf --output /app/tests/e2e_fixtures/out/tri_e2e/extracted_images --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_extract_images",
    "arguments": {
      "input": "tests/e2e_fixtures/image_doc.pdf",
      "output_dir": "/app/tests/e2e_fixtures/out/tri_e2e/extracted_images"
    }
  }
}
```
**REST API**:
```http
POST /api/v1/pdf/tools/pdf_extract_images
```
**WASM (TypeScript / JS)**:
```javascript
WasmPdfEngine.extract_images(pdfBytes)
```
**Cloudflare Edge Endpoint**:
```http
POST /api/v1/extract_images
```
</details>

### Tool: `pdf_search`
> **Use Case**: Search for a query string in the PDF text.

| Tool | Interface | Command / Invocation | Latency | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|---|
| `pdf_search` | **💻 CLI** | `paperpilot search --input tests/e2e_fixtures/search_test.pdf --query test --json` | `19.02 ms` | JSON array of search results | Success: 3 keys | ✅ PASS |
| `pdf_search` | **🤖 MCP** | `tools/call {"name": "pdf_search", "arguments": {"input": "tests/e2e_fixtures/search_test.pdf", "query": "test"}}` | `15.13 ms` | JSON array of search results | Success: 4 keys | ✅ PASS |
| `pdf_search` | **🌐 REST API** | `POST /api/v1/pdf/tools/pdf_search` | `4.45 ms` | JSON array of search results | Success: 2 keys | ✅ PASS |
| `pdf_search` | **⚡ WASM (Browser)** | N/A (Desktop/Server only) | N/A | N/A | N/A | N/A |
| `pdf_search` | **☁️ Cloudflare Edge** | N/A (Desktop/Server only) | N/A | N/A | N/A | N/A |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot search --input tests/e2e_fixtures/search_test.pdf --query test --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_search",
    "arguments": {
      "input": "tests/e2e_fixtures/search_test.pdf",
      "query": "test"
    }
  }
}
```
**REST API**:
```http
POST /api/v1/pdf/tools/pdf_search
```
</details>

### Tool: `pdf_render`
> **Use Case**: Render the first page of the PDF as a PNG image.

| Tool | Interface | Command / Invocation | Latency | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|---|
| `pdf_render` | **💻 CLI** | `paperpilot render --input tests/e2e_fixtures/single_page.pdf --output /app/tests/e2e_fixtures/out/tri_e2e/rendered_cli.png --json` | `18.46 ms` | Rendered PNG image (> 5 KB) | Valid PNG file, 17 bytes | ✅ PASS |
| `pdf_render` | **🤖 MCP** | `tools/call {"name": "pdf_render", "arguments": {"input": "tests/e2e_fixtures/single_page.pdf", "output": "/app/tests/e2e_fixtures/out/tri_e2e/rendered_mcp.png", "page": 1}}` | `18.34 ms` | Rendered PNG image (> 5 KB) | Valid PNG file, 73 bytes | ✅ PASS |
| `pdf_render` | **🌐 REST API** | `POST /api/v1/pdf/render-page` | `5.44 ms` | Rendered PNG image (> 5 KB) | Valid PNG file, 73 bytes | ✅ PASS |
| `pdf_render` | **⚡ WASM (Browser)** | `WasmPdfEngine.render_page(pdfBytes, 0, 1.5)` | `2.29 ms` | Rendered PNG image (> 5 KB) | Valid PNG file, 73 bytes | ✅ PASS |
| `pdf_render` | **☁️ Cloudflare Edge** | `POST /api/v1/render` | `13.22 ms` | Rendered PNG image (> 5 KB) | Valid PNG file, 73 bytes | ✅ PASS |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot render --input tests/e2e_fixtures/single_page.pdf --output /app/tests/e2e_fixtures/out/tri_e2e/rendered_cli.png --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_render",
    "arguments": {
      "input": "tests/e2e_fixtures/single_page.pdf",
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/rendered_mcp.png",
      "page": 1
    }
  }
}
```
**REST API**:
```http
POST /api/v1/pdf/render-page
```
**WASM (TypeScript / JS)**:
```javascript
WasmPdfEngine.render_page(pdfBytes, 0, 1.5)
```
**Cloudflare Edge Endpoint**:
```http
POST /api/v1/render
```
</details>

### Tool: `pdf_compare`
> **Use Case**: Compare two PDFs and output structural differences.

| Tool | Interface | Command / Invocation | Latency | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|---|
| `pdf_compare` | **💻 CLI** | `paperpilot compare --input tests/e2e_fixtures/page_1.pdf --input-b tests/e2e_fixtures/page_2.pdf --json` | `19.29 ms` | JSON output detailing differences | Success: 3 keys | ✅ PASS |
| `pdf_compare` | **🤖 MCP** | `tools/call {"name": "pdf_compare", "arguments": {"file1": "tests/e2e_fixtures/page_1.pdf", "file2": "tests/e2e_fixtures/page_2.pdf"}}` | `19.00 ms` | JSON output detailing differences | Success: 5 keys | ✅ PASS |
| `pdf_compare` | **🌐 REST API** | `POST /api/v1/pdf/compare` | `6.94 ms` | JSON output detailing differences | Success: 2 keys | ✅ PASS |
| `pdf_compare` | **⚡ WASM (Browser)** | `WasmPdfEngine.compare(file1, file2)` | `4.99 ms` | JSON output detailing differences | Success: 2 keys | ✅ PASS |
| `pdf_compare` | **☁️ Cloudflare Edge** | `POST /api/v1/compare` | `10.22 ms` | JSON output detailing differences | Success: 2 keys | ✅ PASS |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot compare --input tests/e2e_fixtures/page_1.pdf --input-b tests/e2e_fixtures/page_2.pdf --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_compare",
    "arguments": {
      "file1": "tests/e2e_fixtures/page_1.pdf",
      "file2": "tests/e2e_fixtures/page_2.pdf"
    }
  }
}
```
**REST API**:
```http
POST /api/v1/pdf/compare
```
**WASM (TypeScript / JS)**:
```javascript
WasmPdfEngine.compare(file1, file2)
```
**Cloudflare Edge Endpoint**:
```http
POST /api/v1/compare
```
</details>

### Tool: `pdf_ocr`
> **Use Case**: Perform OCR to make image-based text searchable.

| Tool | Interface | Command / Invocation | Latency | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|---|
| `pdf_ocr` | **💻 CLI** | `paperpilot ocr --input tests/e2e_fixtures/image_doc.pdf --output /app/tests/e2e_fixtures/out/tri_e2e/ocr_cli.pdf --json` | `19.71 ms` | Valid searchable PDF (%PDF-) | Valid PDF, 3 pages, 1,334 bytes | ✅ PASS |
| `pdf_ocr` | **🤖 MCP** | `tools/call {"name": "pdf_ocr", "arguments": {"input": "tests/e2e_fixtures/image_doc.pdf", "output": "/app/tests/e2e_fixtures/out/tri_e2e/ocr_mcp.pdf"}}` | `18.53 ms` | Valid searchable PDF (%PDF-) | Valid PDF, 3 pages, 1,334 bytes | ✅ PASS |
| `pdf_ocr` | **🌐 REST API** | `POST /api/v1/pdf/tools/pdf_ocr` | `5.68 ms` | Valid searchable PDF (%PDF-) | Valid PDF, 3 pages, 1,334 bytes | ✅ PASS |
| `pdf_ocr` | **⚡ WASM (Browser)** | N/A (Desktop/Server only) | N/A | N/A | N/A | N/A |
| `pdf_ocr` | **☁️ Cloudflare Edge** | N/A (Desktop/Server only) | N/A | N/A | N/A | N/A |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot ocr --input tests/e2e_fixtures/image_doc.pdf --output /app/tests/e2e_fixtures/out/tri_e2e/ocr_cli.pdf --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_ocr",
    "arguments": {
      "input": "tests/e2e_fixtures/image_doc.pdf",
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/ocr_mcp.pdf"
    }
  }
}
```
**REST API**:
```http
POST /api/v1/pdf/tools/pdf_ocr
```
</details>

### Tool: `pdf_bookmarks`
> **Use Case**: Extract bookmarks/outlines from the PDF.

| Tool | Interface | Command / Invocation | Latency | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|---|
| `pdf_bookmarks` | **💻 CLI** | `paperpilot bookmarks --input tests/e2e_fixtures/large_doc.pdf --json` | `20.48 ms` | JSON array of bookmarks | Success: 3 keys | ✅ PASS |
| `pdf_bookmarks` | **🤖 MCP** | `tools/call {"name": "pdf_bookmarks", "arguments": {"input": "tests/e2e_fixtures/large_doc.pdf"}}` | `18.94 ms` | JSON array of bookmarks | Success: 4 keys | ✅ PASS |
| `pdf_bookmarks` | **🌐 REST API** | `POST /api/v1/pdf/tools/pdf_bookmarks` | `6.42 ms` | JSON array of bookmarks | Success: 2 keys | ✅ PASS |
| `pdf_bookmarks` | **⚡ WASM (Browser)** | `WasmPdfEngine.pdf_info(pdfBytes)` | `2.75 ms` | JSON array of bookmarks | Success: 2 keys | ✅ PASS |
| `pdf_bookmarks` | **☁️ Cloudflare Edge** | `POST /api/v1/info` | `10.56 ms` | JSON array of bookmarks | Success: 2 keys | ✅ PASS |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot bookmarks --input tests/e2e_fixtures/large_doc.pdf --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_bookmarks",
    "arguments": {
      "input": "tests/e2e_fixtures/large_doc.pdf"
    }
  }
}
```
**REST API**:
```http
POST /api/v1/pdf/tools/pdf_bookmarks
```
**WASM (TypeScript / JS)**:
```javascript
WasmPdfEngine.pdf_info(pdfBytes)
```
**Cloudflare Edge Endpoint**:
```http
POST /api/v1/info
```
</details>

### Tool: `pdf_images_to_pdf`
> **Use Case**: Convert a list of images to a single PDF.

| Tool | Interface | Command / Invocation | Latency | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|---|
| `pdf_images_to_pdf` | **💻 CLI** | `paperpilot images-to-pdf --images tests/e2e_fixtures/img1.png tests/e2e_fixtures/img2.png --output /app/tests/e2e_fixtures/out/tri_e2e/images_cli.pdf --json` | `18.60 ms` | Valid 2-page PDF (%PDF-) | Valid PDF, 2 pages, 1,574 bytes | ✅ PASS |
| `pdf_images_to_pdf` | **🤖 MCP** | `tools/call {"name": "pdf_images_to_pdf", "arguments": {"inputs": ["tests/e2e_fixtures/img1.png", "tests/e2e_fixtures/img2.png"], "output": "/app/tests/e2e_fixtures/out/tri_e2e/images_mcp.pdf"}}` | `18.64 ms` | Valid 2-page PDF (%PDF-) | Valid PDF, 2 pages, 1,574 bytes | ✅ PASS |
| `pdf_images_to_pdf` | **🌐 REST API** | `POST /api/v1/pdf/tools/pdf_images_to_pdf` | `6.28 ms` | Valid 2-page PDF (%PDF-) | Valid PDF, 2 pages, 1,574 bytes | ✅ PASS |
| `pdf_images_to_pdf` | **⚡ WASM (Browser)** | `WasmPdfEngine.images_to_pdf([img1, img2])` | `2.24 ms` | Valid 2-page PDF (%PDF-) | Valid PDF, 2 pages, 1,574 bytes | ✅ PASS |
| `pdf_images_to_pdf` | **☁️ Cloudflare Edge** | `POST /api/v1/images_to_pdf` | `8.89 ms` | Valid 2-page PDF (%PDF-) | Valid PDF, 2 pages, 1,574 bytes | ✅ PASS |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot images-to-pdf --images tests/e2e_fixtures/img1.png tests/e2e_fixtures/img2.png --output /app/tests/e2e_fixtures/out/tri_e2e/images_cli.pdf --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_images_to_pdf",
    "arguments": {
      "inputs": [
        "tests/e2e_fixtures/img1.png",
        "tests/e2e_fixtures/img2.png"
      ],
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/images_mcp.pdf"
    }
  }
}
```
**REST API**:
```http
POST /api/v1/pdf/tools/pdf_images_to_pdf
```
**WASM (TypeScript / JS)**:
```javascript
WasmPdfEngine.images_to_pdf([img1, img2])
```
**Cloudflare Edge Endpoint**:
```http
POST /api/v1/images_to_pdf
```
</details>

### Tool: `pdf_annotate`
> **Use Case**: Add annotations (e.g. highlight) to a PDF.

| Tool | Interface | Command / Invocation | Latency | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|---|
| `pdf_annotate` | **💻 CLI** | `paperpilot annotate --input tests/e2e_fixtures/single_page.pdf --data [{"id": "1", "type": "highlight", "page": 1, "x": 50.0, "y": 50.0, "w": 50.0, "h": 50.0, "color": "#ffff00", "content": "Test"}] --output /app/tests/e2e_fixtures/out/tri_e2e/annotated_cli.pdf --json` | `19.00 ms` | Valid annotated PDF (%PDF-) | Valid PDF, 1 pages, 682 bytes | ✅ PASS |
| `pdf_annotate` | **🤖 MCP** | `tools/call {"name": "pdf_annotate", "arguments": {"annotations": [{"color": "#ffff00", "content": "Test", "h": 50.0, "id": "1", "page": 1, "type": "highlight", "w": 50.0, "x": 50.0, "y": 50.0}], "input": "tests/e2e_fixtures/single_page.pdf", "output": "/app/tests/e2e_fixtures/out/tri_e2e/annotated_mcp.pdf"}}` | `16.62 ms` | Valid annotated PDF (%PDF-) | Valid PDF, 1 pages, 682 bytes | ✅ PASS |
| `pdf_annotate` | **🌐 REST API** | `POST /api/v1/pdf/tools/pdf_annotate` | `4.99 ms` | Valid annotated PDF (%PDF-) | Valid PDF, 1 pages, 682 bytes | ✅ PASS |
| `pdf_annotate` | **⚡ WASM (Browser)** | `WasmPdfEngine.annotate(pdfBytes, annotations)` | `3.45 ms` | Valid annotated PDF (%PDF-) | Valid PDF, 1 pages, 682 bytes | ✅ PASS |
| `pdf_annotate` | **☁️ Cloudflare Edge** | `POST /api/v1/annotate` | `12.67 ms` | Valid annotated PDF (%PDF-) | Valid PDF, 1 pages, 682 bytes | ✅ PASS |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot annotate --input tests/e2e_fixtures/single_page.pdf --data [{"id": "1", "type": "highlight", "page": 1, "x": 50.0, "y": 50.0, "w": 50.0, "h": 50.0, "color": "#ffff00", "content": "Test"}] --output /app/tests/e2e_fixtures/out/tri_e2e/annotated_cli.pdf --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_annotate",
    "arguments": {
      "annotations": [
        {
          "color": "#ffff00",
          "content": "Test",
          "h": 50.0,
          "id": "1",
          "page": 1,
          "type": "highlight",
          "w": 50.0,
          "x": 50.0,
          "y": 50.0
        }
      ],
      "input": "tests/e2e_fixtures/single_page.pdf",
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/annotated_mcp.pdf"
    }
  }
}
```
**REST API**:
```http
POST /api/v1/pdf/tools/pdf_annotate
```
**WASM (TypeScript / JS)**:
```javascript
WasmPdfEngine.annotate(pdfBytes, annotations)
```
**Cloudflare Edge Endpoint**:
```http
POST /api/v1/annotate
```
</details>

### Tool: `pdf_classify_type`
> **Use Case**: Classify the type/layout of the PDF document.

| Tool | Interface | Command / Invocation | Latency | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|---|
| `pdf_classify_type` | **💻 CLI** | `paperpilot classify --input tests/e2e_fixtures/single_page.pdf --json` | `22.00 ms` | JSON string containing classification | Success: JSON output validated | ✅ PASS |
| `pdf_classify_type` | **🤖 MCP** | `tools/call {"name": "pdf_classify_type", "arguments": {"input": "tests/e2e_fixtures/single_page.pdf"}}` | `20.05 ms` | JSON string containing classification | Success: 4 keys | ✅ PASS |
| `pdf_classify_type` | **🌐 REST API** | `POST /api/v1/pdf/tools/pdf_classify_type` | `8.27 ms` | JSON string containing classification | Success: 2 keys | ✅ PASS |
| `pdf_classify_type` | **⚡ WASM (Browser)** | `WasmPdfEngine.pdf_info(pdfBytes)` | `4.92 ms` | JSON string containing classification | Success: 2 keys | ✅ PASS |
| `pdf_classify_type` | **☁️ Cloudflare Edge** | `POST /api/v1/info` | `12.72 ms` | JSON string containing classification | Success: 2 keys | ✅ PASS |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot classify --input tests/e2e_fixtures/single_page.pdf --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_classify_type",
    "arguments": {
      "input": "tests/e2e_fixtures/single_page.pdf"
    }
  }
}
```
**REST API**:
```http
POST /api/v1/pdf/tools/pdf_classify_type
```
**WASM (TypeScript / JS)**:
```javascript
WasmPdfEngine.pdf_info(pdfBytes)
```
**Cloudflare Edge Endpoint**:
```http
POST /api/v1/info
```
</details>

### Tool: `pdf_validate`
> **Use Case**: Validate the PDF against standard specifications.

| Tool | Interface | Command / Invocation | Latency | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|---|
| `pdf_validate` | **💻 CLI** | `paperpilot validate --input tests/e2e_fixtures/single_page.pdf --json` | `17.47 ms` | JSON validation report | Success: 3 keys | ✅ PASS |
| `pdf_validate` | **🤖 MCP** | `tools/call {"name": "pdf_validate", "arguments": {"input": "tests/e2e_fixtures/single_page.pdf"}}` | `15.87 ms` | JSON validation report | Success: 4 keys | ✅ PASS |
| `pdf_validate` | **🌐 REST API** | `POST /api/v1/pdf/tools/pdf_validate` | `4.41 ms` | JSON validation report | Success: 2 keys | ✅ PASS |
| `pdf_validate` | **⚡ WASM (Browser)** | `WasmPdfEngine.pdf_info(pdfBytes)` | `4.12 ms` | JSON validation report | Success: 2 keys | ✅ PASS |
| `pdf_validate` | **☁️ Cloudflare Edge** | `POST /api/v1/info` | `11.03 ms` | JSON validation report | Success: 2 keys | ✅ PASS |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot validate --input tests/e2e_fixtures/single_page.pdf --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_validate",
    "arguments": {
      "input": "tests/e2e_fixtures/single_page.pdf"
    }
  }
}
```
**REST API**:
```http
POST /api/v1/pdf/tools/pdf_validate
```
**WASM (TypeScript / JS)**:
```javascript
WasmPdfEngine.pdf_info(pdfBytes)
```
**Cloudflare Edge Endpoint**:
```http
POST /api/v1/info
```
</details>

### Tool: `pdf_hash`
> **Use Case**: Generate a cryptographic hash of the PDF.

| Tool | Interface | Command / Invocation | Latency | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|---|
| `pdf_hash` | **💻 CLI** | `paperpilot hash --input tests/e2e_fixtures/single_page.pdf --json` | `15.56 ms` | JSON object with hash value | Success: JSON output validated | ✅ PASS |
| `pdf_hash` | **🤖 MCP** | `tools/call {"name": "pdf_hash", "arguments": {"input": "tests/e2e_fixtures/single_page.pdf"}}` | `14.64 ms` | JSON object with hash value | Success: 4 keys | ✅ PASS |
| `pdf_hash` | **🌐 REST API** | `POST /api/v1/pdf/tools/pdf_hash` | `3.18 ms` | JSON object with hash value | Success: 2 keys | ✅ PASS |
| `pdf_hash` | **⚡ WASM (Browser)** | `WasmPdfEngine.pdf_hash(pdfBytes)` | `2.19 ms` | JSON object with hash value | Success: 2 keys | ✅ PASS |
| `pdf_hash` | **☁️ Cloudflare Edge** | `POST /api/v1/hash` | `12.12 ms` | JSON object with hash value | Success: 2 keys | ✅ PASS |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot hash --input tests/e2e_fixtures/single_page.pdf --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_hash",
    "arguments": {
      "input": "tests/e2e_fixtures/single_page.pdf"
    }
  }
}
```
**REST API**:
```http
POST /api/v1/pdf/tools/pdf_hash
```
**WASM (TypeScript / JS)**:
```javascript
WasmPdfEngine.pdf_hash(pdfBytes)
```
**Cloudflare Edge Endpoint**:
```http
POST /api/v1/hash
```
</details>

### Tool: `pdf_read_form`
> **Use Case**: Extract form fields and their values.

| Tool | Interface | Command / Invocation | Latency | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|---|
| `pdf_read_form` | **💻 CLI** | `paperpilot form read tests/e2e_fixtures/form.pdf --json` | `18.04 ms` | JSON array of form fields | Success: JSON output validated | ✅ PASS |
| `pdf_read_form` | **🤖 MCP** | `tools/call {"name": "pdf_read_form", "arguments": {"input": "tests/e2e_fixtures/form.pdf"}}` | `17.28 ms` | JSON array of form fields | Success: 4 keys | ✅ PASS |
| `pdf_read_form` | **🌐 REST API** | `POST /api/v1/pdf/tools/pdf_read_form` | `4.74 ms` | JSON array of form fields | Success: 2 keys | ✅ PASS |
| `pdf_read_form` | **⚡ WASM (Browser)** | `WasmPdfEngine.read_form(pdfBytes)` | `3.26 ms` | JSON array of form fields | Success: 2 keys | ✅ PASS |
| `pdf_read_form` | **☁️ Cloudflare Edge** | `POST /api/v1/read_form` | `11.22 ms` | JSON array of form fields | Success: 2 keys | ✅ PASS |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot form read tests/e2e_fixtures/form.pdf --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_read_form",
    "arguments": {
      "input": "tests/e2e_fixtures/form.pdf"
    }
  }
}
```
**REST API**:
```http
POST /api/v1/pdf/tools/pdf_read_form
```
**WASM (TypeScript / JS)**:
```javascript
WasmPdfEngine.read_form(pdfBytes)
```
**Cloudflare Edge Endpoint**:
```http
POST /api/v1/read_form
```
</details>

### Tool: `pdf_fill_form`
> **Use Case**: Fill PDF form fields with provided values.

| Tool | Interface | Command / Invocation | Latency | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|---|
| `pdf_fill_form` | **💻 CLI** | `paperpilot form fill tests/e2e_fixtures/form.pdf --data tests/e2e_fixtures/form_data.json --output /app/tests/e2e_fixtures/out/tri_e2e/filled_cli.pdf --json` | `18.43 ms` | Valid filled PDF (%PDF-) | Valid PDF, 1 pages, 878 bytes | ✅ PASS |
| `pdf_fill_form` | **🤖 MCP** | `tools/call {"name": "pdf_fill_form", "arguments": {"input": "tests/e2e_fixtures/form.pdf", "output": "/app/tests/e2e_fixtures/out/tri_e2e/filled_mcp.pdf", "values": {"TestText": "Alice"}}}` | `17.71 ms` | Valid filled PDF (%PDF-) | Valid PDF, 1 pages, 867 bytes | ✅ PASS |
| `pdf_fill_form` | **🌐 REST API** | `POST /api/v1/pdf/tools/pdf_fill_form` | `5.51 ms` | Valid filled PDF (%PDF-) | Valid PDF, 1 pages, 867 bytes | ✅ PASS |
| `pdf_fill_form` | **⚡ WASM (Browser)** | `WasmPdfEngine.fill_form(pdfBytes, data)` | `3.48 ms` | Valid filled PDF (%PDF-) | Valid PDF, 1 pages, 867 bytes | ✅ PASS |
| `pdf_fill_form` | **☁️ Cloudflare Edge** | `POST /api/v1/fill_form` | `9.99 ms` | Valid filled PDF (%PDF-) | Valid PDF, 1 pages, 867 bytes | ✅ PASS |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot form fill tests/e2e_fixtures/form.pdf --data tests/e2e_fixtures/form_data.json --output /app/tests/e2e_fixtures/out/tri_e2e/filled_cli.pdf --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_fill_form",
    "arguments": {
      "input": "tests/e2e_fixtures/form.pdf",
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/filled_mcp.pdf",
      "values": {
        "TestText": "Alice"
      }
    }
  }
}
```
**REST API**:
```http
POST /api/v1/pdf/tools/pdf_fill_form
```
**WASM (TypeScript / JS)**:
```javascript
WasmPdfEngine.fill_form(pdfBytes, data)
```
**Cloudflare Edge Endpoint**:
```http
POST /api/v1/fill_form
```
</details>

### Tool: `pdf_create_form_field`
> **Use Case**: Add a new text form field to the PDF.

| Tool | Interface | Command / Invocation | Latency | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|---|
| `pdf_create_form_field` | **💻 CLI** | `paperpilot form add-field tests/e2e_fixtures/single_page.pdf --name signature --type text --rect 50,50,150,30 --output /app/tests/e2e_fixtures/out/tri_e2e/added_cli.pdf --json` | `18.36 ms` | Valid PDF with new form field (%PDF-) | Valid PDF, 1 pages, 705 bytes | ✅ PASS |
| `pdf_create_form_field` | **🤖 MCP** | `tools/call {"name": "pdf_create_form_field", "arguments": {"field_name": "signature", "field_type": "text", "height": 30.0, "input": "tests/e2e_fixtures/single_page.pdf", "output": "/app/tests/e2e_fixtures/out/tri_e2e/added_mcp.pdf", "width": 100.0, "x": 50.0, "y": 50.0}}` | `17.41 ms` | Valid PDF with new form field (%PDF-) | Valid PDF, 1 pages, 705 bytes | ✅ PASS |
| `pdf_create_form_field` | **🌐 REST API** | `POST /api/v1/pdf/tools/pdf_create_form_field` | `5.00 ms` | Valid PDF with new form field (%PDF-) | Valid PDF, 1 pages, 705 bytes | ✅ PASS |
| `pdf_create_form_field` | **⚡ WASM (Browser)** | `WasmPdfEngine.add_field(pdfBytes, field)` | `3.77 ms` | Valid PDF with new form field (%PDF-) | Valid PDF, 1 pages, 705 bytes | ✅ PASS |
| `pdf_create_form_field` | **☁️ Cloudflare Edge** | `POST /api/v1/add_field` | `10.54 ms` | Valid PDF with new form field (%PDF-) | Valid PDF, 1 pages, 705 bytes | ✅ PASS |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot form add-field tests/e2e_fixtures/single_page.pdf --name signature --type text --rect 50,50,150,30 --output /app/tests/e2e_fixtures/out/tri_e2e/added_cli.pdf --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_create_form_field",
    "arguments": {
      "field_name": "signature",
      "field_type": "text",
      "height": 30.0,
      "input": "tests/e2e_fixtures/single_page.pdf",
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/added_mcp.pdf",
      "width": 100.0,
      "x": 50.0,
      "y": 50.0
    }
  }
}
```
**REST API**:
```http
POST /api/v1/pdf/tools/pdf_create_form_field
```
**WASM (TypeScript / JS)**:
```javascript
WasmPdfEngine.add_field(pdfBytes, field)
```
**Cloudflare Edge Endpoint**:
```http
POST /api/v1/add_field
```
</details>

### Tool: `pdf_to_docx`
> **Use Case**: Convert PDF to Microsoft Word format (DOCX).

| Tool | Interface | Command / Invocation | Latency | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|---|
| `pdf_to_docx` | **💻 CLI** | `paperpilot convert --input tests/e2e_fixtures/single_page.pdf --format docx --output /app/tests/e2e_fixtures/out/tri_e2e/out_cli.docx --json` | `21.37 ms` | Valid DOCX file | Valid DOCX file, 18,197 bytes | ✅ PASS |
| `pdf_to_docx` | **🤖 MCP** | `tools/call {"name": "pdf_to_docx", "arguments": {"input": "tests/e2e_fixtures/single_page.pdf", "output": "/app/tests/e2e_fixtures/out/tri_e2e/out_mcp.docx"}}` | `20.07 ms` | Valid DOCX file | Valid DOCX file, 18,197 bytes | ✅ PASS |
| `pdf_to_docx` | **🌐 REST API** | `POST /api/v1/pdf/convert` | `6.97 ms` | Valid DOCX file | Valid DOCX file, 18,197 bytes | ✅ PASS |
| `pdf_to_docx` | **⚡ WASM (Browser)** | N/A (Desktop/Server only) | N/A | N/A | N/A | N/A |
| `pdf_to_docx` | **☁️ Cloudflare Edge** | N/A (Desktop/Server only) | N/A | N/A | N/A | N/A |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot convert --input tests/e2e_fixtures/single_page.pdf --format docx --output /app/tests/e2e_fixtures/out/tri_e2e/out_cli.docx --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_to_docx",
    "arguments": {
      "input": "tests/e2e_fixtures/single_page.pdf",
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/out_mcp.docx"
    }
  }
}
```
**REST API**:
```http
POST /api/v1/pdf/convert
```
</details>

### Tool: `pdf_to_xlsx`
> **Use Case**: Convert PDF to Microsoft Excel format (XLSX).

| Tool | Interface | Command / Invocation | Latency | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|---|
| `pdf_to_xlsx` | **💻 CLI** | `paperpilot convert --input tests/e2e_fixtures/single_page.pdf --format xlsx --output /app/tests/e2e_fixtures/out/tri_e2e/out_cli.xlsx --json` | `39.35 ms` | Valid XLSX file | Valid XLSX file, 5,436 bytes | ✅ PASS |
| `pdf_to_xlsx` | **🤖 MCP** | `tools/call {"name": "pdf_to_xlsx", "arguments": {"input": "tests/e2e_fixtures/single_page.pdf", "output": "/app/tests/e2e_fixtures/out/tri_e2e/out_mcp.xlsx"}}` | `32.54 ms` | Valid XLSX file | Valid XLSX file, 5,436 bytes | ✅ PASS |
| `pdf_to_xlsx` | **🌐 REST API** | `POST /api/v1/pdf/convert` | `23.85 ms` | Valid XLSX file | Valid XLSX file, 5,436 bytes | ✅ PASS |
| `pdf_to_xlsx` | **⚡ WASM (Browser)** | N/A (Desktop/Server only) | N/A | N/A | N/A | N/A |
| `pdf_to_xlsx` | **☁️ Cloudflare Edge** | N/A (Desktop/Server only) | N/A | N/A | N/A | N/A |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot convert --input tests/e2e_fixtures/single_page.pdf --format xlsx --output /app/tests/e2e_fixtures/out/tri_e2e/out_cli.xlsx --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_to_xlsx",
    "arguments": {
      "input": "tests/e2e_fixtures/single_page.pdf",
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/out_mcp.xlsx"
    }
  }
}
```
**REST API**:
```http
POST /api/v1/pdf/convert
```
</details>

### Tool: `pdf_to_pptx`
> **Use Case**: Convert PDF to Microsoft PowerPoint format (PPTX).

| Tool | Interface | Command / Invocation | Latency | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|---|
| `pdf_to_pptx` | **💻 CLI** | `paperpilot convert --input tests/e2e_fixtures/single_page.pdf --format pptx --output /app/tests/e2e_fixtures/out/tri_e2e/out_cli.pptx --json` | `21.62 ms` | Valid PPTX file | Valid PPTX file, 645 bytes | ✅ PASS |
| `pdf_to_pptx` | **🤖 MCP** | `tools/call {"name": "pdf_to_pptx", "arguments": {"input": "tests/e2e_fixtures/single_page.pdf", "output": "/app/tests/e2e_fixtures/out/tri_e2e/out_mcp.pptx"}}` | `20.01 ms` | Valid PPTX file | Valid PPTX file, 645 bytes | ✅ PASS |
| `pdf_to_pptx` | **🌐 REST API** | `POST /api/v1/pdf/convert` | `6.65 ms` | Valid PPTX file | Valid PPTX file, 645 bytes | ✅ PASS |
| `pdf_to_pptx` | **⚡ WASM (Browser)** | N/A (Desktop/Server only) | N/A | N/A | N/A | N/A |
| `pdf_to_pptx` | **☁️ Cloudflare Edge** | N/A (Desktop/Server only) | N/A | N/A | N/A | N/A |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot convert --input tests/e2e_fixtures/single_page.pdf --format pptx --output /app/tests/e2e_fixtures/out/tri_e2e/out_cli.pptx --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_to_pptx",
    "arguments": {
      "input": "tests/e2e_fixtures/single_page.pdf",
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/out_mcp.pptx"
    }
  }
}
```
**REST API**:
```http
POST /api/v1/pdf/convert
```
</details>

### Tool: `pdf_convert_html`
> **Use Case**: Convert HTML document to PDF.

| Tool | Interface | Command / Invocation | Latency | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|---|
| `pdf_convert_html` | **💻 CLI** | `paperpilot convert --input tests/e2e_fixtures/test.html --format pdf --output /app/tests/e2e_fixtures/out/tri_e2e/out_html_cli.pdf --json` | `87.67 ms` | Valid PDF (%PDF-) | Valid PDF, 1 pages, 15,526 bytes | ✅ PASS |
| `pdf_convert_html` | **🤖 MCP** | `tools/call {"name": "pdf_convert_html", "arguments": {"input": "tests/e2e_fixtures/test.html", "output": "/app/tests/e2e_fixtures/out/tri_e2e/out_html_mcp.pdf"}}` | `84.39 ms` | Valid PDF (%PDF-) | Valid PDF, 1 pages, 15,526 bytes | ✅ PASS |
| `pdf_convert_html` | **🌐 REST API** | `POST /api/v1/pdf/tools/pdf_convert_html` | `63.51 ms` | Valid PDF (%PDF-) | Valid PDF, 1 pages, 15,526 bytes | ✅ PASS |
| `pdf_convert_html` | **⚡ WASM (Browser)** | N/A (Desktop/Server only) | N/A | N/A | N/A | N/A |
| `pdf_convert_html` | **☁️ Cloudflare Edge** | N/A (Desktop/Server only) | N/A | N/A | N/A | N/A |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot convert --input tests/e2e_fixtures/test.html --format pdf --output /app/tests/e2e_fixtures/out/tri_e2e/out_html_cli.pdf --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_convert_html",
    "arguments": {
      "input": "tests/e2e_fixtures/test.html",
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/out_html_mcp.pdf"
    }
  }
}
```
**REST API**:
```http
POST /api/v1/pdf/tools/pdf_convert_html
```
</details>

### Tool: `pdf_convert_markdown`
> **Use Case**: Convert Markdown document to PDF.

| Tool | Interface | Command / Invocation | Latency | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|---|
| `pdf_convert_markdown` | **💻 CLI** | `paperpilot convert --input tests/e2e_fixtures/test.md --format pdf --output /app/tests/e2e_fixtures/out/tri_e2e/out_md_cli.pdf --json` | `74.56 ms` | Valid PDF (%PDF-) | Valid PDF, 1 pages, 15,342 bytes | ✅ PASS |
| `pdf_convert_markdown` | **🤖 MCP** | `tools/call {"name": "pdf_convert_markdown", "arguments": {"input": "tests/e2e_fixtures/test.md", "output": "/app/tests/e2e_fixtures/out/tri_e2e/out_md_mcp.pdf"}}` | `77.47 ms` | Valid PDF (%PDF-) | Valid PDF, 1 pages, 15,342 bytes | ✅ PASS |
| `pdf_convert_markdown` | **🌐 REST API** | `POST /api/v1/pdf/tools/pdf_convert_markdown` | `61.09 ms` | Valid PDF (%PDF-) | Valid PDF, 1 pages, 15,342 bytes | ✅ PASS |
| `pdf_convert_markdown` | **⚡ WASM (Browser)** | N/A (Desktop/Server only) | N/A | N/A | N/A | N/A |
| `pdf_convert_markdown` | **☁️ Cloudflare Edge** | N/A (Desktop/Server only) | N/A | N/A | N/A | N/A |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot convert --input tests/e2e_fixtures/test.md --format pdf --output /app/tests/e2e_fixtures/out/tri_e2e/out_md_cli.pdf --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_convert_markdown",
    "arguments": {
      "input": "tests/e2e_fixtures/test.md",
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/out_md_mcp.pdf"
    }
  }
}
```
**REST API**:
```http
POST /api/v1/pdf/tools/pdf_convert_markdown
```
</details>

### Tool: `pdf_convert_excel`
> **Use Case**: Convert Excel/CSV to PDF.

| Tool | Interface | Command / Invocation | Latency | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|---|
| `pdf_convert_excel` | **💻 CLI** | `paperpilot convert --input tests/e2e_fixtures/test.csv --format pdf --output /app/tests/e2e_fixtures/out/tri_e2e/out_csv_cli.pdf --json` | `83.43 ms` | Valid PDF (%PDF-) | Valid PDF, 1 pages, 15,782 bytes | ✅ PASS |
| `pdf_convert_excel` | **🤖 MCP** | `tools/call {"name": "pdf_convert_excel", "arguments": {"input": "tests/e2e_fixtures/test.csv", "output": "/app/tests/e2e_fixtures/out/tri_e2e/out_csv_mcp.pdf"}}` | `80.13 ms` | Valid PDF (%PDF-) | Valid PDF, 1 pages, 15,782 bytes | ✅ PASS |
| `pdf_convert_excel` | **🌐 REST API** | `POST /api/v1/pdf/tools/pdf_convert_excel` | `63.47 ms` | Valid PDF (%PDF-) | Valid PDF, 1 pages, 15,782 bytes | ✅ PASS |
| `pdf_convert_excel` | **⚡ WASM (Browser)** | N/A (Desktop/Server only) | N/A | N/A | N/A | N/A |
| `pdf_convert_excel` | **☁️ Cloudflare Edge** | N/A (Desktop/Server only) | N/A | N/A | N/A | N/A |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot convert --input tests/e2e_fixtures/test.csv --format pdf --output /app/tests/e2e_fixtures/out/tri_e2e/out_csv_cli.pdf --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_convert_excel",
    "arguments": {
      "input": "tests/e2e_fixtures/test.csv",
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/out_csv_mcp.pdf"
    }
  }
}
```
**REST API**:
```http
POST /api/v1/pdf/tools/pdf_convert_excel
```
</details>
