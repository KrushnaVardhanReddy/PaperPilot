# Phase 5.9.72 — Wave 6 (16 Tools) Penta-Interface E2E Testing

## Objective

This document outlines the testing architecture for Wave 6 tools across all 5 supported interfaces (Penta-Interface Spec). The goal is to assert both real-world integration latency and output correctness (size, structure, and magic bytes).

## The 16 Wave 6 Tools Covered

1. **`pdf_merge` (Merge PDFs):** Combine multiple PDF files into one.
2. **`pdf_merge_alternate` (Alternating PDF Merge):** Interleave pages from two PDF documents.
3. **`pdf_split` (Split PDF):** Divide document into separate pages.
4. **`pdf_split_by_outline` (Split PDF by Outline):** Split multi-chapter PDF documents based on their /Outlines bookmark hierarchy.
5. **`pdf_split_scanned_book` (Split Scanned Book Spine):** Detect center binding gutter spine and split two-up page scans into single pages.
6. **`pdf_rotate` (Rotate Pages):** Rotate pages by 90, 180, or 270 degrees.
7. **`pdf_auto_rotate` (Auto Upright Orientation):** Heuristically detect text baseline and rotation angle to automatically upright pages.
8. **`pdf_crop` (Crop Pages):** Crop pages to rectangular area.
9. **`pdf_scale_pages` (Scale Page Dimensions):** Scale page dimensions between standard formats or percentages.
10. **`pdf_delete_pages` (Delete Pages):** Remove specific pages from document.
11. **`pdf_reorder_pages` (Reorder Pages):** Rearrange the sequence of pages.
12. **`pdf_extract_pages` (Extract Pages):** Save specific pages as a new document.
13. **`pdf_nup` (N-Up Page Imposition):** Combine multiple pages per sheet in a 2, 4, 9, or 16-up grid layout.
14. **`pdf_booklet` (Booklet Imposition):** Reorder and impose pages for 2-sided folding booklet printing.
15. **`pdf_posterize` (Multi-Sheet Poster Tile Slicer):** Tile oversized PDF pages across a grid of standard printable sheets.
16. **`pdf_stitch_canvas` (Stitch Continuous Canvas):** Combine multiple pages vertically or horizontally into a continuous single-page canvas.

## Running the E2E Suite

### Rust (CLI, MCP, API)
```bash
cargo test --test test_wave6_penta_interface -- --nocapture
```
*Outputs auto-generated scorecard to `reports/WAVE_6_PAGE_OPS_PENTA_SCORECARD.md`.*

### Example CLI Commands

```bash
# 1. pdf_merge
paperpilot merge --input file1.pdf file2.pdf --output out.pdf

# 2. pdf_merge_alternate
paperpilot pdf-merge-alternate --input-a a.pdf --input-b b.pdf --output out.pdf

# 3. pdf_split
paperpilot split --input in.pdf --every 1 --output out_dir/

# 4. pdf_split_by_outline
paperpilot pdf-split-by-outline --input in.pdf --output out.zip

# 5. pdf_split_scanned_book
paperpilot pdf-split-scanned-book --input in.pdf --output out.pdf

# 6. pdf_rotate
paperpilot rotate --input in.pdf --angle 90 --output out.pdf

# 7. pdf_auto_rotate
paperpilot pdf-auto-rotate --input in.pdf --output out.pdf

# 8. pdf_crop
paperpilot crop --input in.pdf --rect 0,0,100,100 --output out.pdf

# 9. pdf_scale_pages
paperpilot pdf-scale-pages --input in.pdf --output out.pdf

# 10. pdf_delete_pages
paperpilot delete --input in.pdf --pages 1 --output out.pdf

# 11. pdf_reorder_pages
paperpilot reorder --input in.pdf --order 2,1 --output out.pdf

# 12. pdf_extract_pages
paperpilot extract --input in.pdf --pages 1 --output out.pdf

# 13. pdf_nup
paperpilot pdf-nup --input in.pdf --output out.pdf

# 14. pdf_booklet
paperpilot pdf-booklet --input in.pdf --output out.pdf

# 15. pdf_posterize
paperpilot pdf-posterize --input in.pdf --output out.pdf

# 16. pdf_stitch_canvas
paperpilot pdf-stitch-canvas --input in.pdf --output out.pdf
```

### Example MCP Payloads

```json
// 1. pdf_merge
{"inputs": ["file1.pdf", "file2.pdf"], "output": "out.pdf"}

// 2. pdf_merge_alternate
{"input_a": "a.pdf", "input_b": "b.pdf", "output": "out.pdf"}

// 3. pdf_split
{"input": "in.pdf", "every": 1, "output_dir": "out_dir/"}

// 4. pdf_split_by_outline
{"input": "in.pdf", "output": "out.zip"}

// 5. pdf_split_scanned_book
{"input": "in.pdf", "output": "out.pdf"}

// 6. pdf_rotate
{"input": "in.pdf", "angle": 90, "pages": "1", "output": "out.pdf"}

// 7. pdf_auto_rotate
{"input": "in.pdf", "output": "out.pdf"}

// 8. pdf_crop
{"input": "in.pdf", "box": "0,0,100,100", "output": "out.pdf"}

// 9. pdf_scale_pages
{"input": "in.pdf", "output": "out.pdf"}

// 10. pdf_delete_pages
{"input": "in.pdf", "pages": "1", "output": "out.pdf"}

// 11. pdf_reorder_pages
{"input": "in.pdf", "order": "2,1", "output": "out.pdf"}

// 12. pdf_extract_pages
{"input": "in.pdf", "pages": "1", "output": "out.pdf"}

// 13. pdf_nup
{"input": "in.pdf", "output": "out.pdf"}

// 14. pdf_booklet
{"input": "in.pdf", "output": "out.pdf"}

// 15. pdf_posterize
{"input": "in.pdf", "output": "out.pdf"}

// 16. pdf_stitch_canvas
{"input": "in.pdf", "output": "out.pdf"}
```

### Example REST API Payloads (curl)

```bash
# 1. pdf_merge
curl -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_merge \
  -H "Content-Type: application/json" \
  -d '{"inputs": ["file1.pdf", "file2.pdf"], "output": "out.pdf"}'

# 2. pdf_merge_alternate
curl -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_merge_alternate \
  -H "Content-Type: application/json" \
  -d '{"input_a": "a.pdf", "input_b": "b.pdf", "output": "out.pdf"}'

# 3. pdf_split
curl -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_split \
  -H "Content-Type: application/json" \
  -d '{"input": "in.pdf", "every": 1, "output_dir": "out_dir/"}'

# 4. pdf_split_by_outline
curl -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_split_by_outline \
  -H "Content-Type: application/json" \
  -d '{"input": "in.pdf", "output": "out.zip"}'

# 5. pdf_split_scanned_book
curl -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_split_scanned_book \
  -H "Content-Type: application/json" \
  -d '{"input": "in.pdf", "output": "out.pdf"}'

# 6. pdf_rotate
curl -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_rotate \
  -H "Content-Type: application/json" \
  -d '{"input": "in.pdf", "angle": 90, "pages": "1", "output": "out.pdf"}'

# 7. pdf_auto_rotate
curl -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_auto_rotate \
  -H "Content-Type: application/json" \
  -d '{"input": "in.pdf", "output": "out.pdf"}'

# 8. pdf_crop
curl -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_crop \
  -H "Content-Type: application/json" \
  -d '{"input": "in.pdf", "box": "0,0,100,100", "output": "out.pdf"}'

# 9. pdf_scale_pages
curl -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_scale_pages \
  -H "Content-Type: application/json" \
  -d '{"input": "in.pdf", "output": "out.pdf"}'

# 10. pdf_delete_pages
curl -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_delete_pages \
  -H "Content-Type: application/json" \
  -d '{"input": "in.pdf", "pages": "1", "output": "out.pdf"}'

# 11. pdf_reorder_pages
curl -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_reorder_pages \
  -H "Content-Type: application/json" \
  -d '{"input": "in.pdf", "order": "2,1", "output": "out.pdf"}'

# 12. pdf_extract_pages
curl -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_extract_pages \
  -H "Content-Type: application/json" \
  -d '{"input": "in.pdf", "pages": "1", "output": "out.pdf"}'

# 13. pdf_nup
curl -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_nup \
  -H "Content-Type: application/json" \
  -d '{"input": "in.pdf", "output": "out.pdf"}'

# 14. pdf_booklet
curl -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_booklet \
  -H "Content-Type: application/json" \
  -d '{"input": "in.pdf", "output": "out.pdf"}'

# 15. pdf_posterize
curl -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_posterize \
  -H "Content-Type: application/json" \
  -d '{"input": "in.pdf", "output": "out.pdf"}'

# 16. pdf_stitch_canvas
curl -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_stitch_canvas \
  -H "Content-Type: application/json" \
  -d '{"input": "in.pdf", "output": "out.pdf"}'
```
