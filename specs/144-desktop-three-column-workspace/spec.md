# Spec 144: Desktop Three-Column Pro Workspace Architecture (Task 3.4.16)

## Metadata
- **Spec Number**: 144
- **Phase**: Phase 3.4.16 / Phase 10.6
- **Status**: 📋 Active / Dispatched to Jules
- **Target Branch**: `feature/dev`
- **Impacted Apps & Packages**: `apps/desktop` (`apps/desktop/src/routes/+page.svelte`, `apps/desktop/src/lib/components/layout/*`, `apps/desktop/src/lib/state/app.svelte.ts`)
- **Parent Baseline**: `feature/dev` (commit `db9b8f4` with Spec 141 and Spec 142 merged)

---

## 1. 🚨 Non-Negotiable Directives (ZERO-MOCK MANDATE)

> [!CAUTION]
> **MOCKING OF ANY UI IS STRICTLY PROHIBITED.**  
> If you have ANY doubts or ambiguities about how a data flow works, **ASK in the session conversation BEFORE implementing.** Do NOT invent static mock cards, fake document arrays, hardcoded counts, or dummy folder models.

1. **Real Data Stores Only**:
   - Every file name, file icon, size, and page count MUST come dynamically from `appState.documents` and `appState.documentPaths`.
   - Every tool operation MUST route through the existing `allTools` array from `$lib/types/toolList` and execute via the real Tauri IPC / MCP backend.
2. **Preserve All 75+ Pure-Rust Tools**:
   - No tool may be deleted, hidden, or disconnected.
   - All tool parameter inputs and executors in `OperationsPanel.svelte` must remain completely wired up.
3. **Preserve Real Multi-Format Viewers**:
   - `PdfViewer.svelte` for `.pdf` documents.
   - `SpreadsheetViewer.svelte` for `.xlsx`, `.csv`, `.tsv` sheets.
   - `PdfVisualDiff.svelte` for visual diffing.
   - `PipelineCanvas.svelte` for visual DAG workflow building.
4. **Svelte 5 Runes & High-Contrast Dark Theme**:
   - Strictly use `$state`, `$derived`, and `$props`.
   - All controls, dropdowns, inputs, and canvas backgrounds must adhere to the high-contrast dark theme variables (`--bg-surface`, `--text-primary`, `--border-color`, `color-scheme: dark`).

---

## 2. The Three-Column Workspace Architecture

PaperPilot desktop transitions to a clean, pro-grade 3-column desktop layout (inspired by VS Code, Finder, and Apple Pages):

```
┌──────┬──────────────────────────┬─────────────────────────────────────────────────────────────┬──────────────────────────┐
│ COL 1│ COL 2 (Contextual Panel) │ COL 3 (Main Stage / Hero Canvas)                            │ OPTIONAL RIGHT DRAWER    │
│ Slim │                          │                                                             │ (Contextual Tool Params) │
│ Rail │ ── When No File Open ──  │  ┌───────────────────────────────────────────────────────┐  │                          │
│ ~48px│ 📂 Library (Real docs)   │  │                                                       │  │ 🗜️ Compress PDF Params  │
│      │ [All][Scanned][Signed]   │  │                                                       │  │                          │
│ 📁   │ • invoice.pdf (2 pages)  │  │                  DOCUMENT CANVAS                      │  │ Quality slider: [85%]   │
│ 🕒   │ • sales.xlsx (Sheet)     │  │                  (PDF or Sheets)                      │  │ DPI: [150 DPI]          │
│ 🔄   │                          │  │                                                       │  │                          │
│ 🔗   │ ── When Document Open ── │  │                                                       │  │ [ Run Operation ]       │
│ ⚙️   │ 📑 Thumbnails Outline    │  │                                                       │  │                          │
│      │ [Page 1] [Page 2] [Page 3│  └───────────────────────────────────────────────────────┘  │                          │
│      │ (Drag to reorder / jump) │          [ 🔍 Zoom   ↻ Rotate   ✍ Sign   ⚙️ Tools ]          │                          │
└──────┴──────────────────────────┴─────────────────────────────────────────────────────────────┴──────────────────────────┘
```

### Column 1: Slim Activity Rail (~48px wide)
- Slim vertical navigation dock with crisp SVG icons and active glow indicator:
  - 📁 **Files / Library**: Toggles Column 2 to show loaded documents & smart filter pills.
  - 🕒 **Recent Documents**: Shows recently processed files.
  - 🔄 **Universal Converter**: Quick format conversion drawer (Word, PDF, Excel, Markdown).
  - 🔗 **Pipeline Studio**: Switches Main Stage to the visual DAG pipeline canvas.
  - ⚙️ **Settings**: Opens Settings (including Watched Folders daemon from Spec 142).
- Clicking the active icon toggles Column 2 open or collapsed (shortcut: `Ctrl + B`).

### Column 2: Contextual Side Drawer (~240px wide, Collapsible)
- **State A (When no document is active or user browses library)**:
  - Renders `DocumentList.svelte` with the real smart format filter pills (`All`, `Scanned`, `Signed`, `Forms`, `Multi-Page`).
  - Users can select a document, remove files (❌), or reorder files.
- **State B (When a PDF document is open)**:
  - Smoothly displays `PdfThumbnails.svelte` showing live page thumbnail previews.
  - Clicking a thumbnail jumps to that page; dragging reorders pages via existing `handleReorderPage`.
- Collapse toggle button at the top of Column 2 (`‹` icon) to hide it and maximize Column 3.

### Column 3: The Main Stage (75%–85% of screen)
- Pure, focused canvas for the open document:
  - If PDF: Renders `PdfViewer.svelte` with edge-to-edge dark-neutral canvas.
  - If Spreadsheet: Renders `SpreadsheetViewer.svelte`.
  - If Visual Diff: Renders `PdfVisualDiff.svelte`.
  - If Pipeline mode: Renders `PipelineView.svelte` / `PipelineCanvas.svelte`.
- **Floating Action Pill** (at bottom center of document canvas):
  - Common quick actions: `[ 🔍 Zoom In/Out ] [ ↻ Rotate 90° ] [ ✍ Sign / Annotate ] [ ⚙️ Tool Settings ]`.
  - Clicking "Tool Settings" opens the right-hand slide-out drawer for tool parameters.

### Optional Right-Hand Drawer: Operations Inspector (~320px wide, Slide-Out)
- Houses `OperationsPanel.svelte` containing all 75 tool parameters and run buttons.
- Slides out smoothly from the right when an action is selected.
- Easily dismissed via `✕` or clicking outside, keeping the main reading stage clean.

---

## 3. Implementation Steps

1. **Step 1: Activity Rail (`Sidebar.svelte`)**:
   - Slim down `Sidebar.svelte` into a fixed ~48px icon rail with active indicators and tooltips.
2. **Step 2: Contextual Secondary Rail in `+page.svelte`**:
   - If `activeTab === 'documents'` and `selectedDocumentIndex === null`: render `DocumentList.svelte`.
   - If `selectedDocumentIndex !== null` and active document is PDF: render `PdfThumbnails.svelte` inside Column 2.
   - Add collapse/expand toggle button for Column 2 (`Ctrl + B`).
3. **Step 3: Main Stage & Floating Pill in `+page.svelte`**:
   - Ensure the center stage occupies maximum width without forced static columns.
   - Wire the bottom floating action pill to real events:
     - `Zoom`: dispatches `paperpilot:zoom-in` / `paperpilot:zoom-out`.
     - `Rotate`: calls `pdf_rotate` via safeInvoke.
     - `Sign`: activates annotation/signature tool.
     - `Tools`: toggles the right `OperationsPanel.svelte` slide-out drawer.
4. **Step 4: Slide-Out Drawer for `OperationsPanel.svelte`**:
   - Wrap `OperationsPanel.svelte` in an overlay/docked drawer with a prominent close button (`✕`).

---

## 4. Acceptance Criteria Checklist

- [ ] Zero Mocking:
  - All document names, icons, page counts, and filter pills come directly from `appState.documents`.
  - Thumbnails reflect real pages of the loaded PDF.
- [ ] Three-Column Functionality:
  - Column 1 toggles Column 2 open and closed.
  - Column 2 displays documents when idle, and thumbnails when a PDF is open.
  - Column 3 gives full focus to the active document with floating quick actions.
  - Right drawer slides out when tool configuration is requested and closes cleanly.
- [ ] 75 Tool Access:
  - All 75 tools remain 100% accessible and executable via `OperationsPanel.svelte` and global search bar.
- [ ] Tests:
  - All 147 Vitest tests in `apps/desktop` pass with `pnpm --dir apps/desktop test`.
