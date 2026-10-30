# Manual Desktop UI Testing Guide: Tester 1 (22 Page Operations & Assembly Tools)

**Assigned Tester:** Tester 1  
**Total Tools Assigned:** 22 Tools  
**Domain:** Document Assembly, Page Reordering, Splitting, Imposition & Portfolio Management

---

### Tool 01: `pdf_merge` — Merge Documents (Combine Tabs)
- **UI Access:** Operations Dock > **Pages** > **Merge Documents** (or click *Merge Open Tabs* on top tab bar).
- **Test File:** Open `tests/fixtures/simple.pdf` in Tab 1, and any second PDF in Tab 2.
- **Manual Steps:**
  1. Open the Merge tool. Verify all open documents appear in the file list.
  2. Use drag-and-drop to adjust order.
  3. Click **Merge Documents** and save as `merged_test.pdf`.
- **Expected Result:** Combined document opens or is written to disk; total page count matches sum of inputs.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 02: `pdf_merge_alternate` — Alternating PDF Merge
- **UI Access:** Operations Dock > **Pages** > **Alternating PDF Merge**.
- **Test File:** Two 2-page documents (Doc A: A1, A2; Doc B: B1, B2).
- **Manual Steps:**
  1. Select Doc A as Primary and Doc B as Alternate.
  2. Toggle **Reverse order for Doc B** (e.g. for duplex backside scans).
  3. Click **Interleave Pages**.
- **Expected Result:** Pages are interleaved: A1, B1, A2, B2.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 03: `pdf_split` — Split PDF by Range or Count
- **UI Access:** Operations Dock > **Pages** > **Split PDF** (or right-click thumbnail > *Split*).
- **Test File:** Multi-page PDF (5+ pages).
- **Manual Steps:**
  1. Enter range: `1-2, 3-5`.
  2. Click **Split PDF** and choose destination directory.
- **Expected Result:** Output folder contains `split_1.pdf` (2 pages) and `split_2.pdf` (3 pages).
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 04: `pdf_split_by_outline` — Split by Outline Bookmarks / TOC
- **UI Access:** Operations Dock > **Pages** > **Split by Document Outline**.
- **Test File:** `tests/fixtures/test_bookmarks.pdf` (or any PDF with bookmarks).
- **Manual Steps:**
  1. Set outline level to `1`.
  2. Click **Split by Outline**.
- **Expected Result:** Produces chapter PDFs named after TOC titles (`01_Chapter 1.pdf`, etc.) packed in a ZIP or folder. Falls back cleanly to 1-page split if bookmarks are absent.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 05: `pdf_split_scanned_book` — Split Scanned Book Spine
- **UI Access:** Operations Dock > **Pages** > **Split Scanned Book Spine**.
- **Test File:** Landscape document with 2-up book page scans.
- **Manual Steps:**
  1. Select **Vertical Split** along center gutter.
  2. Select reading order: **Left to Right**.
  3. Click **Split Book Spine**.
- **Expected Result:** Two-page spreads are split into clean single portrait pages.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 06: `pdf_burst` — Burst Document into Single Pages
- **UI Access:** Operations Dock > **Pages** > **Burst PDF**.
- **Test File:** Any 3+ page PDF.
- **Manual Steps:**
  1. Click **Burst PDF** and pick output directory.
- **Expected Result:** Creates `page_001.pdf`, `page_002.pdf`, `page_003.pdf`.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 07: `pdf_crop` — Crop Page Bounding Boxes
- **UI Access:** Operations Dock > **Pages** > **Crop Pages** (or toolbar Crop tool).
- **Test File:** Any PDF.
- **Manual Steps:**
  1. Drag crop rectangle on the canvas.
  2. Verify bounding coordinates populate. Click **Apply Crop**.
- **Expected Result:** Canvas re-renders immediately showing trimmed margins.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 08: `pdf_delete_pages` — Delete Specific Pages
- **UI Access:** Operations Dock > **Pages** > **Delete Pages** (or select thumbnail + press `Delete`).
- **Test File:** 5-page PDF.
- **Manual Steps:**
  1. Enter `2, 4` in Delete Pages input. Click **Delete**.
- **Expected Result:** Pages 2 and 4 are removed; remaining 3 pages re-index.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 09: `pdf_extract_pages` — Extract Page Range to New PDF
- **UI Access:** Operations Dock > **Pages** > **Extract PDF Pages** (or right-click thumbnail > *Extract*).
- **Test File:** 5-page PDF.
- **Manual Steps:**
  1. Enter `2-3`. Click **Extract Pages**.
- **Expected Result:** New document opens containing only pages 2 and 3. Original file remains intact.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 10: `pdf_reorder_pages` — Drag-and-Drop Page Reordering
- **UI Access:** Left thumbnail sidebar rail.
- **Test File:** 4-page PDF.
- **Manual Steps:**
  1. Click and drag thumbnail Page 4 above Page 1.
- **Expected Result:** Page order updates instantly to 4, 1, 2, 3 in both thumbnail rail and canvas.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 11: `pdf_rotate` — Rotate Pages
- **UI Access:** Operations Dock > **Pages** > **Rotate Document** (or toolbar `🔄` button).
- **Test File:** Any PDF.
- **Manual Steps:**
  1. Select `90° Clockwise`, page scope: `All`. Click **Apply Rotation**.
- **Expected Result:** Canvas and thumbnail preview rotate 90° clockwise.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 12: `pdf_auto_rotate` — Auto Upright Orientation
- **UI Access:** Operations Dock > **Pages** > **Auto Upright Orientation**.
- **Test File:** PDF containing inverted or sideways pages.
- **Manual Steps:**
  1. Click **Auto Upright Pages**.
- **Expected Result:** Sideways text is heuristically rotated upright; correctly oriented pages are untouched.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 13: `pdf_remove_blank` — Remove Blank Pages
- **UI Access:** Operations Dock > **Pages** > **Remove Blank Pages**.
- **Test File:** PDF containing at least one empty white page.
- **Manual Steps:**
  1. Set sensitivity to 95%. Click **Remove Blank Pages**.
- **Expected Result:** Blank page is excised; toast announces count of removed pages.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 14: `pdf_scale_pages` — Scale Page Dimensions
- **UI Access:** Operations Dock > **Pages** > **Scale Page Dimensions**.
- **Test File:** Letter or A4 PDF.
- **Manual Steps:**
  1. Choose target size: `A3` (or scale percentage: `150%`). Click **Scale**.
- **Expected Result:** MediaBox dimensions expand proportionally; vector content stays sharp.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 15: `pdf_nup` — N-Up Page Imposition
- **UI Access:** Operations Dock > **Pages** > **N-Up Page Imposition**.
- **Test File:** 4-page or 8-page document.
- **Manual Steps:**
  1. Select `4-Up (2x2 Grid)`. Toggle **Draw Border Lines**. Click **Impose Pages**.
- **Expected Result:** 4 consecutive pages are tiled onto a single composite sheet.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 16: `pdf_booklet` — Booklet Imposition
- **UI Access:** Operations Dock > **Pages** > **Booklet Imposition**.
- **Test File:** 8-page document.
- **Manual Steps:**
  1. Select `Saddle-Stitch Folding Booklet`. Click **Generate Booklet**.
- **Expected Result:** Sheets are paired for center-fold printing (e.g. Sheet 1 has p8 & p1).
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 17: `pdf_posterize` — Multi-Sheet Poster Slicer
- **UI Access:** Operations Dock > **Pages** > **Multi-Sheet Poster Tile Slicer**.
- **Test File:** Large single-page diagram or photo.
- **Manual Steps:**
  1. Set grid: `2x2 Sheets`, overlap: `0.5 in`. Click **Slice Poster**.
- **Expected Result:** Outputs 4 printable pages with dashed cut/alignment guides.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 18: `pdf_stitch_canvas` — Stitch Continuous Canvas
- **UI Access:** Operations Dock > **Pages** > **Stitch Continuous Canvas**.
- **Test File:** 3-page document.
- **Manual Steps:**
  1. Choose `Vertical Scroll Stitch`. Click **Stitch Pages**.
- **Expected Result:** All 3 pages merge into one tall continuous canvas.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 19: `pdf_bookmarks` — Extract / Inspect Bookmarks
- **UI Access:** Left sidebar Outline/Bookmark tab (`📑`).
- **Test File:** `tests/fixtures/test_bookmarks.pdf`.
- **Manual Steps:**
  1. Open Bookmarks sidebar. Click on a chapter item.
- **Expected Result:** Viewer instantly jumps to the bookmarked destination page.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 20: `pdf_attachments` — PDF Portfolio Attachments
- **UI Access:** Operations Dock > **Pages** > **PDF Portfolio Attachments** (or sidebar paperclip icon).
- **Test File:** Any PDF.
- **Manual Steps:**
  1. Click **Add Attachment** and attach any text file or image.
  2. Click **Download / Extract Attachment**.
- **Expected Result:** Embedded file is listed and can be extracted byte-for-byte.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 21: `pdf_images_to_pdf` — Combine Images into PDF
- **UI Access:** Operations Dock > **Convert** > **Images to PDF**.
- **Test File:** 2 or more PNG/JPG files.
- **Manual Steps:**
  1. Drop image files into the queue. Set orientation: **Auto**. Click **Create PDF**.
- **Expected Result:** Generates a multi-page PDF where each page embeds an image with full resolution.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 22: `img_to_pdf` — Image to PDF Single/Multi Synthesis
- **UI Access:** Command Palette (`Ctrl+K`) > `img_to_pdf`.
- **Test File:** Any `.png` or `.jpg`.
- **Manual Steps:**
  1. Select single image file and click **Convert**.
- **Expected Result:** Produces a 1-page PDF exactly matching the image dimensions.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`
