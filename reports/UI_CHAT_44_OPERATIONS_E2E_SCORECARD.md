# UI Chat 44 Operations E2E Scorecard

## Executive Summary
- **Total Tested:** 44
- **Passed:** 44
- **Failed:** 0

## Operations Status

| Tool Intent | Prompt | Status | Latency (ms) | Error Notes |
|-------------|--------|--------|--------------|-------------|
| `Rotate` | "Rotate 90 degrees" | ✅ PASS | 315.76 |  |
| `Compress` | "Compress this PDF" | ✅ PASS | 138.92 |  |
| `Split` | "Split pages 1 to 2" | ✅ PASS | 195.66 |  |
| `Merge` | "Merge documents" | ✅ PASS | 133.57 |  |
| `Extract` | "Extract pages 1, 2" | ✅ PASS | 115.96 |  |
| `Delete` | "Delete page 1" | ✅ PASS | 117.62 |  |
| `Reorder` | "Reorder pages 2, 1" | ✅ PASS | 120.12 |  |
| `Crop` | "Crop margins 0,0,100,100" | ✅ PASS | 116.37 |  |
| `Burst` | "Burst PDF into single pages" | ✅ PASS | 112.88 |  |
| `RemoveBlank` | "Remove all blank pages" | ✅ PASS | 120.70 |  |
| `Repair` | "Repair this PDF" | ✅ PASS | 107.76 |  |
| `Linearize` | "Linearize for fast web view" | ✅ PASS | 111.17 |  |
| `Encrypt` | "Encrypt with password secret123" | ✅ PASS | 110.25 |  |
| `Decrypt` | "Remove password from PDF" | ✅ PASS | 120.23 |  |
| `Watermark` | "Add watermark CONFIDENTIAL" | ✅ PASS | 116.90 |  |
| `Redact` | "Redact text" | ✅ PASS | 116.44 |  |
| `Metadata` | "Sanitize metadata" | ✅ PASS | 118.55 |  |
| `Sign` | "Sign document" | ✅ PASS | 114.89 |  |
| `Flatten` | "Flatten annotations" | ✅ PASS | 117.60 |  |
| `PdfA` | "Convert to PDF/A archive" | ✅ PASS | 116.70 |  |
| `HeaderFooter` | "Add header and footer" | ✅ PASS | 134.65 |  |
| `Bates` | "Add Bates numbering" | ✅ PASS | 132.19 |  |
| `PageNumbers` | "Add page numbers" | ✅ PASS | 117.73 |  |
| `ExtractText` | "Extract all text" | ✅ PASS | 138.83 |  |
| `ExtractImages` | "Extract embedded images" | ✅ PASS | 133.26 |  |
| `Search` | "Search text" | ✅ PASS | 132.38 |  |
| `Render` | "Render page to image" | ✅ PASS | 151.39 |  |
| `Compare` | "Compare documents" | ✅ PASS | 166.58 |  |
| `Ocr` | "Run OCR text recognition" | ✅ PASS | 154.19 |  |
| `Bookmarks` | "Extract bookmarks outline" | ✅ PASS | 144.20 |  |
| `ImagesToPdf` | "Convert images to PDF" | ✅ PASS | 146.08 |  |
| `Annotate` | "Add annotation" | ✅ PASS | 132.82 |  |
| `Classify` | "Classify document type" | ✅ PASS | 133.04 |  |
| `Validate` | "Validate PDF structure" | ✅ PASS | 138.13 |  |
| `Hash` | "Generate SHA-256 hash" | ✅ PASS | 129.28 |  |
| `FormRead` | "Read form data" | ✅ PASS | 133.18 |  |
| `FormFill` | "Fill form data" | ✅ PASS | 133.58 |  |
| `FormCreate` | "Add form field" | ✅ PASS | 136.74 |  |
| `ToDocx` | "Convert to Word docx" | ✅ PASS | 134.01 |  |
| `ToXlsx` | "Convert to Excel spreadsheet" | ✅ PASS | 134.16 |  |
| `ToPptx` | "Convert to PowerPoint presentation" | ✅ PASS | 137.89 |  |
| `ToHtml` | "Convert to HTML" | ✅ PASS | 127.80 |  |
| `ToMarkdown` | "Convert to Markdown" | ✅ PASS | 133.20 |  |
| `ExcelToPdf` | "Convert CSV to PDF" | ✅ PASS | 128.30 |  |
