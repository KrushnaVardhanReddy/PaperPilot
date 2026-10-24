# Spec 104: Web WASM Portal Parity & Categorized Navigation (`apps/web`)

## 1. Background & Executive Summary
PaperPilot's client-side WebAssembly engine (`paperpilot-wasm`) supports over 30+ pure-Rust operations running 100% in the user's browser with zero server uploads.

Currently, the web application in `apps/web` (`OperationsView.svelte`) displays a flat list of 12 core operations. The latest operations—such as N-Up Imposition (`pdf_nup`), Booklet Imposition (`pdf_booklet`), Scale Dimensions (`pdf_scale_pages`), Auto Upright (`pdf_auto_rotate`), Image Stamp (`pdf_stamp`), Stationery Overlay (`pdf_overlay_underlay`), Digital Signature Validation (`pdf_validate_signatures`), and Signature Removal (`pdf_remove_signatures`)—are already compiled into `paperpilot-wasm`, but are not yet exposed to users in the web interface.

**Spec 104** reorganizes the web portal into clean, discoverable functional categories and implements dynamic parameter input panels and structured output viewers for all WASM-supported tools.

---

## 2. Information Architecture & Category Taxonomy

The Web Portal will organize all tools into 5 primary categories with a top-level category filter pill bar:

| Category ID | Display Name | Icon | Included WASM Operations |
|---|---|:---:|---|
| `pages` | **Page Operations & Imposition** | 📑 | `merge`, `split`, `rotate`, `delete_pages`, `extract_pages`, `reorder_pages`, `crop`, `pdf_nup`, `pdf_booklet`, `pdf_scale_pages`, `pdf_auto_rotate` |
| `edit` | **Edit & Stationery** | 🎨 | `watermark`, `header_footer`, `page_numbers`, `pdf_stamp`, `pdf_overlay_underlay`, `flatten`, `set_metadata` |
| `optimize` | **Optimize & Inspect** | 🗜️ | `compress`, `pdf_hash`, `pdf_info` |
| `security` | **Security & Forensics** | 🔒 | `encrypt`, `decrypt`, `sanitize`, `pii_scan`, `pdf_validate_signatures`, `pdf_remove_signatures` |
| `convert` | **Convert & Extract** | 🔁 | `images_to_pdf`, `extract_images`, `extract_text`, `render_page`, `json_to_pdf`, `pdf_to_docx`, `ocr` |

---

## 3. User Input & Parameter Specification by Tool

Each tool presents a clean configuration card above or below the file dropzone tailored to its parameters:

### 3.1 Imposition & Layout Tools
1. **N-Up Imposition (`pdf_nup`)**:
   - `file`: Primary PDF upload.
   - `grid`: Segmented buttons (`2-up`, `4-up`, `9-up`, `16-up`).
   - `ordering`: Segmented buttons (`Horizontal`, `Vertical`).
   - `border`: Checkbox (toggle hairline page borders).
   - `margin_pt`: Number input (default `18`).
2. **Booklet Imposition (`pdf_booklet`)**:
   - `file`: Primary PDF upload.
   - `page_size`: Dropdown (`A4`, `Letter`, `Keep Original`).
   - `duplex`: Checkbox (Generate fold-ready 2-sided booklet).
3. **Scale Pages (`pdf_scale_pages`)**:
   - `file`: Primary PDF upload.
   - `target_format`: Dropdown (`A4`, `Letter`, `Legal`, `A3`, `Custom Percentage`).
   - `custom_percentage`: Number input (if percentage selected, 50% - 200%).
4. **Auto-Rotate Upright (`pdf_auto_rotate`)**:
   - `file`: Primary PDF upload.
   - `target_orientation`: Segmented toggle (`Portrait`, `Landscape`).
   - `pages`: Text input (`all` or page range like `1-5`).

### 3.2 Stationery & Stamping Tools
1. **Image & Logo Stamp (`pdf_stamp`)**:
   - `file`: Primary PDF upload.
   - `stamp_image`: Secondary dropzone / file picker (accepts `.png`, `.jpg`, `.svg`).
   - `position`: Visual 9-point grid picker (`top-left`, `top-center`, `top-right`, `center`, `bottom-right`, etc.).
   - `opacity`: Slider (0.1 to 1.0, default 0.85).
   - `scale`: Slider (0.1 to 1.0, default 0.3).
2. **Overlay & Underlay Stationery (`pdf_overlay_underlay`)**:
   - `file`: Primary PDF upload.
   - `stationery_file`: Secondary dropzone for background/letterhead PDF.
   - `layer`: Segmented toggle (`Underlay (Background)`, `Overlay (Foreground)`).
   - `opacity`: Slider (0.1 to 1.0).

### 3.3 Security & Forensic Tools
1. **Validate Digital Signatures (`pdf_validate_signatures`)**:
   - `file`: Signed PDF upload.
   - `baseline_file`: Optional baseline reference PDF upload (for pre-sign tamper detection).
   - Output: Interactive `<AuditReportViewer />` rendering:
     - Signer names, certificate dates, validity badges (green / red).
     - Baseline integrity verdict (`UNTOUCHED_JUST_SIGNED` vs `CONTENT_ALTERED_PRIOR_TO_SIGNATURE`).
     - Altered pages indicator and diff notes.
2. **Remove Signatures (`pdf_remove_signatures`)**:
   - `file`: Signed PDF upload.
   - Output: Downloads clean unlocked PDF.
3. **Deep Sanitizer (`sanitize`)**:
   - `file`: Primary PDF upload.
   - `strip_javascript`: Checkbox (default `true`).
   - `strip_links`: Checkbox (default `false`).
   - `strip_metadata`: Checkbox (default `true`).
4. **PII Scanner (`pii_scan`)**:
   - `file`: Primary PDF upload.
   - Output: Highlighting identified SSNs, credit card numbers, email addresses, and phone numbers in an audit card with one-click JSON copy.

---

## 4. Technical Architecture: Web Worker Bridge

In `apps/web/src/lib/wasm/`:

1. **`pdfWorker.ts`**:
   - Add switch arms for all newly registered tools calling `WasmPdfEngine[method]`.
   - Pass JSON option strings where required by `paperpilot-wasm` bindings.
2. **`index.ts` (`WasmPdfClient`)**:
   - Add TypeScript signature methods returning `Promise<Uint8Array>` (for PDF output) or `Promise<string>` (for JSON audit reports).
   - Convert binary parameter inputs (such as secondary stamp images or baseline documents) to `Uint8Array`.

---

## 5. Implementation Phases
- **Task 104A**: Web Worker Bridge Expansion & Category Navigation Bar.
- **Task 104B**: Tool Parameter Panels (`pdf_nup`, `pdf_booklet`, `pdf_scale_pages`, `pdf_stamp`, `pdf_overlay_underlay`, `pdf_validate_signatures`, `pdf_remove_signatures`).
- **Task 104C**: Structured Audit Viewer & Playwright E2E Verification.
