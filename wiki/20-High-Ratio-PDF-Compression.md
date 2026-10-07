# High-Ratio PDF Compression Engine

PaperPilot implements a high-ratio PDF compression engine that achieves up to 40%+ size reduction on image-heavy PDFs by traversing internal objects and applying intelligent image downsampling and re-encoding techniques.

## Architecture

1. **Object Traversal:**
   The `CompressOperation` traverses the PDF's internal object tree to identify specific streams with `/Type /XObject` and `/Subtype /Image`.

2. **Decompression & Color Space Extraction:**
   Each image stream is decompressed, and critical image metadata such as `/Width`, `/Height`, and `/ColorSpace` are captured for processing.
   If standard memory loading fails, image buffers are reconstructed directly from raw bytes according to their color space formatting (`DeviceRGB`, `DeviceGray`, etc.).

3. **Downsampling & Re-encoding:**
   Depending on the configured `quality` parameter, images are aggressively downsampled to a maximum dimension while maintaining aspect ratio:
   - **low:** Max dimension 1024px, JPEG quality 45
   - **medium (default):** Max dimension 1600px, JPEG quality 70
   - **high:** Max dimension 2400px, JPEG quality 85
   - **integer 1-100:** User-defined JPEG quality with max dimension capped at 1600px.

4. **Length Comparison Validation:**
   After re-encoding with `image::codecs::jpeg::JpegEncoder`, the new size is validated. If the re-encoded `DCTDecode` stream size is strictly less than the original stream size, the internal PDF dictionary is updated (e.g., swapping to `/Filter /DCTDecode`, modifying `/Length` properties, etc.) and the payload swapped.

5. **Structural Deflation:**
   Finally, `lopdf_doc.inner.compress()` is invoked to structurally compress any remaining non-image streams using Flate/Zlib.

## Supported Interfaces

The compression quality can be adjusted across all integration layers:

- **CLI:** `--quality <low|medium|high|1-100>` passed to the `compress` tool.
- **REST Gateway:** `quality` property provided in the JSON/Multipart upload context (`/api/v1/pdf/compress`).
- **MCP Context:** Passed as the `quality` property in the `pdf_compress` tool schema.
- **Desktop UI:** Exposed in the `OperationsPanel` via a responsive slider binding mapped to the underlying tool command.
- **WASM Client:** Included as an optional parameter (`quality: Option<&str>`) for fully local Browser/Client-side processing.
