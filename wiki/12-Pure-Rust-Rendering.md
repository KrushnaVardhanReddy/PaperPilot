# Pure-Rust Rendering Architecture

## Overview

As part of the roadmap to reach a 100% C++-free and OS-agnostic execution environment, PaperPilot migrated from `pdfium-render` (Google PDFium) to `hayro` and `tiny-skia`. This allows for 100% Client-Side WebAssembly (WASM) execution and Sub-10ms Serverless Edge Deployment.

## Implementation Details

`RenderOperation` (`paperpilot-pdf/src/operations/render.rs`) acts as the core interface. The pipeline works as follows:

1. **Downcasting:** The generic `PdfDocument` trait is downcast to `LopdfDocument`.
2. **Serialization:** To capture unsaved in-memory mutations (such as recently added watermarks or metadata), the document is serialized via `save_to` into a memory buffer.
3. **Parsing:** `hayro_syntax::Pdf::new()` reads the byte buffer.
4. **Rasterization:** Iterating over `pdf.pages()`, `hayro::render` converts the vectors and streams into a raw pixel buffer.
5. **Encoding:** `tiny_skia::Pixmap` takes the BGRA/RGBA layout and encodes it into compressed `PNG` bytes.

## Limitations

Currently, some complex features such as heavily encrypted/password-protected PDFs or knockout groups might face limitations based on upstream `hayro` capabilities. The core 44 operations verify perfectly against standard structures.
