# Spec 032 — Multimodal PDF-to-JSON with Image Extraction & Base64 Inlining

## 1. Overview
PaperPilot provides `pdf_to_json` to export PDF contents into structured machine-readable JSON. To empower multimodal AI pipelines (Gemini, Claude 3.5 Sonnet, GPT-4o Vision), the JSON export must optionally capture embedded images alongside page text—either as external file paths or inline `data:image/<fmt>;base64,<bytes>` URIs with spatial bounding boxes.

---

## 2. Architecture & Design

### 2.1 Schema Definition
`JsonPage` in `paperpilot-pdf/src/operations/conversion.rs` will be expanded to include `images: Vec<JsonImage>`:

```rust
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonImage {
    pub id: String,
    pub page_number: u32,
    pub format: String,             // "png", "jpeg"
    pub width: u32,
    pub height: u32,
    pub bbox: Option<[f32; 4]>,     // [x0, y0, x1, y1] if extractable from CTM/Do operator
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base64_data: Option<String>, // "data:image/png;base64,..."
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonPage {
    pub page_number: u32,
    pub text: String,
    pub char_count: usize,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub images: Vec<JsonImage>,
}
```

### 2.2 CLI & Tool Options
Introduce `--image-mode`:
- `none` (default): Fast text-only extraction (compact JSON).
- `files`: Save extracted images to an output folder (e.g. `./images/page_1_img_0.png`) and reference `file_path` in JSON.
- `base64`: Inline extracted images as base64 data URIs (`data:image/png;base64,...`) for direct zero-filesystem multimodal LLM ingestion.

```bash
paperpilot convert --input doc.pdf --format json --image-mode base64 --output doc.json
```

---

## 3. The 5-Interface Mandate & E2E Protocol Integration

Per [`wiki/25-Penta-Interface-Mandate-And-E2E-Protocol.md`](../../wiki/25-Penta-Interface-Mandate-And-E2E-Protocol.md), this capability must be exposed and verified across all **5 deployment surfaces**:

1. **💻 CLI (`paperpilot-cli`)**: Add `--image-mode <none|files|base64>` to `Commands::Convert` in `src/cli.rs`.
2. **🤖 MCP (`paperpilot-mcp`)**: Add `image_mode` property to `pdf_to_json` in `src/server.rs`.
3. **🌐 REST API (`paperpilot-gateway`)**: Expose `image_mode` in `/api/v1/pdf/convert` and `/api/v1/pdf/tools/pdf_to_json`.
4. **⚡ WASM (`paperpilot-wasm`)**: Expose `pdf_to_json(pdf_bytes, image_mode)` returning JSON string with inline base64 images directly in browser memory.
5. **☁️ Cloudflare Edge (`apps/edge`)**: Route `POST /api/v1/to_json?image_mode=base64` returning parsed JSON response.

---

## 4. Testing & Verification

1. **Rust-Native E2E Suite (`tools/penta-interface-e2e`)**:
   - Add test cases in `src/cases/conversions.rs` across all 4 tiers (Simple, Medium, Complex, Negative) and all 5 interfaces (20 assertions total).
   - Add edge cases to `src/cases/edge_cases.rs`:
     - Multi-image PDF base64 conversion.
     - PDF with 0 images converting cleanly with empty `images: []`.
     - Invalid image mode parameter handling without panics.
2. **Performance Benchmark**:
   - Add `pdf_to_json` to `tools/penta-interface-e2e/src/cases/bench.rs` profiling latency for `image_mode: base64` vs `image_mode: none`.
3. **Unit Tests**:
   - Write tests in `paperpilot-pdf` verifying base64 decodes back to original PNG/JPEG pixels.
4. **Verification Commands**:
   - `cargo test -p paperpilot-pdf`
   - `cargo run -p penta-interface-e2e -- --group conversions`
   - `make test-penta-e2e`
