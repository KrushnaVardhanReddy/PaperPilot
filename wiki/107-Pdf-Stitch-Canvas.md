# PdfStitchCanvas (Continuous Canvas Stitching) Architecture

The `pdf_stitch_canvas` operation provides a pure-Rust implementation to stitch multiple PDF pages into a single continuous canvas, vertically or horizontally.

## Core Mechanism

The engine processes PDFs via an in-memory byte pipeline (`pdf_stitch_canvas_bytes`), making it fully compatible with Penta-Interface constraints (CLI, MCP, Gateway, WASM, Cloudflare Edge).

The operation works in the following phases:

1. **Measurement & Canvas Calculation:**
   - Extracts the `/MediaBox` sizes for each page.
   - Calculates the overall canvas dimensions (width and height) by taking into account the chosen direction (vertical/horizontal), the spacing between pages, and the margin around the canvas.
   - Calculates the exact placement coordinates `(X, Y)` for each original page on the new canvas. Vertical stacking places pages from top to bottom (centered horizontally). Horizontal placement puts pages from left to right (centered vertically).

2. **Form XObject Conversion:**
   - Converts each original page into a self-contained Form XObject (`/Type /XObject /Subtype /Form`).
   - Copies `/Resources` and the page content bytes over to the XObject.
   - Generates unique names for the XObjects (`/Fm0`, `/Fm1`, etc.).

3. **Re-assembling Canvas:**
   - A single unified page is created containing an operation stream that paints each Form XObject using standard `cm` (transformation matrix) and `Do` (draw object) PDF operations.
   - The `/Pages` tree is replaced with a new one that contains only this newly stitched canvas page.
   - The original individual pages are dropped from the tree, and `doc.prune_objects()` is called to erase unreferenced bytes.

## Integration

The tool strictly adheres to the single-core runner pattern:
- Core logic resides entirely in `paperpilot-pdf/src/operations/pdf_stitch_canvas.rs`.
- Wrappers across CLI, MCP, etc. pass straightforward configuration options without duplicating PDF logic.