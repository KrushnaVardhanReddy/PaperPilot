# UI Chat 44 Operations E2E Scorecard

## Executive Summary
- **Total Tested:** 44
- **Passed:** 44
- **Failed:** 0

| Tool Name | Intent | Natural Language Prompt | Status | Latency (ms) | Notes |
|-----------|--------|-------------------------|--------|--------------|-------|
| `pdf_merge` | Merge | "merge" | ✅ PASS | 293.12 |  |
| `pdf_split` | Split | "split" | ✅ PASS | 184.38 |  |
| `pdf_extract_pages` | Extract | "extract pages" | ✅ PASS | 3072.92 |  |
| `pdf_delete_pages` | Delete | "delete pages" | ✅ PASS | 134.32 |  |
| `pdf_reorder_pages` | Reorder | "reorder pages" | ✅ PASS | 3064.88 |  |
| `pdf_rotate` | Rotate | "rotate" | ✅ PASS | 134.93 |  |
| `pdf_crop` | Crop | "crop" | ✅ PASS | 3066.02 |  |
| `pdf_burst` | Burst | "burst" | ✅ PASS | 139.38 |  |
| `pdf_remove_blank` | RemoveBlank | "remove blank" | ✅ PASS | 3062.06 |  |
| `pdf_compress` | Compress | "compress" | ✅ PASS | 134.04 |  |
| `pdf_repair` | Repair | "repair" | ✅ PASS | 164.76 |  |
| `pdf_linearize` | Linearize | "linearize" | ✅ PASS | 3069.72 |  |
| `pdf_encrypt` | Encrypt | "encrypt" | ✅ PASS | 135.45 |  |
| `pdf_decrypt` | Decrypt | "remove password" | ✅ PASS | 3077.87 |  |
| `pdf_watermark` | Watermark | "watermark" | ✅ PASS | 137.65 |  |
| `pdf_redact` | Redact | "redact" | ✅ PASS | 3082.86 |  |
| `pdf_metadata` | Metadata | "metadata" | ✅ PASS | 134.61 |  |
| `pdf_sign` | Sign | "sign" | ✅ PASS | 167.44 |  |
| `pdf_flatten` | Flatten | "flatten" | ✅ PASS | 165.65 |  |
| `pdf_to_pdf_a` | PdfA | "convert to pdf/a" | ✅ PASS | 178.98 |  |
| `pdf_header_footer` | HeaderFooter | "header/footer" | ✅ PASS | 3088.34 |  |
| `pdf_bates` | Bates | "bates" | ✅ PASS | 149.54 |  |
| `pdf_page_numbers` | PageNumbers | "page numbers" | ✅ PASS | 3078.46 |  |
| `pdf_extract_text` | ExtractText | "extract text" | ✅ PASS | 138.97 |  |
| `pdf_extract_images` | ExtractImages | "embedded images" | ✅ PASS | 3076.67 |  |
| `pdf_search` | Search | "search" | ✅ PASS | 134.81 |  |
| `pdf_render` | Render | "render" | ✅ PASS | 3068.40 |  |
| `pdf_compare` | Compare | "compare" | ✅ PASS | 135.69 |  |
| `pdf_ocr` | Ocr | "ocr" | ✅ PASS | 3077.67 |  |
| `pdf_bookmarks` | Bookmarks | "bookmarks" | ✅ PASS | 122.68 |  |
| `pdf_images_to_pdf` | ImagesToPdf | "images to pdf" | ✅ PASS | 3082.00 |  |
| `pdf_annotate` | Annotate | "annotate" | ✅ PASS | 134.34 |  |
| `pdf_classify_type` | Classify | "classify pdf" | ✅ PASS | 3080.88 |  |
| `pdf_validate` | Validate | "validate" | ✅ PASS | 136.08 |  |
| `pdf_hash` | Hash | "hash" | ✅ PASS | 3081.40 |  |
| `pdf_read_form` | FormRead | "read form" | ✅ PASS | 135.10 |  |
| `pdf_fill_form` | FormFill | "fill form" | ✅ PASS | 3108.44 |  |
| `pdf_create_form_field` | FormCreate | "add form field" | ✅ PASS | 218.37 |  |
| `pdf_to_docx` | ToDocx | "docx" | ✅ PASS | 3138.14 |  |
| `pdf_to_xlsx` | ToXlsx | "xlsx" | ✅ PASS | 150.84 |  |
| `pdf_to_pptx` | ToPptx | "pptx" | ✅ PASS | 3066.40 |  |
| `pdf_convert_html` | ToHtml | "html" | ✅ PASS | 150.07 |  |
| `pdf_convert_markdown` | ToMarkdown | "markdown" | ✅ PASS | 3080.59 |  |
| `pdf_convert_excel` | ExcelToPdf | "excel" | ✅ PASS | 131.00 |  |
