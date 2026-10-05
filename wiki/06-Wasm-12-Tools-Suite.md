# Wasm 12 Tools Suite

PaperPilot features a suite of 12 pure-Rust tools executed directly in the browser via WebAssembly (`paperpilot-wasm`).

## Supported Operations
- **Merge**: Combine multiple PDFs into one.
- **Split**: Extract ranges from a PDF.
- **Rotate**: Rotate specific pages.
- **Compress**: Deflate streams and remove unnecessary metadata.
- **Encrypt**: Add a password to the PDF.
- **Watermark**: Embed a text watermark across pages.
- **Delete Pages**: Remove specific pages.
- **Extract Pages**: Keep only specified pages.
- **Reorder Pages**: Rearrange page order.
- **Crop**: Apply crop box margins to all pages.
- **Flatten**: Strip interactive AcroForms.
- **Edit Metadata**: Modify `/Title`, `/Author`, `/Subject`, and `/Keywords`.

## Architecture
The WASM module (`paperpilot-wasm`) wraps `lopdf` functions in `#[wasm_bindgen]` allowing the TypeScript frontend (`apps/web` and `apps/desktop`) to send `Uint8Array` buffers to a Web Worker (`pdfWorker.ts`), run the heavy memory operations off the main thread, and receive back the processed PDF buffer for instant browser download.
