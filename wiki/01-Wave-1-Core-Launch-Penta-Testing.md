# Wave 1: Core Launch Suite (Top 12 Tools) Testing Guide

## Overview
Wave 1 covers the Top 12 foundation tools released in `v0.5.0` for basic document manipulation, organization, text extraction, and tabular spreadsheet export.

## The 12 Wave 1 Tools Covered
1. **`pdf_merge`**: Combine multiple documents into one.
2. **`pdf_split`**: Divide document into individual pages or ranges.
3. **`pdf_rotate`**: Rotate pages by 90/180/270 degrees.
4. **`pdf_delete_pages`**: Remove specific pages.
5. **`pdf_reorder_pages`**: Rearrange sequence of pages.
6. **`pdf_compress`**: Deflate streams and reduce document size.
7. **`pdf_watermark`**: Overlay semi-transparent diagonal text watermarks.
8. **`pdf_ocr`**: On-device text recognition for scanned pages.
9. **`pdf_extract_text`**: High-accuracy layout-aware text extraction.
10. **`pdf_to_docx`**: Convert PDF to editable Word document.
11. **`excel_to_json`**: Convert tabular spreadsheet data to JSON.
12. **`excel_to_csv`**: Convert spreadsheet rows to CSV.

## Running Tests
```bash
make test-wave1
# or
cargo test --test top12_cleanroom_audit -j 2 -- --nocapture
```
*Outputs auto-generated scorecard to `reports/WAVE_1_CORE_LAUNCH_PENTA_SCORECARD.md`.*
