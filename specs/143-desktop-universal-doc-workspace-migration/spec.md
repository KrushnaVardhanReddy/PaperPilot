# Spec 143: Universal Document Workspace & Consumer-Friendly Studio Migration

## Metadata
- **Spec Number**: 143
- **Phase**: Phase 3.4.15 / Phase 10.6 (Desktop Universal Document Workspace)
- **Status**: 📋 In Development
- **Target Branch**: `feat/desktop-universal-doc-workspace-migration`
- **Impacted Apps & Packages**: `apps/desktop` (`apps/desktop/src/lib/components/*`, `apps/desktop/src/lib/state/app.svelte.ts`, `apps/desktop/src/routes/+page.svelte`)
- **Parent Vision**: Phase 10 / Phase 13 ([Universal Document OS](file:///home/krushna/Project/PaperPilot/phases/phase-13-universal-document-os.md))

---

## 1. Overview & Non-Negotiable Directives

### 🚨 Critical Anti-Mocking Rule (MANDATORY)
**This is an actual migration of our real UI, NOT a mocked shell:**
1. **NEVER use hardcoded document counts, static document lists, or fake tool arrays.**
2. **All documents MUST come from `appState.documents` and `appState.documentPaths`.**
3. **All tool executions MUST route through the real `paperpilot_mcp` / Tauri IPC tools via `appState.activeTool` and `OperationsPanel.svelte`.**
4. **All 75+ tools MUST remain 100% accessible and functional.**
5. **Preserve working viewers**: `PdfViewer.svelte` for PDFs, `SpreadsheetViewer.svelte` for `.xlsx` / `.csv` / `.tsv`, and `PdfVisualDiff.svelte` for visual diffing.
6. **Strict Theme & Svelte 5 Compliance**: Use Svelte 5 runes (`$state`, `$derived`, `$props`). Enforce dark theme tokens (`--bg-surface`, `--text-primary`, `--border-color`, `color-scheme: dark`).

---

## 2. Strategic Objectives

Transition the desktop UI from an intimidating, PDF-only administrative form into an intuitive, consumer-friendly **Universal Document Workspace**:
1. **Universal Drop Zone & Empty State**:
   - When no document is open, present a welcoming drop zone indicating support for **PDF, Word (.docx), Excel (.xlsx/.csv), Markdown, and Images**.
   - Show the **Top 8 Everyday Universal Actions** (Compress, Universal Convert, Merge, Sign & Protect, OCR & Extract, Spreadsheet/Data View, Redact & Sanitize, Automate Workflow).
2. **Top Navigation & Global Command Bar (`Ctrl + K`)**:
   - Slim top header with mode switcher: `[ 📄 Studio ] [ 🔗 Automations ]`.
   - Global search input allowing type-ahead filtering of all 75 tools by plain-English names (e.g. typing "excel" or "compress").
3. **Real Document Library with Smart Format Filter Pills**:
   - In `DocumentList.svelte`, dynamically calculate format counts from `appState.documents`:
     - `All Documents (count)`
     - `📄 PDFs (count)`
     - `📊 Spreadsheets / CSV (count)`
     - `📝 Word / Docs (count)`
     - `📷 Scanned / OCR (count)`
   - Selecting a filter pill reactively filters the document list without mocking.
4. **Contextual Active Workspace**:
   - When a document is loaded, automatically render the appropriate viewer (`PdfViewer` for PDF, `SpreadsheetViewer` for XLSX/CSV).
   - Display a sleek floating action pill at the bottom center with the most common actions for that file.
   - The right Operations Panel collapses cleanly via a toggle button, giving maximum screen real estate to the document.

---

## 3. Component Architecture & Migration Plan

### 3.1 `apps/desktop/src/lib/components/workspace/UniversalHomeDashboard.svelte` (New Component)
- Displayed when `appState.documents.length === 0` or when navigating to Home:
  - Header: "PaperPilot: Universal Document OS"
  - Subtitle: "Open, convert, or automate any document — pure Rust, 100% private & on-device."
  - Big Drop Zone: Invokes native file picker supporting all extensions (`.pdf`, `.docx`, `.xlsx`, `.xls`, `.csv`, `.tsv`, `.pptx`, `.md`, `.txt`, `.png`, `.jpg`).
  - Grid of 8 Everyday Action Cards:
    - Each card triggers file selection and pre-selects the corresponding tool in `appState.activeTool`.

### 3.2 `apps/desktop/src/lib/components/ui/DocumentList.svelte` (Upgrade)
- Replace static headers with dynamic format filter pills:
  ```typescript
  let counts = $derived({
    all: appState.documents.length,
    pdf: appState.documents.filter(d => isPdf(d.name)).length,
    sheets: appState.documents.filter(d => isSpreadsheet(d.name)).length,
    word: appState.documents.filter(d => isWord(d.name)).length,
    scanned: appState.documents.filter(d => (d as any).isScanned).length,
  });
  ```
- Renders only the filtered subset reactively based on the active pill.

### 3.3 `apps/desktop/src/lib/components/layout/HeaderNav.svelte` (Upgrade / Creation)
- Top bar with:
  - Branding + Version badge.
  - Mode Switcher: `Studio` (Viewer + Tools) vs `Automations` (Pipeline Studio & Watched Folders).
  - Global `Ctrl + K` Command Bar triggering `GlobalCommandBar.svelte` or `CommandPalette.svelte`.

---

## 4. Acceptance Criteria Checklist

- [ ] Zero Mocking:
  - All document names, icons, sizes, and page counts reflect actual files in `appState.documents`.
  - No dummy folders or hardcoded numbers anywhere in the codebase.
- [ ] Universal File Handling:
  - Opening `.xlsx` / `.csv` renders `SpreadsheetViewer.svelte`.
  - Opening `.pdf` renders `PdfViewer.svelte`.
  - Switching between documents updates the viewer reactively.
- [ ] 75 Tool Accessibility:
  - All 75 tools in `OperationsPanel.svelte` remain fully executable.
  - Global command bar allows searching and executing any tool.
- [ ] Styling & Dark Theme:
  - No white flash, no unstyled select dropdowns, full dark theme harmony (`#0f1015` / `#181922` / `#6366f1`).
- [ ] Test Verification:
  - All existing 117 tests in `apps/desktop` pass with `pnpm --dir apps/desktop test`.
