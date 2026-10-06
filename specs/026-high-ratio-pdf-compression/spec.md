# Spec 026: Universal High-Ratio PDF Compression Engine (Zero-Bloat Image Re-encoding & Stream Optimization)

## Status: APPROVED / READY FOR IMPLEMENTATION

## 1. Problem Statement & Motivation
Currently, PaperPilot provides a `compress` operation across CLI, MCP, REST API, Desktop UI, and WASM. However, its implementation simply calls `lopdf::Document::compress()`. 
In modern PDFs:
1. Content streams are already Flate/Deflate encoded.
2. Raster images (PNG, uncompressed bitmaps, high-DPI scans) represent 80% to 95% of total PDF file size.
3. The `--quality` parameter (e.g., `low`, `medium`, `high`, `75%`) is completely ignored or omitted in MCP/UI payloads.
4. As a result, compressing real-world image-rich PDFs yields **0.00% to 0.40%** file size reduction, misleading users and failing a core value proposition of PaperPilot.

---

## 2. Technical Architecture & Solution

### A. Core Engine (`paperpilot-pdf/src/operations/compress.rs`)
1. **Quality Parameter**: Support `CompressQuality`:
   - `low` / `screen`: 72 DPI target, JPEG quality 45%, maximum aggressive compression (60–80% size reduction).
   - `medium` / `ebook`: 150 DPI target (max dimension ~1600px), JPEG quality 70%, balanced (40–60% size reduction).
   - `high` / `prepress`: 300 DPI target, JPEG quality 85%, preserve fine details (15–35% size reduction).
   - Numeric quality (1–100 integer) supported.
2. **Embedded Image XObject Processing**:
   - Traverse all document objects or page `/Resources /XObject` dictionaries for `/Subtype /Image`.
   - Decompress raw image streams (FlateDecode or raw pixel buffers).
   - Decode image into memory using the workspace `image` crate (`image::load_from_memory` or `ImageBuffer::from_raw` using ColorSpace `/DeviceRGB`, `/DeviceGray`, `/Indexed`).
   - If image resolution exceeds target dimension threshold (e.g., width or height > 1600px for medium/low), downsample using `image::imageops::resize` with `FilterType::Triangle`.
   - Re-encode image bytes to JPEG (`image::codecs::jpeg::JpegEncoder::new_with_quality(&mut buf, quality)`).
   - Update the stream dictionary:
     - `/Filter` -> `/DCTDecode`
     - `/ColorSpace` -> `/DeviceRGB` (or `/DeviceGray` if grayscale)
     - `/BitsPerComponent` -> `8`
     - Remove obsolete PNG-specific predictor entries (`/DecodeParms`).
     - Update `/Width` and `/Height` if downsampled.
     - Replace stream contents with new JPEG bytes.
3. **Stream & Metadata Cleanup**:
   - Prune unused / orphaned objects.
   - Run `doc.compress()` on remaining text and font streams.

### B. Interface Parity (CLI, MCP, REST Gateway, Desktop UI, WASM)
1. **CLI (`paperpilot-cli`)**:
   - `paperpilot compress --input <IN> --output <OUT> [--quality <low|medium|high|1-100>]`
   - Wire `quality` directly into `CompressOperation`.
2. **MCP (`paperpilot-mcp`)**:
   - Update `pdf_compress` tool schema to declare optional parameter `quality` (`string` or `number`, default `"medium"`).
   - Extract `quality` in `server.rs` and pass to `CompressOperation`.
3. **REST Gateway (`paperpilot-gateway`)**:
   - Ensure `/api/v1/pdf/compress` JSON and multipart handlers pass `quality` to `CompressOperation`.
4. **Desktop UI (`apps/desktop`)**:
   - Update `OperationsPanel.svelte` to include `quality: compressQuality` in the `args` payload sent to `invoke_mcp_tool('pdf_compress', args)`.
5. **WASM Engine (`paperpilot-wasm`)**:
   - Update `WasmPdfEngine.compress(file, quality)` or `operations::compress` to apply the same image downsampling and recompression in-memory.

---

## 3. Verification & Acceptance Criteria
1. **Image PDF Size Reduction**: Compressing a PDF with embedded images must achieve **at least 30% to 70%** reduction on `low` / `medium`.
2. **Text PDF Preservation**: Text PDFs remain 100% readable and valid `%PDF-` files without corruption.
3. **Tri-Interface Test Suite**: All 44 operations across CLI, MCP, and API continue to score 100% PASS (132/132 assertions).
4. **Sub-200ms Latency**: In-memory Rust JPEG encoding and resizing runs with sub-200ms performance.
