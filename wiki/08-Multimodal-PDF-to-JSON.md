# Multimodal PDF-to-JSON Architecture

## Overview
The `pdf_to_json` functionality within `paperpilot-pdf` extracts text, structural metadata, and now embedded images (`XObjects`) from PDF documents, preserving their relationships across pages. This allows for powerful multimodal data ingestion into LLMs, providing not just the text content, but also references to, or inline data of, the original imagery found within the document.

## Schema Changes

The JSON representation revolves around three main structs: `JsonDocument`, `JsonPage`, and the newly introduced `JsonImage`.

```rust
pub struct JsonDocument {
    pub document_name: String,
    pub pages: Vec<JsonPage>,
}

pub struct JsonPage {
    pub page_number: u32,
    pub text: String,
    pub char_count: usize,
    pub images: Vec<JsonImage>,
}

pub struct JsonImage {
    pub id: String,
    pub page_number: u32,
    pub format: String,
    pub width: u32,
    pub height: u32,
    pub bbox: Option<[f32; 4]>,
    pub file_path: Option<String>,
    pub base64_data: Option<String>,
}
```

## Image Modes

The engine processes image extractions based on the `image_mode` configuration option:
1.  **`none` (Default):** Skips image extraction entirely. The `images` array remains empty in the resulting JSON, optimizing performance for text-only processing tasks.
2.  **`files`:** Extracts the images to the filesystem alongside the generated JSON report and populates the `file_path` field for each `JsonImage` with the local reference to the generated file. Useful for persistent data analysis tasks.
3.  **`base64`:** Inlines the image byte content as an encoded `data:image/<format>;base64,...` URI string inside the `base64_data` field. It executes the whole pipeline in-memory, crucial for edge computing execution such as Cloudflare Edge or WASM runtimes without local filesystem access.

## Extraction Mechanics
*   **JPEG (`/DCTDecode`):** Raw byte stream data is directly copied and saved to a file or encoded into Base64 format without any decompression logic, saving system memory.
*   **PNG (`/FlateDecode`):** Flate compressed images are decompressed, mapped with appropriate colorspace constraints (`DeviceRGB` and `DeviceGray`), and transformed into raw byte data via `image::ImageBuffer`. Those bytes are then written to disk or converted to Base64 depending on the configuration.

## Penta-Interface Parity
The tool is accessible seamlessly across the entire Penta-Interface deployment surface area:
*   **💻 CLI:** `paperpilot convert --format json --input doc.pdf --image-mode <mode>`
*   **🤖 MCP:** `pdf_to_json` JSON-RPC method call (`image_mode` property available in schema).
*   **🌐 REST API:** Dynamic routing to `pdf_to_json` MCP Tool via `POST /api/v1/pdf/tools/pdf_to_json`.
*   **⚡ WASM:** Exposed via the `WasmPdfEngine::to_json` interface in `paperpilot-wasm`.
*   **☁️ Edge:** Accessible as a REST endpoint in Cloudflare Edge (`/api/v1/to_json`).
