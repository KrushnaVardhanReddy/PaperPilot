# Spec 063: Mobile Touch UI & Adaptive Layout for Desktop/Mobile (`apps/desktop`)

## 1. Overview & Objective
With Tauri 2, `apps/desktop` serves as the universal client across desktop (Windows, macOS, Linux) and mobile (Android, iOS). To provide a premier experience on touchscreens and small handheld displays, the Svelte 5 frontend must adapt seamlessly to mobile viewports.

This specification establishes the adaptive touch-native mobile layout, gesture controls, bottom-sheet parameter dialogs, and native OS sharing.

---

## 2. Requirements & Deliverables

### Requirement 1: Adaptive Viewport & Platform Detection
- In `apps/desktop/src/lib/`:
  - Detect mobile context via `@tauri-apps/api/core` (`isMobile`) or viewport media queries (`max-width: 768px`).
  - Automatically switch layout modes between **Desktop Workspace** and **Mobile Handheld**.

### Requirement 2: Mobile Touch-First Navigation
- On mobile viewports:
  - Replace the desktop sidebar with a native **Bottom Navigation Bar**:
    - **Tools**: Grid of core operations with category tabs.
    - **Viewer**: Full-screen PDF/Document canvas viewer.
    - **History**: Recent jobs and exports.
    - **Settings**: Local AI models and preferences.
  - Operations parameter panels:
    - Display as an accessible, swipeable **Bottom Sheet** instead of a wide side-dock.
    - Increase all interactive hit targets to minimum **48px x 48px**.
    - Safe-area inset handling: apply `env(safe-area-inset-top)` and `env(safe-area-inset-bottom)` for iPhone notches and Android gesture bars.

### Requirement 3: Canvas Viewer Touch Gestures
- In `apps/desktop/src/lib/components/viewer/`:
  - Touch gestures:
    - Pinch-to-zoom with smooth spring-physics scale clamping.
    - Two-finger pan for navigating zoomed documents.
    - Horizontal swipe for next/previous page navigation.
  - Native file picker trigger:
    - Touch-friendly floating action button (FAB) or prominent header button to pick documents via Tauri dialog.

### Requirement 4: Native Share Sheet & Export Integration
- Integrate native sharing for completed operations:
  - Allow users to send output PDFs directly to the native OS share sheet (WhatsApp, Email, AirDrop, Google Drive, Local Files) via Tauri opener / share plugins.

### Requirement 5: Automated Verification & Responsive Tests
- Add responsive layout test in `apps/desktop`:
  - Verify layout renders without horizontal overflow or clipped controls on 375px (iPhone SE), 390px (iPhone 14/15), and 412px (Pixel 7).
- Generate report: `reports/MOBILE_TOUCH_UI_ADAPTATION_REPORT.md`.

---

## 3. Success Criteria
- [ ] Responsive navigation switches to bottom bar cleanly on screen widths `< 768px` or when `isMobile` is true.
- [ ] All touch targets meet accessibility standards (≥48px).
- [ ] Canvas viewer supports pinch-to-zoom and swipe page turns.
- [ ] No regression on existing desktop multi-tab workspace or tools panel.
