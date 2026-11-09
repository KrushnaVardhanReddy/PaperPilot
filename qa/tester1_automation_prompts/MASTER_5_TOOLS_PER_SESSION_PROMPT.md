# Master Instructions: 5 Tools Per Session Protocol for Gemini-CLI

## Context & Objective
We are aligning all 30 tools assigned to **Tester 1** in `qa/manual_desktop_testing/TESTER_1_PAGE_OPERATIONS.md`.

---

## ⭐️ Phase 0: Prerequisite Fixture Generation (Run FIRST before any session)

> **Why this comes first:** Generating and standardizing all ground-truth fixtures upfront eliminates duplicate compilation delays and ensures every subsequent session has deterministic files ready to use immediately.

### Dedicated Session 0: Generate All Standardized Fixtures (`tests/e2e_fixtures/real/`)
Generate or verify the following deterministic test fixtures in `tests/e2e_fixtures/real/` and copy them to `~/Desktop/tester1_fixtures/`:
1. `merge_a.pdf`: 2 pages containing explicit header/body text `MERGE_A_PAGE_1` and `MERGE_A_PAGE_2`.
2. `merge_b.pdf`: 2 pages containing explicit header/body text `MERGE_B_PAGE_1` and `MERGE_B_PAGE_2`.
3. `multi_page.pdf`: 5 pages (`PAGE_1` through `PAGE_5`), with standard PDF outline TOC bookmarks (`Chapter 1`, `Chapter 2`).
4. `single_page.pdf`: Standard Letter format (612x792 pt) with clean text `SINGLE_PAGE_CONTENT`.
5. `with_blank_pages.pdf`: 4 pages total — pages 1 & 3 contain content, pages 2 & 4 are pure white blank.
6. `rotated_landscape.pdf`: 1 landscape page with sideways rotation for auto-upright verification.
7. `two_up_book.pdf`: 1 wide landscape sheet with 2 side-by-side pages for book spine splitting.
8. `sample_photo_doc.pdf`: Multi-page document embedding high-resolution raster images (~3.8 MB) for compression testing.
9. `large_doc.pdf`: Document with duplicate embedded image streams for stream deduplication testing.
10. `malformed.pdf`: Document with deliberately corrupt cross-reference trailer table for syntax repair testing.
11. `img1.png` & `img2.png`: Two valid raster PNG images for image-to-PDF conversion tests.
12. `data.csv`: A sample CSV file for attachment and portfolio testing.

---

## 6 Tool-Execution Sessions (5 Tools Per Session)

### Session 1: Tools 01–05 (Document Assembly & Splitting)
- **Tool 01 (`pdf_merge`)**:
  - *Input*: `merge_a.pdf` (2 pages) + `merge_b.pdf` (2 pages).
  - *Execution*: Combine tabs / merge documents.
  - *Output Validation*: Assert output has 4 pages; page 1 has `MERGE_A`, page 3 has `MERGE_B`.
- **Tool 02 (`pdf_merge_alternate`)**:
  - *Bug Fix*: Add Doc A & Doc B selector dropdowns + "Reverse Doc B" checkbox in `OperationsPanel.svelte`; wire `input_a`, `input_b`, `reverse_b` in `operationArgsBuilder.ts`.
  - *Input*: `merge_a.pdf` (Doc A) + `merge_b.pdf` (Doc B, reversed).
  - *Output Validation*: Assert 4 pages with interleaved sequence [A1, B2, A2, B1].
- **Tool 03 (`pdf_split`)**:
  - *Input*: `multi_page.pdf` (5 pages).
  - *Execution*: Range `1-2, 3-5`.
  - *Output Validation*: Assert `split_1.pdf` has 2 pages (`PAGE_1`, `PAGE_2`), `split_2.pdf` has 3 pages (`PAGE_3`, `PAGE_4`, `PAGE_5`).
- **Tool 04 (`pdf_split_by_outline`)**:
  - *Input*: `multi_page.pdf` with outline TOC bookmarks (Chapters 1 & 2).
  - *Execution*: Outline level `1`.
  - *Output Validation*: Assert output ZIP / chapter PDFs match TOC sections.
- **Tool 05 (`pdf_split_scanned_book`)**:
  - *Input*: `two_up_book.pdf` (2 side-by-side pages on a single landscape sheet).
  - *Execution*: Split spine vertically down 50% width.
  - *Output Validation*: Assert output has 2 separate portrait pages (left page and right page).

---

### Session 2: Tools 06–10 (Page Extraction, Cropping & Reordering)
- **Tool 06 (`pdf_burst`)**:
  - *Input*: `multi_page.pdf` (5 pages).
  - *Execution*: Burst into single pages.
  - *Output Validation*: Assert 5 distinct 1-page PDFs (`page_1.pdf` to `page_5.pdf`) created.
- **Tool 07 (`pdf_crop`)**:
  - *Input*: `single_page.pdf` (Standard Letter 612x792 pt).
  - *Execution*: Crop box `50, 50, 400, 400`.
  - *Output Validation*: Assert output MediaBox dimensions match cropped boundary.
- **Tool 08 (`pdf_delete_pages`)**:
  - *Input*: `multi_page.pdf` (5 pages).
  - *Execution*: Pages `2, 4`.
  - *Output Validation*: Assert exactly 3 pages remain; pages contain `PAGE_1`, `PAGE_3`, `PAGE_5`.
- **Tool 09 (`pdf_extract_pages`)**:
  - *Input*: `multi_page.pdf` (5 pages).
  - *Execution*: Range `2-3`.
  - *Output Validation*: Assert new document has 2 pages (`PAGE_2`, `PAGE_3`). Original unchanged.
- **Tool 10 (`pdf_reorder_pages`)**:
  - *Input*: `multi_page.pdf` (5 pages).
  - *Execution*: Order `4,1,2,3,5`.
  - *Output Validation*: Assert page 1 contains `PAGE_4` and page 2 contains `PAGE_1`.

---

### Session 3: Tools 11–15 (Rotation, Blanks, Scaling & N-Up)
- **Tool 11 (`pdf_rotate`)**:
  - *Input*: `single_page.pdf`.
  - *Execution*: Angle `90` degrees clockwise.
  - *Output Validation*: Assert `/Rotate 90` in dictionary; width and height dimensions swapped.
- **Tool 12 (`pdf_auto_rotate`)**:
  - *Input*: `rotated_landscape.pdf` (sideways text).
  - *Execution*: Auto upright orientation.
  - *Output Validation*: Assert upright orientation detected and applied.
- **Tool 13 (`pdf_remove_blank`)**:
  - *Input*: `with_blank_pages.pdf` (4 pages: 2 with content, 2 pure blank).
  - *Execution*: Sensitivity 95%.
  - *Output Validation*: Assert exactly 2 blank pages removed; output has 2 content pages.
- **Tool 14 (`pdf_scale_pages`)**:
  - *Input*: `single_page.pdf` (Letter 612x792 pt).
  - *Execution*: Target format `A4` (595x842 pt).
  - *Output Validation*: Assert output MediaBox is scaled to 595x842.
- **Tool 15 (`pdf_nup`)**:
  - *Input*: `multi_page.pdf` (4 pages).
  - *Execution*: 2x2 grid (4-Up on a single sheet).
  - *Output Validation*: Assert 4 pages combined onto 1 single sheet.

---

### Session 4: Tools 16–20 (Booklets, Posters, Bookmarks & Attachments)
- **Tool 16 (`pdf_booklet`)**:
  - *Input*: `multi_page.pdf` (8 pages).
  - *Execution*: Saddle-stitch 2-up booklet.
  - *Output Validation*: Assert sheet 1 has pages [8, 1], sheet 2 has [2, 7], etc.
- **Tool 17 (`pdf_posterize`)**:
  - *Input*: `single_page.pdf` (1 large image/map).
  - *Execution*: Sliced into 2x2 grid (4 printable sheets).
  - *Output Validation*: Assert 4 tiled output pages generated.
- **Tool 18 (`pdf_stitch_canvas`)**:
  - *Input*: `multi_page.pdf` (3 pages).
  - *Execution*: Vertical continuous banner stitching.
  - *Output Validation*: Assert single tall page created with height = sum of page heights.
- **Tool 19 (`pdf_bookmarks`)**:
  - *Input*: `multi_page.pdf` with outline bookmarks.
  - *Execution*: Inspect/extract bookmark JSON tree.
  - *Output Validation*: Assert JSON hierarchy matches document outline titles.
- **Tool 20 (`pdf_attachments`)**:
  - *Input*: `single_page.pdf`.
  - *Execution*: Embed test file `data.csv` into PDF portfolio.
  - *Output Validation*: Assert `/EmbeddedFiles` dictionary present with `data.csv`.

---

### Session 5: Tools 21–25 (Image Conversions, Compression & Linearization)
- **Tool 21 (`pdf_images_to_pdf`)**:
  - *Input*: `img1.png` + `img2.png`.
  - *Execution*: Combine into multi-page PDF.
  - *Output Validation*: Assert 2-page PDF created; each page embeds an image XObject.
- **Tool 22 (`img_to_pdf`)**:
  - *Input*: `sample_photo.png`.
  - *Execution*: Convert single image to PDF.
  - *Output Validation*: Assert 1-page PDF created matching image aspect ratio.
- **Tool 23 (`pdf_compress`)**:
  - *Input*: `sample_photo_doc.pdf` (~3.8 MB).
  - *Execution*: Quality `50%`.
  - *Output Validation*: Assert output file size is reduced by $\ge 30\%$, visual fidelity intact.
- **Tool 24 (`pdf_dedup_streams`)**:
  - *Input*: `large_doc.pdf` (contains duplicate image assets).
  - *Execution*: Deduplicate streams.
  - *Output Validation*: Assert unreferenced duplicate streams merged; size reduced.
- **Tool 25 (`pdf_linearize`)**:
  - *Input*: `multi_page.pdf`.
  - *Execution*: Fast web view linearization.
  - *Output Validation*: Assert `/Linearized` dictionary present at the start of the file.

---

### Session 6: Tools 26–30 (Repair, Dark Mode, Render, Diff & Encryption)
- **Tool 26 (`pdf_repair`)**:
  - *Input*: `malformed.pdf` (corrupted cross-reference / trailer).
  - *Execution*: Rebuild PDF syntax.
  - *Output Validation*: Assert output successfully opens and parses without syntax errors.
- **Tool 27 (`pdf_invert_colors`)**:
  - *Input*: `single_page.pdf`.
  - *Execution*: Dark mode inversion.
  - *Output Validation*: Assert color transformation stream applied.
- **Tool 28 (`pdf_render`)**:
  - *Input*: `single_page.pdf`.
  - *Execution*: Render page 1 at 150 DPI to PNG.
  - *Output Validation*: Assert valid PNG file output with non-zero dimensions.
- **Tool 29 (`pdf_compare`)**:
  - *Input*: `merge_a.pdf` and `merge_b.pdf`.
  - *Execution*: Visual pixel diff comparison.
  - *Output Validation*: Assert diff metrics JSON output with mismatch percentage.
- **Tool 30 (`pdf_encrypt`)**:
  - *Input*: `single_page.pdf`.
  - *Execution*: Password `Secret123` with AES-256.
  - *Output Validation*: Assert `/Encrypt` dictionary present; loading without password fails.
