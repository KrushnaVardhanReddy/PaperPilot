# Spec 053 — Adaptive Multi-Layout Workspace & Home Experience (Studio Hub, Split Studio, Hero Actions)

## 1. Overview & Problem Statement
The PaperPilot Settings panel was recently upgraded to an enterprise-grade multi-column layout with sidebar category tabs and a wide full-width canvas (>1200px), eliminating cramped modals and wasted space.

However, the **Home / Workspace View** (`apps/desktop/src/routes/+page.svelte` when `selectedDocumentIndex === null`) still has legacy constraints:
1. **Wasted Desktop Canvas**: The main document drop container is artificially restricted to `max-width: 900px; margin: 0 auto;`, leaving large empty black margins on standard 1080p, 1440p, and 4K displays.
2. **Hidden Capabilities**: The 44+ tool operations are only accessible inside a 320px vertical scroll panel on the right side (`OperationsPanel.svelte`). Users arriving on the Home screen only see an empty drop zone and a command bar, missing the visual breadth of available tools.
3. **Diverse Workflow Needs**: Different user profiles work differently:
   - **Creative & Power Users** prefer a visual workstation (Studio Hub) where tools are front-and-center action cards.
   - **Batch Processing Users** prefer a split-screen queue where files sit on the left and operation configurations sit on the right (Split Studio).
   - **Minimalists & Keyboard Power Users** prefer a clean hero drop with quick action pills and a command bar (Hero Actions).

---

## 2. Technical Architecture & Requirements

### 2.1 Workspace Layout Switcher State
In `apps/desktop/src/lib/state/app.svelte.ts`:
- Define type `WorkspaceLayoutMode = 'studio' | 'split' | 'hero';`
- Add reactive state `workspaceLayout = $state<WorkspaceLayoutMode>('studio');`
- Persist selection to `localStorage.getItem('paperpilot_workspace_layout')` (defaulting to `'studio'`).
- Add helper method `setWorkspaceLayout(mode: WorkspaceLayoutMode)`.

### 2.2 Header Controls & Settings Sync
1. **Workspace Header Switcher**:
   - In `apps/desktop/src/routes/+page.svelte` within `.view-header`, render a segmented layout toggle button group:
     - `🎛️ Studio Hub` (`studio`)
     - `📑 Split Studio` (`split`)
     - `⚡ Hero Actions` (`hero`)
   - Style with active indicator, smooth transitions, and tooltip descriptions.
2. **Settings Panel Preference**:
   - In `apps/desktop/src/lib/components/layout/SettingsPanel.svelte` under the **General & Storage** category (`activeCategory === 'general'`), add a **Default Workspace Layout** selector with options for Studio Hub, Split Studio, and Hero Actions.

### 2.3 The 3 Workspace Layouts

#### Layout A: `🎛️ Studio Hub` (Default - Enterprise Workstation)
- **Top System Status & Drop Banner**:
  - Horizontal status pills: `🔒 100% On-Device & Private` • `Port 7823 REST Gateway` • `Runtime: Offline NLP / Local Llamafile`.
  - Compact full-width drop zone banner (`Drop PDF or Office files to get started • or Browse`).
- **Full-Width Categorical Operations Grid**:
  - Render categorized action cards across the wide canvas:
    - **⚡ Quick Actions**: Compress PDF, Merge Documents, Rotate Pages, OCR Text, PDF → Word, Compare.
    - **🔄 Conversions & Office**: Excel to JSON, Excel to CSV, JSON to PDF, Markdown, HTML.
    - **🔒 Security & Editing**: Protect with Password, Add Watermark, Redact Text, Sanitize Metadata.
  - Hover effects, tool tags, and click handler dispatching directly into the tool inspector.
- **Active Document Shelf**:
  - When files are loaded, render a horizontal document preview strip showing file cards with page counts, file sizes, and quick removal.

#### Layout B: `📑 Split Studio` (Queue & Operations Matrix)
- **Left Column (360px - 400px)**:
  - Document Queue / Batch Workspace:
    - Drop zone optimized for adding multiple files.
    - Reorderable file list with drag/up/down buttons and batch clear.
    - Summary banner (total files, total size, estimated memory).
- **Right Column (Full-Width Flex Canvas)**:
  - Multi-column tool matrix with real-time search input and category filter chips (`All`, `PDF Tools`, `Office Tools`, `Security`, `AI`).
  - Clicking any tool opens the tool parameter controls directly in the main canvas.

#### Layout C: `⚡ Hero Actions` (Minimalist Command & Drop Focus)
- **Centered Hero Card**:
  - Prominent glassmorphic drop card with subtle glow and drag hover feedback.
- **1-Click Quick Action Pills**:
  - `[🗜️ Compress to < 2MB]`
  - `[📑 Merge All Open Files]`
  - `[👁️ Scanned to Searchable OCR]`
  - `[📝 Export to Word (.docx)]`
- **Global Command Bar**:
  - Pinned input with auto-complete (`@filename rotate 90`, `compress all`).
- **Recent Execution History**:
  - Displays the last executed tools, latencies, and output file links.

### 2.4 Backward Compatibility & Safety Constraints
1. **Right-Side OperationsPanel (`.operations-panel`)**:
   - MUST remain mounted in the DOM to satisfy existing Playwright tests (`tests/e2e_ui.spec.ts` line 32 and 39) and desktop operation workflows.
   - May be collapsed or hidden on widescreen Studio Hub when the central canvas tools are active, but must respect the existing `.operations-panel` CSS selector and toggle button `#operations-panel-toggle`.
2. **Document Viewer Unaffected**:
   - When a document is opened (`selectedDocumentIndex !== null`), the full PDF viewer, thumbnails, toolbars, and `ViewerRightPanel` remain exactly as they are.

---

## 3. Verification & Acceptance Criteria
1. **Vitest Unit Tests**:
   - `pnpm --dir apps/desktop test` must pass 100% (all 15 test suites).
   - Add unit tests verifying `workspaceLayout` switching and persistence in `apps/desktop/src/lib/components/__tests__/OperationsPanel.test.ts` or a new test.
2. **Playwright E2E**:
   - `pnpm --dir apps/desktop exec playwright test tests/e2e_real_desktop_execution.spec.ts` must pass **12/12 real desktop operations**.
   - `pnpm --dir apps/desktop exec playwright test tests/e2e_ui.spec.ts` must pass launch and navigation checks.
3. **No regressions in Tauri IPC or Gateway**:
   - Concurrency limit `-j 6` strictly maintained on any cargo commands.
