# Manual Desktop UI Testing Guide: Tester 1 (30 Tools — Page Operations, Imposition, Optimization & Rendering)

**Assigned Tester:** Tester 1  
**Total Tools Assigned:** 30 Tools (Tools 01 to 30)  
**Domain:** Document Assembly, Page Reordering, Splitting, Imposition, Portfolio Management, Optimization, Rendering & Password Encryption  
**Target Branch:** `feature/dev`

---

## 🚀 Quick Setup & Test Fixtures (Run Once Before Testing)

All 30 tools can be tested using pre-existing real test fixtures from the repository.

### 1. Launch the Tauri Desktop Application
Open a terminal in the project root and start the desktop app:
```bash
cd apps/desktop
npm run tauri dev
```
*(Keep system RAM footprint light; close heavy unused browser tabs).*

### 2. One-Liner Fixture Setup Command
Run this one command in your terminal to create a clean `tester1_fixtures` folder on your Desktop with all required test files:
```bash
mkdir -p ~/Desktop/tester1_fixtures
cp tests/e2e_fixtures/real/multi_page.pdf ~/Desktop/tester1_fixtures/
cp tests/e2e_fixtures/real/single_page.pdf ~/Desktop/tester1_fixtures/
cp tests/e2e_fixtures/real/merge_a.pdf ~/Desktop/tester1_fixtures/
cp tests/e2e_fixtures/real/merge_b.pdf ~/Desktop/tester1_fixtures/
cp tests/e2e_fixtures/real/large_doc.pdf ~/Desktop/tester1_fixtures/
cp tests/e2e_fixtures/real/sample_photo_doc.pdf ~/Desktop/tester1_fixtures/
cp tests/e2e_fixtures/real/img1.png ~/Desktop/tester1_fixtures/
cp tests/e2e_fixtures/real/img2.png ~/Desktop/tester1_fixtures/
cp tests/fixtures/malformed.pdf ~/Desktop/tester1_fixtures/
```

### 📋 Fixture-to-Tool Mapping Matrix

| Test Fixture File | Format / Purpose | Tools Covered |
| :--- | :--- | :--- |
| `~/Desktop/tester1_fixtures/multi_page.pdf` | 5-Page Standard PDF | **Tools 03, 04, 05, 06, 07, 08, 09, 10, 11, 12, 13, 14, 15, 16, 18, 19, 20, 25, 27, 28** |
| `~/Desktop/tester1_fixtures/merge_a.pdf` & `merge_b.pdf` | Pair of PDFs for Merging & Diffing | **Tools 01, 02, 29** |
| `~/Desktop/tester1_fixtures/single_page.pdf` | Clean 1-Page Document | **Tools 17, 30** |
| `~/Desktop/tester1_fixtures/img1.png` & `img2.png` | Raster PNG Images | **Tools 20, 21, 22** |
| `~/Desktop/tester1_fixtures/large_doc.pdf` & `sample_photo_doc.pdf` | Image-Heavy / Stream-Rich PDFs | **Tools 23, 24** |
| `~/Desktop/tester1_fixtures/malformed.pdf` | Corrupted / Damaged PDF Syntax | **Tool 26** |

---

## 🛠️ Step-by-Step Testing Guide (Tools 01 to 30)

---

### Tool 01: `pdf_merge` — Merge Documents (Combine Tabs)
- **UI Access:** Operations Dock > **Pages** > **Merge Documents** (or click *Merge Open Tabs* on top tab bar).
- **Test File:** Open `~/Desktop/tester1_fixtures/merge_a.pdf` in Tab 1, and `merge_b.pdf` in Tab 2.
- **Manual Steps:**
  1. Open the Merge tool. Verify all open documents appear in the file list.
  2. Use drag-and-drop to adjust order.
  3. Click **Merge Documents** and save as `merged_test.pdf`.
- **Expected Result:** Combined document opens or is written to disk; total page count matches sum of inputs.
- **Sign-off:** `[X] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 02: `pdf_merge_alternate` — Alternating PDF Merge
- **UI Access:** Operations Dock > **Pages** > **Alternating PDF Merge**.
- **Test File:** `~/Desktop/tester1_fixtures/merge_a.pdf` and `multi_page.pdf`.
- **Manual Steps:**
  1. Select Doc A as Primary and Doc B as Alternate.
  2. Toggle **Reverse order for Doc B** (e.g. for duplex backside scans).
  3. Click **Interleave Pages**.
- **Expected Result:** Pages are interleaved (A1, B1, A2, B2...) cleanly into a new PDF.
- **Sign-off:** `[ ] PASS` | `[X] FAIL` | Notes: `Unable to locate file-File not found___________________`

---

### Tool 03: `pdf_split` — Split PDF by Range or Count
- **UI Access:** Operations Dock > **Pages** > **Split PDF** (or right-click thumbnail > *Split*).
- **Test File:** `~/Desktop/tester1_fixtures/multi_page.pdf` (5 pages).
- **Manual Steps:**
  1. Enter range: `1-2, 3-5`.
  2. Click **Split PDF** and choose destination directory.
- **Expected Result:** Output folder contains `split_1.pdf` (2 pages) and `split_2.pdf` (3 pages).
- **Sign-off:** `[X] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 04: `pdf_split_by_outline` — Split by Outline Bookmarks / TOC
- **UI Access:** Operations Dock > **Pages** > **Split by Document Outline**.
- **Test File:** `~/Desktop/tester1_fixtures/multi_page.pdf`.
- **Manual Steps:**
  1. Set outline level to `1`.
  2. Click **Split by Outline**.
- **Expected Result:** Produces chapter PDFs named after TOC titles packed in a ZIP or folder, or falls back cleanly to page split if bookmarks are absent.
- **Sign-off:** `[ ] PASS` | `[X] FAIL` | Notes: `_Unexpected error unable to lcoate doc___________________`

---

### Tool 05: `pdf_split_scanned_book` — Split Scanned Book Spine
- **UI Access:** Operations Dock > **Pages** > **Split Scanned Book Spine**.
- **Test File:** `~/Desktop/tester1_fixtures/multi_page.pdf`.
- **Manual Steps:**
  1. Select **Vertical Split** along center gutter.
  2. Select reading order: **Left to Right**.
  3. Click **Split Book Spine**.
- **Expected Result:** Each spread is split into two half-width single pages (page count doubles).
- **Sign-off:** `[X] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 06: `pdf_burst` — Burst Document into Single Pages
- **UI Access:** Operations Dock > **Pages** > **Burst PDF**.
- **Test File:** `~/Desktop/tester1_fixtures/multi_page.pdf`.
- **Manual Steps:**
  1. Click **Burst PDF** and pick output directory.
- **Expected Result:** Creates individual single-page PDFs (`page_001.pdf`, `page_002.pdf`, etc.).
- **Sign-off:** `[X] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 07: `pdf_crop` — Crop Page Bounding Boxes
- **UI Access:** Operations Dock > **Pages** > **Crop Pages** (or toolbar Crop tool).
- **Test File:** `~/Desktop/tester1_fixtures/multi_page.pdf`.
- **Manual Steps:**
  1. Drag crop rectangle on the canvas or enter margin insets.
  2. Verify bounding coordinates populate. Click **Apply Crop**.
- **Expected Result:** Canvas re-renders immediately showing trimmed margins.
- **Sign-off:** `[ ] PASS` | `[X] FAIL` | Notes: `_Un expected issue-not working___________________`

---

### Tool 08: `pdf_delete_pages` — Delete Specific Pages
- **UI Access:** Operations Dock > **Pages** > **Delete Pages** (or select thumbnail + press `Delete`).
- **Test File:** `~/Desktop/tester1_fixtures/multi_page.pdf` (5 pages).
- **Manual Steps:**
  1. Enter `2, 4` in Delete Pages input. Click **Delete**.
- **Expected Result:** Pages 2 and 4 are removed; remaining 3 pages re-index.
- **Sign-off:** `[ ] PASS` | `[X] FAIL` | Notes: `UI is working, tool is not working____________________`

---

### Tool 09: `pdf_extract_pages` — Extract Page Range to New PDF
- **UI Access:** Operations Dock > **Pages** > **Extract PDF Pages** (or right-click thumbnail > *Extract*).
- **Test File:** `~/Desktop/tester1_fixtures/multi_page.pdf`.
- **Manual Steps:**
  1. Enter `2-3`. Click **Extract Pages**.
- **Expected Result:** New document is generated containing only pages 2 and 3. Original file remains intact.
- **Sign-off:** `[X] PASS` | `[ ] FAIL` | Notes: `_Tool 2 is working but not 1, we should remove 1___________________`

---

### Tool 10: `pdf_reorder_pages` — Drag-and-Drop Page Reordering
- **UI Access:** Left thumbnail sidebar rail (or Operations Dock > **Pages** > **Reorder Pages**).
- **Test File:** `~/Desktop/tester1_fixtures/multi_page.pdf`.
- **Manual Steps:**
  1. Click and drag thumbnail Page 4 above Page 1 (or specify order `4,1,2,3,5`).
- **Expected Result:** Page order updates instantly in both thumbnail rail and canvas.
- **Sign-off:** `[ ] PASS` | `[X] FAIL` | Notes: `Out of bound error____________________`

---

### Tool 11: `pdf_rotate` — Rotate Pages
- **UI Access:** Operations Dock > **Pages** > **Rotate Document** (or toolbar `🔄` button).
- **Test File:** `~/Desktop/tester1_fixtures/multi_page.pdf`.
- **Manual Steps:**
  1. Select `90° Clockwise`, page scope: `All`. Click **Apply Rotation**.
- **Expected Result:** Canvas and thumbnail preview rotate 90° clockwise.
- **Sign-off:** `[X] PASS` | `[ ] FAIL` | Notes: `Ui is working and tool 2 is working, need to remove tool 1____________________`

---

### Tool 12: `pdf_auto_rotate` — Auto Upright Orientation
- **UI Access:** Operations Dock > **Pages** > **Auto Upright Orientation**.
- **Test File:** `~/Desktop/tester1_fixtures/multi_page.pdf` (rotated 90° or 180°).
- **Manual Steps:**
  1. Click **Auto Upright Pages**.
- **Expected Result:** Sideways text is heuristically rotated upright; correctly oriented pages are untouched.
- **Sign-off:** `[ ] PASS` | `[X] FAIL` | Notes: `_Same as input no change___________________`

---

### Tool 13: `pdf_remove_blank` — Remove Blank Pages
- **UI Access:** Operations Dock > **Pages** > **Remove Blank Pages**.
- **Test File:** `~/Desktop/tester1_fixtures/multi_page.pdf`.
- **Manual Steps:**
  1. Set sensitivity to 95%. Click **Remove Blank Pages**.
- **Expected Result:** Blank pages are excised; toast announces count of removed pages (or confirms 0 blank pages removed without corrupting document).
- **Sign-off:** `[X] PASS` | `[ ] FAIL` | Notes: `_working - need to seee 2 tools___________________`

---

### Tool 14: `pdf_scale_pages` — Scale Page Dimensions
- **UI Access:** Operations Dock > **Pages** > **Scale Page Dimensions**.
- **Test File:** `~/Desktop/tester1_fixtures/multi_page.pdf`.
- **Manual Steps:**
  1. Choose target size: `A3` (or scale percentage: `150%`). Click **Scale**.
- **Expected Result:** MediaBox dimensions expand proportionally; vector content stays sharp.
- **Sign-off:** `[X] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 15: `pdf_nup` — N-Up Page Imposition
- **UI Access:** Operations Dock > **Pages** > **N-Up Page Imposition**.
- **Test File:** `~/Desktop/tester1_fixtures/multi_page.pdf`.
- **Manual Steps:**
  1. Select `4-Up (2x2 Grid)`. Toggle **Draw Border Lines**. Click **Impose Pages**.
- **Expected Result:** 4 consecutive pages are tiled onto a single composite sheet.
- **Sign-off:** `[ ] PASS` | `[X] FAIL` | Notes: `generating empty document only with no borders and unable to do selection like 2up, 6up,9up__- no borders also__________________`

---

### Tool 16: `pdf_booklet` — Booklet Imposition
- **UI Access:** Operations Dock > **Pages** > **Booklet Imposition**.
- **Test File:** `~/Desktop/tester1_fixtures/multi_page.pdf`.
- **Manual Steps:**
  1. Select `Saddle-Stitch Folding Booklet`. Click **Generate Booklet**.
- **Expected Result:** Sheets are paired for center-fold printing with proper outer/inner signature ordering.
- **Sign-off:** `[ ] PASS` | `[X] FAIL` | Notes: `Document generated , need to validate it____________________`

---

### Tool 17: `pdf_posterize` — Multi-Sheet Poster Slicer
- **UI Access:** Operations Dock > **Pages** > **Multi-Sheet Poster Tile Slicer**.
- **Test File:** `~/Desktop/tester1_fixtures/single_page.pdf`.
- **Manual Steps:**
  1. Set grid: `2x2 Sheets`, overlap: `0.5 in`. Click **Slice Poster**.
- **Expected Result:** Outputs 4 printable pages with dashed cut/alignment guides.
- **Sign-off:** `[X] PASS` | `[ ] FAIL` | Notes: `generated a document seems to be working fine____________________`

---

### Tool 18: `pdf_stitch_canvas` — Stitch Continuous Canvas
- **UI Access:** Operations Dock > **Pages** > **Stitch Continuous Canvas**.
- **Test File:** `~/Desktop/tester1_fixtures/multi_page.pdf`.
- **Manual Steps:**
  1. Choose `Vertical Scroll Stitch`. Click **Stitch Pages**.
- **Expected Result:** All pages merge into one tall continuous canvas page.
- **Sign-off:** `[X] PASS` | `[ ] FAIL` | Notes: `Looks good____________________`

---

### Tool 19: `pdf_bookmarks` — Extract / Inspect Bookmarks
- **UI Access:** Left sidebar Outline/Bookmark tab (`📑`) or Operations Dock > **Pages** > **Bookmarks**.
- **Test File:** `~/Desktop/tester1_fixtures/multi_page.pdf`.
- **Manual Steps:**
  1. Open Bookmarks sidebar or tool. Inspect or add/navigate a chapter bookmark item.
- **Expected Result:** Bookmarks are extracted cleanly and clicking a bookmark jumps to its target page.
- **Sign-off:** `[ ] PASS` | `[X] FAIL` | Notes: `_Generated file but need to validate___________________`

---

### Tool 20: `pdf_attachments` — PDF Portfolio Attachments
- **UI Access:** Operations Dock > **Pages** > **PDF Portfolio Attachments** (or sidebar paperclip icon).
- **Test File:** `~/Desktop/tester1_fixtures/multi_page.pdf` + `img1.png`.
- **Manual Steps:**
  1. Click **Add Attachment** and attach `img1.png`.
  2. Click **Download / Extract Attachment**.
- **Expected Result:** Embedded file is listed in the portfolio and can be extracted byte-for-byte.
- **Sign-off:** `[ ] PASS` | `[X] FAIL` | Notes: `Unable to select screenshot/Image - Drag and drop also not working for pdf and image____________________`

---

### Tool 21: `pdf_images_to_pdf` — Combine Images into PDF
- **UI Access:** Operations Dock > **Convert** > **Images to PDF**.
- **Test File:** `~/Desktop/tester1_fixtures/img1.png` and `img2.png`.
- **Manual Steps:**
  1. Drop both PNG files into the queue. Set orientation: **Auto**. Click **Create PDF**.
- **Expected Result:** Generates a 2-page PDF where each page embeds one image at full resolution.
- **Sign-off:** `[ ] PASS` | `[X] FAIL` | Notes: `_Unable to select any image___________________`

---

### Tool 22: `img_to_pdf` — Image to PDF Single/Multi Synthesis
- **UI Access:** Command Palette (`Ctrl+K`) > `img_to_pdf`.
- **Test File:** `~/Desktop/tester1_fixtures/img1.png`.
- **Manual Steps:**
  1. Select `img1.png` and click **Convert**.
- **Expected Result:** Produces a 1-page PDF matching the image dimensions.
- **Sign-off:** `[ ] PASS` | `[X] FAIL` | Notes: `_image selection is not showing up___________________`

---

### Tool 23: `pdf_compress` — Compress PDF File Size
- **UI Access:** Operations Dock > **Optimize** > **Compress PDF**.
- **Test File:** `~/Desktop/tester1_fixtures/sample_photo_doc.pdf` (or `large_doc.pdf`).
- **Manual Steps:**
  1. Select preset: **eBook (150 DPI)**. Click **Compress PDF**.
- **Expected Result:** Toast reports compression completion and size metrics; output file opens cleanly.
- **Sign-off:** `[X] PASS` | `[ ] FAIL` | Notes: `_Working well and fast___________________`

---

### Tool 24: `pdf_dedup_streams` — Lossless Stream Deduplication
- **UI Access:** Operations Dock > **Optimize** > **Lossless Stream Deduplication**.
- **Test File:** `~/Desktop/tester1_fixtures/large_doc.pdf`.
- **Manual Steps:**
  1. Click **Deduplicate Streams**.
- **Expected Result:** Redundant image/font objects are merged into single indirect references with zero visual quality loss.
- **Sign-off:** `[X] PASS` | `[ ] FAIL` | Notes: `_working fine___________________`

---

### Tool 25: `pdf_linearize` — Fast Web View (Linearize)
- **UI Access:** Operations Dock > **Optimize** > **Fast Web View (Linearize)**.
- **Test File:** `~/Desktop/tester1_fixtures/multi_page.pdf`.
- **Manual Steps:**
  1. Click **Linearize Document**.
- **Expected Result:** Output PDF is optimized with a `/Linearized` header dictionary for progressive byte-range loading.
- **Sign-off:** `[ ] PASS` | `[X] FAIL` | Notes: `File generated but same size ____________________`

---

### Tool 26: `pdf_repair` — Rebuild Damaged PDF Syntax
- **UI Access:** Operations Dock > **Optimize** > **Repair PDF**.
- **Test File:** `~/Desktop/tester1_fixtures/malformed.pdf` (or `multi_page.pdf`).
- **Manual Steps:**
  1. Load file into Repair PDF tool. Click **Rebuild XREF Tables**.
- **Expected Result:** Damaged XREF tables/dictionaries are reconstructed or cleanly reported without crashing.
- **Sign-off:** `[ ] PASS` | `[X] FAIL` | Notes: `unable to test usecase____________________`

---

### Tool 27: `pdf_invert_colors` — High-Contrast Dark Mode
- **UI Access:** Operations Dock > **Edit** > **Invert Colors (Dark Mode)**.
- **Test File:** `~/Desktop/tester1_fixtures/multi_page.pdf`.
- **Manual Steps:**
  1. Select **High-Contrast Dark Mode**. Click **Invert Colors**.
- **Expected Result:** White backgrounds become dark charcoal; black text inverts to clear white.
- **Sign-off:** `[ ] PASS` | `[X] FAIL` | Notes: `file generated but no difference____________________`

---

### Tool 28: `pdf_render` — High-DPI Page Rasterization to PNG
- **UI Access:** Operations Dock > **Convert** > **Render Page** (or right-click canvas > *Export as PNG*).
- **Test File:** `~/Desktop/tester1_fixtures/multi_page.pdf`.
- **Manual Steps:**
  1. Set DPI: `300 DPI`. Select Page 1. Click **Render & Save PNG**.
- **Expected Result:** High-resolution `.png` file is created on disk with valid PNG header bytes.
- **Sign-off:** `[ ] PASS` | `[X] FAIL` | Notes: `_generated pdf, but no png___________________`

---

### Tool 29: `pdf_compare` — Visual Pixel Diff Slider
- **UI Access:** Operations Dock > **Quick** > **Compare PDFs** (or *Tools > Visual Pixel Diff*).
- **Test File:** `~/Desktop/tester1_fixtures/merge_a.pdf` and `merge_b.pdf`.
- **Manual Steps:**
  1. Open Doc 1 in Tab 1, Doc 2 in Tab 2.
  2. Launch **Compare PDFs**. Drag the interactive center split curtain.
  3. Toggle **Color-Coded Highlights**.
- **Expected Result:** Real-time visual comparison reveals differences between revisions with color highlights.
- **Sign-off:** `[ ] PASS` | `[X] FAIL` | Notes: `Not working ____________________`

---

### Tool 30: `pdf_encrypt` — Protect Document with AES-256 Password
- **UI Access:** Operations Dock > **Security** > **Encrypt with Password**.
- **Test File:** `~/Desktop/tester1_fixtures/single_page.pdf`.
- **Manual Steps:**
  1. Enter User Password: `pass123`. Click **Encrypt & Save**.
  2. Reopen the saved document in PaperPilot.
- **Expected Result:** Unlock password modal appears blocking canvas. Entering `pass123` unlocks the document.
- **Sign-off:** `[X] PASS` | `[ ] FAIL` | Notes: `_File generated with Encrypted___________________`

---

## 🏁 Summary Sign-Off Scorecard: Tester 1 (30 Tools)

| Tool # | Tool Identifier | UI Location | Test Input File | Status | Notes |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **01** | `pdf_merge` | Operations Dock > Pages | `merge_a.pdf`, `merge_b.pdf` | `[ ] PASS` | |
| **02** | `pdf_merge_alternate` | Operations Dock > Pages | `merge_a.pdf`, `multi_page.pdf` | `[ ] PASS` | |
| **03** | `pdf_split` | Operations Dock > Pages | `multi_page.pdf` | `[ ] PASS` | |
| **04** | `pdf_split_by_outline` | Operations Dock > Pages | `multi_page.pdf` | `[ ] PASS` | |
| **05** | `pdf_split_scanned_book` | Operations Dock > Pages | `multi_page.pdf` | `[ ] PASS` | |
| **06** | `pdf_burst` | Operations Dock > Pages | `multi_page.pdf` | `[ ] PASS` | |
| **07** | `pdf_crop` | Operations Dock > Pages | `multi_page.pdf` | `[ ] PASS` | |
| **08** | `pdf_delete_pages` | Operations Dock > Pages | `multi_page.pdf` | `[ ] PASS` | |
| **09** | `pdf_extract_pages` | Operations Dock > Pages | `multi_page.pdf` | `[ ] PASS` | |
| **10** | `pdf_reorder_pages` | Left Thumbnail Rail | `multi_page.pdf` | `[ ] PASS` | |
| **11** | `pdf_rotate` | Operations Dock > Pages | `multi_page.pdf` | `[ ] PASS` | |
| **12** | `pdf_auto_rotate` | Operations Dock > Pages | `multi_page.pdf` | `[ ] PASS` | |
| **13** | `pdf_remove_blank` | Operations Dock > Pages | `multi_page.pdf` | `[ ] PASS` | |
| **14** | `pdf_scale_pages` | Operations Dock > Pages | `multi_page.pdf` | `[ ] PASS` | |
| **15** | `pdf_nup` | Operations Dock > Pages | `multi_page.pdf` | `[ ] PASS` | |
| **16** | `pdf_booklet` | Operations Dock > Pages | `multi_page.pdf` | `[ ] PASS` | |
| **17** | `pdf_posterize` | Operations Dock > Pages | `single_page.pdf` | `[ ] PASS` | |
| **18** | `pdf_stitch_canvas` | Operations Dock > Pages | `multi_page.pdf` | `[ ] PASS` | |
| **19** | `pdf_bookmarks` | Left Sidebar `📑` / Pages | `multi_page.pdf` | `[ ] PASS` | |
| **20** | `pdf_attachments` | Operations Dock > Pages | `multi_page.pdf` + `img1.png` | `[ ] PASS` | |
| **21** | `pdf_images_to_pdf` | Operations Dock > Convert | `img1.png`, `img2.png` | `[ ] PASS` | |
| **22** | `img_to_pdf` | Command Palette (`Ctrl+K`) | `img1.png` | `[ ] PASS` | |
| **23** | `pdf_compress` | Operations Dock > Optimize | `sample_photo_doc.pdf` | `[ ] PASS` | |
| **24** | `pdf_dedup_streams` | Operations Dock > Optimize | `large_doc.pdf` | `[ ] PASS` | |
| **25** | `pdf_linearize` | Operations Dock > Optimize | `multi_page.pdf` | `[ ] PASS` | |
| **26** | `pdf_repair` | Operations Dock > Optimize | `malformed.pdf` | `[ ] PASS` | |
| **27** | `pdf_invert_colors` | Operations Dock > Edit | `multi_page.pdf` | `[ ] PASS` | |
| **28** | `pdf_render` | Operations Dock > Convert | `multi_page.pdf` | `[ ] PASS` | |
| **29** | `pdf_compare` | Operations Dock > Quick | `merge_a.pdf`, `merge_b.pdf` | `[ ] PASS` | |
| **30** | `pdf_encrypt` | Operations Dock > Security | `single_page.pdf` | `[ ] PASS` | |

**Tester Signature:** ___________________________  
**Date of Testing:** ___________________________  
**Overall Result:** `[ ] FULL PASS (30/30)` | `[ ] ISSUES DETECTED`
