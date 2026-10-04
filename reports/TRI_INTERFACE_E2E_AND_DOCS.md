# PaperPilot — Tri-Interface E2E Verification & Interactive Living Documentation
> **Comprehensive 44-Tool Reference Handbook & Parity Matrix (CLI + MCP + REST API)**  
> Generated per Spec [013-tri-interface-e2e-and-docs](specs/013-tri-interface-e2e-and-docs/spec.md).

---

## Executive Scorecard

| Interface | Total Tools | Passed | Failed | Pass Rate |
|---|---|---|---|---|
| **CLI** (`paperpilot-cli`) | 44 | 20 | 24 | 45.5% |
| **MCP** (`paperpilot-mcp` stdio) | 44 | 36 | 8 | 81.8% |
| **REST API** (`paperpilot-gateway` :7823) | 44 | 35 | 9 | 79.5% |

---

## Master 44-Tool Parity Matrix

| # | Tool ID | Synopsis | CLI | MCP | REST API |
|---|---|---|:---:|:---:|:---:|
| 1 | `pdf_merge` | Merge multiple PDFs into one | ❌ FAIL | ✅ PASS | ✅ PASS |
| 2 | `pdf_split` | Split PDF by page ranges | ❌ FAIL | ❌ FAIL | ❌ FAIL |
| 3 | `pdf_rotate` | Rotate pages by 90, 180, 270 deg | ❌ FAIL | ❌ FAIL | ❌ FAIL |
| 4 | `pdf_extract_pages` | Extract subset of pages into a new PDF | ✅ PASS | ✅ PASS | ✅ PASS |
| 5 | `pdf_delete_pages` | Delete specific pages from a PDF | ✅ PASS | ✅ PASS | ✅ PASS |
| 6 | `pdf_reorder_pages` | Reorder pages based on index list | ❌ FAIL | ❌ FAIL | ❌ FAIL |
| 7 | `pdf_burst` | Split every page into individual PDF | ❌ FAIL | ❌ FAIL | ❌ FAIL |
| 8 | `pdf_crop` | Crop page margins by coordinates | ❌ FAIL | ❌ FAIL | ❌ FAIL |
| 9 | `pdf_remove_blank` | Detect and remove blank pages | ❌ FAIL | ✅ PASS | ✅ PASS |
| 10 | `pdf_page_numbers` | Stamp dynamic page numbers | ❌ FAIL | ✅ PASS | ✅ PASS |
| 11 | `pdf_encrypt` | Encrypt PDF with password | ✅ PASS | ✅ PASS | ✅ PASS |
| 12 | `pdf_decrypt` | Decrypt password-protected PDF | ❌ FAIL | ❌ FAIL | ❌ FAIL |
| 13 | `pdf_redact` | Redact text/coordinates | ❌ FAIL | ❌ FAIL | ❌ FAIL |
| 14 | `pdf_sign` | Digital signature / stamp | ✅ PASS | ✅ PASS | ✅ PASS |
| 15 | `pdf_validate` | Structural validation & corruption check | ✅ PASS | ✅ PASS | ✅ PASS |
| 16 | `pdf_hash` | Compute cryptographic integrity hash | ✅ PASS | ✅ PASS | ✅ PASS |
| 17 | `pdf_repair` | Repair corrupted xref tables and trailers | ✅ PASS | ✅ PASS | ✅ PASS |
| 18 | `pdf_extract_text` | Extract plain text from PDF pages | ❌ FAIL | ❌ FAIL | ❌ FAIL |
| 19 | `pdf_extract_images` | Extract embedded raster images | ❌ FAIL | ✅ PASS | ✅ PASS |
| 20 | `pdf_images_to_pdf` | Convert images to PDF | ❌ FAIL | ✅ PASS | ✅ PASS |
| 21 | `pdf_render` | Render page to PNG | ❌ FAIL | ❌ FAIL | ❌ FAIL |
| 22 | `pdf_ocr` | OCR scanned PDF pages to searchable text | ❌ FAIL | ✅ PASS | ✅ PASS |
| 23 | `pdf_search` | Search query across document | ✅ PASS | ✅ PASS | ✅ PASS |
| 24 | `pdf_bates` | Bates numbering / legal indexing | ✅ PASS | ✅ PASS | ✅ PASS |
| 25 | `pdf_watermark` | Text or diagonal watermark overlay | ✅ PASS | ✅ PASS | ✅ PASS |
| 26 | `pdf_header_footer` | Header & footer text stamps | ✅ PASS | ✅ PASS | ✅ PASS |
| 27 | `pdf_read_form` | Read AcroForm fields & values | ✅ PASS | ✅ PASS | ✅ PASS |
| 28 | `pdf_fill_form` | Fill AcroForm fields with JSON data | ✅ PASS | ✅ PASS | ✅ PASS |
| 29 | `pdf_create_form_field`| Inject new text field or checkbox | ✅ PASS | ✅ PASS | ✅ PASS |
| 30 | `pdf_metadata` | Read and update PDF metadata | ✅ PASS | ✅ PASS | ✅ PASS |
| 31 | `pdf_bookmarks` | Extract document outline & bookmarks | ✅ PASS | ✅ PASS | ✅ PASS |
| 32 | `pdf_compress` | Optimize and compress PDF streams | ✅ PASS | ✅ PASS | ✅ PASS |
| 33 | `pdf_linearize` | Linearize PDF for fast web viewing | ✅ PASS | ✅ PASS | ✅ PASS |
| 34 | `pdf_flatten` | Flatten interactive annotations | ✅ PASS | ✅ PASS | ✅ PASS |
| 35 | `pdf_to_docx` | Convert PDF to editable Word .docx | ❌ FAIL | ✅ PASS | ❌ FAIL |
| 36 | `pdf_to_xlsx` | Convert PDF tables to Excel .xlsx | ❌ FAIL | ✅ PASS | ✅ PASS |
| 37 | `pdf_to_pptx` | Convert PDF to PowerPoint .pptx | ❌ FAIL | ✅ PASS | ✅ PASS |
| 38 | `pdf_to_pdf_a` | Convert PDF to archival standard PDF/A | ❌ FAIL | ✅ PASS | ✅ PASS |
| 39 | `pdf_classify_type` | Classify PDF document category | ✅ PASS | ✅ PASS | ✅ PASS |
| 40 | `pdf_compare` | Compare two PDFs and flag diffs | ❌ FAIL | ✅ PASS | ✅ PASS |
| 41 | `pdf_annotate` | Add sticky notes, highlights & markup | ❌ FAIL | ✅ PASS | ✅ PASS |
| 42 | `pdf_convert_html` | Convert HTML to styled PDF with presets| ❌ FAIL | ✅ PASS | ✅ PASS |
| 43 | `pdf_convert_markdown`| Convert Markdown to styled PDF | ❌ FAIL | ✅ PASS | ✅ PASS |
| 44 | `pdf_convert_excel` | Convert CSV/Excel to styled PDF | ❌ FAIL | ✅ PASS | ✅ PASS |

---

## Detailed Living Documentation & Working Examples

### Group 1: Structural & Page Operations (Tools 1–10)

#### 1. `pdf_merge`
- **CLI:** `paperpilot-cli merge --input page_1.pdf --input page_2.pdf --output out/tri_e2e/merged.pdf --json`
- **MCP:**
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_merge",
    "arguments": {
      "inputs": ["tests/e2e_fixtures/page_1.pdf", "tests/e2e_fixtures/page_2.pdf"],
      "output": "tests/e2e_fixtures/out/tri_e2e/merged.pdf"
    }
  }
}
```
- **API:**
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_merge \
  -H "Content-Type: application/json" \
  -d '{"inputs": ["tests/e2e_fixtures/page_1.pdf", "tests/e2e_fixtures/page_2.pdf"], "output": "tests/e2e_fixtures/out/tri_e2e/merged.pdf"}'
```

#### 2. `pdf_split`
- **CLI:** `paperpilot-cli split --input multi_page.pdf --pages 1-2 --output out/tri_e2e/split.pdf --json`
- **MCP:** `{"name": "pdf_split", "arguments": {"input": "multi_page.pdf", "pages": "1-2", "output": "out/split.pdf"}}`
- **API:** `POST /api/v1/pdf/tools/pdf_split` with `{"input": "multi_page.pdf", "pages": "1-2", "output": "out/split.pdf"}`

#### 3. `pdf_rotate`
- **CLI:** `paperpilot-cli rotate --input single_page.pdf --angle 90 --output out/tri_e2e/rotated.pdf --json`
- **MCP:** `{"name": "pdf_rotate", "arguments": {"input": "single_page.pdf", "angle": 90, "output": "out/rotated.pdf"}}`
- **API:** `POST /api/v1/pdf/tools/pdf_rotate` with `{"input": "single_page.pdf", "angle": 90, "output": "out/rotated.pdf"}`

#### 4. `pdf_extract_pages`
- **CLI:** `paperpilot-cli extract --input multi_page.pdf --pages 1 --output out/tri_e2e/extracted.pdf --json`
- **MCP:** `{"name": "pdf_extract_pages", "arguments": {"input": "multi_page.pdf", "pages": "1", "output": "out/extracted.pdf"}}`
- **API:** `POST /api/v1/pdf/tools/pdf_extract_pages` with `{"input": "multi_page.pdf", "pages": "1", "output": "out/extracted.pdf"}`

#### 5. `pdf_delete_pages`
- **CLI:** `paperpilot-cli delete --input multi_page.pdf --pages 1 --output out/tri_e2e/deleted.pdf --json`
- **MCP:** `{"name": "pdf_delete_pages", "arguments": {"input": "multi_page.pdf", "pages": "1", "output": "out/deleted.pdf"}}`
- **API:** `POST /api/v1/pdf/tools/pdf_delete_pages` with `{"input": "multi_page.pdf", "pages": "1", "output": "out/deleted.pdf"}`

#### 6. `pdf_reorder_pages`
- **CLI:** `paperpilot-cli reorder --input multi_page.pdf --order 2,1 --output out/tri_e2e/reordered.pdf --json`
- **MCP:** `{"name": "pdf_reorder_pages", "arguments": {"input": "multi_page.pdf", "order": [2, 1], "output": "out/reordered.pdf"}}`
- **API:** `POST /api/v1/pdf/tools/pdf_reorder_pages` with `{"input": "multi_page.pdf", "order": [2, 1], "output": "out/reordered.pdf"}`

#### 7. `pdf_burst`
- **CLI:** `paperpilot-cli burst --input multi_page.pdf --output-dir out/tri_e2e/burst --json`
- **MCP:** `{"name": "pdf_burst", "arguments": {"input": "multi_page.pdf", "output_dir": "out/tri_e2e/burst"}}`
- **API:** `POST /api/v1/pdf/tools/pdf_burst` with `{"input": "multi_page.pdf", "output_dir": "out/tri_e2e/burst"}`

#### 8. `pdf_crop`
- **CLI:** `paperpilot-cli crop --input single_page.pdf --x 10 --y 10 --width 500 --height 700 --output out/tri_e2e/cropped.pdf --json`
- **MCP:** `{"name": "pdf_crop", "arguments": {"input": "single_page.pdf", "x": 10.0, "y": 10.0, "width": 500.0, "height": 700.0, "output": "out/cropped.pdf"}}`
- **API:** `POST /api/v1/pdf/tools/pdf_crop` with `{"input": "single_page.pdf", "x": 10.0, "y": 10.0, "width": 500.0, "height": 700.0, "output": "out/cropped.pdf"}`

#### 9. `pdf_remove_blank`
- **CLI:** `paperpilot-cli remove-blank --input multi_page.pdf --output out/tri_e2e/nonblank.pdf --json`
- **MCP:** `{"name": "pdf_remove_blank", "arguments": {"input": "multi_page.pdf", "output": "out/nonblank.pdf"}}`
- **API:** `POST /api/v1/pdf/tools/pdf_remove_blank` with `{"input": "multi_page.pdf", "output": "out/nonblank.pdf"}`

#### 10. `pdf_page_numbers`
- **CLI:** `paperpilot-cli page-numbers --input multi_page.pdf --position bottom-center --output out/tri_e2e/numbered.pdf --json`
- **MCP:** `{"name": "pdf_page_numbers", "arguments": {"input": "multi_page.pdf", "position": "bottom-center", "output": "out/numbered.pdf"}}`
- **API:** `POST /api/v1/pdf/tools/pdf_page_numbers` with `{"input": "multi_page.pdf", "position": "bottom-center", "output": "out/numbered.pdf"}`

---

### Group 2: Security & Content Operations (Tools 11–20)

#### 11. `pdf_encrypt`
- **CLI:** `paperpilot-cli encrypt --input single_page.pdf --user-password secret123 --owner-password secret123 --output out/tri_e2e/encrypted.pdf --json`
- **MCP:** `{"name": "pdf_encrypt", "arguments": {"input": "single_page.pdf", "password": "secret123", "owner_password": "secret123", "output": "out/encrypted.pdf"}}`
- **API:** `POST /api/v1/pdf/tools/pdf_encrypt` with `{"input": "single_page.pdf", "password": "secret123", "owner_password": "secret123", "output": "out/encrypted.pdf"}`

#### 12. `pdf_decrypt`
- **CLI:** `paperpilot-cli decrypt --input encrypted.pdf --password testpass123 --output out/tri_e2e/decrypted.pdf --json`
- **MCP:** `{"name": "pdf_decrypt", "arguments": {"input": "encrypted.pdf", "password": "testpass123", "output": "out/decrypted.pdf"}}`
- **API:** `POST /api/v1/pdf/tools/pdf_decrypt` with `{"input": "encrypted.pdf", "password": "testpass123", "output": "out/decrypted.pdf"}`

#### 13. `pdf_redact`
- **CLI:** `paperpilot-cli redact --input search_test.pdf --pages 1 --rect 50,50,200,50 --output out/tri_e2e/redacted.pdf --json`
- **MCP:** `{"name": "pdf_redact", "arguments": {"input": "search_test.pdf", "page": 1, "x": 50, "y": 50, "width": 200, "height": 50, "output": "out/redacted.pdf"}}`
- **API:** `POST /api/v1/pdf/tools/pdf_redact` with `{"input": "search_test.pdf", "page": 1, "x": 50, "y": 50, "width": 200, "height": 50, "output": "out/redacted.pdf"}`

#### 14. `pdf_sign`
- **CLI:** `paperpilot-cli signature --input single_page.pdf --cert dummy.p12 --output out/tri_e2e/signed.pdf --json`
- **MCP:** `{"name": "pdf_sign", "arguments": {"input": "single_page.pdf", "cert_path": "dummy.p12", "password": "pass", "reason": "Verified", "output": "out/signed.pdf"}}`
- **API:** `POST /api/v1/pdf/tools/pdf_sign` with `{"input": "single_page.pdf", "cert_path": "dummy.p12", "password": "pass", "reason": "Verified", "output": "out/signed.pdf"}`

#### 15. `pdf_validate`
- **CLI:** `paperpilot-cli validate --input single_page.pdf --json`
- **MCP:** `{"name": "pdf_validate", "arguments": {"input": "single_page.pdf"}}`
- **API:** `POST /api/v1/pdf/tools/pdf_validate` with `{"input": "single_page.pdf"}`

#### 16. `pdf_hash`
- **CLI:** `paperpilot-cli hash --input single_page.pdf --json`
- **MCP:** `{"name": "pdf_hash", "arguments": {"input": "single_page.pdf"}}`
- **API:** `POST /api/v1/pdf/tools/pdf_hash` with `{"input": "single_page.pdf"}`

#### 17. `pdf_repair`
- **CLI:** `paperpilot-cli repair --input single_page.pdf --output out/tri_e2e/repaired.pdf --json`
- **MCP:** `{"name": "pdf_repair", "arguments": {"input": "single_page.pdf", "output": "out/repaired.pdf"}}`
- **API:** `POST /api/v1/pdf/tools/pdf_repair` with `{"input": "single_page.pdf", "output": "out/repaired.pdf"}`

#### 18. `pdf_extract_text`
- **CLI:** `paperpilot-cli extract-text --input search_test.pdf --output out/tri_e2e/extracted_text.txt --json`
- **MCP:** `{"name": "pdf_extract_text", "arguments": {"input": "search_test.pdf", "output": "out/extracted_text.txt"}}`
- **API:** `POST /api/v1/pdf/tools/pdf_extract_text` with `{"input": "search_test.pdf", "output": "out/extracted_text.txt"}`

#### 19. `pdf_extract_images`
- **CLI:** `paperpilot-cli extract-images --input image_doc.pdf --output out/tri_e2e/extracted_images --json`
- **MCP:** `{"name": "pdf_extract_images", "arguments": {"input": "image_doc.pdf", "output_dir": "out/extracted_images.mcp"}}`
- **API:** `POST /api/v1/pdf/tools/pdf_extract_images` with `{"input": "image_doc.pdf", "output_dir": "out/extracted_images.api"}`

#### 20. `pdf_images_to_pdf`
- **CLI:** `paperpilot-cli images-to-pdf --images img1.png --images img2.png --output out/tri_e2e/images_converted.pdf --json`
- **MCP:** `{"name": "pdf_images_to_pdf", "arguments": {"inputs": ["img1.png", "img2.png"], "output": "out/images_converted.pdf"}}`
- **API:** `POST /api/v1/pdf/tools/pdf_images_to_pdf` with `{"inputs": ["img1.png", "img2.png"], "output": "out/images_converted.pdf"}`

---

### Group 3: OCR, Forms & Stamps (Tools 21–30)

#### 21. `pdf_render`
- **CLI:** `paperpilot-cli render --input single_page.pdf --output out/tri_e2e/rendered.png --json`
- **MCP:** `{"name": "pdf_render", "arguments": {"input": "single_page.pdf", "page": 1, "output": "out/rendered.png"}}`
- **API:** `POST /api/v1/pdf/render-page` with `{"input": "single_page.pdf", "page": 1, "output": "out/rendered.png"}`

#### 22. `pdf_ocr`
- **CLI:** `paperpilot-cli ocr --input image_doc.pdf --output out/tri_e2e/ocr_result.pdf --json`
- **MCP:** `{"name": "pdf_ocr", "arguments": {"input": "image_doc.pdf", "output": "out/ocr_result.pdf"}}`
- **API:** `POST /api/v1/pdf/tools/pdf_ocr` with `{"input": "image_doc.pdf", "output": "out/ocr_result.pdf"}`

#### 23. `pdf_search`
- **CLI:** `paperpilot-cli search --input search_test.pdf --query test --json`
- **MCP:** `{"name": "pdf_search", "arguments": {"input": "search_test.pdf", "query": "test"}}`
- **API:** `POST /api/v1/pdf/tools/pdf_search` with `{"input": "search_test.pdf", "query": "test"}`

#### 24. `pdf_bates`
- **CLI:** `paperpilot-cli bates --input multi_page.pdf --prefix CONF- --start 1 --output out/tri_e2e/bates.pdf --json`
- **MCP:** `{"name": "pdf_bates", "arguments": {"input": "multi_page.pdf", "prefix": "CONF-", "start_number": 1, "padding": 6, "output": "out/bates.pdf"}}`
- **API:** `POST /api/v1/pdf/tools/pdf_bates` with `{"input": "multi_page.pdf", "prefix": "CONF-", "start_number": 1, "padding": 6, "output": "out/bates.pdf"}`

#### 25. `pdf_watermark`
- **CLI:** `paperpilot-cli watermark --input single_page.pdf --text SAMPLE --output out/tri_e2e/watermarked.pdf --json`
- **MCP:** `{"name": "pdf_watermark", "arguments": {"input": "single_page.pdf", "text": "SAMPLE", "output": "out/watermarked.pdf"}}`
- **API:** `POST /api/v1/pdf/watermark` with `{"input": "single_page.pdf", "text": "SAMPLE", "output": "out/watermarked.pdf"}`

#### 26. `pdf_header_footer`
- **CLI:** `paperpilot-cli header-footer --input multi_page.pdf --text "Confidential - Page" --output out/tri_e2e/header_footer.pdf --json`
- **MCP:** `{"name": "pdf_header_footer", "arguments": {"input": "multi_page.pdf", "header_left": "Confidential", "footer_center": "Page", "output": "out/header_footer.pdf"}}`
- **API:** `POST /api/v1/pdf/tools/pdf_header_footer` with `{"input": "multi_page.pdf", "header_left": "Confidential", "footer_center": "Page", "output": "out/header_footer.pdf"}`

#### 27. `pdf_read_form`
- **CLI:** `paperpilot-cli form read form.pdf --json`
- **MCP:** `{"name": "pdf_read_form", "arguments": {"input": "form.pdf"}}`
- **API:** `POST /api/v1/pdf/tools/pdf_read_form` with `{"input": "form.pdf"}`

#### 28. `pdf_fill_form`
- **CLI:** `paperpilot-cli form fill form.pdf --data form_data.json --output out/tri_e2e/filled_form.pdf --json`
- **MCP:** `{"name": "pdf_fill_form", "arguments": {"input": "form.pdf", "values": {"TestText": "Alice"}, "output": "out/filled_form.pdf"}}`
- **API:** `POST /api/v1/pdf/tools/pdf_fill_form` with `{"input": "form.pdf", "values": {"TestText": "Alice"}, "output": "out/filled_form.pdf"}`

#### 29. `pdf_create_form_field`
- **CLI:** `paperpilot-cli form add-field single_page.pdf --name signature --type text --rect 50,50,150,30 --output out/tri_e2e/field_added.pdf --json`
- **MCP:** `{"name": "pdf_create_form_field", "arguments": {"input": "single_page.pdf", "field_type": "text", "field_name": "signature", "x": 50, "y": 50, "width": 150, "height": 30, "output": "out/field_added.pdf"}}`
- **API:** `POST /api/v1/pdf/tools/pdf_create_form_field` with `{"input": "single_page.pdf", "field_type": "text", "field_name": "signature", "x": 50, "y": 50, "width": 150, "height": 30, "output": "out/field_added.pdf"}`

#### 30. `pdf_metadata`
- **CLI:** `paperpilot-cli metadata --input single_page.pdf --output out/tri_e2e/metadata.json --json`
- **MCP:** `{"name": "pdf_metadata", "arguments": {"input": "single_page.pdf"}}`
- **API:** `POST /api/v1/pdf/info` with `{"input": "single_page.pdf"}`

---

### Group 4: Conversions, Bookmarks & Advanced Tools (Tools 31–44)

#### 31. `pdf_bookmarks`
- **CLI:** `paperpilot bookmarks --input multi_page.pdf --json`
- **MCP:** `{"name": "pdf_bookmarks", "arguments": {"input": "multi_page.pdf"}}`
- **API:** `POST /api/v1/pdf/tools/pdf_bookmarks` with `{"input": "multi_page.pdf"}`

#### 32. `pdf_compress`
- **CLI:** `paperpilot compress --input large_doc.pdf --quality medium --output out/tri_e2e/compressed.pdf --json`
- **MCP:** `{"name": "pdf_compress", "arguments": {"input": "large_doc.pdf", "quality": "medium", "output": "out/compressed.pdf"}}`
- **API:** `POST /api/v1/pdf/compress` with `{"input": "large_doc.pdf", "quality": "medium", "output": "out/compressed.pdf"}`

#### 33. `pdf_linearize`
- **CLI:** `paperpilot linearize --input single_page.pdf --output out/tri_e2e/linearized.pdf --json`
- **MCP:** `{"name": "pdf_linearize", "arguments": {"input": "single_page.pdf", "output": "out/linearized.pdf"}}`
- **API:** `POST /api/v1/pdf/tools/pdf_linearize` with `{"input": "single_page.pdf", "output": "out/linearized.pdf"}`

#### 34. `pdf_flatten`
- **CLI:** `paperpilot flatten --input form_filled.pdf --output out/tri_e2e/flattened.pdf --json`
- **MCP:** `{"name": "pdf_flatten", "arguments": {"input": "form_filled.pdf", "output": "out/flattened.pdf"}}`
- **API:** `POST /api/v1/pdf/tools/pdf_flatten` with `{"input": "form_filled.pdf", "output": "out/flattened.pdf"}`

#### 35. `pdf_to_docx`
- **CLI:** `paperpilot to-docx --input search_test.pdf --output out/tri_e2e/document.docx --json`
- **MCP:** `{"name": "pdf_to_docx", "arguments": {"input": "search_test.pdf", "output": "out/document.docx"}}`
- **API:** `POST /api/v1/pdf/convert` with `{"input": "search_test.pdf", "output": "out/document.docx", "format": "docx"}`

#### 36. `pdf_to_xlsx`
- **CLI:** `paperpilot to-xlsx --input search_test.pdf --output out/tri_e2e/document.xlsx --json`
- **MCP:** `{"name": "pdf_to_xlsx", "arguments": {"input": "search_test.pdf", "output": "out/document.xlsx"}}`
- **API:** `POST /api/v1/pdf/tools/pdf_to_xlsx` with `{"input": "search_test.pdf", "output": "out/document.xlsx"}`

#### 37. `pdf_to_pptx`
- **CLI:** `paperpilot to-pptx --input single_page.pdf --output out/tri_e2e/presentation.pptx --json`
- **MCP:** `{"name": "pdf_to_pptx", "arguments": {"input": "single_page.pdf", "output": "out/presentation.pptx"}}`
- **API:** `POST /api/v1/pdf/tools/pdf_to_pptx` with `{"input": "single_page.pdf", "output": "out/presentation.pptx"}`

#### 38. `pdf_to_pdf_a`
- **CLI:** `paperpilot to-pdf-a --input single_page.pdf --output out/tri_e2e/archival.pdf --json`
- **MCP:** `{"name": "pdf_to_pdf_a", "arguments": {"input": "single_page.pdf", "output": "out/archival.pdf"}}`
- **API:** `POST /api/v1/pdf/tools/pdf_to_pdf_a` with `{"input": "single_page.pdf", "output": "out/archival.pdf"}`

#### 39. `pdf_classify_type`
- **CLI:** `paperpilot classify --input form.pdf --json`
- **MCP:** `{"name": "pdf_classify_type", "arguments": {"input": "form.pdf"}}`
- **API:** `POST /api/v1/pdf/tools/pdf_classify_type` with `{"input": "form.pdf"}`

#### 40. `pdf_compare`
- **CLI:** `paperpilot compare --file1 page_1.pdf --file2 page_2.pdf --json`
- **MCP:** `{"name": "pdf_compare", "arguments": {"file1": "page_1.pdf", "file2": "page_2.pdf"}}`
- **API:** `POST /api/v1/pdf/compare` with `{"file1": "page_1.pdf", "file2": "page_2.pdf"}`

#### 41. `pdf_annotate`
- **CLI:** `paperpilot annotate --input single_page.pdf --annotations '[{"type":"note","page":1,"x":50,"y":50,"content":"Test"}]' --output out/tri_e2e/annotated.pdf --json`
- **MCP:** `{"name": "pdf_annotate", "arguments": {"input": "single_page.pdf", "annotations": [{"type":"note","page":1,"x":50,"y":50,"content":"Test"}], "output": "out/annotated.pdf"}}`
- **API:** `POST /api/v1/pdf/tools/pdf_annotate` with `{"input": "single_page.pdf", "annotations": [{"type":"note","page":1,"x":50,"y":50,"content":"Test"}], "output": "out/annotated.pdf"}`

#### 42. `pdf_convert_html`
- **CLI:** `paperpilot from-html --input test.html --output out/tri_e2e/html_out.pdf --preset elegant --json`
- **MCP:** `{"name": "pdf_convert_html", "arguments": {"input": "test.html", "output": "out/html_out.pdf", "preset": "elegant"}}`
- **API:** `POST /api/v1/pdf/tools/pdf_convert_html` with `{"input": "test.html", "output": "out/html_out.pdf", "preset": "elegant"}`

#### 43. `pdf_convert_markdown`
- **CLI:** `paperpilot from-markdown --input test.md --output out/tri_e2e/md_out.pdf --preset github --json`
- **MCP:** `{"name": "pdf_convert_markdown", "arguments": {"input": "test.md", "output": "out/md_out.pdf", "preset": "github"}}`
- **API:** `POST /api/v1/pdf/tools/pdf_convert_markdown` with `{"input": "test.md", "output": "out/md_out.pdf", "preset": "github"}`

#### 44. `pdf_convert_excel`
- **CLI:** `paperpilot from-excel --input test.csv --output out/tri_e2e/excel_out.pdf --preset minimal --json`
- **MCP:** `{"name": "pdf_convert_excel", "arguments": {"input": "test.csv", "output": "out/excel_out.pdf", "preset": "minimal"}}`
- **API:** `POST /api/v1/pdf/tools/pdf_convert_excel` with `{"input": "test.csv", "output": "out/excel_out.pdf", "preset": "minimal"}`
