# Pure-Rust PDF Tabular Extraction Engine

## Overview
As part of the **Penta-Interface Parity** strategy, PaperPilot implements a fully pure-Rust tabular extraction engine (`pdf_extract_tables`) that reliably groups PDF text streams into structured tables (rows and columns) and can emit results in JSON matrices or RFC 4180 CSV formats.

## Architecture & Implementation
The core extractor lives in `paperpilot-pdf/src/operations/table.rs`.

1. **Spatial Positioning**:
   The extraction utilizes `pdf_oxide::api::Pdf::from_bytes` and `extract_words()` to process the underlying PDF stream and extract precise `[x, y, w, h]` bounding boxes for all textual fragments on each requested page.

2. **Row Clustering (Y-Axis Grouping)**:
   - Words are sorted top-to-bottom descending by their `bbox.y`.
   - Iterating through the words, those that share an overlapping $Y$-interval (within a threshold, e.g., 8pt) are merged into the same horizontal "row".

3. **Column Clustering (X-Axis Projection)**:
   - For all words within clustered rows, the algorithm projects their start and end `[bbox.x, bbox.x + bbox.width]` intervals into a global 1D line.
   - Merging these intervals reveals the document's columns and gutters.
   - The number of distinct merged X-intervals determines the matrix width ($N$ columns).

4. **Cell Assignment & Formatting**:
   - For each row, words are matched against the column intervals using their centroid $X$ coordinate.
   - Matched words are grouped and space-joined into cells.
   - The top row is automatically treated as headers (`headers`), while subsequent rows form `data: Vec<Vec<String>>`.

5. **Penta-Interface Integration**:
   - The engine returns a `TableExtractResult` structured type which seamlessly integrates into the MCP JSON-RPC router, CLI runner, REST API axum gateway, and the Wasm edge compilation targets.
