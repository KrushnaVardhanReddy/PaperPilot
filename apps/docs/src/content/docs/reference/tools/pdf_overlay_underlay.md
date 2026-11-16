---
title: "pdf_overlay_underlay"
description: "Reference, CLI, MCP, REST API, and WASM commands for pdf_overlay_underlay."
sidebar:
  order: 56
---

**Status**: ✅ Verified  |  **Format**: PDF (`.pdf`) $\to$ PDF (`.pdf`)

### What it does
**Applies letterheads, watermarked stationary, and templates as background or foreground.**  
Leverages PDF Form XObjects to embed stationary templates once and project them across selected target pages with configurable opacity and layer stacking mode (`underlay` behind existing content or `overlay` in the foreground).

---

### CLI
Apply letterhead or stationary onto target pages using `paperpilot pdf-overlay-underlay`:

```bash
paperpilot pdf-overlay-underlay \
  --input tests/e2e_fixtures/multi_page.pdf \
  --stationery tests/e2e_fixtures/single_page.pdf \
  --output tests/e2e_fixtures/out/overlay_underlay.pdf \
  --mode underlay \
  --opacity 0.85 \
  --pages "all" \
  --json
```

#### CLI Parameters
- `--input <INPUT>` *(string, required)*: Path to the input PDF document.
- `--output <OUTPUT>` *(string, required)*: Path where the processed PDF document will be written.
- `--stationery <STATIONERY>` or `--template <TEMPLATE>` *(string, optional)*: Path to the template/letterhead PDF file to apply.
- `--mode <MODE>` *(string, optional, default: `"underlay"`)*: Placement mode:
  - `"underlay"`: Renders template underneath existing page content (ideal for background stationery and watermarks).
  - `"overlay"`: Renders template on top of existing page content (ideal for stamps, framing, or security overlays).
- `--pages <PAGES>` *(string, optional, default: `"all"`)*: Page selection filter: `"all"`, `"first"`, `"last"`, or comma-delimited page indices such as `"1,3,5"`.
- `--opacity <OPACITY>` *(float, optional, default: `1.0`)*: Transparency factor ranging between `0.0` (invisible) and `1.0` (fully opaque).
- `--json` *(flag, optional)*: Emit structured JSON execution receipt upon completion.

---

### MCP JSON-RPC
Invoke the tool via Model Context Protocol (MCP) clients using `pdf_overlay_underlay`:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_overlay_underlay",
    "arguments": {
      "input": "tests/e2e_fixtures/multi_page.pdf",
      "output": "tests/e2e_fixtures/out/overlay_mcp.pdf",
      "template_path": "tests/e2e_fixtures/single_page.pdf",
      "mode": "underlay",
      "opacity": 0.85,
      "pages": "all"
    }
  },
  "id": 1
}
```

#### MCP Schema Parameters
- `input` *(string, required)*: Path to the source document.
- `output` *(string, required)*: Destination file path for the composite output.
- `template_path`, `stationery`, or `template` *(string, optional)*: File path to the overlay/underlay stationary document.
- `template_data` *(string, optional)*: Inline Base64-encoded PDF template data (for environments without shared filesystem access).
- `mode` *(string, optional, default: `"underlay"`)*: `"underlay"` (background) or `"overlay"` (foreground).
- `opacity` *(number, optional, default: `1.0`)*: Alpha opacity factor from `0.0` to `1.0`.
- `pages` *(string, optional, default: `"all"`)*: Target page specification (`"all"`, `"first"`, `"last"`, `"1,2"`).

---

### REST API
Execute stationery imposition using PaperPilot Gateway:

#### 1. Dedicated Endpoint (`/api/v1/pdf/overlay-underlay`)
```bash
curl -s --noproxy '*' -X POST http://127.0.0.1:7823/api/v1/pdf/overlay-underlay \
  -H "Content-Type: application/json" \
  -d '{
    "input": "tests/e2e_fixtures/multi_page.pdf",
    "template_path": "tests/e2e_fixtures/single_page.pdf",
    "output": "tests/e2e_fixtures/out/overlay_api.pdf",
    "mode": "underlay",
    "opacity": 0.85,
    "pages": "all"
  }'
```

#### 2. Canonical Tool Route (`/api/v1/pdf/tools/pdf_overlay_underlay`)
```bash
curl -s --noproxy '*' -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_overlay_underlay \
  -H "Content-Type: application/json" \
  -d '{
    "input": "tests/e2e_fixtures/multi_page.pdf",
    "template_path": "tests/e2e_fixtures/single_page.pdf",
    "output": "tests/e2e_fixtures/out/overlay_canonical.pdf",
    "mode": "overlay",
    "opacity": 1.0
  }'
```

---

### WebAssembly (WASM)
Execute client-side or worker stationery composition using `@paperpilot/wasm`:

```javascript
import { WasmPdfEngine } from 'paperpilot-wasm';

const options = JSON.stringify({
  template_data: "JVBERi0xLjUKMSAwIG9iago8PAovVHlwZSAvQ2F0YWxvZwovUGFnZXMgMiAwIFIKPj4KZW5kb2Jq...", // Base64 template PDF
  mode: "underlay",
  pages: "all",
  opacity: 0.8
});

// Returns Uint8Array of the watermarked/overlayed PDF bytes
const outputBytes = WasmPdfEngine.pdf_overlay_underlay(inputPdfBytes, options);
```

---

### Known Limitations & Troubleshooting
- **Template Page Selection**: When the template document contains multiple pages, page 1 of the template is matched to page 1 of the target document, or repeated across all pages if the template has fewer pages than the target.
- **Page Dimensions**: The stationary template Form XObject assumes matching MediaBox dimensions. If imposing a Letter template onto an A4 document, content may anchor to the bottom-left coordinate origin.
- **Proxy Considerations**: When invoking local REST endpoints, ensure `--noproxy '*'` or appropriate loopback bypass is configured if corporate or sandbox proxies intercept `127.0.0.1`.
