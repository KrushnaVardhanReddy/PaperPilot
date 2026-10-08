# UI Chat 44 Operations E2E Scorecard

## Executive Summary
- **Total Tested:** 44
- **Passed:** 44
- **Failed:** 0

## Operations Status

| Tool Intent | Prompt | Status | Latency (ms) | Error Notes |
|-------------|--------|--------|--------------|-------------|
| `Rotate` | "Rotate 90 degrees" | ✅ PASS | 375.70 |  |
| `Compress` | "Compress this PDF" | ✅ PASS | 165.25 |  |
| `Split` | "Split pages 1 to 2" | ✅ PASS | 111.05 |  |
| `Merge` | "Merge documents" | ✅ PASS | 90.43 |  |
| `Extract` | "Extract pages 1, 2" | ✅ PASS | 84.85 |  |
| `Delete` | "Delete page 1" | ✅ PASS | 97.02 |  |
| `Reorder` | "Reorder pages 2, 1" | ✅ PASS | 108.85 |  |
| `Crop` | "Crop margins 0,0,100,100" | ✅ PASS | 114.31 |  |
| `Burst` | "Burst PDF into single pages" | ✅ PASS | 101.71 |  |
| `RemoveBlank` | "Remove all blank pages" | ✅ PASS | 118.63 |  |
| `Repair` | "Repair this PDF" | ✅ PASS | 125.61 |  |
| `Linearize` | "Linearize for fast web view" | ✅ PASS | 113.28 |  |
| `Encrypt` | "Encrypt with password secret123" | ✅ PASS | 109.06 |  |
| `Decrypt` | "Remove password from PDF" | ✅ PASS | 118.34 |  |
| `Watermark` | "Add watermark CONFIDENTIAL" | ✅ PASS | 127.65 |  |
| `Redact` | "Redact text" | ✅ PASS | 125.21 |  |
| `Metadata` | "Sanitize metadata" | ✅ PASS | 103.88 |  |
| `Sign` | "Sign document" | ✅ PASS | 98.65 |  |
| `Flatten` | "Flatten annotations" | ✅ PASS | 111.75 |  |
| `PdfA` | "Convert to PDF/A archive" | ✅ PASS | 103.28 |  |
| `HeaderFooter` | "Add header and footer" | ✅ PASS | 85.04 |  |
| `Bates` | "Add Bates numbering" | ✅ PASS | 134.91 |  |
| `PageNumbers` | "Add page numbers" | ✅ PASS | 89.71 |  |
| `ExtractText` | "Extract all text" | ✅ PASS | 62.72 |  |
| `ExtractImages` | "Extract embedded images" | ✅ PASS | 90.82 |  |
| `Search` | "Search text" | ✅ PASS | 89.19 |  |
| `Render` | "Render page to image" | ✅ PASS | 128.80 |  |
| `Compare` | "Compare documents" | ✅ PASS | 114.70 |  |
| `Ocr` | "Run OCR text recognition" | ✅ PASS | 90.12 |  |
| `Bookmarks` | "Extract bookmarks outline" | ✅ PASS | 138.21 |  |
| `ImagesToPdf` | "Convert images to PDF" | ✅ PASS | 80.26 |  |
| `Annotate` | "Add annotation" | ✅ PASS | 85.18 |  |
| `Classify` | "Classify document type" | ✅ PASS | 97.25 |  |
| `Validate` | "Validate PDF structure" | ✅ PASS | 113.01 |  |
| `Hash` | "Generate SHA-256 hash" | ✅ PASS | 92.25 |  |
| `FormRead` | "Read form data" | ✅ PASS | 145.42 |  |
| `FormFill` | "Fill form data" | ✅ PASS | 131.01 |  |
| `FormCreate` | "Add form field" | ✅ PASS | 101.94 |  |
| `ToDocx` | "Convert to Word docx" | ✅ PASS | 130.25 |  |
| `ToXlsx` | "Convert to Excel spreadsheet" | ✅ PASS | 115.82 |  |
| `ToPptx` | "Convert to PowerPoint presentation" | ✅ PASS | 104.87 |  |
| `ToHtml` | "Convert to HTML" | ✅ PASS | 99.31 |  |
| `ToMarkdown` | "Convert to Markdown" | ✅ PASS | 101.55 |  |
| `ExcelToPdf` | "Convert CSV to PDF" | ✅ PASS | 133.61 |  |
