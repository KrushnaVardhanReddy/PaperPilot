# Task 3.4.16: Desktop Three-Column Pro Workspace Architecture

## Objectives
Implement Spec 144 (`specs/144-desktop-three-column-workspace/spec.md`) in `apps/desktop`.
Refactor the desktop application into a modern, pro-grade 3-column architecture (Activity Rail -> Contextual Drawer -> Main Stage Hero Canvas) with zero mocking and full preservation of all 75+ tools and viewers.

## Subtasks
- [ ] **Task 144.1: Activity Rail Refactoring (`Sidebar.svelte`)**
  - Refactor `Sidebar.svelte` into a slim ~48px-wide vertical activity bar with clean icons, tooltips, and active pills.
  - Clicking the active icon toggles Column 2 (Contextual Drawer) open or collapsed.
- [ ] **Task 144.2: Contextual Column 2 in `+page.svelte`**
  - Implement dynamic Column 2:
    - When `selectedDocumentIndex === null`: render `DocumentList.svelte` with real smart filter pills (`All`, `Scanned`, `Signed`, `Forms`, `Multi-Page`).
    - When a PDF is active: smoothly render `PdfThumbnails.svelte` for fast thumbnail navigation and drag-and-drop page reordering.
  - Add collapse/expand toggle button for Column 2 (`‹` icon and `Ctrl + B`).
- [ ] **Task 144.3: Main Stage Hero Canvas & Floating Action Pill**
  - Ensure Column 3 occupies full screen space with edge-to-edge dark-neutral canvas.
  - Implement floating frosted-glass action pill at the bottom center of the document (`Zoom`, `Rotate`, `Sign`, `Tool Settings`).
  - Wire actions to real events and methods.
- [ ] **Task 144.4: Slide-Out Operations Drawer**
  - Refactor the right-hand `OperationsPanel.svelte` container into an expandable slide-out drawer with close button (`✕`).
  - Ensure all 75 tools remain 100% accessible and executable.
- [ ] **Task 144.5: Tests & Zero-Mock Verification**
  - Verify all 147 Vitest tests in `apps/desktop` pass with `pnpm --dir apps/desktop test`.
  - Verify that no static mock data or hardcoded arrays exist in any updated file.
