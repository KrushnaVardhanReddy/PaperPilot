# Manual Desktop UI Testing Guide: Tester 2 (Imposition, Optimization & Rendering)

**Assigned Tester:** Tester 2  
**Total Tools Assigned:** 13 Tools  
**Domain:** File Size Optimization, Document Imposition, Printing Layouts, Stream Deduplication & Visual Rendering

---

## Tool 1: Compress PDF (`compress`)
- **Category in UI:** `Quick Actions` / `Optimize & Repair`
- **UI Access:** Operations Dock > **Optimize** > **Compress PDF** (or top toolbar compression pill).
- **Sample File:** An image-heavy PDF (e.g., > 1 MB).
- **Manual Steps:**
  1. Open the document in the Desktop Viewer.
  2. Open the Compress tool. Select a quality preset: **Screen (72 DPI)**, **eBook (150 DPI)**, or **Print (300 DPI)**.
  3. Toggle **Remove unused objects**.
  4. Click **Compress PDF** and save the file.
- **Visual Acceptance Criteria:**
  - [ ] Toast shows compression statistics (e.g. *"Saved 42% (2.1 MB → 1.2 MB)"*).
  - [ ] Compressed PDF opens cleanly in viewer without blurriness or corruption.
- **Status:** `[ ] PASS` | `[ ] FAIL` | **Notes:** `____________________`

---

## Tool 2: Repair PDF (`repair`)
- **Category in UI:** `Optimize & Repair`
- **UI Access:** Operations Dock > **Optimize** > **Repair PDF**.
- **Sample File:** Any corrupted, truncated, or unindexed PDF.
- **Manual Steps:**
  1. Select or drag the damaged PDF into the Repair tool dialog.
  2. Click **Rebuild XREF & Streams**.
  3. Select output file location.
- **Visual Acceptance Criteria:**
  - [ ] Broken cross-reference tables and corrupt stream dictionaries are rebuilt.
  - [ ] Document opens successfully in the viewer.
- **Status:** `[ ] PASS` | `[ ] FAIL` | **Notes:** `____________________`

---

## Tool 3: Fast Web View / Linearize (`linearize`)
- **Category in UI:** `Optimize & Repair`
- **UI Access:** Operations Dock > **Optimize** > **Fast Web View (Linearize)**.
- **Sample File:** Multi-page PDF.
- **Manual Steps:**
  1. Open PDF and click **Linearize Document**.
  2. Save the output PDF.
- **Visual Acceptance Criteria:**
  - [ ] File size remains compact and document loads page 1 instantly without waiting for whole-file download.
- **Status:** `[ ] PASS` | `[ ] FAIL` | **Notes:** `____________________`

---

## Tool 4: Lossless Stream Deduplication (`pdf_dedup_streams`)
- **Category in UI:** `Optimize & Repair`
- **UI Access:** Operations Dock > **Optimize** > **Lossless Stream Deduplication**.
- **Sample File:** Document containing duplicated logos, repeated background headers, or fonts across many pages.
- **Manual Steps:**
  1. Click **Deduplicate Streams**.
  2. Wait for hash-based duplicate stream identification.
- **Visual Acceptance Criteria:**
  - [ ] Identical image streams and embedded font instances are consolidated into single indirect objects.
  - [ ] Zero visual quality loss while reducing byte size.
- **Status:** `[ ] PASS` | `[ ] FAIL` | **Notes:** `____________________`

---

## Tool 5: Scale Page Dimensions (`pdf_scale_pages`)
- **Category in UI:** `Page Management`
- **UI Access:** Operations Dock > **Pages** > **Scale Page Dimensions**.
- **Sample File:** Any letter or A4 document.
- **Manual Steps:**
  1. Select target scale mode: **Standard Size** (e.g. A4 to US Letter, or Letter to A3) or **Percentage** (e.g. 80%).
  2. Toggle **Scale Content to Fit MediaBox**.
  3. Click **Scale Pages**.
- **Visual Acceptance Criteria:**
  - [ ] Canvas dimensions adjust to the newly chosen standard format.
  - [ ] Page aspect ratio and text positioning scale proportionally without clipping.
- **Status:** `[ ] PASS` | `[ ] FAIL` | **Notes:** `____________________`

---

## Tool 6: N-Up Page Imposition (`pdf_nup`)
- **Category in UI:** `Page Management`
- **UI Access:** Operations Dock > **Pages** > **N-Up Page Imposition**.
- **Sample File:** Multi-page document (4 to 8 pages).
- **Manual Steps:**
  1. Choose layout grid: **2-Up (1x2)**, **4-Up (2x2)**, or **9-Up (3x3)**.
  2. Toggle **Draw Border Around Pages** and set spacing margin (e.g., 5 pt).
  3. Click **Impose Pages**.
- **Visual Acceptance Criteria:**
  - [ ] 4 source pages are tiled cleanly onto a single composite sheet.
  - [ ] Sheet orientation adjusts appropriately (e.g., landscape for 2-up).
- **Status:** `[ ] PASS` | `[ ] FAIL` | **Notes:** `____________________`

---

## Tool 7: Booklet Imposition (`pdf_booklet`)
- **Category in UI:** `Page Management`
- **UI Access:** Operations Dock > **Pages** > **Booklet Imposition**.
- **Sample File:** 8-page document (or any multiple of 4).
- **Manual Steps:**
  1. Select booklet binding type: **Saddle Stitch (Fold in Center)**.
  2. Select sheet size: **Letter Landscape** or **A4 Landscape**.
  3. Click **Generate Booklet**.
- **Visual Acceptance Criteria:**
  - [ ] Page order rearranges into folding pairs (e.g., Sheet 1 contains Page 8 and Page 1; Sheet 2 contains Page 2 and Page 7).
  - [ ] Printing duplex and folding in half creates a sequentially ordered book.
- **Status:** `[ ] PASS` | `[ ] FAIL` | **Notes:** `____________________`

---

## Tool 8: Multi-Sheet Poster Tile Slicer (`pdf_posterize`)
- **Category in UI:** `Page Management`
- **UI Access:** Operations Dock > **Pages** > **Multi-Sheet Poster Tile Slicer**.
- **Sample File:** Single large page or architectural blueprint.
- **Manual Steps:**
  1. Choose target sheet size: **Letter** or **A4**.
  2. Set overlap margin: **0.5 inches** (with cut guides enabled).
  3. Set grid dimension: **2x2 Sheets** (4 pages total) or **3x3 Sheets**.
  4. Click **Slice into Poster Tiles**.
- **Visual Acceptance Criteria:**
  - [ ] Output PDF contains a multi-page grid corresponding to the poster pieces.
  - [ ] Overlap margins show dashed cut lines for physical tape assembly.
- **Status:** `[ ] PASS` | `[ ] FAIL` | **Notes:** `____________________`

---

## Tool 9: Stitch Continuous Canvas (`pdf_stitch_canvas`)
- **Category in UI:** `Page Management`
- **UI Access:** Operations Dock > **Pages** > **Stitch Continuous Canvas**.
- **Sample File:** Multi-page document (3 pages).
- **Manual Steps:**
  1. Select stitching direction: **Vertical (Scroll roll)** or **Horizontal (Panoramic)**.
  2. Set gap spacing: **0 pt**.
  3. Click **Stitch into Single Canvas**.
- **Visual Acceptance Criteria:**
  - [ ] All pages merge into a single, seamless, continuous tall or wide canvas page.
- **Status:** `[ ] PASS` | `[ ] FAIL` | **Notes:** `____________________`

---

## Tool 10: Invert Colors / Dark Mode (`pdf_invert_colors`)
- **Category in UI:** `Edit & Markup` / `Optimize & Repair`
- **UI Access:** Operations Dock > **Edit** > **Invert Colors (Dark Mode)**, or bottom status bar theme switch.
- **Sample File:** Standard white-background PDF with black text.
- **Manual Steps:**
  1. Select mode: **High-Contrast Dark Mode** or **Full Color Inversion**.
  2. Click **Invert Document Colors**.
- **Visual Acceptance Criteria:**
  - [ ] White page backgrounds become deep black/charcoal.
  - [ ] Black text inverts to crisp white/light gray for comfortable nighttime reading.
- **Status:** `[ ] PASS` | `[ ] FAIL` | **Notes:** `____________________`

---

## Tool 11: PDF Portfolio Attachments (`pdf_attachments`)
- **Category in UI:** `Page Management` / `Security & Privacy`
- **UI Access:** Operations Dock > **Pages** > **PDF Portfolio Attachments**, or left sidebar paperclip icon.
- **Sample File:** Any PDF.
- **Manual Steps:**
  1. Open Attachments panel.
  2. Click **Add Attachment** and choose a file (e.g. `notes.txt` or an image).
  3. Verify attachment displays in list with file size.
  4. Click **Extract All** or download icon.
- **Visual Acceptance Criteria:**
  - [ ] Embedded files are listed in the portfolio drawer.
  - [ ] Extracted files match the exact original byte contents.
- **Status:** `[ ] PASS` | `[ ] FAIL` | **Notes:** `____________________`

---

## Tool 12: Compare Documents / Pixel Diff Slider (`compare`)
- **Category in UI:** `Quick Actions` / `Page Management`
- **UI Access:** Operations Dock > **Quick** > **Compare PDFs** (or top menu *Tools > Visual Pixel Diff*).
- **Sample File:** Two slightly different versions of a PDF (e.g. revision v1 and revision v2).
- **Manual Steps:**
  1. Open Doc A in Tab 1 and Doc B in Tab 2.
  2. Launch **Compare PDFs**.
  3. Drag the interactive split slider left and right across the canvas.
  4. Toggle **Highlight Additions / Deletions (Red/Green)**.
- **Visual Acceptance Criteria:**
  - [ ] Interactive split curtain reveals changes between the two documents in real time.
  - [ ] Modified text or repositioned graphics highlight in contrasting colors.
- **Status:** `[ ] PASS` | `[ ] FAIL` | **Notes:** `____________________`

---

## Tool 13: Render Page to Image (`render`)
- **Category in UI:** `Convert & Export`
- **UI Access:** Operations Dock > **Convert** > **Render Page** (or right-click canvas > *Export as PNG*).
- **Sample File:** Any PDF page.
- **Manual Steps:**
  1. Choose resolution: **72 DPI**, **150 DPI**, or **300 DPI (High-Res Print)**.
  2. Select page number (e.g. Page 1).
  3. Click **Render & Export PNG**.
- **Visual Acceptance Criteria:**
  - [ ] Save dialog appears and writes a valid `.png` file.
  - [ ] Opening the image confirms high-resolution rasterization with sharp text and vector curves.
- **Status:** `[ ] PASS` | `[ ] FAIL` | **Notes:** `____________________`
