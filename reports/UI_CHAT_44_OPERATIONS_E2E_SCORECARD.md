# UI Chat 44 Operations E2E Scorecard

## Executive Summary
- **Total Tested:** 44
- **Passed:** 44
- **Failed:** 0

## Operations Status

| Tool Intent | Prompt | Status | Latency (ms) | Error Notes |
|-------------|--------|--------|--------------|-------------|
| `Rotate` | "Rotate 90 degrees" | ✅ PASS | 313.00 |  |
| `Compress` | "Compress this PDF" | ✅ PASS | 134.83 |  |
| `Split` | "Split pages 1 to 2" | ✅ PASS | 196.56 |  |
| `Merge` | "Merge documents" | ✅ PASS | 106.32 |  |
| `Extract` | "Extract pages 1, 2" | ✅ PASS | 116.63 |  |
| `Delete` | "Delete page 1" | ✅ PASS | 116.60 |  |
| `Reorder` | "Reorder pages 2, 1" | ✅ PASS | 116.52 |  |
| `Crop` | "Crop margins 0,0,100,100" | ✅ PASS | 116.99 |  |
| `Burst` | "Burst PDF into single pages" | ✅ PASS | 116.32 |  |
| `RemoveBlank` | "Remove all blank pages" | ✅ PASS | 101.01 |  |
| `Repair` | "Repair this PDF" | ✅ PASS | 112.89 |  |
| `Linearize` | "Linearize for fast web view" | ✅ PASS | 119.25 |  |
| `Encrypt` | "Encrypt with password secret123" | ✅ PASS | 116.99 |  |
| `Decrypt` | "Remove password from PDF" | ✅ PASS | 117.20 |  |
| `Watermark` | "Add watermark CONFIDENTIAL" | ✅ PASS | 116.11 |  |
| `Redact` | "Redact text" | ✅ PASS | 117.37 |  |
| `Metadata` | "Sanitize metadata" | ✅ PASS | 115.70 |  |
| `Sign` | "Sign document" | ✅ PASS | 117.30 |  |
| `Flatten` | "Flatten annotations" | ✅ PASS | 115.72 |  |
| `PdfA` | "Convert to PDF/A archive" | ✅ PASS | 117.26 |  |
| `HeaderFooter` | "Add header and footer" | ✅ PASS | 115.68 |  |
| `Bates` | "Add Bates numbering" | ✅ PASS | 118.39 |  |
| `PageNumbers` | "Add page numbers" | ✅ PASS | 119.98 |  |
| `ExtractText` | "Extract all text" | ✅ PASS | 129.68 |  |
| `ExtractImages` | "Extract embedded images" | ✅ PASS | 117.26 |  |
| `Search` | "Search text" | ✅ PASS | 117.34 |  |
| `Render` | "Render page to image" | ✅ PASS | 135.95 |  |
| `Compare` | "Compare documents" | ✅ PASS | 136.16 |  |
| `Ocr` | "Run OCR text recognition" | ✅ PASS | 132.23 |  |
| `Bookmarks` | "Extract bookmarks outline" | ✅ PASS | 126.86 |  |
| `ImagesToPdf` | "Convert images to PDF" | ✅ PASS | 135.52 |  |
| `Annotate` | "Add annotation" | ✅ PASS | 114.94 |  |
| `Classify` | "Classify document type" | ✅ PASS | 116.96 |  |
| `Validate` | "Validate PDF structure" | ✅ PASS | 116.30 |  |
| `Hash` | "Generate SHA-256 hash" | ✅ PASS | 133.70 |  |
| `FormRead` | "Read form data" | ✅ PASS | 116.27 |  |
| `FormFill` | "Fill form data" | ✅ PASS | 116.40 |  |
| `FormCreate` | "Add form field" | ✅ PASS | 117.33 |  |
| `ToDocx` | "Convert to Word docx" | ✅ PASS | 117.27 |  |
| `ToXlsx` | "Convert to Excel spreadsheet" | ✅ PASS | 116.95 |  |
| `ToPptx` | "Convert to PowerPoint presentation" | ✅ PASS | 115.83 |  |
| `ToHtml` | "Convert to HTML" | ✅ PASS | 115.98 |  |
| `ToMarkdown` | "Convert to Markdown" | ✅ PASS | 140.18 |  |
| `ExcelToPdf` | "Convert CSV to PDF" | ✅ PASS | 130.27 |  |
