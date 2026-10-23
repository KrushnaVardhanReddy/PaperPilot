# Booklet Imposition Engine (`pdf_booklet`)

The `pdf_booklet` engine implements standard booklet imposition for PDF documents within the PaperPilot framework. This allows generating saddle-stitch signature sheets (2-up, double-sided layouts) directly from sequential PDF pages.

## Architecture & Integration

- **Core Module**: `paperpilot-pdf/src/operations/pdf_booklet.rs`
- **Penta-Interface Wrappers**:
  - `paperpilot-cli` (`paperpilot booklet`)
  - `paperpilot-mcp` (Tool `pdf_booklet`)
  - `paperpilot-gateway` (`POST /api/v1/pdf/booklet`)
  - `paperpilot-wasm` & Edge Runtime (`WasmPdfEngine.pdf_booklet`)

## Key Features
- **Saddle-Stitch Calculation**: Automatically calculates the required N-up sequence to map linear pages onto folding sheets.
- **Padding Blanks**: Detects if the total page count is not a multiple of 4, appending transparent blank pages automatically.
- **In-Memory Manipulation**: Only utilizes reference modifications inside the `lopdf` Document Catalog's `/Pages` node to maintain peak performance and avoid costly data re-encoding.

## Tool Execution Flow

1. Parse raw bytes into an `lopdf::Document`.
2. Extract the current `/Pages` map and evaluate total count.
3. Compute modulo-4 padding. 
4. Synthesize the sequence (e.g., an 8 page layout maps to `[8, 1, 2, 7, 6, 3, 4, 5]`).
5. Overwrite the `/Kids` array and `/Count` of the root `Pages` dictionary.
6. Return a serialized PDF ready for duplex printing.
