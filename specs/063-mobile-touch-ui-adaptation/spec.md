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

### Requirement 2: Unified Adaptive Viewport & Platform Architecture (Single Codebase, No UI Duplication)
- In `apps/desktop/src/`:
  - Utilize **Tailwind CSS v4** responsive utility classes (`md:`, `lg:`) and reactive `isMobile` state.
  - **No duplicate components or dual UIs**: Maintain a single codebase where identical tool cards, parameter forms, and canvas viewers adapt responsively.
  - On desktop (`>= 768px`): Wide 3-column studio layout with left sidebar, center document canvas, and docked right operations panel.
  - On mobile (`< 768px`): Single-column clean-room layout with bottom navigation bar and slide-up bottom sheet drawer.

### Requirement 3: Mobile Touch-First Navigation & In-Memory Document Intake
- **In-Memory File Intake on Android**:
  - In `DropZone.svelte` and file pickers, support direct `File` blob streaming into `arrayBuffer()` / `Uint8Array`.
  - Android content URIs (`content://...`) cannot be resolved with desktop `std::fs::metadata` / `std::fs::read`. Use in-memory buffer processing via `process_mobile_buffer` to eliminate path resolution errors.
- On mobile viewports:
  - Replace desktop sidebar with a native **Bottom Navigation Bar**:
    - **📄 Documents**: Active document canvas viewer.
    - **📷 Scan**: Camera Document Scanner capture intake.
    - **⚡ Tools**: Categorized grid of all 50+ operations.
    - **⚙️ Settings**: On-device AI models, theme, and export options.
  - Operations parameter panels:
    - Display the exact same `OperationsPanel.svelte` wrapped as an accessible, swipeable **Bottom Sheet** instead of a wide side-dock.
    - Increase all interactive hit targets to minimum **48px x 48px**.
    - Safe-area inset handling: apply `env(safe-area-inset-top)` and `env(safe-area-inset-bottom)` for iPhone notches and Android gesture bars.

### Requirement 4: Lean Mobile APK Size Optimization
- In `apps/desktop/src-tauri/Cargo.toml` and Android build configuration:
  - Enforce `--no-default-features` on mobile builds to exclude `paperpilot-gateway` (Axum REST server) and `paperpilot-mcp` (JSON-RPC server).
  - Strip DWARF debug symbols on Android builds (remove `keepDebugSymbols` from `build.gradle.kts` and enable `strip = true` in Rust release profile) so compiled APK drops from 1.4GB down to lean ~25-35MB.

### Requirement 5: Canvas Viewer Touch Gestures
- In `apps/desktop/src/lib/components/viewer/`:
  - Touch gestures:
    - Pinch-to-zoom with smooth spring-physics scale clamping.
    - Two-finger pan for navigating zoomed documents.
    - Horizontal swipe for next/previous page navigation.
  - Native file picker trigger:
    - Touch-friendly floating action button (FAB) or prominent header button to pick documents via Tauri dialog.

### Requirement 6: Native Share Sheet & Export Integration
- Integrate native sharing for completed operations:
  - Allow users to send output PDFs directly to the native OS share sheet (WhatsApp, Email, AirDrop, Google Drive, Local Files) via Tauri opener / share plugins.

### Requirement 7: Automated Verification & Responsive Tests
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
