---
title: PaperPilot Tools Handbook
description: Comprehensive reference for all 76 PaperPilot tools across CLI, MCP, and REST API interfaces.
---

# PaperPilot Tools Handbook (76 Tools)

This handbook provides verified execution snippets and parameter references for all **76 PaperPilot tools** across the CLI, MCP (AI Agent JSON-RPC), and local REST API interfaces.

## 1. `cli_ask`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

The `paperpilot ask` command enables plain English instructions to manipulate PDFs.

### CLI
```bash
N/A
```

### MCP JSON-RPC
```json
N/A
```

### REST API
```bash
N/A
```

## 2. `cli_fill_batch`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

`paperpilot fill-batch` performs local AcroForm mail merge. It parses the template
once, fills an independent copy for each record, and writes a local
`execution_receipt.json` with row outcomes, paths, SHA-256 hashes and timings.
PDF processing uses the same in-memory form engine as the single-document tools.

Text fields (including multiline fields) and checkboxes are supported. Values
are stored using PDF text-string encoding, including UTF-16 for Chinese text.
Appearances use Helvetica for ASCII and the standard STSong CJK fallback for
non-ASCII BMP text. Viewers need standard CJK font support; fonts are not
downloaded or embedded by this command. Non-BMP values, unsupported field types,
read-only templates and XFA forms are rejected explicitly. Values too long to
fit at a readable size fail that row instead of being silently clipped.

### CLI
```bash
paperpilot fill-batch --template form.pdf --data records.csv --output-dir ./out \
  --name-template '{row.name}_{date}_{index:04}.pdf' --json
```

### MCP JSON-RPC
```json
N/A
```

### REST API
```bash
N/A
```

## 3. `document_pii_scan`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Scans document streams for personally identifiable information (PII) and data leaks.**  
Identifies Social Security numbers, credit card numbers, email addresses, phone numbers, and credentials with offset and confidence tracking.

### CLI
```bash
paperpilot pii-scan --input tests/e2e_fixtures/real/single_page.pdf --output tests/e2e_fixtures/out/penta_e2e/pii_report.json --json
```

### MCP JSON-RPC
```json
{
  "name": "document_pii_scan",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/pii_report.json"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/document-pii-scan -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/pii_report.json"}'
```

## 4. `excel_to_csv`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Exports Excel workbooks (.xlsx, .xls) into standard CSV text files.**  
Extracts spreadsheet data quickly without needing Microsoft Office. When a workbook has multiple sheets, each sheet is exported to a clean individual CSV file.

### CLI
```bash
paperpilot office excel-to-csv --input tests/e2e_fixtures/real/test.xlsx --output tests/e2e_fixtures/out/penta_e2e/table.csv
```

### MCP JSON-RPC
```json
{
  "name": "excel_to_csv",
  "arguments": {
    "input": "tests/e2e_fixtures/real/test.xlsx",
    "output": "tests/e2e_fixtures/out/penta_e2e/table.csv"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/excel-to-csv -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/test.xlsx", "output": "tests/e2e_fixtures/out/penta_e2e/table.csv"}'
```

## 5. `excel_to_json`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Converts Excel spreadsheets into structured JSON arrays.**  
Parses rows and columns into key-value JSON objects ready for AI agents, APIs, or database imports.

### CLI
```bash
paperpilot office excel-to-json --input tests/e2e_fixtures/real/test.xlsx
```

### MCP JSON-RPC
```json
{
  "name": "excel_to_json",
  "arguments": {
    "input": "tests/e2e_fixtures/real/test.xlsx"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/excel-to-json -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/test.xlsx"}'
```

## 6. `json_to_pdf`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Generates a publication-quality PDF from structured JSON data.**  
Takes layout, styled text blocks, and embedded base64 images from JSON and compiles them directly into a rendered PDF.

### CLI
```bash
paperpilot process --input tests/e2e_fixtures/real/single_page.pdf --output tests/e2e_fixtures/out/penta_e2e/out.pdf --json
```

### MCP JSON-RPC
```json
N/A
```

### REST API
```bash
N/A
```

## 7. `office_extract_text`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Dumps all plain text from Microsoft Office files (.docx, .xlsx, .pptx).**  
Bypasses formatting and extracts pure readable text strings for fast indexing, search, or LLM ingestion.

### CLI
```bash
paperpilot office extract-text --input tests/e2e_fixtures/real/test.xlsx --output tests/e2e_fixtures/out/penta_e2e/out.txt
```

### MCP JSON-RPC
```json
{
  "name": "office_extract_text",
  "arguments": {
    "input": "tests/e2e_fixtures/real/test.xlsx",
    "output": "tests/e2e_fixtures/out/penta_e2e/out.txt"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/office-extract-text -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/test.xlsx", "output": "tests/e2e_fixtures/out/penta_e2e/out.txt"}'
```

## 8. `office_modernize_legacy`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Upgrades legacy binary Office files (.doc, .xls, .ppt) to modern OpenXML formats (.docx, .xlsx, .pptx).**  
Eliminates obsolete binary structures and modernizes them into standard XML-based Office packages.

### CLI
```bash
paperpilot office modernize --input tests/e2e_fixtures/real/test.xls --output tests/e2e_fixtures/out/penta_e2e/out.xlsx
```

### MCP JSON-RPC
```json
{
  "name": "office_modernize_legacy",
  "arguments": {
    "input": "tests/e2e_fixtures/real/test.xls",
    "output": "tests/e2e_fixtures/out/penta_e2e/out.xlsx"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/office-modernize-legacy -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/test.xls", "output": "tests/e2e_fixtures/out/penta_e2e/out.xlsx"}'
```

## 9. `office_to_markdown`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Converts Word, Excel, and PowerPoint documents into clean Markdown.**  
Preserves heading hierarchies, bullet lists, bold/italics, and data tables in human-readable Markdown format for AI RAG pipelines.

### CLI
```bash
paperpilot office to-markdown --input tests/e2e_fixtures/real/test.xlsx --output tests/e2e_fixtures/out/penta_e2e/doc.md
```

### MCP JSON-RPC
```json
{
  "name": "office_to_markdown",
  "arguments": {
    "input": "tests/e2e_fixtures/real/test.xlsx",
    "output": "tests/e2e_fixtures/out/penta_e2e/doc.md"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/office-to-markdown -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/test.xlsx", "output": "tests/e2e_fixtures/out/penta_e2e/doc.md"}'
```

## 10. `pdf_annotate`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Appends visual annotations like text highlights, underlines, and sticky notes onto PDF pages.**  
Injects standard PDF annotation elements (`/Highlight`, `/Underline`, `/Text`) at exact spatial coordinates.

### CLI
```bash
paperpilot annotate --input tests/e2e_fixtures/real/single_page.pdf --output tests/e2e_fixtures/out/penta_e2e_security/annotate_cli_Simple.pdf --data [{\"id\":\"1\",\"type\":\"highlight\",\"page\":1,\"x\":50.0,\"y\":50.0,\"w\":50.0,\"h\":50.0,\"color\":\"#ffff00\",\"content\":\"Test\"}] --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_annotate",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "annotations": [],
    "output": "tests/e2e_fixtures/out/penta_e2e/annotated.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/annotate -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "annotations": [], "output": "tests/e2e_fixtures/out/penta_e2e/annotated.pdf"}'
```

## 11. `pdf_attachments`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

Inspect, extract, or embed file attachments into PDF portfolios.

### CLI
```bash
N/A
```

### MCP JSON-RPC
```json
N/A
```

### REST API
```bash
N/A
```

## 12. `pdf_auto_rotate`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

The `pdf_auto_rotate` tool automatically detects and corrects the orientation of pages in a PDF document based on their physical dimensions. It rotates landscape-oriented pages to portrait or vice versa, ensuring consistent page orientations throughout the document.

### CLI
```bash
paperpilot pdf-auto-rotate input.pdf output.pdf --target-orientation portrait
```

### MCP JSON-RPC
```json
N/A
```

### REST API
```bash
curl -X POST http://localhost:8080/tools/pdf_auto_rotate \
  -H "Content-Type: application/json" \
  -d '{"file_path": "input.pdf", "output_path": "output.pdf", "target_orientation": "portrait"}'
```

## 13. `pdf_bates`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Applies consecutive legal Bates numbering stamps across documents.**  
Stamps formatted identification numbers (e.g. `DOC-000001`, `EXHIBIT-A-00042`) onto page margins for legal discovery and court filings.

### CLI
```bash
paperpilot bates --input tests/e2e_fixtures/real/single_page.pdf --output tests/e2e_fixtures/out/penta_e2e_security/bates_cli_Simple.pdf --prefix BATES --start 1 --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_bates",
  "arguments": {
    "input": "tests/e2e_fixtures/real/multi_page.pdf",
    "prefix": "CONF-",
    "start": 1,
    "output": "tests/e2e_fixtures/out/penta_e2e/bates.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/bates -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/multi_page.pdf", "prefix": "CONF-", "start": 1, "output": "tests/e2e_fixtures/out/penta_e2e/bates.pdf"}'
```

## 14. `pdf_booklet`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Transforms a document into print-ready booklet signatures.**  
Calculates saddle-stitch page pairings and positions pages side-by-side on sheets so that when folded down the spine, the pages read in natural sequence.

### CLI
```bash
paperpilot booklet --input tests/e2e_fixtures/real/multi_page.pdf --output tests/e2e_fixtures/out/penta_e2e/booklet.pdf --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_booklet",
  "arguments": {
    "input": "tests/e2e_fixtures/real/multi_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/booklet.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/booklet -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/multi_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/booklet.pdf"}'
```

## 15. `pdf_bookmarks`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Reads, extracts, or updates the document outline and Table of Contents bookmarks.**  
Allows inspection of internal document navigation trees and page target destinations.

### CLI
```bash
paperpilot bookmarks --input tests/e2e_fixtures/real/multi_page.pdf --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_bookmarks",
  "arguments": {
    "input": "tests/e2e_fixtures/real/multi_page.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/bookmarks -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/multi_page.pdf"}'
```

## 16. `pdf_burst`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Explodes a multi-page PDF into separate 1-page PDF files.**  
If you have a 10-page document, this creates 10 individual single-page files (`page_1.pdf`, `page_2.pdf`, etc.) in an output directory.
- **Opposite of Merge**: Where `merge` combines multiple files into 1, `burst` splits 1 file into every individual page.
- **Different from Split**: `split` requires cut points or page ranges; `burst` automatically breaks up all pages individually.

### CLI
```bash
paperpilot burst --input tests/e2e_fixtures/real/multi_page.pdf --output tests/e2e_fixtures/out/penta_e2e/burst --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_burst",
  "arguments": {
    "input": "tests/e2e_fixtures/real/multi_page.pdf",
    "output_dir": "tests/e2e_fixtures/out/penta_e2e/burst"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/burst -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/multi_page.pdf", "output_dir": "tests/e2e_fixtures/out/penta_e2e/burst"}'
```

## 17. `pdf_change_permissions`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

Configure user and owner permissions for printing, copying, and modification according to the ISO 32000-1 specification. This tool enables granular control over user rights across document viewing platforms.

### CLI
```bash
N/A
```

### MCP JSON-RPC
```json
N/A
```

### REST API
```bash
N/A
```

## 18. `pdf_classify_type`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Classifies a PDF document into standard taxonomy types using structural analysis.**  
Automatically identifies whether a document is an Invoice, Contract, Scientific Paper, Presentation, or Form based on layout and typography.

### CLI
```bash
paperpilot classify --input tests/e2e_fixtures/real/single_page.pdf --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_classify_type",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/classify-type -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf"}'
```

## 19. `pdf_compare`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Compares two PDF documents and highlights visual and textual differences.**  
Performs side-by-side diffing, identifying added, removed, or shifted text and graphics between document revisions.

### CLI
```bash
paperpilot compare --input tests/e2e_fixtures/real/merge_a.pdf --input-b tests/e2e_fixtures/real/merge_b.pdf --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_compare",
  "arguments": {
    "file1": "tests/e2e_fixtures/real/merge_a.pdf",
    "file2": "tests/e2e_fixtures/real/merge_b.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/compare -H 'Content-Type: application/json' -d '{"file1": "tests/e2e_fixtures/real/merge_a.pdf", "file2": "tests/e2e_fixtures/real/merge_b.pdf"}'
```

## 20. `pdf_compress`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Reduces PDF file size through intelligent image resampling and stream compression.**  
Re-encodes high-resolution images, applies Flate compression to content streams, and strips redundant orphan objects to make PDFs email and web friendly.

### CLI
```bash
paperpilot compress --input tests/e2e_fixtures/real/single_page.pdf --quality medium --output tests/e2e_fixtures/out/penta_e2e/compressed.pdf --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_compress",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "quality": "medium",
    "output": "tests/e2e_fixtures/out/penta_e2e/compressed.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/compress -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "quality": "medium", "output": "tests/e2e_fixtures/out/penta_e2e/compressed.pdf"}'
```

## 21. `pdf_convert_excel`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Extracts tables and tabular columns from PDFs into editable Excel spreadsheets (.xlsx).**  
Detects table gridlines and cell boundaries to reconstruct structured rows and columns in an Excel workbook.

### CLI
```bash
paperpilot convert --input tests/e2e_fixtures/real/test.csv --format pdf --output tests/e2e_fixtures/out/penta_e2e/out_csv.pdf --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_convert_excel",
  "arguments": {
    "input": "tests/e2e_fixtures/real/test.csv",
    "output": "tests/e2e_fixtures/out/penta_e2e/out_csv.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/convert-excel -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/test.csv", "output": "tests/e2e_fixtures/out/penta_e2e/out_csv.pdf"}'
```

## 22. `pdf_convert_html`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Converts PDF documents into responsive, semantic HTML5 web pages.**  
Translates layout, headings, paragraphs, and embedded graphics into clean HTML and CSS suitable for browser viewing.

### CLI
```bash
paperpilot convert --input tests/e2e_fixtures/real/test.html --format pdf --output tests/e2e_fixtures/out/penta_e2e/out_html.pdf --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_convert_html",
  "arguments": {
    "input": "tests/e2e_fixtures/real/test.html",
    "output": "tests/e2e_fixtures/out/penta_e2e/out_html.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/convert-html -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/test.html", "output": "tests/e2e_fixtures/out/penta_e2e/out_html.pdf"}'
```

## 23. `pdf_convert_markdown`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Extracts PDF document structure into clean, LLM-ready Markdown.**  
Preserves section titles, bullet points, code blocks, and tables while discarding complex layout noise for AI pipelines.

### CLI
```bash
paperpilot convert --input tests/e2e_fixtures/real/test.md --format pdf --output tests/e2e_fixtures/out/penta_e2e/out_md.pdf --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_convert_markdown",
  "arguments": {
    "input": "tests/e2e_fixtures/real/test.md",
    "output": "tests/e2e_fixtures/out/penta_e2e/out_md.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/convert-markdown -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/test.md", "output": "tests/e2e_fixtures/out/penta_e2e/out_md.pdf"}'
```

## 24. `pdf_create_form_field`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Programmatically adds new interactive AcroForm form fields into a PDF.**  
Inserts text inputs, checkboxes, or signature fields at specified coordinates so users can fill them out in any PDF reader.

### CLI
```bash
paperpilot form add-field tests/e2e_fixtures/real/single_page.pdf --name signature --type text --rect 50,50,150,30 --output tests/e2e_fixtures/out/penta_e2e_security/create_form_field_cli_Simple.pdf --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_create_form_field",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "name": "f1",
    "rect": [
      0,
      0,
      10,
      10
    ],
    "output": "tests/e2e_fixtures/out/penta_e2e/added_field.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/create-form-field -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "name": "f1", "rect": [0,0,10,10], "output": "tests/e2e_fixtures/out/penta_e2e/added_field.pdf"}'
```

## 25. `pdf_crop`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Trims the margins or crops pages to a custom rectangular bounding box.**  
Adjusts the PDF `/CropBox` and `/MediaBox` to remove printer crop marks, excessive white borders, or isolate a specific page region.

### CLI
```bash
paperpilot crop --input tests/e2e_fixtures/real/single_page.pdf --rect 10,10,200,200 --output tests/e2e_fixtures/out/penta_e2e/cropped.pdf --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_crop",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "box": "10,10,200,200",
    "output": "tests/e2e_fixtures/out/penta_e2e/cropped.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/crop -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "box": "10,10,200,200", "output": "tests/e2e_fixtures/out/penta_e2e/cropped.pdf"}'
```

## 26. `pdf_decrypt`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Removes password protection and usage restrictions from an unlocked PDF.**  
Decrypts the document using the known password and outputs an unrestricted PDF that can be opened, printed, and edited freely.

### CLI
```bash
paperpilot decrypt --input tests/e2e_fixtures/real/encrypted.pdf --output tests/e2e_fixtures/out/penta_e2e_security/decrypt_cli_Simple.pdf --password testpass --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_decrypt",
  "arguments": {
    "input": "tests/e2e_fixtures/real/encrypted.pdf",
    "password": "testpass",
    "output": "tests/e2e_fixtures/out/penta_e2e/decrypted.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/decrypt -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/encrypted.pdf", "password": "testpass", "output": "tests/e2e_fixtures/out/penta_e2e/decrypted.pdf"}'
```

## 27. `pdf_dedup_streams`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

The `pdf_dedup_streams` tool performs a lossless object stream deduplication pass on the PDF document. It leverages cryptographic hashing (SHA-256) to identify byte-identical streams across the entire document structure, ensuring zero false positives. Duplicate streams are merged into a single canonical indirect object, reducing file size significantly without sacrificing any data.

### CLI
```bash
N/A
```

### MCP JSON-RPC
```json
N/A
```

### REST API
```bash
N/A
```

## 28. `pdf_delete_pages`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Removes specific pages from a document (e.g., delete pages 2, 5, and 8).**  
Prunes unwanted pages from the PDF page tree and rewrites the document catalog cleanly.

### CLI
```bash
paperpilot delete --input tests/e2e_fixtures/real/multi_page.pdf --pages 2,4 --output tests/e2e_fixtures/out/penta_e2e/deleted.pdf --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_delete_pages",
  "arguments": {
    "input": "tests/e2e_fixtures/real/multi_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/deleted.pdf",
    "pages": "2,4"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/delete-pages -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/multi_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/deleted.pdf", "pages": "2,4"}'
```

## 29. `pdf_encrypt`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Secures a PDF document with standard AES-256 password encryption.**  
Sets user opening passwords and owner permission passwords to restrict printing, copying text, or editing.

### CLI
```bash
paperpilot encrypt --input tests/e2e_fixtures/real/single_page.pdf --output tests/e2e_fixtures/out/penta_e2e_security/encrypt_cli_Simple.pdf --user-password testpass --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_encrypt",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "password": "testpass",
    "output": "tests/e2e_fixtures/out/penta_e2e/encrypted.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/encrypt -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "password": "testpass", "output": "tests/e2e_fixtures/out/penta_e2e/encrypted.pdf"}'
```

## 30. `pdf_extract_images`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Pulls out all photos, graphics, and figures embedded inside a PDF file and saves them to a folder.**  
Extracts embedded image streams without lossy re-compression directly to `.png` or `.jpg` image files, making it easy to recover high-res graphics and photos from documents.

### CLI
```bash
paperpilot extract-images --input tests/e2e_fixtures/real/single_page.pdf --output tests/e2e_fixtures/out/penta_e2e/images --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_extract_images",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output_dir": "tests/e2e_fixtures/out/penta_e2e/images"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/extract-images -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "output_dir": "tests/e2e_fixtures/out/penta_e2e/images"}'
```

## 31. `pdf_extract_pages`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Extracts specific pages or page ranges (e.g., pages 1-3, 5) into a new PDF.**  
Pulls the selected pages into a standalone document without affecting the original file.

### CLI
```bash
paperpilot extract --input tests/e2e_fixtures/real/multi_page.pdf --pages 1,3 --output tests/e2e_fixtures/out/penta_e2e/extracted.pdf --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_extract_pages",
  "arguments": {
    "input": "tests/e2e_fixtures/real/multi_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/extracted.pdf",
    "pages": "1,3"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/extract-pages -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/multi_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/extracted.pdf", "pages": "1,3"}'
```

## 32. `pdf_extract_text`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Extracts all readable text from a PDF with layout and reading-order awareness.**  
Reads text character streams and reconstructs logical paragraphs and columns into a clean `.txt` output.

### CLI
```bash
paperpilot extract-text --input tests/e2e_fixtures/real/single_page.pdf --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_extract_text",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/extract-text -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf"}'
```

## 33. `pdf_fill_form`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Fills interactive PDF AcroForm fields automatically from a JSON key-value map.**  
Populates text fields and checkboxes, including fully qualified hierarchical names and UTF-16 PDF values. Regenerates widget appearances so filled text is visible before and after form flattening. Radio buttons, pushbuttons and other unsupported field types return an error.

For CSV/JSON mail merge across many records, use [`paperpilot fill-batch`](/reference/tools/cli_fill_batch/).

### CLI
```bash
paperpilot form fill tests/e2e_fixtures/form.pdf --data tests/e2e_fixtures/form_data.json --output tests/e2e_fixtures/out/penta_e2e_security/fill_form_cli_Simple.pdf --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_fill_form",
  "arguments": {
    "input": "tests/e2e_fixtures/real/form.pdf",
    "data": {},
    "output": "tests/e2e_fixtures/out/penta_e2e/filled.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/fill-form -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/form.pdf", "data": {}, "output": "tests/e2e_fixtures/out/penta_e2e/filled.pdf"}'
```

## 34. `pdf_flatten`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Paints AcroForm widget appearances into page content and removes the interactive form fields.**
Preserves existing page resources and unrelated annotations. Missing appearances for supported text fields and checkboxes are generated from their values. Hidden widgets are removed without drawing their values. Existing normal appearances, including transformed appearance boxes, retain their placement. This operation does not remove or flatten unrelated annotation types and does not establish signature validity.

### CLI
```bash
paperpilot flatten --input tests/e2e_fixtures/form.pdf --output tests/e2e_fixtures/out/penta_e2e_security/flatten_cli_Simple.pdf --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_flatten",
  "arguments": {
    "input": "tests/e2e_fixtures/real/form.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/flattened.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/flatten -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/form.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/flattened.pdf"}'
```

## 35. `pdf_hash`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Calculates cryptographic checksums (MD5, SHA-256) of a PDF document.**  
Produces a unique digital fingerprint to verify document integrity, detect tampering, and validate chain of custody.

### CLI
```bash
paperpilot hash --input tests/e2e_fixtures/real/missing.pdf --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_hash",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/hash -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf"}'
```

## 36. `pdf_header_footer`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Adds custom header and footer text with configurable margins and alignments across pages.**  
Stamps company names, document titles, dates, or confidentiality notices at top and bottom margins.

### CLI
```bash
paperpilot header-footer --input tests/e2e_fixtures/real/single_page.pdf --output tests/e2e_fixtures/out/penta_e2e_security/header_footer_cli_Simple.pdf --text HEADER --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_header_footer",
  "arguments": {
    "input": "tests/e2e_fixtures/real/multi_page.pdf",
    "text": "Confidential",
    "output": "tests/e2e_fixtures/out/penta_e2e/header.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/header-footer -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/multi_page.pdf", "text": "Confidential", "output": "tests/e2e_fixtures/out/penta_e2e/header.pdf"}'
```

## 37. `pdf_images_to_pdf`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Combines one or more image files (PNG, JPG, WebP) into a single PDF document.**  
Each image is placed onto its own page with original image dimensions preserved at standard print resolution (72 DPI baseline). Ideal for photo albums, scanned receipts, or creating a PDF presentation from graphic slides.

### CLI
```bash
paperpilot images-to-pdf --images tests/e2e_fixtures/real/test.png --output tests/e2e_fixtures/out/penta_e2e/img2pdf.pdf --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_images_to_pdf",
  "arguments": {
    "images": [
      "tests/e2e_fixtures/real/test.png"
    ],
    "output": "tests/e2e_fixtures/out/penta_e2e/img2pdf.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/images-to-pdf -H 'Content-Type: application/json' -d '{"images": ["tests/e2e_fixtures/real/test.png"], "output": "tests/e2e_fixtures/out/penta_e2e/img2pdf.pdf"}'
```

## 38. `pdf_invert_colors`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

This tool processes vector PDFs and intelligently inverts colors or applies a true dark mode, all while preserving selectable vector text and embedded fonts.

### CLI
```bash
N/A
```

### MCP JSON-RPC
```json
N/A
```

### REST API
```bash
N/A
```

## 39. `pdf_linearize`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Optimizes a PDF for Fast Web View (byte-serving linearization).**  
Restructures internal cross-reference tables and streams so web browsers can instantly render Page 1 before the entire file downloads.

### CLI
```bash
paperpilot linearize --input tests/e2e_fixtures/real/single_page.pdf --output tests/e2e_fixtures/out/penta_e2e/linearized.pdf --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_linearize",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/linearized.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/linearize -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/linearized.pdf"}'
```

## 40. `pdf_merge`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Combines multiple separate PDF documents into a single continuous file.**  
Appends pages in order, unifies document outlines/bookmarks, and consolidates shared font resources into one master document.

### CLI
```bash
paperpilot merge --input tests/e2e_fixtures/real/merge_a.pdf tests/e2e_fixtures/real/merge_b.pdf --output tests/e2e_fixtures/out/penta_e2e/merge_cli_simple.pdf --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_merge",
  "arguments": {
    "inputs": [
      "tests/e2e_fixtures/real/merge_a.pdf",
      "tests/e2e_fixtures/real/merge_b.pdf"
    ],
    "output": "tests/e2e_fixtures/out/penta_e2e/merge_mcp_simple.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/merge -H 'Content-Type: application/json' -d '{"inputs": ["tests/e2e_fixtures/real/merge_a.pdf", "tests/e2e_fixtures/real/merge_b.pdf"], "output": "tests/e2e_fixtures/out/penta_e2e/merge_mcp_simple.pdf"}'
```

## 41. `pdf_merge_alternate`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

The `pdf_merge_alternate` tool allows you to interleave pages from two different PDF documents. This is a critical operation for manual duplex scanning, where you might have one PDF containing all the front pages and another PDF containing the back pages.

### CLI
```bash
N/A
```

### MCP JSON-RPC
```json
N/A
```

### REST API
```bash
N/A
```

## 42. `pdf_metadata`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Inspects, edits, or sanitizes document metadata (Title, Author, Subject, Keywords, Creator).**  
Allows viewing document properties or completely scrubbing personal tracking and author information for privacy compliance.

### CLI
```bash
paperpilot metadata --input tests/e2e_fixtures/real/single_page.pdf --output tests/e2e_fixtures/out/penta_e2e/metadata.json --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_metadata",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/metadata -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf"}'
```

## 43. `pdf_nup`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Imposes multiple pages onto a single sheet in an N-Up grid layout.**  
Arranges pages in 2-up, 4-up, 8-up, or 16-up orientations with automatic aspect ratio scaling and customizable margins for paper-saving prints and slide handouts.

### CLI
```bash
paperpilot nup --input tests/e2e_fixtures/real/multi_page.pdf --output tests/e2e_fixtures/out/penta_e2e/nup.pdf --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_nup",
  "arguments": {
    "input": "tests/e2e_fixtures/real/multi_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/nup.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/nup -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/multi_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/nup.pdf"}'
```

## 44. `pdf_ocr`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Recognizes text in scanned PDFs and camera photos using local neural OCR.**  
Detects text in image-only documents and embeds an invisible searchable text layer behind the scan without sending data to the cloud.

### CLI
```bash
paperpilot ocr --input tests/e2e_fixtures/real/single_page.pdf --output tests/e2e_fixtures/out/penta_e2e/ocr.pdf --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_ocr",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/ocr.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/ocr -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/ocr.pdf"}'
```

## 45. `pdf_overlay_underlay`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

Overlay or underlay a template PDF (e.g., stationery letterhead or watermark) onto a target document.

### CLI
```bash
N/A
```

### MCP JSON-RPC
```json
N/A
```

### REST API
```bash
N/A
```

## 46. `pdf_page_numbers`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Adds dynamic page numbering stamps (e.g. "Page 1 of 5" or "1") to your document.**  
Stamps clean, professional page numbers at your chosen position (bottom-center, bottom-right, top-right, etc.) across the document without shifting or altering any existing text.

### CLI
```bash
paperpilot page-numbers --input tests/e2e_fixtures/real/single_page.pdf --output tests/e2e_fixtures/out/penta_e2e_security/page_numbers_cli_Simple.pdf --position bottom-right --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_page_numbers",
  "arguments": {
    "input": "tests/e2e_fixtures/real/multi_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/numbers.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/page-numbers -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/multi_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/numbers.pdf"}'
```

## 47. `pdf_posterize`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

The `pdf_posterize` tool takes a single-page or multi-page PDF with large dimensions and tiles each page across a grid of smaller standard sheets (e.g., 2x2, 3x3, or auto-calculated). It allows you to print oversized documents (like maps, schematics, or posters) on standard home or office printers.

### CLI
```bash
paperpilot-cli pdf-posterize --input large-map.pdf --output poster-tiles.pdf --target-sheet a4 --overlap 15 --scale 1.5
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "call_tool",
  "params": {
    "name": "pdf_posterize",
    "arguments": {
      "input": "/path/to/large-map.pdf",
      "output": "/path/to/poster-tiles.pdf",
      "target_sheet": "a4",
      "overlap_pt": 15.0,
      "scale": 1.5,
      "include_guides": true
    }
  },
  "id": 1
}
```

### REST API
```bash
curl -X POST http://localhost:8080/api/v1/pdf/posterize \\
  -H "Content-Type: application/json" \\
  -d '{
    "input": "/path/to/large-map.pdf",
    "output": "/path/to/poster-tiles.pdf",
    "target_sheet": "a4",
    "overlap_pt": 15.0,
    "scale": 1.5
  }'
```

## 48. `pdf_rag_chunks`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Splits a PDF into semantically meaningful text chunks tailored for AI RAG embeddings.**  
Divides documents by headings, paragraphs, and token limits with overlap, returning structured JSON chunks for vector search.

### CLI
```bash
paperpilot rag-chunks --input tests/e2e_fixtures/real/single_page.pdf
```

### MCP JSON-RPC
```json
{
  "name": "pdf_rag_chunks",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/rag-chunks -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf"}'
```

## 49. `pdf_read_form`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Reads and exports form field names and values from a PDF into JSON.**
Exports AcroForm field names and values, including hierarchical field names and UTF-16 text strings. Checkbox values retain their PDF export state. Cyclic or excessively nested field trees return an error.

### CLI
```bash
paperpilot form read tests/e2e_fixtures/real/form.pdf --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_read_form",
  "arguments": {
    "input": "tests/e2e_fixtures/real/form.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/read-form -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/form.pdf"}'
```

## 50. `pdf_redact`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Permanently blackouts and scrubs sensitive text or coordinates from a PDF.**  
Unlike simple black highlight rectangles, this completely deletes the underlying text operators and vector data so redacted information cannot be recovered.

### CLI
```bash
paperpilot redact --input tests/e2e_fixtures/real/single_page.pdf --output tests/e2e_fixtures/out/penta_e2e_security/redact_cli_Simple.pdf --pages 1 --rect 0,0,100,100 --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_redact",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "page": 1,
    "rect": [
      50,
      50,
      200,
      50
    ],
    "output": "tests/e2e_fixtures/out/penta_e2e/redacted.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/redact -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "page": 1, "rect": [50, 50, 200, 50], "output": "tests/e2e_fixtures/out/penta_e2e/redacted.pdf"}'
```

## 51. `pdf_remove_blank`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Scans a PDF and strips out all completely blank or empty pages.**  
Analyzes text operators, vector paths, and pixel ink density across every page. Any accidental empty pages (common from double-sided scanning or printer slips) are safely removed while preserving all content on the remaining pages.

### CLI
```bash
paperpilot remove-blank --input tests/e2e_fixtures/real/multi_page.pdf --output tests/e2e_fixtures/out/penta_e2e/noblank.pdf --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_remove_blank",
  "arguments": {
    "input": "tests/e2e_fixtures/real/multi_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/noblank.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/remove-blank -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/multi_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/noblank.pdf"}'
```

## 52. `pdf_remove_signatures`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Strips digital signatures, signature field appearances, and AcroForm signature dictionaries from a PDF.**  
Flattens or strips signature annotations while preserving document tree integrity and remaining form fields.

### CLI
```bash
paperpilot pdf-remove-signatures --input tests/e2e_fixtures/real/single_page.pdf --output tests/e2e_fixtures/out/penta_e2e/stripped.pdf --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_remove_signatures",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/stripped.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/remove-signatures -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/stripped.pdf"}'
```

## 53. `pdf_render`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Renders PDF pages into high-resolution raster images (PNG, JPEG).**  
Rasterizes vector paths, text glyphs, and embedded graphics at target DPI for web previews, thumbnail generation, or image processing.

### CLI
```bash
paperpilot render --input tests/e2e_fixtures/real/single_page.pdf --output tests/e2e_fixtures/out/penta_e2e/rendered.png --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_render",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/rendered.png"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/render -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/rendered.png"}'
```

## 54. `pdf_reorder_pages`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Rearranges the sequence of pages in a PDF according to a custom order.**  
Re-indexes the document page tree (e.g. `[3, 1, 2]`) to fix inverted scans or reorder presentation slides.

### CLI
```bash
paperpilot reorder --input tests/e2e_fixtures/real/multi_page.pdf --order 2,1,3,4,5 --output tests/e2e_fixtures/out/penta_e2e/reordered.pdf --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_reorder_pages",
  "arguments": {
    "input": "tests/e2e_fixtures/real/multi_page.pdf",
    "order": "2,1,3,4,5",
    "output": "tests/e2e_fixtures/out/penta_e2e/reordered.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/reorder-pages -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/multi_page.pdf", "order": "2,1,3,4,5", "output": "tests/e2e_fixtures/out/penta_e2e/reordered.pdf"}'
```

## 55. `pdf_repair`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Rebuilds corrupted, truncated, or unreadable PDF files.**  
Scans raw file streams, recovers damaged cross-reference tables (xref), fixes broken object trailers, and salvages readable pages.

### CLI
```bash
paperpilot repair --input tests/e2e_fixtures/real/single_page.pdf --output tests/e2e_fixtures/out/penta_e2e/repaired.pdf --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_repair",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/repaired.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/repair -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/repaired.pdf"}'
```

## 56. `pdf_rotate`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Rotates PDF pages by 90, 180, or 270 degrees clockwise.**  
Adjusts the `/Rotate` dictionary attribute to fix upside-down or sideways orientation across all pages or selected pages.

### CLI
```bash
paperpilot rotate --input tests/e2e_fixtures/real/single_page.pdf --degrees 90 --pages 1 --output tests/e2e_fixtures/out/penta_e2e/rotated.pdf --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_rotate",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "angle": 90,
    "pages": "1",
    "output": "tests/e2e_fixtures/out/penta_e2e/rotated.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/rotate -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "angle": 90, "pages": "1", "output": "tests/e2e_fixtures/out/penta_e2e/rotated.pdf"}'
```

## 57. `pdf_sanitize`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Performs deep structural sanitization and threat neutralization on PDF files.**  
Strips metadata, removes embedded file attachments, neutralizes JavaScript actions and launch triggers, deletes embedded thumbnails, and cleans private application data.

### CLI
```bash
paperpilot sanitize --input tests/e2e_fixtures/real/multi_page.pdf --output tests/e2e_fixtures/out/penta_e2e/sanitized.pdf --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_sanitize",
  "arguments": {
    "input": "tests/e2e_fixtures/real/multi_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/sanitized.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/sanitize -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/multi_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/sanitized.pdf"}'
```

## 58. `pdf_scale_pages`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Scales PDF pages between standard formats (A4, Letter, Legal, A3) or custom percentages.**  
Transforms page media and crop geometry, calculates exact scaling ratios with aspect ratio preservation, and centers content on target dimensions.

### CLI
```bash
paperpilot pdf-scale-pages --input tests/e2e_fixtures/real/single_page.pdf --output tests/e2e_fixtures/out/penta_e2e/scaled.pdf --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_scale_pages",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/scaled.pdf",
    "options": {
      "target_size": "A4",
      "scale_factor": 1.0,
      "maintain_aspect_ratio": true,
      "center_content": true
    }
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/scale-pages -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/scaled.pdf"}'
```

## 59. `pdf_search`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Searches for text within a PDF and returns exact page numbers and spatial bounding boxes.**  
Instead of just telling you if a word is present, it returns coordinates `[x, y, width, height]` for each occurrence, allowing applications to highlight matches on screen or feed target locations into redaction workflows.

### CLI
```bash
paperpilot search --input tests/e2e_fixtures/real/single_page.pdf --query test --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_search",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "query": "test"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/search -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "query": "test"}'
```

## 60. `pdf_sign`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Applies cryptographic digital signatures and signature stamps to a PDF.**  
Embeds PKCS#7 / CMS digital signatures with X.509 certificates to verify signer identity and document authenticity.

### CLI
```bash
paperpilot sign --input tests/e2e_fixtures/real/single_page.pdf --output tests/e2e_fixtures/out/penta_e2e_security/sign_cli_Simple.pdf --cert tests/e2e_fixtures/real/cert.pem --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_sign",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/signed.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/sign -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/signed.pdf"}'
```

## 61. `pdf_split`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Splits a PDF into smaller documents based on page numbers or split points.**  
Cuts a document into multiple files (e.g. split at page 3 gives pages 1-2 in one file, pages 3+ in another).

### CLI
```bash
paperpilot split --input tests/e2e_fixtures/real/multi_page.pdf --pages 1,3 --output tests/e2e_fixtures/out/penta_e2e/pdf_split_simple --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_split",
  "arguments": {
    "input": "tests/e2e_fixtures/real/multi_page.pdf",
    "output_dir": "tests/e2e_fixtures/out/penta_e2e/pdf_split_simple",
    "pages": "1,3"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/split -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/multi_page.pdf", "output_dir": "tests/e2e_fixtures/out/penta_e2e/pdf_split_simple", "pages": "1,3"}'
```

## 62. `pdf_split_by_outline`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

The `pdf_split_by_outline` tool automatically extracts sections of a PDF document based on its internal Bookmark (`/Outlines`) structure. This is particularly useful for textbooks or technical manuals where you want a single file split into multiple chapter PDFs.

### CLI
```bash
paperpilot pdf-split-by-outline --input my-book.pdf --output output.zip --max-depth 1
```

### MCP JSON-RPC
```json
N/A
```

### REST API
```bash
curl -X POST http://localhost:7823/api/v1/pdf/split-by-outline \
  -H "Content-Type: application/json" \
  -d '{
    "input": "my-book.pdf",
    "output": "output.zip",
    "max_depth": 1,
    "sanitize_names": true
  }'
```

## 63. `pdf_split_scanned_book`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

Detect center binding gutter spine and split two-up page scans into single pages without modifying underlying vector streams, avoiding rasterization blur.

### CLI
```bash
N/A
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_split_scanned_book",
    "arguments": {
      "input": "scan_spread.pdf",
      "output": "split_pages.pdf",
      "reading_direction": "ltr",
      "gutter_overlap": 5.0,
      "split_horizontal": false
    }
  }
}
```

### REST API
```bash
N/A
```

## 64. `pdf_stamp`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

Places an image (such as a logo, signature, or watermark) onto specified pages of a PDF document at specific coordinates, with optional transparency.

### CLI
```bash
penta pdf-stamp input.pdf output.pdf --image-path stamp.png --x 100 --y 100 --width 200 --height 50 --opacity 0.5 --pages all
```

### MCP JSON-RPC
```json
N/A
```

### REST API
```bash
curl -X POST http://localhost:8080/api/v1/pdf/stamp \
  -F "file=@input.pdf" \
  -F 'options={"image_path":"stamp.png","x":100.0,"y":100.0,"width":200.0,"height":50.0,"opacity":0.5,"pages":"all"}' \
  -o output.pdf
```

## 65. `pdf_stitch_canvas`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

Combine multiple pages of a PDF document into a single continuous canvas, either vertically or horizontally.

### CLI
```bash
paperpilot pdf-stitch-canvas --input input.pdf --output stitched.pdf --direction horizontal --margin 10 --spacing 5
```

### MCP JSON-RPC
```json
N/A
```

### REST API
```bash
curl -X POST http://localhost:8000/api/v1/pdf/stitch-canvas \
  -H "Content-Type: multipart/form-data" \
  -F "file=@input.pdf" \
  -F "direction=vertical" \
  -F "margin_pt=10" \
  -F "spacing_pt=5"
```

## 66. `pdf_to_docx`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Converts PDF documents into editable Microsoft Word (.docx) files.**  
Reconstructs text layout, headings, side-by-side tables, and embeds images into a fully editable Word document.

### CLI
```bash
paperpilot convert --input tests/e2e_fixtures/real/single_page.pdf --format docx --output tests/e2e_fixtures/out/penta_e2e/out.docx --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_to_docx",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/out.docx"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/to-docx -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/out.docx"}'
```

## 67. `pdf_to_epub`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

Convert a standard PDF document into an accessible and reflowable EPUB 3.0 file format.

### CLI
```bash
N/A
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "call_tool",
  "params": {
    "name": "pdf_to_epub",
    "arguments": {
      "input_file": "/tmp/book.pdf",
      "options": {
        "title": "My Book",
        "author": "Jane Doe",
        "language": "en"
      }
    }
  },
  "id": 1
}
```

### REST API
```bash
curl -X POST https://api.paperpilot.test/v1/pdf/epub \
  -F "file=@book.pdf" \
  -F 'options={"title":"My Book","author":"Jane Doe","language":"en"}'
```

## 68. `pdf_to_html`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Converts a PDF into styled, standalone HTML5.**  
Extracts document layout, fonts, and inline media into responsive web-friendly markup without requiring browser plugins.

### CLI
```bash
paperpilot convert --input tests/e2e_fixtures/real/single_page.pdf --format html --output tests/e2e_fixtures/out/penta_e2e/doc.html
```

### MCP JSON-RPC
```json
{
  "name": "pdf_to_html",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/doc.html"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/to-html -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/doc.html"}'
```

## 69. `pdf_to_pdf_a`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Converts a PDF into the ISO 19005 standard archival format (PDF/A).**  
Embeds all fonts, strips external device dependencies and dynamic actions to guarantee the document can be read decades into the future.

### CLI
```bash
paperpilot pdf-a --input tests/e2e_fixtures/real/single_page.pdf --output tests/e2e_fixtures/out/penta_e2e/pdf_a.pdf --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_to_pdf_a",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/pdf_a.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/to-pdf-a -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/pdf_a.pdf"}'
```

## 70. `pdf_to_pptx`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Converts PDF pages into an editable Microsoft PowerPoint presentation (.pptx).**  
Maps each page into a presentation slide with text boxes and background graphics.

### CLI
```bash
paperpilot convert --input tests/e2e_fixtures/real/single_page.pdf --format pptx --output tests/e2e_fixtures/out/penta_e2e/out.pptx --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_to_pptx",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/out.pptx"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/to-pptx -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/out.pptx"}'
```

## 71. `pdf_to_xlsx`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Extracts tabular data and financial statements from PDF into an Excel workbook (.xlsx).**  
Detects table structures and cell values to generate clean, editable spreadsheets.

### CLI
```bash
paperpilot convert --input tests/e2e_fixtures/real/single_page.pdf --format xlsx --output tests/e2e_fixtures/out/penta_e2e/out.xlsx --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_to_xlsx",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/out.xlsx"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/to-xlsx -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/out.xlsx"}'
```

## 72. `pdf_validate`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Runs deep diagnostic checks on PDF structure and compliance.**  
Validates cross-reference tables, font descriptors, catalog integrity, and identifies syntax errors or corruption warnings.

### CLI
```bash
paperpilot validate --input tests/e2e_fixtures/real/single_page.pdf --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_validate",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/validate -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf"}'
```

## 73. `pdf_validate_signatures`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

Cryptographically verify digital signatures, certificate chains, and document integrity. Supports dual-layer audit by checking against a baseline document for pre-signature structural/semantic alterations (Spec 101).

### CLI
```bash
paperpilot pdf-validate-signatures --input signed.pdf --output report.json
# With baseline verification
paperpilot pdf-validate-signatures --input signed.pdf --output report.json --baseline original.pdf
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "id": "1",
  "method": "callTool",
  "params": {
    "name": "pdf_validate_signatures",
    "arguments": {
      "input": "signed.pdf",
      "output": "report.json",
      "baseline": true
    }
  }
}
```

### REST API
```bash
N/A
```

## 74. `pdf_watermark`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Stamps semi-transparent text or image watermarks across PDF pages.**  
Adds diagonal or centered branding (e.g., 'CONFIDENTIAL', 'DRAFT') with custom opacity, font size, and rotation.

### CLI
```bash
paperpilot watermark --input tests/e2e_fixtures/real/single_page.pdf --output tests/e2e_fixtures/out/penta_e2e_security/watermark_cli_Simple.pdf --text WATERMARK --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_watermark",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "text": "CONFIDENTIAL",
    "output": "tests/e2e_fixtures/out/penta_e2e/watermarked.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/watermark -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "text": "CONFIDENTIAL", "output": "tests/e2e_fixtures/out/penta_e2e/watermarked.pdf"}'
```

## 75. `semantic_search`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

The semantic search tool queries the `paperpilot-storage` vector store database using cosine similarity to rank chunks.

### CLI
```bash
N/A
```

### MCP JSON-RPC
```json
N/A
```

### REST API
```bash
N/A
```

## 76. `workspace_search`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Searches across all documents in a local folder for matching keywords.**  
Performs multi-file document discovery, returning matching files, page numbers, and preview excerpts.

### CLI
```bash
paperpilot search --query 'test' --dir tests/e2e_fixtures/real
```

### MCP JSON-RPC
```json
{
  "name": "workspace_search",
  "arguments": {
    "query": "test",
    "dir": "tests/e2e_fixtures/real"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/workspace-search -H 'Content-Type: application/json' -d '{"query": "test", "dir": "tests/e2e_fixtures/real"}'
```

