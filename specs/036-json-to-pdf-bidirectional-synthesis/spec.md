# Spec 036 — Bidirectional JSON-to-PDF Synthesis with Multimodal Images (`json_to_pdf`)

## 1. Overview & Objectives
PaperPilot currently supports `pdf_to_json` to extract document text and multimodal images into structured JSON ([Spec 032](../032-pdf-to-json-multimodal-images/spec.md)). To close the complete **Read ➔ Modify ➔ Synthesize** loop for autonomous AI agents, we introduce **`json_to_pdf`**.

This enables any AI Agent (Gemini, Claude, GPT-4) or API client to:
1. Ingest existing PDFs as JSON (`pdf_to_json`).
2. Programmatically alter text, translate languages, inject dynamic data, or replace images in the JSON payload.
3. Synthesize a brand new, valid, publication-grade PDF in single-digit milliseconds (`json_to_pdf`).
4. Generate new PDFs from scratch simply by emitting structured JSON without learning PDF byte specifications.

Per the **Penta-Interface Mandate & New Tool Law** ([`wiki/25-Penta-Interface-Mandate-And-E2E-Protocol.md`](../../wiki/25-Penta-Interface-Mandate-And-E2E-Protocol.md)), `json_to_pdf` is implemented as a first-class citizen across all **5 deployment surfaces**:
1. 💻 **CLI (`paperpilot-cli`)**: `paperpilot convert --input doc.json --format pdf --output doc.pdf`
2. 🤖 **MCP (`paperpilot-mcp`)**: `json_to_pdf` tool with `{ "input": "...", "output": "..." }` or raw `{ "json_content": "..." }`
3. 🌐 **REST API (`paperpilot-gateway`)**: POST `/api/v1/pdf/convert` and `/api/v1/pdf/tools/json_to_pdf`
4. ⚡ **WASM (`paperpilot-wasm`)**: `json_to_pdf(json_bytes)` returning PDF bytes in-browser memory
5. ☁️ **Cloudflare Edge (`apps/edge`)**: POST `/api/v1/json_to_pdf` streaming raw PDF bytes

---

## 2. JSON Schema Definition (Compatible with Spec 032)

`json_to_pdf` accepts the exact `JsonDocument` schema emitted by `pdf_to_json`:

```rust
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonImage {
    pub id: String,
    pub page_number: u32,
    pub format: String,             // "png" or "jpeg"
    pub width: u32,
    pub height: u32,
    pub bbox: Option<[f32; 4]>,     // [x0, y0, x1, y1] coordinates on page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base64_data: Option<String>, // "data:image/png;base64,..." or raw base64
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonPage {
    pub page_number: u32,
    pub text: String,
    #[serde(default)]
    pub char_count: usize,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub images: Vec<JsonImage>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonDocument {
    pub document_name: String,
    pub pages: Vec<JsonPage>,
}
```

---

## 3. Engine Architecture & Rendering Strategy (Approach 1: Native Pure-Rust Engine)

### 3.1 Architectural Decision: Native Engine (`lopdf` + `image`) vs Intermediate HTML (`fulgur`)
We adopt **Approach 1 (Pure-Rust Native Engine via `lopdf` + `image`)** as the official architectural standard for `json_to_pdf`:
1. **100% 5-Surface Parity**: Both `lopdf` and `image` compile cleanly to WebAssembly (`paperpilot-wasm`) and Cloudflare Workers (`apps/edge`) without depending on system fonts or heavyweight layout crates (`blitz`/`stylo`) required by HTML renderers.
2. **Pixel-Exact Spatial Fidelity**: Directly places images at their extractable `bbox: [x0, y0, x1, y1]` coordinates and restores exact original page geometry without CSS reflow artifacts.
3. **Sub-5ms Execution Latency**: Directly writes in-memory object streams with <2MB RSS memory consumption, maintaining our strict <35MB memory ceiling.

### 3.2 Implementation Details (`paperpilot-pdf`)
Implement `JsonToPdfOperation` in `paperpilot-pdf/src/operations/conversion.rs`:
1. Parse input JSON string/bytes into `JsonDocument`.
2. Construct a fresh `LopdfDocument` (`Document::with_version("1.5")`).
3. For each `JsonPage` in `pages`:
   - Create a `Page` object (Letter or A4: `[0, 0, 612, 792]`).
   - Add standard Helvetica font dictionary.
   - Text Layout: Split `page.text` by newlines and emit content stream operators (`BT`, `Tf`, `Td`, `Tj`, `ET`) maintaining line breaks and margins.
   - Image Layout: If `images` are present:
     - Decode image bytes from `base64_data` or load from `file_path`.
     - Inspect format and dimensions using `image` crate.
     - Inject `XObject` Image stream into PDF objects map (`ColorSpace: DeviceRGB`, `BitsPerComponent: 8`).
     - Position the image according to `bbox` (`[x0, y0, x1, y1]`) using `q`, `cm` (transformation matrix), `Do`, `Q` stream operations.
4. Finalize catalog, page tree hierarchy, trailer, and cross-reference tables.
5. In-memory execution: Can output directly to byte vector or save to path.

---

## 4. The 5 Deployment Surfaces

### 4.1 CLI (`paperpilot-cli`)
- Add support to `Commands::Convert`:
  ```bash
  paperpilot convert --input doc.json --format pdf --output doc.pdf
  ```
- Support reading JSON from stdin:
  ```bash
  cat doc.json | paperpilot convert --input - --format pdf --output doc.pdf
  ```

### 4.2 MCP Server (`paperpilot-mcp`)
- Tool Name: `json_to_pdf`
- JSON Schema:
  ```json
  {
    "name": "json_to_pdf",
    "description": "Synthesizes a publication-ready PDF document from structured JSON text and embedded images",
    "parameters": {
      "type": "object",
      "properties": {
        "input": { "type": "string", "description": "Path to input JSON file" },
        "json_content": { "type": "string", "description": "Raw JSON string (optional if input file path is given)" },
        "output": { "type": "string", "description": "Path to output PDF file" }
      },
      "required": ["output"]
    }
  }
  ```

### 4.3 REST API Gateway (`paperpilot-gateway`)
- POST `/api/v1/pdf/convert` with `format=pdf` accepting `application/json`.
- POST `/api/v1/pdf/tools/json_to_pdf` returning streaming PDF or JSON response.

### 4.4 In-Memory WASM Engine (`paperpilot-wasm`)
- Expose `json_to_pdf(json_str: &str) -> Result<Vec<u8>, JsValue>`
- Operates entirely in browser memory without disk access.

### 4.5 Cloudflare Edge Worker (`apps/edge`)
- Route `POST /api/v1/json_to_pdf` accepting JSON payload and streaming PDF response.

---

## 5. Verification & Testing Requirements

1. **Round-Trip Fidelity Assertion**:
   - `PDF_Original` ➔ `pdf_to_json` ➔ `doc.json` ➔ `json_to_pdf` ➔ `PDF_Synthesized` ➔ `pdf_to_json` ➔ `doc2.json`.
   - Assert `doc.pages.len() == doc2.pages.len()` and text content matches.
2. **20 Penta-Interface Assertions (4 Tiers × 5 Surfaces)** in `tools/penta-interface-e2e/src/cases/conversions.rs`.
3. **Edge Cases**:
   - Empty pages array (returns valid 0-byte or error without panic).
   - Base64 corrupt image data (graceful error handling).
   - Special UTF-8 characters and multi-line strings.
4. **Performance SLA**:
   - <25ms synthesis latency for 10-page document.
