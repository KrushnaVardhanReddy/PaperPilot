# Manual Desktop UI Testing Guide: Tester 1 (Page Operations & Assembly)

**Assigned Tester:** Tester 1  
**Total Tools Assigned:** 13 Tools  
**Domain:** Core Page Operations, Imposition Splitting, Reordering, and Alignment

---

## Tool 1: Merge Documents (`merge`)
- **Category in UI:** `Quick Actions` / `Page Management`
- **UI Access:** 
  1. Open Operations Dock (`🛠️` right sidebar icon).
  2. Select category **Pages** or search `Merge`.
  3. Alternatively, open 2 or more PDF files into separate tabs, then click the **Merge Tabs** button on the tab bar.
- **Sample File:** Open `tests/fixtures/simple.pdf` in Tab 1, and any other PDF in Tab 2 (or drop 2 files into the merge file list).
- **Manual Steps:**
  1. In the Merge tool panel, verify both open documents or files appear in the merge queue.
  2. Reorder files in the queue using drag-and-drop or the `↑` / `↓` buttons.
  3. Click **Merge Documents**.
  4. When prompted, choose the destination file path (e.g., `merged_output.pdf`).
- **Visual Acceptance Criteria:**
  - [ ] A success toast notification appears: *"Merged documents successfully"*.
  - [ ] A new tab opens with the combined document, or the saved file exists on disk.
  - [ ] Verify total page count equals the sum of the input documents.
- **Status:** `[ ] PASS` | `[ ] FAIL` | **Notes:** `____________________`

---

## Tool 2: Split PDF (`split`)
- **Category in UI:** `Page Management`
- **UI Access:** Operations Dock > **Pages** > **Split PDF** (or right-click thumbnail > *Split Document*).
- **Sample File:** Open a multi-page PDF (e.g. 5+ pages).
- **Manual Steps:**
  1. In the Split tool panel, review split modes: **By Range** (e.g., `1-2, 3-5`), **Every N Pages**, or **Fixed Sets**.
  2. Enter range `1-2, 3-4`.
  3. Click **Split PDF** and select an output folder.
- **Visual Acceptance Criteria:**
  - [ ] The output folder opens or contains `split_1.pdf` (pages 1-2) and `split_2.pdf` (pages 3-4).
  - [ ] Opening both output files confirms exact page partitions without blank pages.
- **Status:** `[ ] PASS` | `[ ] FAIL` | **Notes:** `____________________`

---

## Tool 3: Rotate Document (`rotate`)
- **Category in UI:** `Quick Actions` / `Page Management`
- **UI Access:** 
  1. Operations Dock > **Pages** > **Rotate Document**.
  2. Or click the Rotate icon on the top viewer toolbar (`🔄`).
  3. Or right-click a thumbnail and click *Rotate Clockwise (90°)*.
- **Sample File:** `tests/fixtures/simple.pdf` (or any single/multi-page PDF).
- **Manual Steps:**
  1. Select angle: **90°**, **180°**, or **270°**.
  2. Select page scope: **All Pages**, **Current Page**, or custom (e.g., `1, 3`).
  3. Click **Apply Rotation**.
- **Visual Acceptance Criteria:**
  - [ ] The main canvas immediately re-renders with the rotated orientation.
  - [ ] The thumbnail in the left sidebar updates to reflect the new angle.
  - [ ] Saving the file preserves the rotation when reopened.
- **Status:** `[ ] PASS` | `[ ] FAIL` | **Notes:** `____________________`

---

## Tool 4: Delete Pages (`delete_pages`)
- **Category in UI:** `Page Management`
- **UI Access:** Operations Dock > **Pages** > **Delete Pages**, or select thumbnail and press `Delete` key.
- **Sample File:** Multi-page PDF (at least 3 pages).
- **Manual Steps:**
  1. In the tool panel, enter page numbers to delete (e.g., `2`).
  2. Alternatively, select page 2 in the thumbnail rail, right-click, and choose **Delete Page**.
  3. Confirm the deletion prompt if a modal appears.
- **Visual Acceptance Criteria:**
  - [ ] Page 2 is removed from the thumbnail rail and main viewer.
  - [ ] Total page count decreases by the number of deleted pages.
  - [ ] Remaining page numbers adjust sequentially.
- **Status:** `[ ] PASS` | `[ ] FAIL` | **Notes:** `____________________`

---

## Tool 5: Reorder Pages (`reorder_pages`)
- **Category in UI:** `Page Management`
- **UI Access:** Thumbnail sidebar (drag-and-drop) or Operations Dock > **Pages** > **Reorder Pages**.
- **Sample File:** Multi-page document (pages 1, 2, 3, 4).
- **Manual Steps:**
  1. In the thumbnail sidebar, click and drag Page 3 above Page 1.
  2. Or in the tool panel, enter order: `3, 1, 2, 4` and click **Reorder**.
- **Visual Acceptance Criteria:**
  - [ ] The thumbnails re-index instantly with the new order.
  - [ ] Scrolling the main canvas shows the reordered pages.
  - [ ] File saves with the exact new page sequence.
- **Status:** `[ ] PASS` | `[ ] FAIL` | **Notes:** `____________________`

---

## Tool 6: Extract Pages (`extract_pages`)
- **Category in UI:** `Page Management`
- **UI Access:** Operations Dock > **Pages** > **Extract PDF Pages**, or right-click thumbnail > *Extract*.
- **Sample File:** Multi-page document.
- **Manual Steps:**
  1. Specify page range to extract (e.g., `2-3`).
  2. Choose whether to **Open in new tab** or **Save to file**.
  3. Click **Extract Pages**.
- **Visual Acceptance Criteria:**
  - [ ] A new document is generated containing strictly pages 2 and 3.
  - [ ] Original document remains open and unmodified.
- **Status:** `[ ] PASS` | `[ ] FAIL` | **Notes:** `____________________`

---

## Tool 7: Burst PDF (`burst`)
- **Category in UI:** `Page Management`
- **UI Access:** Operations Dock > **Pages** > **Burst PDF**.
- **Sample File:** Any 3+ page PDF.
- **Manual Steps:**
  1. Click **Burst PDF**.
  2. Select output folder when prompted.
- **Visual Acceptance Criteria:**
  - [ ] Each page is saved as an individual PDF: `page_001.pdf`, `page_002.pdf`, `page_003.pdf`.
  - [ ] Success notification shows total number of single-page files created.
- **Status:** `[ ] PASS` | `[ ] FAIL` | **Notes:** `____________________`

---

## Tool 8: Crop Pages (`crop`)
- **Category in UI:** `Page Management`
- **UI Access:** Operations Dock > **Pages** > **Crop Pages**, or toolbar crop selection tool.
- **Sample File:** Any single page PDF.
- **Manual Steps:**
  1. In the viewer, drag a rectangular crop box over the active page.
  2. Check coordinates in the Crop tool panel (`X`, `Y`, `Width`, `Height`).
  3. Click **Apply Crop**.
- **Visual Acceptance Criteria:**
  - [ ] The page boundaries shrink to the chosen rectangle.
  - [ ] Margins outside the bounding box are trimmed.
- **Status:** `[ ] PASS` | `[ ] FAIL` | **Notes:** `____________________`

---

## Tool 9: Remove Blank Pages (`remove_blank`)
- **Category in UI:** `Page Management`
- **UI Access:** Operations Dock > **Pages** > **Remove Blank Pages**.
- **Sample File:** A document containing at least one blank page.
- **Manual Steps:**
  1. Set sensitivity slider (default 95%).
  2. Click **Remove Blank Pages**.
- **Visual Acceptance Criteria:**
  - [ ] Toast reports: *"Removed N blank pages"*.
  - [ ] Only pages with text/graphics remain; blank pages are purged.
- **Status:** `[ ] PASS` | `[ ] FAIL` | **Notes:** `____________________`

---

## Tool 10: Alternating PDF Merge (`pdf_merge_alternate`)
- **Category in UI:** `Page Management`
- **UI Access:** Operations Dock > **Pages** > **Alternating PDF Merge**.
- **Sample File:** Two 2-page documents (Doc A: pages A1, A2; Doc B: pages B1, B2).
- **Manual Steps:**
  1. Select Doc A as Primary and Doc B as Alternate.
  2. Toggle **Reverse order for Doc B** (useful for duplex-scanned backsides).
  3. Click **Interleave Pages**.
- **Visual Acceptance Criteria:**
  - [ ] The output document orders pages: A1, B1, A2, B2 (or A1, B2, A2, B1 if reversed).
  - [ ] Visual page content alternates predictably.
- **Status:** `[ ] PASS` | `[ ] FAIL` | **Notes:** `____________________`

---

## Tool 11: Split by Document Outline (`pdf_split_by_outline`)
- **Category in UI:** `Page Management`
- **UI Access:** Operations Dock > **Pages** > **Split by Document Outline**.
- **Sample File:** Document with Bookmarks/TOC (`fixtures/test_bookmarks.pdf` or any document outline).
- **Manual Steps:**
  1. Click **Split by Outline**.
  2. Select maximum outline depth (e.g., Level 1).
  3. Click **Execute Split**.
- **Visual Acceptance Criteria:**
  - [ ] A ZIP archive or output directory is created with chapter files named according to bookmarks (e.g. `01_Chapter 1.pdf`, `02_Chapter 2.pdf`).
  - [ ] For documents without bookmarks, gracefully falls back to single-page split without errors.
- **Status:** `[ ] PASS` | `[ ] FAIL` | **Notes:** `____________________`

---

## Tool 12: Split Scanned Book Spine (`pdf_split_scanned_book`)
- **Category in UI:** `Page Management`
- **UI Access:** Operations Dock > **Pages** > **Split Scanned Book Spine**.
- **Sample File:** A 2-up landscape scanned book page.
- **Manual Steps:**
  1. Select split orientation: **Vertical (Left/Right)** or **Horizontal (Top/Bottom)**.
  2. Choose reading order: **Left-to-Right** or **Right-to-Left (Manga/Hebrew)**.
  3. Click **Split Book Spine**.
- **Visual Acceptance Criteria:**
  - [ ] Each landscape two-page spread is cut down the middle into two separate portrait pages.
- **Status:** `[ ] PASS` | `[ ] FAIL` | **Notes:** `____________________`

---

## Tool 13: Auto Upright Orientation (`pdf_auto_rotate`)
- **Category in UI:** `Page Management`
- **UI Access:** Operations Dock > **Pages** > **Auto Upright Orientation**.
- **Sample File:** Document containing upside-down or sideways pages.
- **Manual Steps:**
  1. Click **Detect & Upright Pages**.
  2. Observe progress indicator while text baseline detection runs.
- **Visual Acceptance Criteria:**
  - [ ] Sideways and inverted pages automatically rotate to upright reading orientation.
  - [ ] Already upright pages remain untouched.
- **Status:** `[ ] PASS` | `[ ] FAIL` | **Notes:** `____________________`
