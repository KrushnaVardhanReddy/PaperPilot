# Spec 050: Multi-Format Document Workspace Search (`Ctrl+F` Across PDF & Office)

## Overview
PaperPilot supports opening and processing both PDF files and Microsoft Office documents (`.docx`, `.xlsx`, `.pptx`, `.doc`, `.xls`, `.ppt`).
Currently, the search bar (`PdfSearchBar.svelte`) only searches the active PDF document rendered via PDF.js. When users have multiple documents open in tabs (including Office spreadsheets and Word docs), there is no unified workspace-wide search:
1. Users cannot search across all open tabs at once.
2. Office documents (`.docx`, `.xlsx`, `.pptx`) lack client-side in-memory text search.
3. Users cannot see snippet previews with match occurrences or jump directly to the matched document/page.

This specification details **Phase 5.9.22**:
A unified **Multi-Format Workspace Search Engine** that scans all loaded PDF and Office documents in the desktop workspace, displaying match counts, sentence snippet context, and one-click tab navigation.

---

## 1. Technical Architecture & Engine

### 1.1 Multi-Format Text Extraction Service (`workspaceSearch.ts`)
Create `apps/desktop/src/lib/services/workspaceSearch.ts`:
- **For PDF Documents**:
  - Uses PDF.js text layer `getTextContent()` for each page.
- **For Office Documents (`.docx`, `.xlsx`, `.pptx`)**:
  - Uses backend IPC command `extract_document_text` or `office_extract_text` via `safeInvoke`.
  - Caches extracted text per document index so subsequent keystrokes run in $<5\text{ms}$.

### 1.2 Search Result Model
```typescript
export interface SearchSnippetMatch {
  docIndex: number;
  docName: string;
  docType: 'pdf' | 'docx' | 'xlsx' | 'pptx';
  location: string; // e.g. "Page 3" or "Sheet 'Q3 Revenue', Row 14"
  pageOrSheetNum: number;
  snippet: string;   // Sentence snippet with highlight markers
  matchIndex: number;
}

export interface WorkspaceSearchResult {
  query: string;
  totalMatches: number;
  matchesByDoc: {
    docIndex: number;
    docName: string;
    matches: SearchSnippetMatch[];
  }[];
}
```

---

## 2. Desktop UI Integration

### 2.1 Workspace Search Modal / Drawer (`WorkspaceSearchModal.svelte`)
1. **Shortcut Activation**:
   - `Ctrl+Shift+F` (or `Cmd+Shift+F`) opens the Workspace Search palette.
   - `Ctrl+F` in the viewer defaults to searching the active document with a toggle: `[This Document] | [All Open Documents]`.
2. **Search Input & Filters**:
   - Fast debounced input with match count pill (e.g. `14 matches across 3 files`).
   - Filter chips: `All Files`, `PDFs`, `Word (.docx)`, `Excel (.xlsx)`.
   - Match options: Match Case (`Aa`), Whole Word (`\b`).
3. **Snippet Results View**:
   - Displays grouped results by document tab.
   - Shows sentence context with highlighted match text (`...quarterly <mark>revenue</mark> exceeded projections...`).
   - Clicking a snippet jumps directly to that document tab (`appState.selectDocument(docIndex)`) and navigates to the target page or sheet.

---

## 3. Quality & Acceptance Criteria
1. Fast debounce execution ($<50\text{ms}$) across up to 8 open tabs.
2. In-memory caching ensures re-searching same files is instantaneous.
3. `pnpm --filter desktop check` passes with 0 errors.
4. Vitest unit tests added for `workspaceSearch.test.ts`.
5. Works on Linux, macOS, and Windows.
