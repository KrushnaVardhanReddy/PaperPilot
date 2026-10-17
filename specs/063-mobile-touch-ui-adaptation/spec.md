# Spec 063: Mobile Touch UI, Camera Scanner with Auto-OCR & Adaptive Layout (`apps/desktop`)

## 1. Overview & Objective
With Tauri 2, `apps/desktop` serves as the universal client across desktop (Windows, macOS, Linux) and mobile (Android, iOS). 

For a true flagship mobile document experience, PaperPilot delivers a full **"Snap, OCR & Convert to PDF"** mobile workflow:
1. **Camera Scanner with Auto-OCR**: Capture one or multiple pages/receipts via device camera, automatically run on-device pure-Rust OCR (`rusto-rs` PaddleOCR v4 / `ocrs`), synthesize a searchable PDF with invisible selectable text overlay via `images_to_pdf` + `ocr`, and load it directly into the viewer.
2. **Mobile Touch-First Navigation**: Bottom navigation bar (Documents, Scan, Tools, Settings), swipeable bottom sheets for tool parameters, safe-area inset protection.
3. **Canvas Gestures**: Pinch-to-zoom, two-finger pan, horizontal page-turn swipe.
4. **Native Share Sheet**: Export output PDFs directly to WhatsApp, Mail, AirDrop, Google Drive, or Files.

---

## 2. Requirements & Deliverables

### Requirement 1: Camera Scanner with On-Device Auto-OCR ("Scan & Searchable PDF")
- In `apps/desktop`:
  - Camera & Storage permissions:
    - Android: `android.permission.CAMERA` in `gen/android/app/src/main/AndroidManifest.xml`.
    - iOS: `NSCameraUsageDescription` ("PaperPilot uses your camera to scan documents and run on-device OCR").
  - Camera Capture UI:
    - Prominent **"📷 Scan Document"** button in Mobile Bottom Nav and Studio Home.
    - Multi-shot intake: Capture sequential camera shots (Page 1, Page 2, Page 3), preview thumbnails in a bottom carousel, reorder or retake blurry shots.
  - Auto-OCR Toggle & Execution:
    - Checkbox / Switch: **"Make Searchable with OCR"** (enabled by default).
    - Pipeline Execution:
      1. Converts captured image buffers into PDF pages via `images_to_pdf`.
      2. If OCR is enabled and on-device models are present, runs `pdf_ocr` on the synthesized pages to embed invisible selectable text and exact bounding box text layers.
      3. If OCR models are not yet downloaded, seamlessly synthesizes the visual PDF and displays a non-blocking toast prompt to download the offline OCR feature pack.
    - Immediately opens the resulting searchable PDF in the canvas viewer with searchable text highlighting.

### Requirement 2: Adaptive Viewport & Platform Detection
- In `apps/desktop/src/lib/`:
  - Detect mobile context via `@tauri-apps/api/core` (`isMobile`) or viewport media queries (`max-width: 768px`).
  - Automatically switch layout modes between **Desktop Workspace** and **Mobile Handheld**.

### Requirement 3: Mobile Touch-First Navigation
- On mobile viewports:
  - Replace the desktop sidebar with a native **Bottom Navigation Bar**:
    - **📄 Documents**: Active document canvas viewer.
    - **📷 Scan**: Camera Document Scanner capture intake.
    - **⚡ Tools**: Categorized grid of all 50+ operations.
    - **⚙️ Settings**: On-device AI models, theme, and export options.
  - Operations parameter panels:
    - Display as an accessible, swipeable **Bottom Sheet** instead of a wide side-dock.
    - Increase all interactive hit targets to minimum **48px x 48px**.
    - Safe-area inset handling: apply `env(safe-area-inset-top)` and `env(safe-area-inset-bottom)` for iPhone notches and Android gesture bars.

### Requirement 4: Canvas Viewer Touch Gestures
- In `apps/desktop/src/lib/components/viewer/`:
  - Touch gestures:
    - Pinch-to-zoom with smooth spring-physics scale clamping.
    - Two-finger pan for navigating zoomed documents.
    - Horizontal swipe for next/previous page navigation.
  - Native file picker trigger:
    - Touch-friendly floating action button (FAB) or prominent header button to pick documents via Tauri dialog.

### Requirement 5: Native Share Sheet & Export Integration
- Integrate native sharing for completed operations:
  - Allow users to send output PDFs directly to the native OS share sheet (WhatsApp, Email, AirDrop, Google Drive, Local Files) via Tauri opener / share plugins.

### Requirement 6: Automated Verification & Responsive Tests
- Add responsive layout & camera scan test in `apps/desktop`:
  - Verify layout renders without horizontal overflow or clipped controls on 375px (iPhone SE), 390px (iPhone 14/15), and 412px (Pixel 7).
  - Test simulated multi-image capture piping into `images_to_pdf` with OCR text overlay verification.
- Generate report: `reports/MOBILE_TOUCH_UI_ADAPTATION_REPORT.md`.

---

## 3. Success Criteria
- [ ] Camera scanner captures single & multi-page receipts/documents, performs auto-OCR, and synthesizes clean, searchable PDF.
- [ ] Text in scanned PDF is selectable and searchable using on-device OCR models.
- [ ] Responsive navigation switches to bottom bar cleanly on screen widths `< 768px` or when `isMobile` is true.
- [ ] All touch targets meet accessibility standards (≥48px).
- [ ] Canvas viewer supports pinch-to-zoom and swipe page turns.
- [ ] No regression on existing desktop multi-tab workspace or tools panel.
