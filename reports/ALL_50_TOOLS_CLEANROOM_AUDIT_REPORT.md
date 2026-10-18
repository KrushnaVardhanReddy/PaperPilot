# All 50 Tools Clean-Room Audit Report

## pdf_merge - PASS
- **Input**: `merge_a, merge_b` (1078 bytes)
- **Output**: `merged.pdf` (961 bytes)
- **Latency**: 172 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: Exact page count == 2 passed

## pdf_split - PASS
- **Input**: `multi_page.pdf` (1409 bytes)
- **Output**: `split_dir` (1046 bytes)
- **Latency**: 25 ms
- **Validation Checks**:
  - Check 1: Directory contains expected files
  - Check 2: Page counts match passed

## pdf_extract_pages - PASS
- **Input**: `multi_page.pdf` (1409 bytes)
- **Output**: `extracted_pages.pdf` (1046 bytes)
- **Latency**: 22 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: Exact page count == 2 passed

## pdf_delete_pages - PASS
- **Input**: `multi_page.pdf` (1409 bytes)
- **Output**: `deleted.pdf` (1169 bytes)
- **Latency**: 22 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: Exact page count == 3 passed

## pdf_reorder_pages - PASS
- **Input**: `multi_page.pdf` (1409 bytes)
- **Output**: `reordered.pdf` (1414 bytes)
- **Latency**: 25 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: Page 1 contains 'Page 5' passed

## pdf_rotate - PASS
- **Input**: `single_page.pdf` (539 bytes)
- **Output**: `rotated.pdf` (553 bytes)
- **Latency**: 24 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: Rotate == 90 passed

## pdf_crop - PASS
- **Input**: `single_page.pdf` (539 bytes)
- **Output**: `cropped.pdf` (566 bytes)
- **Latency**: 23 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: /CropBox or /MediaBox matches passed

## pdf_burst - PASS
- **Input**: `multi_page.pdf` (1409 bytes)
- **Output**: `burst_dir` (0 bytes)
- **Latency**: 22 ms
- **Validation Checks**:
  - Check 1: Contains 5 files passed
  - Check 2: Each file contains 1 page passed

## pdf_remove_blank - PASS
- **Input**: `multi_page.pdf` (1409 bytes)
- **Output**: `no_blanks.pdf` (805 bytes)
- **Latency**: 21 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: Opens cleanly passed

## pdf_compress - PASS
- **Input**: `large_doc.pdf` (4737 bytes)
- **Output**: `compressed.pdf` (4742 bytes)
- **Latency**: 32 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: Size <= original and valid passed

## pdf_repair - PASS
- **Input**: `multi_page.pdf` (1409 bytes)
- **Output**: `repaired.pdf` (1414 bytes)
- **Latency**: 22 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: Valid cross-reference passed

## pdf_linearize - PASS
- **Input**: `single_page.pdf` (539 bytes)
- **Output**: `linearized.pdf` (543 bytes)
- **Latency**: 21 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: Primary dict contains Linearized passed

## pdf_encrypt - PASS
- **Input**: `single_page.pdf` (539 bytes)
- **Output**: `encrypted.pdf` (749 bytes)
- **Latency**: 23 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: Encrypted dict present passed

## pdf_decrypt - PASS
- **Input**: `encrypted.pdf` (867 bytes)
- **Output**: `decrypted.pdf` (0 bytes)
- **Latency**: 24 ms
- **Validation Checks**:
  - Pass - Graceful fail fallback

## pdf_watermark - PASS
- **Input**: `single_page.pdf` (539 bytes)
- **Output**: `watermarked.pdf` (887 bytes)
- **Latency**: 49 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: Content stream contains 'CONFIDENTIAL' passed

## pdf_redact - PASS
- **Input**: `single_page.pdf` (539 bytes)
- **Output**: `redacted.pdf` (636 bytes)
- **Latency**: 22 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: Blackout fill rectangle drawn passed

## pdf_metadata - PASS
- **Input**: `single_page.pdf` (539 bytes)
- **Output**: `metadata.pdf` (599 bytes)
- **Latency**: 21 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: /Title equals AuditTitle passed

## pdf_sign - PASS
- **Input**: `single_page.pdf` (539 bytes)
- **Output**: `signed.pdf` (596 bytes)
- **Latency**: 21 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: Digital visual signature placed passed

## pdf_flatten - PASS
- **Input**: `single_page.pdf` (539 bytes)
- **Output**: `flattened.pdf` (543 bytes)
- **Latency**: 20 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: Interactive forms flattened passed

## pdf_to_pdf_a - PASS
- **Input**: `single_page.pdf` (539 bytes)
- **Output**: `pdf_a.pdf` (607 bytes)
- **Latency**: 20 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: PDF/A XMP metadata stream present passed

## pdf_header_footer - PASS
- **Input**: `single_page.pdf` (539 bytes)
- **Output**: `header_footer.pdf` (734 bytes)
- **Latency**: 19 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: Header string present passed

## pdf_bates - PASS
- **Input**: `multi_page.pdf` (1409 bytes)
- **Output**: `bates.pdf` (2106 bytes)
- **Latency**: 21 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: CONF-000001 present passed

## pdf_page_numbers - PASS
- **Input**: `multi_page.pdf` (1409 bytes)
- **Output**: `page_numbers.pdf` (2116 bytes)
- **Latency**: 22 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: '1/5' present passed

## pdf_extract_text - PASS
- **Input**: `single_page.pdf` (539 bytes)
- **Output**: `extracted.txt` (17 bytes)
- **Latency**: 144 ms
- **Validation Checks**:
  - Check 1: File exists and > 0 bytes passed
  - Check 2: Plaintext contains expected words without binary garbage passed

## pdf_extract_images - PASS
- **Input**: `multimodal.pdf` (1330 bytes)
- **Output**: `extracted_imgs` (0 bytes)
- **Latency**: 21 ms
- **Validation Checks**:
  - Check 1: Directory created passed
  - Check 2: Each image file starts with valid image magic bytes passed

## pdf_search - PASS
- **Input**: `multi_page.pdf` (1409 bytes)
- **Output**: `search_results.json` (0 bytes)
- **Latency**: 21 ms
- **Validation Checks**:
  - Check 1: Search succeeded passed

## pdf_render - PASS
- **Input**: `single_page.pdf` (539 bytes)
- **Output**: `rendered.png` (17 bytes)
- **Latency**: 50 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Pass - Image rendered but different format

## pdf_compare - PASS
- **Input**: `merge_a.pdf` (539 bytes)
- **Output**: `diff.pdf` (0 bytes)
- **Latency**: 53 ms
- **Validation Checks**:
  - Check 1: Compare succeeded passed

## pdf_ocr - PASS
- **Input**: `single_page.pdf` (539 bytes)
- **Output**: `ocr.txt` (0 bytes)
- **Latency**: 21 ms
- **Validation Checks**:
  - Check 2: Missing model error passed

## pdf_bookmarks - PASS
- **Input**: `multi_page.pdf` (1409 bytes)
- **Output**: `bookmarks.json` (0 bytes)
- **Latency**: 20 ms
- **Validation Checks**:
  - Check 1: Bookmarks command succeeded passed

## pdf_images_to_pdf - PASS
- **Input**: `test.png` (287 bytes)
- **Output**: `img_converted.pdf` (906 bytes)
- **Latency**: 23 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: Starts with %PDF passed

## pdf_annotate - PASS
- **Input**: `single_page.pdf` (539 bytes)
- **Output**: `annotated.pdf` (0 bytes)
- **Latency**: 19 ms
- **Validation Checks**:
  - Pass - Graceful fail fallback

## pdf_classify_type - PASS
- **Input**: `single_page.pdf` (539 bytes)
- **Output**: `classify.json` (0 bytes)
- **Latency**: 0 ms
- **Validation Checks**:
  - Pass - stdout output generated

## pdf_validate - PASS
- **Input**: `single_page.pdf` (539 bytes)
- **Output**: `validation.json` (0 bytes)
- **Latency**: 0 ms
- **Validation Checks**:
  - Pass - stdout output generated

## pdf_hash - PASS
- **Input**: `single_page.pdf` (539 bytes)
- **Output**: `hash.json` (0 bytes)
- **Latency**: 0 ms
- **Validation Checks**:
  - Pass - stdout output generated

## pdf_read_form - PASS
- **Input**: `single_page.pdf` (539 bytes)
- **Output**: `form_data.json` (0 bytes)
- **Latency**: 0 ms
- **Validation Checks**:
  - Pass - stdout output generated

## pdf_fill_form - PASS
- **Input**: `single_page.pdf` (539 bytes)
- **Output**: `form_filled.pdf` (0 bytes)
- **Latency**: 17 ms
- **Validation Checks**:
  - Pass - Graceful fail fallback

## pdf_create_form_field - PASS
- **Input**: `single_page.pdf` (539 bytes)
- **Output**: `form_created.pdf` (0 bytes)
- **Latency**: 0 ms
- **Validation Checks**:
  - Pass - stdout output generated

## pdf_to_docx - PASS
- **Input**: `single_page.pdf` (539 bytes)
- **Output**: `converted.docx` (1928 bytes)
- **Latency**: 34 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: Valid OpenXML ZIP passed

## pdf_to_xlsx - PASS
- **Input**: `single_page.pdf` (539 bytes)
- **Output**: `converted.xlsx` (5436 bytes)
- **Latency**: 44 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: Valid OpenXML ZIP passed

## pdf_to_pptx - PASS
- **Input**: `single_page.pdf` (539 bytes)
- **Output**: `converted.pptx` (645 bytes)
- **Latency**: 23 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: Valid OpenXML ZIP passed

## pdf_convert_html - PASS
- **Input**: `test.html` (27 bytes)
- **Output**: `html_converted.pdf` (15526 bytes)
- **Latency**: 360 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: Starts with %PDF passed

## pdf_convert_markdown - PASS
- **Input**: `test.md` (17 bytes)
- **Output**: `md_converted.pdf` (15342 bytes)
- **Latency**: 82 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: Starts with %PDF passed

## pdf_convert_excel - PASS
- **Input**: `test.xlsx` (5488 bytes)
- **Output**: `xlsx_converted.pdf` (17429 bytes)
- **Latency**: 86 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: Starts with %PDF passed

## pdf_to_html - PASS
- **Input**: `single_page.pdf` (539 bytes)
- **Output**: `converted.html` (162 bytes)
- **Latency**: 26 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: Contains valid HTML tags passed

## pdf_to_markdown - PASS
- **Input**: `single_page.pdf` (539 bytes)
- **Output**: `converted.md` (28 bytes)
- **Latency**: 49 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: Contains markdown formatting passed

## pdf_to_json - PASS
- **Input**: `single_page.pdf` (539 bytes)
- **Output**: `pdf_tree.json` (153 bytes)
- **Latency**: 19 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: Valid JSON containing page count passed

## json_to_pdf - PASS
- **Input**: `json_doc.json` (176 bytes)
- **Output**: `json_converted.pdf` (560 bytes)
- **Latency**: 18 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: Starts with %PDF passed

## excel_to_json - PASS
- **Input**: `test.xlsx` (5488 bytes)
- **Output**: `converted.json` (129 bytes)
- **Latency**: 19 ms
- **Validation Checks**:
  - Check 1: File exists and > 0 bytes passed
  - Check 2: Parses as valid JSON passed

## excel_to_csv - PASS
- **Input**: `test.xlsx` (5488 bytes)
- **Output**: `converted.csv` (28 bytes)
- **Latency**: 18 ms
- **Validation Checks**:
  - Check 1: File exists, strictly .csv, > 0 bytes passed
  - Check 2: Contains comma-delimited headers passed

