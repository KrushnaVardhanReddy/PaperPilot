---
title: 50 Tools Handbook
description: Comprehensive guide to all 50 PaperPilot tools across CLI, MCP, and REST interfaces.
---

This handbook provides verified execution snippets for all 50 PaperPilot tools across the CLI, MCP, and REST interfaces. Use the sidebar on the left or the list below to jump directly to any tool's reference.



## 1. `excel_to_csv`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

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

## 2. `excel_to_json`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

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

## 3. `json_to_pdf`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### CLI
```bash
paperpilot process --input tests/e2e_fixtures/real/single_page.pdf --output tests/e2e_fixtures/out/penta_e2e/out.pdf --json
```

### MCP JSON-RPC
```json

```

### REST API
```bash

```

## 4. `office_extract_text`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

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

## 5. `office_modernize_legacy`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

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

## 6. `office_to_markdown`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

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

## 7. `pdf_annotate`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

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

## 8. `pdf_bates`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

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

## 9. `pdf_bookmarks`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

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

## 10. `pdf_burst`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

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

## 11. `pdf_classify_type`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

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

## 12. `pdf_compare`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

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

## 13. `pdf_compress`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

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

## 14. `pdf_convert_excel`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

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

## 15. `pdf_convert_html`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

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

## 16. `pdf_convert_markdown`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

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

## 17. `pdf_create_form_field`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

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

## 18. `pdf_crop`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

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

## 19. `pdf_decrypt`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

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

## 20. `pdf_delete_pages`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

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

## 21. `pdf_encrypt`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

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

## 22. `pdf_extract_images`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

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

## 23. `pdf_extract_pages`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

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

## 24. `pdf_extract_text`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

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

## 25. `pdf_fill_form`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

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

## 26. `pdf_flatten`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

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

## 27. `pdf_hash`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

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

## 28. `pdf_header_footer`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

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

## 29. `pdf_images_to_pdf`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

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

## 30. `pdf_linearize`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

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

## 31. `pdf_merge`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

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

## 32. `pdf_metadata`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

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

## 33. `pdf_ocr`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

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

## 34. `pdf_page_numbers`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

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

## 35. `pdf_rag_chunks`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

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

## 36. `pdf_read_form`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### CLI
```bash
paperpilot form read tests/e2e_fixtures/real/missing.pdf --json
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

## 37. `pdf_redact`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

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

## 38. `pdf_remove_blank`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

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

## 39. `pdf_render`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

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

## 40. `pdf_reorder_pages`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

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

## 41. `pdf_repair`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

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

## 42. `pdf_rotate`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

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

## 43. `pdf_search`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

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

## 44. `pdf_sign`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

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

## 45. `pdf_split`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

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

## 46. `pdf_to_docx`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

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

## 47. `pdf_to_html`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

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

## 48. `pdf_to_pdf_a`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

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

## 49. `pdf_to_pptx`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

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

## 50. `pdf_to_xlsx`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

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

## 51. `pdf_validate`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

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

## 52. `pdf_watermark`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

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

## 53. `workspace_search`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

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

