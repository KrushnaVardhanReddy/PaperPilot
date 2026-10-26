# Task 121A: Implement Mobile Touch Gestures, Viewport Panning & Dual-State Zoom Engine

## Status
Ready for Implementation.

## Description
Implement isolated canvas zooming, 2D viewport panning, double-tap toggle, and safe-area dock clearance in `PdfPreview.svelte` per `specs/121-mobile-touch-gestures-and-viewport/spec.md`.

## Deliverables
1. **Component Update**: `apps/desktop/src/lib/components/ui/PdfPreview.svelte`:
   - Isolate transform `transform: translate({panX}px, {panY}px) scale({currentScale})` exclusively to `.preview-content` (or an inner wrapper), keeping the header fixed at the top.
   - Add double-tap detection (<300ms) with animated toggle between `1.0x` and `2.5x` (resetting pan coordinates `panX = 0, panY = 0` on zoom reset).
   - In single-pointer move mode:
     - When zoomed in (`currentScale > 1.05`), translate viewport `(panX, panY)` with bounding clamps.
     - When zoomed out (`currentScale <= 1.05`), detect horizontal swipe (>80px) and trigger page transition toasts (`Swiped Left` / `Swiped Right`).
   - Add bottom dock clearance padding (`padding-bottom: calc(90px + env(safe-area-inset-bottom, 16px))`) to `.thumbnails-grid`.
2. **Automated Vitest Tests**: `apps/desktop/src/lib/components/ui/PdfPreviewGestures.test.ts`:
   - Expand test suite to assert double-tap toggling zoom scale.
   - Assert single-pointer drag updates pan translation when zoomed in (`currentScale > 1.05`).
   - Assert single-pointer drag triggers swipe action when zoomed out.
   - Verify tests pass with `pnpm --dir apps/desktop test`.
3. **Documentation**:
   - Update `PHASES.md` and `HANDOFF.md` tracking Task 121A.
