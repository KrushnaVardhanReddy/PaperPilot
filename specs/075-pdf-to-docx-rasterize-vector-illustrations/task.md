# Task Plan: Spec 075 — Generic Dynamic Layout & Image Extraction for PDF-to-DOCX

- [ ] Task 1: Refactor `pulldown-cmark` parser loop in `PdfToDocxOperation`
  - Implement full dynamic Markdown table handling (`Event::Start(Tag::Table)`, `TableHead`, `TableRow`, `TableCell`).
  - Calculate column widths dynamically based on `10080 / column_count` dxa.
  - Remove the hardcoded `current_heading_level == 3` -> 2-column table coupling.
- [ ] Task 2: Implement Real Graphic Extraction & Vector Rasterization
  - For raster images: extract bytes directly from `/Subtype /Image`.
  - For vector Form XObjects: render page via `hayro` at 150 DPI and crop bounding boxes to PNG.
  - Eliminate all occurrences of `[ Feature N Illustration ]` text placeholders.
- [ ] Task 3: Support Standard Text Flow
  - Normal headings (`H1`..`H6`), blockquotes, lists, and paragraphs must render as standard Word document blocks.
  - Ensure centered titles, paragraph spacing, and font sizes scale cleanly across arbitrary PDF layouts.
- [ ] Task 4: Multi-PDF Validation Suite
  - Validate with `fixtures/Get_Started_With_Smallpdf.pdf` (brochure with vector graphics).
  - Validate with standard text and tabular PDF fixtures.
  - Ensure `test_pdf_to_docx` and `all50_cleanroom_audit` both pass 100%.
