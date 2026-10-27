# Spec 124: Mobile Web / Browser Sandbox Responsive Overhaul

## 1. Overview & Problem Statement
On mobile devices (specifically tested and reported on Firefox for Android by user `mkl` on Hacker News), the tool parameter upload box and configuration dialogs on `usepaperpilot.com` (`apps/web/src/components/HeroPlayground.svelte`) are cramped. Users can only see a small portion of the interface at a time, forcing awkward scrolling to access controls and upload buttons.

## 2. Requirements & Objectives
1. **Full-Width Drawer / Modal on Mobile (`< 640px`)**:
   - Instead of a fixed-height scrollbox inside the hero section, expand active tool parameter panels into a bottom-sheet drawer or full-width fluid view on mobile viewports.
2. **Touch-Optimized Controls**:
   - Enforce a minimum 44px touch target on all buttons, sliders, file dropzones, and select inputs.
3. **Responsive Parameter Controls**:
   - Wrap parameter inputs (ranges, text boxes, radio groups) into single-column vertical flows on screens `<= 640px`.
4. **Browser Verification**:
   - Validate layout on mobile dimensions (375x667, 412x915) across Chrome and Firefox mobile user agents.
