# Spec 181: Desktop UI Polish (3-Pane Document-First Layout)

## Overview
Polish the PaperPilot Desktop UI (`apps/desktop`) with a clean, document-first 3-pane layout that reduces visual clutter while keeping all existing functionality 100% intact.

## Requirements

### 1. Top Bar Mode Switcher
- Add a clean **Mode Switcher** in the top bar / `Titlebar.svelte` (or top workspace bar):
  `All tools | Read | Edit | Convert | AI & RAG`
- Selecting a mode filters/organizes tools cleanly without clutter (e.g., via `appState` or a mode state passed to `OperationsPanel` / `GlobalCommandBar` / `PdfToolbar`).
- Default mode is `All tools`.

### 2. Smart Context Banner
- Display a compact **Smart Context Banner** above the PDF/document canvas when a document is loaded.
- Shows document context (e.g. filename, page count, current page) and quick actions (e.g. Summarize / Ask AI, Convert, Annotate, Sign).

### 3. Floating Canvas Mini-Toolbar
- Display a **Floating Canvas Mini-Toolbar** at the bottom-center of the PDF canvas when a PDF is open in the viewer.
- Provides quick tool switching: **Select**, **Hand**, **Annotate / Highlight**, and **Sign / Draw**, wired to the existing annotation/signature/viewer states.

### 4. Right-Side Inspector & Zoom Rail
- Polish `ViewerRightPanel.svelte` with a compact **Inspector & Zoom Rail** (compact icon rail for Annotations / Info / Chat + zoom & page controls when collapsed or alongside the panel), keeping existing resize, collapse, and tab functionality 100% intact.

## Task Breakdown
- [ ] Create `specs/181-desktop-ui-polish/spec.md`
- [ ] Add `uiMode` (`'all' | 'read' | 'edit' | 'convert' | 'ai'`) to `appState` or local state and wire Mode Switcher in `Titlebar.svelte`
- [ ] Add Smart Context Banner above the canvas in `+page.svelte` when a document is loaded
- [ ] Add Floating Canvas Mini-Toolbar in `+page.svelte` (or `PdfViewer.svelte`) when a PDF is open
- [ ] Polish `ViewerRightPanel.svelte` with compact icon rail + zoom/page controls
- [ ] Verify build (`vite build`) and tests (`vitest` if available)
- [ ] Update knowledge graph (`graphify-rs build`), commit, and push to `feature/desktop-ui-polish`
