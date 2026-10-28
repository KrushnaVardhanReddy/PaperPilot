# Spec 140: Desktop V2 Workspace UX Refresh

**Status:** 📋 Planned (Ready for implementation after Task 3.4.11 merges)
**Phase:** Phase 3.4.13 (Desktop & Mobile Application — Operations UI)
**Task ID:** 3.4.13
**Parent:** [Phase 3 — Desktop & Mobile App](../../phases/phase-03-desktop-mobile-app.md)
**Blocked By:** Spec 138 (Task 3.4.11 — Conditional Branching UI)
**Related Specs:**
- [Spec 139: Hot Folder Watcher Daemon](../139-hot-folder-watcher-daemon/spec.md)
- [Spec 138: Desktop Conditional Branching UI](../138-desktop-conditional-branching-ui/spec.md)

---

## 1. Problem Statement

PaperPilot Desktop currently uses a **tab-switching navigation model**: users click between a Documents tab, Pipeline tab, API tab, and Settings tab — each replacing the entire workspace area. While functional, this creates friction:

- Users must **leave** the document they're reading to apply an operation (navigate to Tools tab, configure, run, come back).
- Background automation (Watched Folders, job progress) is **invisible** while working in the viewer.
- Smart document classification (via on-device SLM) exists in the backend but is **never surfaced in the UI**.
- The app feels like a utility tool rather than a professional document workspace.

This spec defines the V2 Workspace redesign that turns PaperPilot Desktop into a **premium, unified document workspace** — keeping the document, tools, and automation context simultaneously visible.

---

## 2. Design Mockup

![Desktop V2 Workspace Mockup](./mockup_v2.jpg)

*Three-column integrated workspace. Left: Library & Automations. Center: Focused document canvas. Right: Contextual AI tool dock.*

---

## 3. Goals & Non-Goals

### Goals
- **3-Pane Integrated Layout**: Library sidebar + document canvas + tool dock visible simultaneously.
- **Watched Folders in Sidebar**: Surface active `paperpilot watch` daemons (Task 4.7.1) as a visible "WATCHED FOLDERS" section with live green `Watching` pill badges.
- **AI Recommendation Dock**: Right panel uses on-device SLM classification to suggest the most relevant tools for the currently open document at the top of the tool list.
- **Frosted-Glass Floating Toolbar**: Replace static toolbar with a centered floating pill above the document canvas with annotation, zoom, and navigation tools.
- **Document Status Bar**: Thin bottom bar showing `Page X of Y · PDF/A · X.X MB · Modified DATE`.
- **Smart Categories with Count Badges**: Sidebar auto-groups loaded documents by type (Contracts, Invoices, Personnel Records, Tax Forms) with document counts and `To Review` amber badge.
- **Shared With You Section**: Show files recently shared by collaborators (populated from the gateway API).

### Non-Goals
- Cloud storage or remote file synchronization (Community tier — local only).
- Replacing the Pipeline Builder tab (it remains as a dedicated tab for complex multi-step flows).
- Enterprise SAML/SCIM authentication UI (reserved for PaperPilot-Enterprise).

---

## 4. Detailed Component Architecture

### 4.1 Left Sidebar: Library & Automations (240px)

**Section: RECENT**
- Compact list of the last 5–10 opened documents.
- Each row: `[FileTypeColorIcon] [FileName truncated] [Date | X pages]`
- File type icons: Blue for contracts/PDFs, green for invoices, red for scanned, orange for spreadsheets.
- Click selects document in center canvas.

**Section: SMART CATEGORIES**
- Populated by the on-device SLM (`paperpilot-nlp` classifier).
- Each row: `[Category Label] [Count badge]` — e.g. `Contracts  470`
- Special row: `To Review` with a bright **amber badge** showing count of documents needing attention (flagged by SLM as unsigned, unarchived, or anomalous).
- Click filters the center canvas to show only documents in that category.

**Section: WATCHED FOLDERS** *(requires Task 4.7.1)*
- Section header has a green animated pulse dot when any folder is actively watching.
- Each folder row: `[FolderIcon] [~/path]  [Watching]` — the `Watching` pill is green and subtle.
- Click opens a quick popover showing: folder path, recipe in use, last processed file, and a "Stop Watching" button.
- `[+]` button in header opens folder picker to add a new watched folder and link it to a `.ppflow` recipe.

**Section: SHARED WITH YOU**
- Compact rows with collaborator avatar circle (initials fallback).
- `[Avatar] [FileName] [Sender Name]`
- Populated from gateway REST API (`GET /v1/shared`).

---

### 4.2 Center Canvas: Focused Document Viewer

**Floating Pill Toolbar** (frosted glassmorphism, `backdrop-filter: blur(12px)`):
```
[ ← ] [ 2 / 7 ] [ → ]  |  [ − ] 87% [ + ]  |  🔍  |  ✏️ 🔆 💬  |  🔖  📤  ···
```
- Positioned: `position: sticky; top: 12px; z-index: 10; margin: auto;`
- Page navigation, zoom slider, search, annotation tools (pencil, highlight, comment), bookmark, share, overflow menu.
- Auto-hides annotation controls when no document is open.

**Document Canvas**:
- Warm white background (`#F9F7F5`) with `box-shadow: 0 8px 40px rgba(0,0,0,0.35)`.
- Generous padding (48px horizontal, 24px vertical).
- Renders existing `PdfViewer.svelte` / `SpreadsheetViewer.svelte` — no changes needed to rendering logic.
- Page thumbnail strip remains optionally collapsible on the left sub-panel (toggle via toolbar icon).

**Status Bar** (24px thin bar at bottom of center column):
```
Page 2 of 7  ·  PDF/A-2B  ·  3.8 MB  ·  Encrypted  ·  Modified Oct 28, 2026
```
- Reads from existing `pdfDoc.numPages`, document metadata, and file stat.

---

### 4.3 Right Dock: Contextual AI Tool Dock (260px)

**AI Recommendations Block** (top, always visible):
- Section header: `AI Recommendations` (left) + animated green pulse dot (right).
- Sub-label: `Based on [document classification]` (e.g. "Based on detected contract").
- 2–4 recommended tool rows, ordered by SLM confidence:
  - **Redact PII** — amber left-border highlight when PII detected.
  - **Validate Signature** — green left-border highlight when signature field detected.
  - **Extract Tables** — shown when tables detected.
  - **Run OCR** — shown when image-based PDF detected (no text layer).
- Each row: `[Icon] [Tool Name] [→ chevron]` — click immediately invokes the tool.

**Tools Sections** (compact text list, not icon grid):
```
SIGNING
  ✍️  Sign with Certificate         ›
  ⏰  Timestamp                     ›
  🖊  Sign                          ›

DOCUMENT SECURITY
  🛡  Add Password                  ›
  🔏  Add Watermark                 ›
  🧹  Sanitize                      ›
  📄  Flatten                       ›
  🔐  Change Permissions            ›

REVIEW
  🏷  Edit Metadata                 ›
  📑  Edit Table of Contents        ›

PAGE FORMATTING
  ✂️  Crop                          ›
  🔄  Rotate                        ›
  ✂️  Split                         ›
  📋  Reorganize Pages              ›
  📐  Resize Pages                  ›
```

**Bottom Engine Badge** (pinned, always visible):
```
🛡  Local Engine ACTIVE  ·  0 Cloud Bytes
```
- Background: dark green pill (`rgba(34, 197, 94, 0.1)` border).
- Text: `#22c55e` (tailwind green-500 equivalent).
- Communicates privacy-first guarantee to users constantly.

---

## 5. Svelte Component Map

| New / Modified File | Change |
|---|---|
| `src/routes/+layout.svelte` | Add `<LibraryPanel>` component alongside `<Sidebar>` |
| `src/routes/+page.svelte` | Restructure outer layout from tab-switch to 3-column flex row; keep Pipeline/API tabs as alternate full-screen views |
| `src/lib/components/layout/Sidebar.svelte` | Refactor to icon-only nav rail (collapsed to 56px); navigation icons for Documents, Pipeline, API, Settings |
| `src/lib/components/layout/LibraryPanel.svelte` | **NEW** — Left 240px panel with Recent, Smart Categories, Watched Folders, Shared sections |
| `src/lib/components/layout/ToolDock.svelte` | **NEW** — Right 260px contextual tool dock with AI Recommendations + grouped tool rows |
| `src/lib/components/layout/FloatingToolbar.svelte` | **NEW** — Frosted-glass pill toolbar for center viewer |
| `src/lib/components/layout/StatusBar.svelte` | **MODIFY** — Add document metadata (PDF/A type, size, encryption, modified date) |
| `src/lib/state/library.svelte.ts` | **NEW** — Reactive state for smart categories, watched folders list, recent files |
| `src/lib/services/watchedFolders.ts` | **NEW** — IPC bridge to `paperpilot watch` daemon status (requires Task 4.7.1) |

---

## 6. State Management

New `library.svelte.ts` store:
```typescript
interface LibraryState {
  recentFiles: RecentFile[];
  smartCategories: { label: string; count: number; toReview: boolean }[];
  watchedFolders: { path: string; recipe: string; status: 'watching' | 'paused' | 'error'; lastFile?: string }[];
  sharedFiles: { name: string; sender: string; avatarUrl?: string }[];
}
```

`watchedFolders` is populated by polling `invoke('get_watch_status')` every 5 seconds (Tauri IPC → Rust daemon PID check).

---

## 7. CSS Design Tokens (additions to `app.css`)

```css
/* V2 Layout */
--layout-sidebar-width: 56px;        /* icon-only nav rail */
--layout-library-width: 240px;
--layout-dock-width: 260px;

/* Frosted Glass */
--glass-bg: rgba(30, 30, 35, 0.75);
--glass-blur: blur(12px);
--glass-border: rgba(255,255,255,0.08);

/* Status colors */
--watching-green: #22c55e;
--review-amber: #f59e0b;
--engine-green-bg: rgba(34, 197, 94, 0.08);

/* Document canvas */
--canvas-bg: #F9F7F5;
--canvas-shadow: 0 8px 40px rgba(0,0,0,0.35);
```

---

## 8. Verification & Test Plan

1. **Visual Regression**: Playwright screenshot comparison of 3-pane layout at 1280×800, 1440×900, 1920×1080.
2. **Watched Folders UI**: Mock `get_watch_status` IPC, verify `Watching` badge appears; simulate `error` status, verify badge turns red.
3. **AI Recommendations**: Feed a contract PDF, verify at least one of `Redact PII`, `Validate Signature`, or `Extract Tables` appears in recommendations.
4. **Tool Invocation from Dock**: Click `Compress` in dock, verify `invoke_mcp_tool` called with correct args and progress appears inline.
5. **Status Bar**: Verify page count, file size, and encryption status render correctly for a known fixture PDF.
6. **All existing Vitest tests (`pnpm --dir apps/desktop test`) remain 112/112 passing.**

---

## 9. Open Questions

1. Should the left library panel be **resizable** (drag handle) or fixed-width? Recommendation: Fixed for V1, resizable in a follow-up.
2. Should Smart Categories be **editable** (user can rename/delete categories)? Recommendation: Read-only SLM-generated for V1.
3. Should the AI Recommendations be computed **on document open** (slight delay) or **on hover** (lazy)? Recommendation: On document open, with a subtle spinner.
