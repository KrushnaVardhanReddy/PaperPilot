---
title: "pdf_page_numbers"
description: "Reference, CLI, MCP, REST API, and WASM specifications for pdf_page_numbers."
sidebar:
  order: 34
---

**Status**: ✅ Verified  |  **Format**: PDF (`.pdf`)

### What it does
**Adds dynamic page numbering stamps (e.g. "Page 1 of 5", "1 / 10", or "Page 1") across PDF pages.**  
Stamps clean, professional page numbers at designated screen coordinates (`bottom-center`, `bottom-right`, `top-center`, `top-right`) without shifting or altering preexisting text or vector graphics.

---

### CLI
Apply page numbering stamps to PDF documents from the command line:

#### 1. Bottom-Center Positioning
```bash
paperpilot page-numbers \
  --input tests/e2e_fixtures/real/single_page.pdf \
  --position bottom-center \
  --output tests/e2e_fixtures/out/batch3_test/page_numbers.pdf \
  --json
```

#### 2. Bottom-Right Positioning
```bash
paperpilot page-numbers \
  --input tests/e2e_fixtures/real/multi_page.pdf \
  --position bottom-right \
  --output tests/e2e_fixtures/out/batch3_test/page_numbers_br.pdf \
  --json
```

#### CLI Parameters
- `--input` *(string, required)*: Path to source PDF document.
- `--output` *(string, required)*: Path to write the numbered PDF.
- `--position` *(string, optional, default: `"bottom-center"`)*: Stamp position anchor:
  - `bottom-center`
  - `bottom-right`
  - `top-center`
  - `top-right`
- `--json` *(flag, optional)*: Emit structured execution status in JSON format.

---

### MCP JSON-RPC
Invoke `pdf_page_numbers` via Model Context Protocol (MCP) clients with customizable template formats:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_page_numbers",
    "arguments": {
      "input": "tests/e2e_fixtures/real/single_page.pdf",
      "position": "bottom-right",
      "format": "Page {n} of {total}",
      "output": "tests/e2e_fixtures/out/batch3_test/mcp_page_numbers.pdf"
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
        "text": "{\"success\":true,\"message\":\"Page numbers added successfully.\",\"output_path\":\"tests/e2e_fixtures/out/batch3_test/mcp_page_numbers.pdf\",\"data\":null}"
      }
    ],
    "isError": false
  }
}
```

#### MCP Schema Parameters
- `input` *(string, required)*: The input PDF file path.
- `output` *(string, required)*: The output PDF file path.
- `position` *(string, optional, default: `"bottom-right"`)*: Target position (`"bottom-center"`, `"bottom-right"`, `"top-center"`, `"top-right"`).
- `format` *(string, optional, default: `"Page {n} of {total}"`)*: Custom numbering template using tokens:
  - `{n}`: Current page number (1-based index).
  - `{total}` or `{N}`: Total page count in document.

---

### REST API
Execute page numbering stamps via PaperPilot Gateway:

```bash
# Universal MCP Tool Route
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_page_numbers \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "position": "bottom-center",
    "format": "Page {n} of {total}",
    "output": "tests/e2e_fixtures/out/batch3_test/api_page_numbers.pdf"
  }'
```

---

### WASM (Web & Edge)
Run zero-dependency page numbering in web browsers or edge workers with `paperpilot-wasm`:

```typescript
import { WasmPdfEngine } from "paperpilot-wasm";

// Add page numbers to in-memory PDF buffer
const numberedBytes: Uint8Array = WasmPdfEngine.page_numbers(
  inputBytes,            // Uint8Array of input PDF
  "Page {n} of {total}", // Format string template
  "bottom-center"        // Position: "bottom-center" | "bottom-right" | "top-center" | "top-right"
);
```

---

### Known Limitations & Troubleshooting
- **MediaBox Relative Positioning**: Stamps calculate boundary margins relative to each page's declared `/MediaBox`. If a PDF contains non-zero bounding box origins (e.g. artboard crops or prepress bleeds), numbers will position relative to the lower-left corner of the printable MediaBox.
- **Header & Footer Collisions**: For multi-column headers and footers with custom left, center, and right text simultaneously, use `pdf_header_footer`. For legal discovery consecutive pagination, use `pdf_bates`.
