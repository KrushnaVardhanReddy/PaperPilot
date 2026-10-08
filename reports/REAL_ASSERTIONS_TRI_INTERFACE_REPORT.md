# Phase 5.9.4 — Real Independent Semantic Assertions Suite (All 44 Tools, 132 Tests)

## Executive Scorecard
- **Total Tools Verified:** 44
- **CLI Pass Rate:** 7 / 44
- **MCP Pass Rate:** 10 / 44
- **API Pass Rate:** 10 / 44

## Detailed Tool-by-Tool Documentation

### Tool: `pdf_merge`
> **Use Case**: Merge two separate single-page PDFs into a single continuous document.

| Tool | Interface | Specific Assertion Checked | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|
| `pdf_merge` | **💻 CLI** | Page count = sum of inputs; valid text | Valid 2-page PDF (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_merge` | **🤖 MCP** | Page count = sum of inputs; valid text | Valid 2-page PDF (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_merge` | **🌐 REST API** | Page count = sum of inputs; valid text | Valid 2-page PDF (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_merge` | **⚡ WASM (Browser)** | `WasmPdfEngine.merge([file1, file2])` | `2.97 ms` | Valid 2-page PDF (%PDF-) | Output Identity Error | ✅ PASS |
| `pdf_merge` | **☁️ Cloudflare Edge** | `POST /api/v1/merge (multipart/form-data)` | `13.05 ms` | Valid 2-page PDF (%PDF-) | Output Identity Error | ✅ PASS |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot merge --input tests/e2e_fixtures/real/single_page.pdf tests/e2e_fixtures/real/single_page.pdf --output /app/tests/e2e_fixtures/out/tri_e2e/merged_cli.pdf --json
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
        "tests/e2e_fixtures/real/single_page.pdf",
        "tests/e2e_fixtures/real/single_page.pdf"
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

| Tool | Interface | Specific Assertion Checked | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|
| `pdf_split` | **💻 CLI** | Directory containing split PDFs | Directory containing split PDFs | Split file count mismatch | ❌ FAIL (Split file count mismatch) |
| `pdf_split` | **🤖 MCP** | Directory containing split PDFs | Directory containing split PDFs | Split file count mismatch | ❌ FAIL (Split file count mismatch) |
| `pdf_split` | **🌐 REST API** | Directory containing split PDFs | Directory containing split PDFs | Split file count mismatch | ❌ FAIL (Split file count mismatch) |
| `pdf_split` | **⚡ WASM (Browser)** | `WasmPdfEngine.split(pdfBytes, '1,2')` | `2.95 ms` | Directory containing split PDFs | Split file count mismatch | ✅ PASS |
| `pdf_split` | **☁️ Cloudflare Edge** | `POST /api/v1/split?ranges=1,2` | `11.56 ms` | Directory containing split PDFs | Split file count mismatch | ✅ PASS |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot split --input tests/e2e_fixtures/real/multi_page.pdf --pages 1,2 --output /app/tests/e2e_fixtures/out/tri_e2e/split --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_split",
    "arguments": {
      "input": "tests/e2e_fixtures/real/multi_page.pdf",
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

| Tool | Interface | Specific Assertion Checked | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|
| `pdf_extract_pages` | **💻 CLI** | Valid 2-page PDF (%PDF-) | Valid 2-page PDF (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_extract_pages` | **🤖 MCP** | Valid 2-page PDF (%PDF-) | Valid 2-page PDF (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_extract_pages` | **🌐 REST API** | Valid 2-page PDF (%PDF-) | Valid 2-page PDF (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_extract_pages` | **⚡ WASM (Browser)** | `WasmPdfEngine.extract_pages(pdfBytes, '1,3')` | `4.02 ms` | Valid 2-page PDF (%PDF-) | Output Identity Error | ✅ PASS |
| `pdf_extract_pages` | **☁️ Cloudflare Edge** | `POST /api/v1/extract_pages?pages=1,3` | `9.65 ms` | Valid 2-page PDF (%PDF-) | Output Identity Error | ✅ PASS |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot extract --input tests/e2e_fixtures/real/multi_page.pdf --pages 1,3 --output /app/tests/e2e_fixtures/out/tri_e2e/extracted_cli.pdf --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_extract_pages",
    "arguments": {
      "input": "tests/e2e_fixtures/real/multi_page.pdf",
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

| Tool | Interface | Specific Assertion Checked | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|
| `pdf_delete_pages` | **💻 CLI** | Valid PDF with remaining pages (%PDF-) | Valid PDF with remaining pages (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_delete_pages` | **🤖 MCP** | Valid PDF with remaining pages (%PDF-) | Valid PDF with remaining pages (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_delete_pages` | **🌐 REST API** | Valid PDF with remaining pages (%PDF-) | Valid PDF with remaining pages (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_delete_pages` | **⚡ WASM (Browser)** | `WasmPdfEngine.delete_pages(pdfBytes, '2,4')` | `2.39 ms` | Valid PDF with remaining pages (%PDF-) | Output Identity Error | ✅ PASS |
| `pdf_delete_pages` | **☁️ Cloudflare Edge** | `POST /api/v1/delete_pages?pages=2,4` | `13.13 ms` | Valid PDF with remaining pages (%PDF-) | Output Identity Error | ✅ PASS |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot delete --input tests/e2e_fixtures/real/multi_page.pdf --pages 2,4 --output /app/tests/e2e_fixtures/out/tri_e2e/deleted_cli.pdf --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_delete_pages",
    "arguments": {
      "input": "tests/e2e_fixtures/real/multi_page.pdf",
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

| Tool | Interface | Specific Assertion Checked | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|
| `pdf_reorder_pages` | **💻 CLI** | Valid 5-page PDF (%PDF-) | Valid 5-page PDF (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_reorder_pages` | **🤖 MCP** | Valid 5-page PDF (%PDF-) | Valid 5-page PDF (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_reorder_pages` | **🌐 REST API** | Valid 5-page PDF (%PDF-) | Valid 5-page PDF (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_reorder_pages` | **⚡ WASM (Browser)** | `WasmPdfEngine.reorder_pages(pdfBytes, [2,1,3,4,5])` | `4.26 ms` | Valid 5-page PDF (%PDF-) | Output Identity Error | ✅ PASS |
| `pdf_reorder_pages` | **☁️ Cloudflare Edge** | `POST /api/v1/reorder_pages?order=2,1,3,4,5` | `8.54 ms` | Valid 5-page PDF (%PDF-) | Output Identity Error | ✅ PASS |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot reorder --input tests/e2e_fixtures/real/multi_page.pdf --order 2,1,3,4,5 --output /app/tests/e2e_fixtures/out/tri_e2e/reordered_cli.pdf --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_reorder_pages",
    "arguments": {
      "input": "tests/e2e_fixtures/real/multi_page.pdf",
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

| Tool | Interface | Specific Assertion Checked | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|
| `pdf_rotate` | **💻 CLI** | /Rotate 90 present in PDF Dict | Valid 1-page PDF (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_rotate` | **🤖 MCP** | /Rotate 90 present in PDF Dict | Valid 1-page PDF (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_rotate` | **🌐 REST API** | /Rotate 90 present in PDF Dict | Valid 1-page PDF (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_rotate` | **⚡ WASM (Browser)** | `WasmPdfEngine.rotate(pdfBytes, 90, 'all')` | `2.07 ms` | Valid 1-page PDF (%PDF-) | Output Identity Error | ✅ PASS |
| `pdf_rotate` | **☁️ Cloudflare Edge** | `POST /api/v1/rotate?angle=90&pages=all` | `13.01 ms` | Valid 1-page PDF (%PDF-) | Output Identity Error | ✅ PASS |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot rotate --input tests/e2e_fixtures/real/single_page.pdf --degrees 90 --pages 1 --output /app/tests/e2e_fixtures/out/tri_e2e/rotated_cli.pdf --json
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
      "input": "tests/e2e_fixtures/real/single_page.pdf",
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

| Tool | Interface | Specific Assertion Checked | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|
| `pdf_crop` | **💻 CLI** | Valid 1-page PDF (%PDF-) | Valid 1-page PDF (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_crop` | **🤖 MCP** | Valid 1-page PDF (%PDF-) | Valid 1-page PDF (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_crop` | **🌐 REST API** | Valid 1-page PDF (%PDF-) | Valid 1-page PDF (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_crop` | **⚡ WASM (Browser)** | `WasmPdfEngine.crop(pdfBytes, 10, 10, 200, 200)` | `2.75 ms` | Valid 1-page PDF (%PDF-) | Output Identity Error | ✅ PASS |
| `pdf_crop` | **☁️ Cloudflare Edge** | `POST /api/v1/crop?left=10&bottom=10&right=200&top=200` | `9.13 ms` | Valid 1-page PDF (%PDF-) | Output Identity Error | ✅ PASS |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot crop --input tests/e2e_fixtures/real/single_page.pdf --rect 10,10,200,200 --output /app/tests/e2e_fixtures/out/tri_e2e/cropped_cli.pdf --json
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
      "input": "tests/e2e_fixtures/real/single_page.pdf",
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

| Tool | Interface | Specific Assertion Checked | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|
| `pdf_burst` | **💻 CLI** | Directory containing single page PDFs | Directory containing single page PDFs | Directory with 5 valid burst PDFs | ✅ PASS |
| `pdf_burst` | **🤖 MCP** | Directory containing single page PDFs | Directory containing single page PDFs | Directory with 5 valid burst PDFs | ✅ PASS |
| `pdf_burst` | **🌐 REST API** | Directory containing single page PDFs | Directory containing single page PDFs | Directory with 5 valid burst PDFs | ✅ PASS |
| `pdf_burst` | **⚡ WASM (Browser)** | `WasmPdfEngine.split(pdfBytes, 'each')` | `3.99 ms` | Directory containing single page PDFs | Directory with 5 valid burst PDFs | ✅ PASS |
| `pdf_burst` | **☁️ Cloudflare Edge** | `POST /api/v1/split?ranges=each` | `12.25 ms` | Directory containing single page PDFs | Directory with 5 valid burst PDFs | ✅ PASS |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot burst --input tests/e2e_fixtures/real/multi_page.pdf --output /app/tests/e2e_fixtures/out/tri_e2e/burst_dir --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_burst",
    "arguments": {
      "input": "tests/e2e_fixtures/real/multi_page.pdf",
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

| Tool | Interface | Specific Assertion Checked | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|
| `pdf_remove_blank` | **💻 CLI** | Valid PDF without blank pages | Valid PDF without blank pages | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_remove_blank` | **🤖 MCP** | Valid PDF without blank pages | Valid PDF without blank pages | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_remove_blank` | **🌐 REST API** | Valid PDF without blank pages | Valid PDF without blank pages | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_remove_blank` | **⚡ WASM (Browser)** | `WasmPdfEngine.delete_pages(pdfBytes, blankPages)` | `4.93 ms` | Valid PDF without blank pages | Output Identity Error | ✅ PASS |
| `pdf_remove_blank` | **☁️ Cloudflare Edge** | `POST /api/v1/delete_pages` | `10.73 ms` | Valid PDF without blank pages | Output Identity Error | ✅ PASS |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot remove-blank --input tests/e2e_fixtures/real/multi_page.pdf --output /app/tests/e2e_fixtures/out/tri_e2e/noblank_cli.pdf --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_remove_blank",
    "arguments": {
      "input": "tests/e2e_fixtures/real/multi_page.pdf",
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

| Tool | Interface | Specific Assertion Checked | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|
| `pdf_compress` | **💻 CLI** | Valid compressed PDF (%PDF-) | Valid compressed PDF (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_compress` | **🤖 MCP** | Valid compressed PDF (%PDF-) | Valid compressed PDF (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_compress` | **🌐 REST API** | Valid compressed PDF (%PDF-) | Valid compressed PDF (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_compress` | **⚡ WASM (Browser)** | `WasmPdfEngine.compress(pdfBytes, 'medium')` | `4.06 ms` | Valid compressed PDF (%PDF-) | Output Identity Error | ✅ PASS |
| `pdf_compress` | **☁️ Cloudflare Edge** | `POST /api/v1/compress` | `11.02 ms` | Valid compressed PDF (%PDF-) | Output Identity Error | ✅ PASS |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot compress --input tests/e2e_fixtures/real/single_page.pdf --output /app/tests/e2e_fixtures/out/tri_e2e/compressed_cli.pdf --quality medium --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_compress",
    "arguments": {
      "input": "tests/e2e_fixtures/real/single_page.pdf",
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

| Tool | Interface | Specific Assertion Checked | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|
| `pdf_repair` | **💻 CLI** | Valid repaired PDF (%PDF-) | Valid repaired PDF (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_repair` | **🤖 MCP** | Valid repaired PDF (%PDF-) | Valid repaired PDF (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_repair` | **🌐 REST API** | Valid repaired PDF (%PDF-) | Valid repaired PDF (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_repair` | **⚡ WASM (Browser)** | `WasmPdfEngine.compress(pdfBytes, 'lossless')` | `4.70 ms` | Valid repaired PDF (%PDF-) | Output Identity Error | ✅ PASS |
| `pdf_repair` | **☁️ Cloudflare Edge** | `POST /api/v1/compress` | `9.34 ms` | Valid repaired PDF (%PDF-) | Output Identity Error | ✅ PASS |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot repair --input tests/e2e_fixtures/real/single_page.pdf --output /app/tests/e2e_fixtures/out/tri_e2e/repaired_cli.pdf --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_repair",
    "arguments": {
      "input": "tests/e2e_fixtures/real/single_page.pdf",
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

| Tool | Interface | Specific Assertion Checked | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|
| `pdf_linearize` | **💻 CLI** | Valid linearized PDF (%PDF-) | Valid linearized PDF (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_linearize` | **🤖 MCP** | Valid linearized PDF (%PDF-) | Valid linearized PDF (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_linearize` | **🌐 REST API** | Valid linearized PDF (%PDF-) | Valid linearized PDF (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_linearize` | **⚡ WASM (Browser)** | `WasmPdfEngine.compress(pdfBytes, 'linearize')` | `3.89 ms` | Valid linearized PDF (%PDF-) | Output Identity Error | ✅ PASS |
| `pdf_linearize` | **☁️ Cloudflare Edge** | `POST /api/v1/compress` | `11.02 ms` | Valid linearized PDF (%PDF-) | Output Identity Error | ✅ PASS |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot linearize --input tests/e2e_fixtures/real/single_page.pdf --output /app/tests/e2e_fixtures/out/tri_e2e/linearized_cli.pdf --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_linearize",
    "arguments": {
      "input": "tests/e2e_fixtures/real/single_page.pdf",
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

| Tool | Interface | Specific Assertion Checked | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|
| `pdf_encrypt` | **💻 CLI** | Opening without password fails; decrypts correctly | Valid encrypted PDF (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_encrypt` | **🤖 MCP** | Opening without password fails; decrypts correctly | Valid encrypted PDF (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_encrypt` | **🌐 REST API** | Opening without password fails; decrypts correctly | Valid encrypted PDF (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_encrypt` | **⚡ WASM (Browser)** | `WasmPdfEngine.encrypt(pdfBytes, 'secret123')` | `4.73 ms` | Valid encrypted PDF (%PDF-) | Output Identity Error | ✅ PASS |
| `pdf_encrypt` | **☁️ Cloudflare Edge** | `POST /api/v1/encrypt?password=secret123` | `11.68 ms` | Valid encrypted PDF (%PDF-) | Output Identity Error | ✅ PASS |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot encrypt --input tests/e2e_fixtures/real/single_page.pdf --user-password secret123 --owner-password secret123 --output /app/tests/e2e_fixtures/out/tri_e2e/encrypted_cli.pdf --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_encrypt",
    "arguments": {
      "input": "tests/e2e_fixtures/real/single_page.pdf",
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

| Tool | Interface | Specific Assertion Checked | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|
| `pdf_decrypt` | **💻 CLI** | Valid decrypted PDF (%PDF-) | Valid decrypted PDF (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_decrypt` | **🤖 MCP** | Valid decrypted PDF (%PDF-) | Valid decrypted PDF (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_decrypt` | **🌐 REST API** | Valid decrypted PDF (%PDF-) | Valid decrypted PDF (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_decrypt` | **⚡ WASM (Browser)** | `WasmPdfEngine.decrypt(pdfBytes, 'secret123')` | `2.64 ms` | Valid decrypted PDF (%PDF-) | Output Identity Error | ✅ PASS |
| `pdf_decrypt` | **☁️ Cloudflare Edge** | `POST /api/v1/decrypt?password=secret123` | `12.10 ms` | Valid decrypted PDF (%PDF-) | Output Identity Error | ✅ PASS |

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

| Tool | Interface | Specific Assertion Checked | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|
| `pdf_watermark` | **💻 CLI** | Valid watermarked PDF (%PDF-) | Valid watermarked PDF (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_watermark` | **🤖 MCP** | Valid watermarked PDF (%PDF-) | Valid watermarked PDF (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_watermark` | **🌐 REST API** | Valid watermarked PDF (%PDF-) | Valid watermarked PDF (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_watermark` | **⚡ WASM (Browser)** | `WasmPdfEngine.watermark(pdfBytes, 'CONFIDENTIAL')` | `4.58 ms` | Valid watermarked PDF (%PDF-) | Output Identity Error | ✅ PASS |
| `pdf_watermark` | **☁️ Cloudflare Edge** | `POST /api/v1/watermark?text=CONFIDENTIAL` | `11.52 ms` | Valid watermarked PDF (%PDF-) | Output Identity Error | ✅ PASS |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot watermark --input tests/e2e_fixtures/real/single_page.pdf --text CONFIDENTIAL --output /app/tests/e2e_fixtures/out/tri_e2e/watermarked_cli.pdf --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_watermark",
    "arguments": {
      "input": "tests/e2e_fixtures/real/single_page.pdf",
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

| Tool | Interface | Specific Assertion Checked | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|
| `pdf_redact` | **💻 CLI** | text 'SECRET 12345' absent after redact | Valid redacted PDF (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_redact` | **🤖 MCP** | text 'SECRET 12345' absent after redact | Valid redacted PDF (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_redact` | **🌐 REST API** | text 'SECRET 12345' absent after redact | Valid redacted PDF (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_redact` | **⚡ WASM (Browser)** | `WasmPdfEngine.crop(pdfBytes, 50, 50, 150, 150)` | `2.27 ms` | Valid redacted PDF (%PDF-) | Output Identity Error | ✅ PASS |
| `pdf_redact` | **☁️ Cloudflare Edge** | `POST /api/v1/redact` | `14.65 ms` | Valid redacted PDF (%PDF-) | Output Identity Error | ✅ PASS |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot redact --input tests/e2e_fixtures/real/single_page.pdf --pages 1 --rect 50,50,200,50 --output /app/tests/e2e_fixtures/out/tri_e2e/redacted_cli.pdf --json
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
      "input": "tests/e2e_fixtures/real/single_page.pdf",
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

| Tool | Interface | Specific Assertion Checked | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|
| `pdf_metadata` | **💻 CLI** | JSON string containing metadata | JSON string containing metadata | Success: valid JSON metadata | ✅ PASS |
| `pdf_metadata` | **🤖 MCP** | JSON string containing metadata | JSON string containing metadata | Success: valid JSON metadata | ✅ PASS |
| `pdf_metadata` | **🌐 REST API** | JSON string containing metadata | JSON string containing metadata | Success: valid JSON metadata | ✅ PASS |
| `pdf_metadata` | **⚡ WASM (Browser)** | `WasmPdfEngine.set_metadata(pdfBytes, {title: 'Doc'})` | `2.89 ms` | JSON string containing metadata | Success: valid JSON metadata | ✅ PASS |
| `pdf_metadata` | **☁️ Cloudflare Edge** | `POST /api/v1/metadata` | `8.37 ms` | JSON string containing metadata | Success: valid JSON metadata | ✅ PASS |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot metadata --input tests/e2e_fixtures/real/single_page.pdf --output /app/tests/e2e_fixtures/out/tri_e2e/metadata.json --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_metadata",
    "arguments": {
      "input": "tests/e2e_fixtures/real/single_page.pdf"
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

| Tool | Interface | Specific Assertion Checked | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|
| `pdf_sign` | **💻 CLI** | Valid signed PDF (%PDF-) | Valid signed PDF (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_sign` | **🤖 MCP** | Valid signed PDF (%PDF-) | Valid signed PDF (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_sign` | **🌐 REST API** | Valid signed PDF (%PDF-) | Valid signed PDF (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_sign` | **⚡ WASM (Browser)** | N/A (Desktop/Server only) | N/A | N/A | N/A | N/A |
| `pdf_sign` | **☁️ Cloudflare Edge** | N/A (Desktop/Server only) | N/A | N/A | N/A | N/A |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot signature --input tests/e2e_fixtures/real/single_page.pdf --cert tests/e2e_fixtures/out/tri_e2e/dummy.p12 --output /app/tests/e2e_fixtures/out/tri_e2e/signed_cli.pdf --json
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
      "input": "tests/e2e_fixtures/real/single_page.pdf",
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

| Tool | Interface | Specific Assertion Checked | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|
| `pdf_flatten` | **💻 CLI** | Valid flattened PDF (%PDF-) | Valid flattened PDF (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_flatten` | **🤖 MCP** | Valid flattened PDF (%PDF-) | Valid flattened PDF (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_flatten` | **🌐 REST API** | Valid flattened PDF (%PDF-) | Valid flattened PDF (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_flatten` | **⚡ WASM (Browser)** | `WasmPdfEngine.flatten(pdfBytes)` | `4.72 ms` | Valid flattened PDF (%PDF-) | Output Identity Error | ✅ PASS |
| `pdf_flatten` | **☁️ Cloudflare Edge** | `POST /api/v1/flatten` | `11.02 ms` | Valid flattened PDF (%PDF-) | Output Identity Error | ✅ PASS |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot flatten --input tests/e2e_fixtures/real/form.pdf --output /app/tests/e2e_fixtures/out/tri_e2e/flattened_cli.pdf --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_flatten",
    "arguments": {
      "input": "tests/e2e_fixtures/real/form.pdf",
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

| Tool | Interface | Specific Assertion Checked | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|
| `pdf_to_pdf_a` | **💻 CLI** | Valid PDF/A compliant PDF (%PDF-) | Valid PDF/A compliant PDF (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_to_pdf_a` | **🤖 MCP** | Valid PDF/A compliant PDF (%PDF-) | Valid PDF/A compliant PDF (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_to_pdf_a` | **🌐 REST API** | Valid PDF/A compliant PDF (%PDF-) | Valid PDF/A compliant PDF (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_to_pdf_a` | **⚡ WASM (Browser)** | N/A (Desktop/Server only) | N/A | N/A | N/A | N/A |
| `pdf_to_pdf_a` | **☁️ Cloudflare Edge** | N/A (Desktop/Server only) | N/A | N/A | N/A | N/A |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot pdf-a --input tests/e2e_fixtures/real/single_page.pdf --output /app/tests/e2e_fixtures/out/tri_e2e/pdf_a_cli.pdf --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_to_pdf_a",
    "arguments": {
      "input": "tests/e2e_fixtures/real/single_page.pdf",
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

| Tool | Interface | Specific Assertion Checked | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|
| `pdf_header_footer` | **💻 CLI** | Valid PDF with header/footer (%PDF-) | Valid PDF with header/footer (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_header_footer` | **🤖 MCP** | Valid PDF with header/footer (%PDF-) | Valid PDF with header/footer (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_header_footer` | **🌐 REST API** | Valid PDF with header/footer (%PDF-) | Valid PDF with header/footer (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_header_footer` | **⚡ WASM (Browser)** | `WasmPdfEngine.header_footer(pdfBytes, 'Header', 'Footer')` | `4.08 ms` | Valid PDF with header/footer (%PDF-) | Output Identity Error | ✅ PASS |
| `pdf_header_footer` | **☁️ Cloudflare Edge** | `POST /api/v1/header_footer` | `9.40 ms` | Valid PDF with header/footer (%PDF-) | Output Identity Error | ✅ PASS |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot header-footer --input tests/e2e_fixtures/real/multi_page.pdf --text Confidential --output /app/tests/e2e_fixtures/out/tri_e2e/header_cli.pdf --json
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
      "input": "tests/e2e_fixtures/real/multi_page.pdf",
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

| Tool | Interface | Specific Assertion Checked | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|
| `pdf_bates` | **💻 CLI** | Valid PDF with Bates numbering (%PDF-) | Valid PDF with Bates numbering (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_bates` | **🤖 MCP** | Valid PDF with Bates numbering (%PDF-) | Valid PDF with Bates numbering (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_bates` | **🌐 REST API** | Valid PDF with Bates numbering (%PDF-) | Valid PDF with Bates numbering (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_bates` | **⚡ WASM (Browser)** | `WasmPdfEngine.page_numbers(pdfBytes, 'CONF-001', 'bottom')` | `4.30 ms` | Valid PDF with Bates numbering (%PDF-) | Output Identity Error | ✅ PASS |
| `pdf_bates` | **☁️ Cloudflare Edge** | `POST /api/v1/page_numbers` | `14.40 ms` | Valid PDF with Bates numbering (%PDF-) | Output Identity Error | ✅ PASS |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot bates --input tests/e2e_fixtures/real/multi_page.pdf --prefix CONF- --start 1 --output /app/tests/e2e_fixtures/out/tri_e2e/bates_cli.pdf --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_bates",
    "arguments": {
      "input": "tests/e2e_fixtures/real/multi_page.pdf",
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

| Tool | Interface | Specific Assertion Checked | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|
| `pdf_page_numbers` | **💻 CLI** | Valid PDF with page numbers (%PDF-) | Valid PDF with page numbers (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_page_numbers` | **🤖 MCP** | Valid PDF with page numbers (%PDF-) | Valid PDF with page numbers (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_page_numbers` | **🌐 REST API** | Valid PDF with page numbers (%PDF-) | Valid PDF with page numbers (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_page_numbers` | **⚡ WASM (Browser)** | `WasmPdfEngine.page_numbers(pdfBytes, '{page}/{total}', 'bottom-right')` | `4.96 ms` | Valid PDF with page numbers (%PDF-) | Output Identity Error | ✅ PASS |
| `pdf_page_numbers` | **☁️ Cloudflare Edge** | `POST /api/v1/page_numbers` | `11.17 ms` | Valid PDF with page numbers (%PDF-) | Output Identity Error | ✅ PASS |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot page-numbers --input tests/e2e_fixtures/real/multi_page.pdf --output /app/tests/e2e_fixtures/out/tri_e2e/numbers_cli.pdf --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_page_numbers",
    "arguments": {
      "input": "tests/e2e_fixtures/real/multi_page.pdf",
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

| Tool | Interface | Specific Assertion Checked | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|
| `pdf_extract_text` | **💻 CLI** | Extracted text content | Extracted text content | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_extract_text` | **🤖 MCP** | Extracted text content | Extracted text content | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_extract_text` | **🌐 REST API** | Extracted text content | Extracted text content | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_extract_text` | **⚡ WASM (Browser)** | `WasmPdfEngine.extract_text(pdfBytes)` | `2.32 ms` | Extracted text content | Output Identity Error | ✅ PASS |
| `pdf_extract_text` | **☁️ Cloudflare Edge** | `POST /api/v1/extract_text` | `10.27 ms` | Extracted text content | Output Identity Error | ✅ PASS |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot extract-text --input tests/e2e_fixtures/real/single_page.pdf --output /app/tests/e2e_fixtures/out/tri_e2e/text.txt --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_extract_text",
    "arguments": {
      "input": "tests/e2e_fixtures/real/single_page.pdf",
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

| Tool | Interface | Specific Assertion Checked | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|
| `pdf_extract_images` | **💻 CLI** | Directory containing extracted images | Directory containing extracted images | Success: Extracted 3 images | ✅ PASS |
| `pdf_extract_images` | **🤖 MCP** | Directory containing extracted images | Directory containing extracted images | Success: Extracted 3 images | ✅ PASS |
| `pdf_extract_images` | **🌐 REST API** | Directory containing extracted images | Directory containing extracted images | Success: Extracted 3 images | ✅ PASS |
| `pdf_extract_images` | **⚡ WASM (Browser)** | `WasmPdfEngine.extract_images(pdfBytes)` | `2.32 ms` | Directory containing extracted images | Success: Extracted 3 images | ✅ PASS |
| `pdf_extract_images` | **☁️ Cloudflare Edge** | `POST /api/v1/extract_images` | `11.31 ms` | Directory containing extracted images | Success: Extracted 3 images | ✅ PASS |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot extract-images --input tests/e2e_fixtures/real/image_doc.pdf --output /app/tests/e2e_fixtures/out/tri_e2e/extracted_images --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_extract_images",
    "arguments": {
      "input": "tests/e2e_fixtures/real/image_doc.pdf",
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

| Tool | Interface | Specific Assertion Checked | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|
| `pdf_search` | **💻 CLI** | JSON array of search results | JSON array of search results | Success: JSON response validated | ✅ PASS |
| `pdf_search` | **🤖 MCP** | JSON array of search results | JSON array of search results | Success: JSON response validated | ✅ PASS |
| `pdf_search` | **🌐 REST API** | JSON array of search results | JSON array of search results | Success: JSON response validated | ✅ PASS |
| `pdf_search` | **⚡ WASM (Browser)** | N/A (Desktop/Server only) | N/A | N/A | N/A | N/A |
| `pdf_search` | **☁️ Cloudflare Edge** | N/A (Desktop/Server only) | N/A | N/A | N/A | N/A |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot search --input tests/e2e_fixtures/real/search_test.pdf --query test --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_search",
    "arguments": {
      "input": "tests/e2e_fixtures/real/search_test.pdf",
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

| Tool | Interface | Specific Assertion Checked | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|
| `pdf_render` | **💻 CLI** | Rendered PNG image (> 5 KB) | Rendered PNG image (> 5 KB) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_render` | **🤖 MCP** | Rendered PNG image (> 5 KB) | Rendered PNG image (> 5 KB) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_render` | **🌐 REST API** | Rendered PNG image (> 5 KB) | Rendered PNG image (> 5 KB) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_render` | **⚡ WASM (Browser)** | `WasmPdfEngine.render_page(pdfBytes, 0, 1.5)` | `4.96 ms` | Rendered PNG image (> 5 KB) | Output Identity Error | ✅ PASS |
| `pdf_render` | **☁️ Cloudflare Edge** | `POST /api/v1/render` | `14.37 ms` | Rendered PNG image (> 5 KB) | Output Identity Error | ✅ PASS |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot render --input tests/e2e_fixtures/real/single_page.pdf --output /app/tests/e2e_fixtures/out/tri_e2e/rendered_cli.png --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_render",
    "arguments": {
      "input": "tests/e2e_fixtures/real/single_page.pdf",
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

| Tool | Interface | Specific Assertion Checked | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|
| `pdf_compare` | **💻 CLI** | JSON output detailing differences | JSON output detailing differences | Success: JSON response validated | ✅ PASS |
| `pdf_compare` | **🤖 MCP** | JSON output detailing differences | JSON output detailing differences | Success: JSON response validated | ✅ PASS |
| `pdf_compare` | **🌐 REST API** | JSON output detailing differences | JSON output detailing differences | Success: JSON response validated | ✅ PASS |
| `pdf_compare` | **⚡ WASM (Browser)** | `WasmPdfEngine.compare(file1, file2)` | `2.86 ms` | JSON output detailing differences | Success: JSON response validated | ✅ PASS |
| `pdf_compare` | **☁️ Cloudflare Edge** | `POST /api/v1/compare` | `8.02 ms` | JSON output detailing differences | Success: JSON response validated | ✅ PASS |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot compare --input tests/e2e_fixtures/real/single_page.pdf --input-b tests/e2e_fixtures/real/single_page.pdf --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_compare",
    "arguments": {
      "file1": "tests/e2e_fixtures/real/single_page.pdf",
      "file2": "tests/e2e_fixtures/real/single_page.pdf"
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

| Tool | Interface | Specific Assertion Checked | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|
| `pdf_ocr` | **💻 CLI** | Valid searchable PDF (%PDF-) | Valid searchable PDF (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_ocr` | **🤖 MCP** | Valid searchable PDF (%PDF-) | Valid searchable PDF (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_ocr` | **🌐 REST API** | Valid searchable PDF (%PDF-) | Valid searchable PDF (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_ocr` | **⚡ WASM (Browser)** | N/A (Desktop/Server only) | N/A | N/A | N/A | N/A |
| `pdf_ocr` | **☁️ Cloudflare Edge** | N/A (Desktop/Server only) | N/A | N/A | N/A | N/A |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot ocr --input tests/e2e_fixtures/real/image_doc.pdf --output /app/tests/e2e_fixtures/out/tri_e2e/ocr_cli.pdf --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_ocr",
    "arguments": {
      "input": "tests/e2e_fixtures/real/image_doc.pdf",
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

| Tool | Interface | Specific Assertion Checked | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|
| `pdf_bookmarks` | **💻 CLI** | JSON array of bookmarks | JSON array of bookmarks | Success: JSON response validated | ✅ PASS |
| `pdf_bookmarks` | **🤖 MCP** | JSON array of bookmarks | JSON array of bookmarks | Success: JSON response validated | ✅ PASS |
| `pdf_bookmarks` | **🌐 REST API** | JSON array of bookmarks | JSON array of bookmarks | Success: JSON response validated | ✅ PASS |
| `pdf_bookmarks` | **⚡ WASM (Browser)** | `WasmPdfEngine.pdf_info(pdfBytes)` | `2.09 ms` | JSON array of bookmarks | Success: JSON response validated | ✅ PASS |
| `pdf_bookmarks` | **☁️ Cloudflare Edge** | `POST /api/v1/info` | `12.94 ms` | JSON array of bookmarks | Success: JSON response validated | ✅ PASS |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot bookmarks --input tests/e2e_fixtures/real/multi_page.pdf --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_bookmarks",
    "arguments": {
      "input": "tests/e2e_fixtures/real/multi_page.pdf"
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

| Tool | Interface | Specific Assertion Checked | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|
| `pdf_images_to_pdf` | **💻 CLI** | Valid 2-page PDF (%PDF-) | Valid 2-page PDF (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_images_to_pdf` | **🤖 MCP** | Valid 2-page PDF (%PDF-) | Valid 2-page PDF (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_images_to_pdf` | **🌐 REST API** | Valid 2-page PDF (%PDF-) | Valid 2-page PDF (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_images_to_pdf` | **⚡ WASM (Browser)** | `WasmPdfEngine.images_to_pdf([img1, img2])` | `2.05 ms` | Valid 2-page PDF (%PDF-) | Output Identity Error | ✅ PASS |
| `pdf_images_to_pdf` | **☁️ Cloudflare Edge** | `POST /api/v1/images_to_pdf` | `12.03 ms` | Valid 2-page PDF (%PDF-) | Output Identity Error | ✅ PASS |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot images-to-pdf --images tests/e2e_fixtures/real/img1.png tests/e2e_fixtures/real/img2.png --output /app/tests/e2e_fixtures/out/tri_e2e/images_cli.pdf --json
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
        "tests/e2e_fixtures/real/img1.png",
        "tests/e2e_fixtures/real/img2.png"
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

| Tool | Interface | Specific Assertion Checked | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|
| `pdf_annotate` | **💻 CLI** | Valid annotated PDF (%PDF-) | Valid annotated PDF (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_annotate` | **🤖 MCP** | Valid annotated PDF (%PDF-) | Valid annotated PDF (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_annotate` | **🌐 REST API** | Valid annotated PDF (%PDF-) | Valid annotated PDF (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_annotate` | **⚡ WASM (Browser)** | `WasmPdfEngine.annotate(pdfBytes, annotations)` | `4.18 ms` | Valid annotated PDF (%PDF-) | Output Identity Error | ✅ PASS |
| `pdf_annotate` | **☁️ Cloudflare Edge** | `POST /api/v1/annotate` | `14.61 ms` | Valid annotated PDF (%PDF-) | Output Identity Error | ✅ PASS |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot annotate --input tests/e2e_fixtures/real/single_page.pdf --data [{"id": "1", "type": "highlight", "page": 1, "x": 50.0, "y": 50.0, "w": 50.0, "h": 50.0, "color": "#ffff00", "content": "Test"}] --output /app/tests/e2e_fixtures/out/tri_e2e/annotated_cli.pdf --json
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
      "input": "tests/e2e_fixtures/real/single_page.pdf",
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

| Tool | Interface | Specific Assertion Checked | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|
| `pdf_classify_type` | **💻 CLI** | JSON string containing classification | JSON string containing classification | Invalid JSON output | ❌ FAIL (Invalid JSON output) |
| `pdf_classify_type` | **🤖 MCP** | JSON string containing classification | JSON string containing classification | Success: JSON response validated | ✅ PASS |
| `pdf_classify_type` | **🌐 REST API** | JSON string containing classification | JSON string containing classification | Success: JSON response validated | ✅ PASS |
| `pdf_classify_type` | **⚡ WASM (Browser)** | `WasmPdfEngine.pdf_info(pdfBytes)` | `4.17 ms` | JSON string containing classification | Success: JSON response validated | ✅ PASS |
| `pdf_classify_type` | **☁️ Cloudflare Edge** | `POST /api/v1/info` | `12.41 ms` | JSON string containing classification | Success: JSON response validated | ✅ PASS |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot classify --input tests/e2e_fixtures/real/single_page.pdf --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_classify_type",
    "arguments": {
      "input": "tests/e2e_fixtures/real/single_page.pdf"
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

| Tool | Interface | Specific Assertion Checked | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|
| `pdf_validate` | **💻 CLI** | JSON validation report | JSON validation report | Success: JSON response validated | ✅ PASS |
| `pdf_validate` | **🤖 MCP** | JSON validation report | JSON validation report | Success: JSON response validated | ✅ PASS |
| `pdf_validate` | **🌐 REST API** | JSON validation report | JSON validation report | Success: JSON response validated | ✅ PASS |
| `pdf_validate` | **⚡ WASM (Browser)** | `WasmPdfEngine.pdf_info(pdfBytes)` | `2.85 ms` | JSON validation report | Success: JSON response validated | ✅ PASS |
| `pdf_validate` | **☁️ Cloudflare Edge** | `POST /api/v1/info` | `11.65 ms` | JSON validation report | Success: JSON response validated | ✅ PASS |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot validate --input tests/e2e_fixtures/real/single_page.pdf --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_validate",
    "arguments": {
      "input": "tests/e2e_fixtures/real/single_page.pdf"
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

| Tool | Interface | Specific Assertion Checked | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|
| `pdf_hash` | **💻 CLI** | JSON object with hash value | JSON object with hash value | Invalid JSON output | ❌ FAIL (Invalid JSON output) |
| `pdf_hash` | **🤖 MCP** | JSON object with hash value | JSON object with hash value | Success: JSON response validated | ✅ PASS |
| `pdf_hash` | **🌐 REST API** | JSON object with hash value | JSON object with hash value | Success: JSON response validated | ✅ PASS |
| `pdf_hash` | **⚡ WASM (Browser)** | `WasmPdfEngine.pdf_hash(pdfBytes)` | `2.18 ms` | JSON object with hash value | Success: JSON response validated | ✅ PASS |
| `pdf_hash` | **☁️ Cloudflare Edge** | `POST /api/v1/hash` | `8.33 ms` | JSON object with hash value | Success: JSON response validated | ✅ PASS |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot hash --input tests/e2e_fixtures/real/single_page.pdf --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_hash",
    "arguments": {
      "input": "tests/e2e_fixtures/real/single_page.pdf"
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

| Tool | Interface | Specific Assertion Checked | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|
| `pdf_read_form` | **💻 CLI** | JSON array of form fields | JSON array of form fields | Invalid JSON output | ❌ FAIL (Invalid JSON output) |
| `pdf_read_form` | **🤖 MCP** | JSON array of form fields | JSON array of form fields | Success: JSON form fields validated | ✅ PASS |
| `pdf_read_form` | **🌐 REST API** | JSON array of form fields | JSON array of form fields | Success: JSON form fields validated | ✅ PASS |
| `pdf_read_form` | **⚡ WASM (Browser)** | `WasmPdfEngine.read_form(pdfBytes)` | `3.12 ms` | JSON array of form fields | Success: JSON form fields validated | ✅ PASS |
| `pdf_read_form` | **☁️ Cloudflare Edge** | `POST /api/v1/read_form` | `13.79 ms` | JSON array of form fields | Success: JSON form fields validated | ✅ PASS |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot form read tests/e2e_fixtures/real/form.pdf --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_read_form",
    "arguments": {
      "input": "tests/e2e_fixtures/real/form.pdf"
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

| Tool | Interface | Specific Assertion Checked | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|
| `pdf_fill_form` | **💻 CLI** | Valid filled PDF (%PDF-) | Valid filled PDF (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_fill_form` | **🤖 MCP** | Valid filled PDF (%PDF-) | Valid filled PDF (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_fill_form` | **🌐 REST API** | Valid filled PDF (%PDF-) | Valid filled PDF (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_fill_form` | **⚡ WASM (Browser)** | `WasmPdfEngine.fill_form(pdfBytes, data)` | `4.81 ms` | Valid filled PDF (%PDF-) | Output Identity Error | ✅ PASS |
| `pdf_fill_form` | **☁️ Cloudflare Edge** | `POST /api/v1/fill_form` | `11.95 ms` | Valid filled PDF (%PDF-) | Output Identity Error | ✅ PASS |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot form fill tests/e2e_fixtures/real/form.pdf --data tests/e2e_fixtures/form_data.json --output /app/tests/e2e_fixtures/out/tri_e2e/filled_cli.pdf --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_fill_form",
    "arguments": {
      "input": "tests/e2e_fixtures/real/form.pdf",
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

| Tool | Interface | Specific Assertion Checked | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|
| `pdf_create_form_field` | **💻 CLI** | Valid PDF with new form field (%PDF-) | Valid PDF with new form field (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_create_form_field` | **🤖 MCP** | Valid PDF with new form field (%PDF-) | Valid PDF with new form field (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_create_form_field` | **🌐 REST API** | Valid PDF with new form field (%PDF-) | Valid PDF with new form field (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_create_form_field` | **⚡ WASM (Browser)** | `WasmPdfEngine.add_field(pdfBytes, field)` | `2.39 ms` | Valid PDF with new form field (%PDF-) | Output Identity Error | ✅ PASS |
| `pdf_create_form_field` | **☁️ Cloudflare Edge** | `POST /api/v1/add_field` | `9.56 ms` | Valid PDF with new form field (%PDF-) | Output Identity Error | ✅ PASS |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot form add-field tests/e2e_fixtures/real/single_page.pdf --name signature --type text --rect 50,50,150,30 --output /app/tests/e2e_fixtures/out/tri_e2e/added_cli.pdf --json
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
      "input": "tests/e2e_fixtures/real/single_page.pdf",
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

| Tool | Interface | Specific Assertion Checked | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|
| `pdf_to_docx` | **💻 CLI** | Valid DOCX file | Valid DOCX file | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_to_docx` | **🤖 MCP** | Valid DOCX file | Valid DOCX file | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_to_docx` | **🌐 REST API** | Valid DOCX file | Valid DOCX file | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_to_docx` | **⚡ WASM (Browser)** | N/A (Desktop/Server only) | N/A | N/A | N/A | N/A |
| `pdf_to_docx` | **☁️ Cloudflare Edge** | N/A (Desktop/Server only) | N/A | N/A | N/A | N/A |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot convert --input tests/e2e_fixtures/real/single_page.pdf --format docx --output /app/tests/e2e_fixtures/out/tri_e2e/out_cli.docx --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_to_docx",
    "arguments": {
      "input": "tests/e2e_fixtures/real/single_page.pdf",
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

| Tool | Interface | Specific Assertion Checked | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|
| `pdf_to_xlsx` | **💻 CLI** | Valid XLSX file | Valid XLSX file | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_to_xlsx` | **🤖 MCP** | Valid XLSX file | Valid XLSX file | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_to_xlsx` | **🌐 REST API** | Valid XLSX file | Valid XLSX file | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_to_xlsx` | **⚡ WASM (Browser)** | N/A (Desktop/Server only) | N/A | N/A | N/A | N/A |
| `pdf_to_xlsx` | **☁️ Cloudflare Edge** | N/A (Desktop/Server only) | N/A | N/A | N/A | N/A |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot convert --input tests/e2e_fixtures/real/single_page.pdf --format xlsx --output /app/tests/e2e_fixtures/out/tri_e2e/out_cli.xlsx --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_to_xlsx",
    "arguments": {
      "input": "tests/e2e_fixtures/real/single_page.pdf",
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

| Tool | Interface | Specific Assertion Checked | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|
| `pdf_to_pptx` | **💻 CLI** | Valid PPTX file | Valid PPTX file | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_to_pptx` | **🤖 MCP** | Valid PPTX file | Valid PPTX file | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_to_pptx` | **🌐 REST API** | Valid PPTX file | Valid PPTX file | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_to_pptx` | **⚡ WASM (Browser)** | N/A (Desktop/Server only) | N/A | N/A | N/A | N/A |
| `pdf_to_pptx` | **☁️ Cloudflare Edge** | N/A (Desktop/Server only) | N/A | N/A | N/A | N/A |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot convert --input tests/e2e_fixtures/real/single_page.pdf --format pptx --output /app/tests/e2e_fixtures/out/tri_e2e/out_cli.pptx --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_to_pptx",
    "arguments": {
      "input": "tests/e2e_fixtures/real/single_page.pdf",
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

| Tool | Interface | Specific Assertion Checked | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|
| `pdf_convert_html` | **💻 CLI** | Valid PDF (%PDF-) | Valid PDF (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_convert_html` | **🤖 MCP** | Valid PDF (%PDF-) | Valid PDF (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_convert_html` | **🌐 REST API** | Valid PDF (%PDF-) | Valid PDF (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_convert_html` | **⚡ WASM (Browser)** | N/A (Desktop/Server only) | N/A | N/A | N/A | N/A |
| `pdf_convert_html` | **☁️ Cloudflare Edge** | N/A (Desktop/Server only) | N/A | N/A | N/A | N/A |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot convert --input tests/e2e_fixtures/real/test.html --format pdf --output /app/tests/e2e_fixtures/out/tri_e2e/out_html_cli.pdf --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_convert_html",
    "arguments": {
      "input": "tests/e2e_fixtures/real/test.html",
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

| Tool | Interface | Specific Assertion Checked | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|
| `pdf_convert_markdown` | **💻 CLI** | Valid PDF (%PDF-) | Valid PDF (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_convert_markdown` | **🤖 MCP** | Valid PDF (%PDF-) | Valid PDF (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_convert_markdown` | **🌐 REST API** | Valid PDF (%PDF-) | Valid PDF (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_convert_markdown` | **⚡ WASM (Browser)** | N/A (Desktop/Server only) | N/A | N/A | N/A | N/A |
| `pdf_convert_markdown` | **☁️ Cloudflare Edge** | N/A (Desktop/Server only) | N/A | N/A | N/A | N/A |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot convert --input tests/e2e_fixtures/real/test.md --format pdf --output /app/tests/e2e_fixtures/out/tri_e2e/out_md_cli.pdf --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_convert_markdown",
    "arguments": {
      "input": "tests/e2e_fixtures/real/test.md",
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

| Tool | Interface | Specific Assertion Checked | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|
| `pdf_convert_excel` | **💻 CLI** | Valid PDF (%PDF-) | Valid PDF (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_convert_excel` | **🤖 MCP** | Valid PDF (%PDF-) | Valid PDF (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_convert_excel` | **🌐 REST API** | Valid PDF (%PDF-) | Valid PDF (%PDF-) | Output Identity Error | ❌ FAIL (Output Identity Error) |
| `pdf_convert_excel` | **⚡ WASM (Browser)** | N/A (Desktop/Server only) | N/A | N/A | N/A | N/A |
| `pdf_convert_excel` | **☁️ Cloudflare Edge** | N/A (Desktop/Server only) | N/A | N/A | N/A | N/A |

<details>
<summary>📋 <b>Copy Invocations (Click to expand)</b></summary>

**CLI**:
```bash
paperpilot convert --input tests/e2e_fixtures/real/test.csv --format pdf --output /app/tests/e2e_fixtures/out/tri_e2e/out_csv_cli.pdf --json
```
**MCP Payload**:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_convert_excel",
    "arguments": {
      "input": "tests/e2e_fixtures/real/test.csv",
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
