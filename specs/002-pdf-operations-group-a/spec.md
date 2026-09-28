# Specification: PDF Operations (Group A)

## Feature Description
Group A contains the core structural page manipulations for PDFs. These operations will be implemented in the `paperpilot-pdf` crate, implementing the `PdfOperation` trait defined in `paperpilot-core`. The engine backing these structural changes will be `lopdf`.

## Scope (Group A)
1. **Merge**: Combine N PDFs into one.
2. **Split**: Split a PDF by page ranges.
3. **Extract pages**: Pull specific pages into a new PDF.
4. **Delete pages**: Remove specific pages.
5. **Reorder pages**: Reorder based on an index array.
6. **Rotate pages**: Rotate by 90, 180, 270 degrees.
7. **Crop pages**: Trim the content box.
8. **Burst**: Split every page into its own file.

## Non-Goals
- Text extraction or rendering (handled in later groups by `pdfium-render`).
- GUI integration (this is purely the backend library layer).

## Technical Constraints
- Must use `lopdf` as the underlying engine.
- Must implement the `PdfOperation` trait from `paperpilot-core::traits::PdfOperation`.
- Must return `OperationResult` from `paperpilot-core::error`.
