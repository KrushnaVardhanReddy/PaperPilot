# PDF Oxide Layout Analysis Report
## Algorithm Details
1. **Spatial Extraction**: As the PDF content stream is parsed, we track the `x` and `y` coordinates using transformations and text operations (`Tm`, `Td`, `TD`, `T*`, etc.). Each decoded run of text is captured as a `TextSpan` containing typographic data and coordinates.
2. **Table Recognition**: We cluster spans by `y` coordinates (with a small tolerance) to form rows. If multiple contiguous rows contain multiple aligned spans, we group them into a `LayoutItem::Table` which ultimately emits as OpenXML `w:tbl` objects.
3. **Column De-weaving (XY-Cut)**: For non-table spans, we project horizontal boundaries vertically to find clear whitespace gutters (e.g. gaps $\ge 15\,\text{pt}$). When a gutter spans the block, it splits the layout into distinct columns, ensuring multi-column text is correctly read column-by-column rather than row-by-row interleaving.
## Test Verification
- Tests implemented in `paperpilot-pdf` (`conversion.rs`) and `paperpilot-wasm` (`operations.rs`) to ensure tables and multi-column inputs are parsed correctly.
- All tests pass in single-threaded/multi-threaded `cargo test` execution.
