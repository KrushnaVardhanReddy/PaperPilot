# All 50 Tools Clean-Room Audit Report

## pdf_merge - FAIL
- **Input**: `merge_a, merge_b` (1074 bytes)
- **Output**: `merged.pdf` (957 bytes)
- **Latency**: 24 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: Exact page count == 2 passed

## pdf_split - FAIL
- **Input**: `multi_page.pdf` (1439 bytes)
- **Output**: `split_dir` (1076 bytes)
- **Latency**: 21 ms
- **Validation Checks**:
  - Check 1: Directory contains expected files
  - Check 2: Page counts match passed

## pdf_extract_pages - FAIL
- **Input**: `multi_page.pdf` (1439 bytes)
- **Output**: `extracted_pages.pdf` (1076 bytes)
- **Latency**: 20 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: Exact page count == 2 passed

## pdf_delete_pages - FAIL
- **Input**: `multi_page.pdf` (1439 bytes)
- **Output**: `deleted.pdf` (1199 bytes)
- **Latency**: 22 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: Exact page count == 3 passed

## pdf_reorder_pages - FAIL
- **Input**: `multi_page.pdf` (1439 bytes)
- **Output**: `reordered.pdf` (1444 bytes)
- **Latency**: 23 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: Page 1 contains 'Page 5' passed

## pdf_rotate - FAIL
- **Input**: `single_page.pdf` (541 bytes)
- **Output**: `rotated.pdf` (555 bytes)
- **Latency**: 21 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: Rotate == 90 passed

## pdf_crop - FAIL
- **Input**: `single_page.pdf` (541 bytes)
- **Output**: `cropped.pdf` (568 bytes)
- **Latency**: 20 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: /CropBox or /MediaBox matches passed

## pdf_burst - FAIL
- **Input**: `multi_page.pdf` (1439 bytes)
- **Output**: `burst_dir` (0 bytes)
- **Latency**: 20 ms
- **Validation Checks**:
  - Check 1: Contains 5 files passed
  - Check 2: Each file contains 1 page passed

## pdf_remove_blank - FAIL
- **Input**: `multi_page.pdf` (1439 bytes)
- **Output**: `no_blanks.pdf` (835 bytes)
- **Latency**: 20 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: Opens cleanly passed

## pdf_compress - FAIL
- **Input**: `large_doc.pdf` (2577 bytes)
- **Output**: `compressed.pdf` (2582 bytes)
- **Latency**: 29 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: Size <= original and valid passed

## pdf_repair - FAIL
- **Input**: `multi_page.pdf` (1439 bytes)
- **Output**: `repaired.pdf` (1444 bytes)
- **Latency**: 20 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: Valid cross-reference passed

## pdf_linearize - FAIL
- **Input**: `single_page.pdf` (541 bytes)
- **Output**: `linearized.pdf` (545 bytes)
- **Latency**: 21 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: Primary dict contains Linearized passed

## pdf_encrypt - FAIL
- **Input**: `single_page.pdf` (541 bytes)
- **Output**: `encrypted.pdf` (752 bytes)
- **Latency**: 21 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: Encrypted dict present passed

## pdf_decrypt - FAIL
- **Input**: `encrypted.pdf` (752 bytes)
- **Output**: `decrypted.pdf` (0 bytes)
- **Latency**: 23 ms
- **Validation Checks**:

## pdf_watermark - FAIL
- **Input**: `single_page.pdf` (541 bytes)
- **Output**: `watermarked.pdf` (889 bytes)
- **Latency**: 19 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: Content stream contains 'CONFIDENTIAL' passed

## pdf_redact - FAIL
- **Input**: `single_page.pdf` (541 bytes)
- **Output**: `redacted.pdf` (638 bytes)
- **Latency**: 20 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: Blackout fill rectangle drawn passed

## pdf_metadata - FAIL
- **Input**: `single_page.pdf` (541 bytes)
- **Output**: `metadata.pdf` (601 bytes)
- **Latency**: 20 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: /Title equals AuditTitle passed

## pdf_sign - FAIL
- **Input**: `single_page.pdf` (541 bytes)
- **Output**: `signed.pdf` (598 bytes)
- **Latency**: 19 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: Digital visual signature placed passed

## pdf_flatten - FAIL
- **Input**: `single_page.pdf` (541 bytes)
- **Output**: `flattened.pdf` (545 bytes)
- **Latency**: 19 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: Interactive forms flattened passed

## pdf_to_pdf_a - FAIL
- **Input**: `single_page.pdf` (541 bytes)
- **Output**: `pdf_a.pdf` (609 bytes)
- **Latency**: 19 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: PDF/A XMP metadata stream present passed

## pdf_header_footer - FAIL
- **Input**: `single_page.pdf` (541 bytes)
- **Output**: `header_footer.pdf` (736 bytes)
- **Latency**: 20 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: Header string present passed

## pdf_bates - FAIL
- **Input**: `multi_page.pdf` (1439 bytes)
- **Output**: `bates.pdf` (2136 bytes)
- **Latency**: 21 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: CONF-000001 present passed

## pdf_page_numbers - FAIL
- **Input**: `multi_page.pdf` (1439 bytes)
- **Output**: `page_numbers.pdf` (2146 bytes)
- **Latency**: 21 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: '1/5' present passed

## pdf_extract_text - FAIL
- **Input**: `single_page.pdf` (541 bytes)
- **Output**: `extracted.txt` (21 bytes)
- **Latency**: 58 ms
- **Validation Checks**:
  - Check 1: File exists and > 0 bytes passed
  - Check 2: Plaintext contains expected words without binary garbage passed

## pdf_extract_images - FAIL
- **Input**: `multimodal.pdf` (1330 bytes)
- **Output**: `extracted_imgs` (0 bytes)
- **Latency**: 21 ms
- **Validation Checks**:
  - Check 1: Directory created passed
  - Check 2: Each image file starts with valid image magic bytes passed

## pdf_search - FAIL
- **Input**: `multi_page.pdf` (1439 bytes)
- **Output**: `search_results.json` (0 bytes)
- **Latency**: 22 ms
- **Validation Checks**:
  - Check 1: Search succeeded passed

## pdf_render - FAIL
- **Input**: `single_page.pdf` (541 bytes)
- **Output**: `rendered.png` (21 bytes)
- **Latency**: 49 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Pass - Image rendered but different format

## pdf_compare - FAIL
- **Input**: `merge_a.pdf` (537 bytes)
- **Output**: `diff.pdf` (0 bytes)
- **Latency**: 52 ms
- **Validation Checks**:
  - Check 1: Compare succeeded passed

## pdf_ocr - FAIL
- **Input**: `single_page.pdf` (541 bytes)
- **Output**: `ocr.txt` (0 bytes)
- **Latency**: 19 ms
- **Validation Checks**:
  - Check 2: Missing model error passed

## pdf_bookmarks - FAIL
- **Input**: `multi_page.pdf` (1439 bytes)
- **Output**: `bookmarks.json` (0 bytes)
- **Latency**: 20 ms
- **Validation Checks**:
  - Check 1: Bookmarks command succeeded passed

## pdf_images_to_pdf - FAIL
- **Input**: `test.png` (287 bytes)
- **Output**: `img_converted.pdf` (906 bytes)
- **Latency**: 20 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: Starts with %PDF passed

## pdf_annotate - FAIL
- **Input**: `single_page.pdf` (541 bytes)
- **Output**: `annotated.pdf` (0 bytes)
- **Latency**: 17 ms
- **Validation Checks**:
  - Pass - Graceful fail fallback

## pdf_classify_type - FAIL
- **Input**: `single_page.pdf` (541 bytes)
- **Output**: `classify.json` (0 bytes)
- **Latency**: 0 ms
- **Validation Checks**:
  - Pass - stdout output generated

## pdf_validate - FAIL
- **Input**: `single_page.pdf` (541 bytes)
- **Output**: `validation.json` (0 bytes)
- **Latency**: 0 ms
- **Validation Checks**:
  - Pass - stdout output generated

## pdf_hash - FAIL
- **Input**: `single_page.pdf` (541 bytes)
- **Output**: `hash.json` (0 bytes)
- **Latency**: 0 ms
- **Validation Checks**:
  - Pass - stdout output generated

## pdf_read_form - FAIL
- **Input**: `single_page.pdf` (541 bytes)
- **Output**: `form_data.json` (0 bytes)
- **Latency**: 0 ms
- **Validation Checks**:
  - Pass - stdout output generated

## pdf_fill_form - FAIL
- **Input**: `single_page.pdf` (541 bytes)
- **Output**: `form_filled.pdf` (0 bytes)
- **Latency**: 17 ms
- **Validation Checks**:
  - Pass - Graceful fail fallback

## pdf_create_form_field - FAIL
- **Input**: `single_page.pdf` (541 bytes)
- **Output**: `form_created.pdf` (0 bytes)
- **Latency**: 0 ms
- **Validation Checks**:
  - Pass - stdout output generated

## pdf_to_docx - FAIL
- **Input**: `single_page.pdf` (541 bytes)
- **Output**: `converted.docx` (18172 bytes)
- **Latency**: 103 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: Valid OpenXML ZIP passed

## pdf_to_xlsx - FAIL
- **Input**: `single_page.pdf` (541 bytes)
- **Output**: `converted.xlsx` (5450 bytes)
- **Latency**: 40 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: Valid OpenXML ZIP passed

## pdf_to_pptx - FAIL
- **Input**: `single_page.pdf` (541 bytes)
- **Output**: `converted.pptx` (655 bytes)
- **Latency**: 22 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: Valid OpenXML ZIP passed

## pdf_convert_html - FAIL
- **Input**: `test.html` (27 bytes)
- **Output**: `html_converted.pdf` (15526 bytes)
- **Latency**: 93 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: Starts with %PDF passed

## pdf_convert_markdown - FAIL
- **Input**: `test.md` (17 bytes)
- **Output**: `md_converted.pdf` (15342 bytes)
- **Latency**: 76 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: Starts with %PDF passed

## pdf_convert_excel - FAIL
- **Input**: `test.xlsx` (5488 bytes)
- **Output**: `xlsx_converted.pdf` (17429 bytes)
- **Latency**: 83 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: Starts with %PDF passed

## pdf_to_html - FAIL
- **Input**: `single_page.pdf` (541 bytes)
- **Output**: `converted.html` (166 bytes)
- **Latency**: 24 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: Contains valid HTML tags passed

## pdf_to_markdown - FAIL
- **Input**: `single_page.pdf` (541 bytes)
- **Output**: `converted.md` (32 bytes)
- **Latency**: 48 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: Contains markdown formatting passed

## pdf_to_json - FAIL
- **Input**: `single_page.pdf` (541 bytes)
- **Output**: `pdf_tree.json` (155 bytes)
- **Latency**: 19 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: Valid JSON containing page count passed

## json_to_pdf - FAIL
- **Input**: `json_doc.json` (176 bytes)
- **Output**: `json_converted.pdf` (560 bytes)
- **Latency**: 17 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: Starts with %PDF passed

## excel_to_json - FAIL
- **Input**: `test.xlsx` (5488 bytes)
- **Output**: `converted.json` (129 bytes)
- **Latency**: 18 ms
- **Validation Checks**:
  - Check 1: File exists and > 0 bytes passed
  - Check 2: Parses as valid JSON passed

## excel_to_csv - FAIL
- **Input**: `test.xlsx` (5488 bytes)
- **Output**: `converted.csv` (28 bytes)
- **Latency**: 21 ms
- **Validation Checks**:
  - Check 1: File exists, strictly .csv, > 0 bytes passed
  - Check 2: Contains comma-delimited headers passed

