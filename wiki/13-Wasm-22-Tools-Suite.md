# WASM 22 Tools Suite

PaperPilot employs a cutting-edge client-side WebAssembly engine to process all 22 PDF tools directly in the browser via Web Workers. This ensures user documents never leave their device and computation remains extremely fast.

## Available Capabilities

### Document Manipulation
- **`merge(files)`**: Combines multiple PDFs into one.
- **`split(file, ranges)`**: Slices a PDF into distinct parts based on page ranges.
- **`rotate(file, angle, pages)`**: Rotates specific or all pages.
- **`compress(file)`**: Shrinks file sizes by stripping metadata and compressing object streams.
- **`delete_pages(file, pages)`**: Removes specified pages.
- **`extract_pages(file, pages)`**: Creates a new PDF consisting solely of the targeted pages.
- **`reorder_pages(file, new_order)`**: Reconstructs the PDF's internal catalog to display pages in a custom sequence.
- **`crop(file, x, y, w, h)`**: Adjusts the `CropBox` for trimming pages.
- **`flatten(file)`**: Burns interactive `AcroForm` data directly onto the page surface.

### Extraction & Analysis
- **`extract_text(file)`**: Parses `/Contents` operators to extract clean UTF-8 text from the layout.
- **`extract_images(file)`**: Rips embedded `XObject` jpegs and pngs out of a PDF.
- **`pdf_info(file)`**: Yields JSON output describing structural properties, counts, and metadata.
- **`pdf_hash(file)`**: Performs a local SHA-256 hash calculation.
- **`render_page(file, page_index, scale)`**: Rasterizes vector pages into PNG format utilizing `hayro`.
- **`ocr(file)`**: Uses on-device inference via `rten` and `ocrs` to pull text from embedded pictures.

### Security
- **`encrypt(file, password)`**: Locks the PDF with AES-compatible protections.
- **`decrypt(file, password)`**: Unlocks a previously encrypted document.

### Stamping & Overlays
- **`watermark(file, text)`**: Stamps diagonal watermark graphics.
- **`page_numbers(file, format, position)`**: Dynamic calculation injecting "Page N of M".
- **`header_footer(file, header, footer)`**: Absolute positioning text injection.
- **`set_metadata(file, title, author, subject, keywords)`**: Directly rewrites the PDF `/Info` dictionary.
- **`images_to_pdf(images)`**: Synthesizes a new PDF with image frames mapped pixel-for-pixel to physical dimensions.

## Integration
Developers interfacing with these methods will find `WasmPdfClient` heavily typed within `apps/web/src/lib/wasm/index.ts`. Due to the computationally heavy nature of some tools (e.g. `ocr`), interactions route through an asynchronous message-passing interface hosted in `pdfWorker.ts`.
