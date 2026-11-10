# Top 12 Core Launch Tools Audit Report

## pdf_merge - PASS
- **Input**: `merge_a.pdf, merge_b.pdf` (59166 bytes)
- **Output**: `merged.pdf` (58183 bytes)
- **Latency**: 30 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: Exact page count == 2 passed

## pdf_split - PASS
- **Input**: `multi_page.pdf` (1675 bytes)
- **Output**: `split_dir/` (1307 bytes)
- **Latency**: 25 ms
- **Validation Checks**:
  - Check 1: Directory contains split_1.pdf, split_2.pdf, split_3.pdf
  - Check 2: split_1 (2p), split_2 (2p), split_3 (1p) passed

## pdf_rotate - PASS
- **Input**: `single_page.pdf` (29583 bytes)
- **Output**: `rotated.pdf` (29173 bytes)
- **Latency**: 26 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: Rotate attribute == 90 passed

## pdf_delete_pages - PASS
- **Input**: `multi_page.pdf` (1675 bytes)
- **Output**: `deleted.pdf` (1230 bytes)
- **Latency**: 24 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: Exact page count == 3 passed

## pdf_reorder_pages - FAIL
- **Input**: `multi_page.pdf` (1675 bytes)
- **Output**: `reordered.pdf` (1680 bytes)
- **Latency**: 25 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: Page 1 text did not contain 'Page 5', got 'PAGE_5
'

## pdf_compress - PASS
- **Input**: `large_doc.pdf` (4737 bytes)
- **Output**: `compressed.pdf` (4742 bytes)
- **Latency**: 40 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: Output file size <= original size and opens without syntax corruption passed

## pdf_watermark - PASS
- **Input**: `single_page.pdf` (29583 bytes)
- **Output**: `watermarked.pdf` (29566 bytes)
- **Latency**: 27 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: Content stream contains 'CONFIDENTIAL' passed

## pdf_ocr - FAIL
- **Input**: `single_page.pdf` (29583 bytes)
- **Output**: `ocr.txt` (0 bytes)
- **Latency**: 166 ms
- **Validation Checks**:
  - Check 1: Output text is 0 bytes

## pdf_extract_text - PASS
- **Input**: `single_page.pdf` (29583 bytes)
- **Output**: `extracted.txt` (1506 bytes)
- **Latency**: 111 ms
- **Validation Checks**:
  - Check 1: File exists and > 0 bytes passed
  - Check 2: Contains expected text without binary garbage passed

## pdf_to_docx - PASS
- **Input**: `single_page.pdf` (29583 bytes)
- **Output**: `converted.docx` (29367 bytes)
- **Latency**: 122 ms
- **Validation Checks**:
  - Check 1: File > 0 bytes passed
  - Check 2: Valid OpenXML ZIP archive (PK\x03\x04) passed

## excel_to_json - PASS
- **Input**: `test.xlsx` (19188 bytes)
- **Output**: `converted.json` (1522 bytes)
- **Latency**: 24 ms
- **Validation Checks**:
  - Check 1: File exists and > 0 bytes passed
  - Check 2: Parses as valid JSON containing rows passed

## excel_to_csv - PASS
- **Input**: `test.xlsx` (19188 bytes)
- **Output**: `converted.csv` (392 bytes)
- **Latency**: 24 ms
- **Validation Checks**:
  - Check 1: File exists, extension strictly .csv, > 0 bytes passed
  - Check 2: Contains comma-delimited headers and numerical rows passed

