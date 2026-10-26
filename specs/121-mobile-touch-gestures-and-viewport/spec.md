# Spec 121: Mobile Touch Gestures, Viewport Panning & Dual-State Zoom Engine

## 1. Overview
PaperPilot runs natively on Android mobile devices via Tauri 2.
While basic pointer events and pinch detection were scaffolded in `PdfPreview.svelte`, the mobile reading and inspection experience requires standard touch-first UX:
1. **Isolated Viewport Zooming**: The pinch-to-zoom transform must apply strictly to the document preview canvas (`.preview-content`), leaving the top navigation header and bottom controls fixed in place.
2. **Double-Tap to Reset / Fit**: A double-tap gesture (<300ms between taps) toggles smoothly between 1.0x (fit/reset) and 2.5x zoom.
3. **Dual-Mode Single-Finger Touch**:
   - When zoomed out (`currentScale <= 1.05`): Single-finger horizontal swipe navigates between pages (with optional subtle haptic vibration `navigator.vibrate?.(10)`).
   - When zoomed in (`currentScale > 1.05`): Single-finger drag pans the document viewport in 2D (`panX`, `panY`), bounded to prevent the document from disappearing off-screen.
4. **Mobile Bottom Navigation Clearance**:
   - The document preview grid must have bottom scroll padding (`padding-bottom: calc(90px + env(safe-area-inset-bottom, 16px))`) so thumbnails and actions are never blocked by the floating island dock.

---

## 2. Architecture & State (`PdfPreview.svelte`)

### Touch State Management
```typescript
let currentScale = $state(1.0);
let panX = $state(0);
let panY = $state(0);
let lastTapTime = $state(0);
let isPanning = $state(false);
let startPanCoords = $state<{ x: number; y: number } | null>(null);
```

### Gestures:
- **Pinch**: When `activePointers.size === 2`, compute Euclidean distance delta and scale `currentScale` clamped between `1.0` and `3.5`.
- **Double Tap**: On pointer down with 1 pointer, if `Date.now() - lastTapTime < 300`:
  - If `currentScale > 1.2`, smoothly animate `currentScale = 1.0`, `panX = 0`, `panY = 0`.
  - If `currentScale <= 1.2`, zoom into tapped point (`currentScale = 2.5`).
- **Pan vs Swipe**:
  - If `currentScale > 1.05`: track `(deltaX, deltaY)` to update `(panX, panY)`.
  - If `currentScale <= 1.05`: track horizontal drag for page transition (`Swiped Left` / `Swiped Right`).

---

## 3. Automated Vitest Tests (`PdfPreviewGestures.test.ts`)
- Test double tap triggers zoom toggle (1.0x -> 2.5x -> 1.0x).
- Test pinch gesture adjusts `currentScale`.
- Test panning updates translate coordinates when zoomed in.
- Test swipe triggers page transition when zoomed out.
