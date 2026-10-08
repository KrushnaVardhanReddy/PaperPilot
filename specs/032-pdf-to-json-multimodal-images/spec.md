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

## 3. Implementation Plan
1. **Engine**: Update `PdfToJsonOperation` in `paperpilot-pdf/src/operations/conversion.rs` using `lopdf` XObject / image extraction logic from `operations/extract_images.rs`.
2. **CLI**: Add `--image-mode` argument to `Commands::Convert` in `paperpilot-cli/src/cli.rs`.
3. **MCP**: Expose `image_mode` (`"none" | "files" | "base64"`) in MCP tool `pdf_to_json` schema in `paperpilot-mcp/src/server.rs`.
4. **REST API**: Expose `image_mode` parameter in POST `/api/v1/pdf/convert` in `paperpilot-gateway/src/handlers/`.
5. **Testing**: Add unit and E2E assertions verifying that `image-mode base64` produces valid `data:image/...` headers and decodeable image bytes.
