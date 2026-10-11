# Spec 045 — Desktop UX: Office + PDF Unified Intake, Run Button Feedback & Context-Aware Tool Navigation

## 1. Executive Summary & Problem Statement

PaperPilot's desktop app (Svelte 5 + Tauri 2) has three critical UX gaps that must be resolved now that both
`office_oxide` (Spec 043, PR #179) and `rusto-rs` OCR (Spec 044, PR #178) have shipped:

### Gap 1 — Silent Run Button (No Loading Feedback)
When a user clicks **"Run OCR Text Recognition"** (or any operation), the button provides zero visual feedback.
The Tauri IPC call is async and can take 2–60 seconds (OCR, compress, merge). The user has no way to know
whether the operation started, is running, or silently failed. This is the **#1 usability regression**.

### Gap 2 — PDF-Only Drop Zone Rejects Office Files
`DropZone.svelte` currently hardcodes "Drop PDF files here", filters Tauri native dialog to `extensions: ['pdf']`
only, and shows a generic `📄` icon. Now that `office_oxide` exposes `office_to_markdown`, `office_extract_text`,
and `office_modernize_legacy` via Tauri, the drop zone must accept `.docx`, `.xlsx`, `.pptx`, `.doc`, `.xls`, `.ppt`.

### Gap 3 — Flat Tool List Doesn't Scale to 50+ Tools
With Office and AI tools joining the 50 PDF tools, the current flat list becomes overwhelming. A context-aware
two-tier navigation (File Type context → Tool Category) is needed.

---

## 2. Target Features

### 2.1 — Run Button Loading State (Priority: P0 — Critical)

**File:** `apps/desktop/src/lib/components/layout/OperationsPanel.svelte`

**Changes:**

1. Add reactive state:
```typescript
let isRunning = $state(false);
let elapsedSeconds = $state(0);
let elapsedDisplay = $derived(
  `${String(Math.floor(elapsedSeconds / 60)).padStart(2, '0')}:${String(elapsedSeconds % 60).padStart(2, '0')}`
);
```

2. Wrap `handleRunOperation` with try/finally:
```typescript
async function handleRunOperation() {
  if (isRunning) return;
  isRunning = true;
  elapsedSeconds = 0;
  const timer = setInterval(() => { elapsedSeconds++; }, 1000);
  try {
    // ... existing implementation unchanged ...
  } finally {
    isRunning = false;
    clearInterval(timer);
  }
}
```

3. Replace the run button HTML (around line 859–868) with:
```html
<div class="action-area">
  <button
    class="run-btn"
    class:run-btn--loading={isRunning}
    id="btn-run-operation"
    disabled={isRunning || !activeTool || (!['md_to_pdf','html_to_pdf','img_to_pdf','json_to_pdf'].includes(activeTool.id) && appState.documents.length === 0)}
    onclick={handleRunOperation}
  >
    {#if isRunning}
      <span class="spinner" aria-hidden="true"></span>
      Running...
    {:else}
      Run {activeTool.title}
    {/if}
  </button>
  {#if isRunning}
    <div class="elapsed-timer" aria-live="polite">⏱ {elapsedDisplay} elapsed</div>
  {/if}
</div>
```

4. Add CSS in the `<style>` block:
```css
.spinner {
  display: inline-block;
  width: 14px;
  height: 14px;
  border: 2px solid rgba(255,255,255,0.3);
  border-top-color: #ffffff;
  border-radius: 50%;
  animation: spin 0.8s linear infinite;
  margin-right: 8px;
  vertical-align: middle;
}
@keyframes spin {
  to { transform: rotate(360deg); }
}
.run-btn--loading {
  opacity: 0.72;
  cursor: not-allowed;
}
.elapsed-timer {
  font-size: 0.78rem;
  color: var(--text-muted);
  text-align: center;
  margin-top: 6px;
  font-variant-numeric: tabular-nums;
  letter-spacing: 0.02em;
}
```

---

### 2.2 — Unified Multi-Format Drop Zone (Priority: P0 — Critical)

**File:** `apps/desktop/src/lib/components/ui/DropZone.svelte`

**Changes:**

1. Add `fileContext` prop with default:
```typescript
let { ondrop, fileContext = 'none' }: {
  ondrop: (files: File[]) => void;
  fileContext?: 'pdf' | 'office' | 'mixed' | 'none';
} = $props();
```

2. Derive dynamic icon from `fileContext`:
```typescript
let dropIcon = $derived(() => {
  switch (fileContext) {
    case 'pdf':    return '📄';
    case 'office': return '📁';
    case 'mixed':  return '🗂️';
    default:       return '📂';
  }
});
let dropTitle = $derived(() =>
  fileContext === 'none' ? 'Drop files here' : 'Add more files'
);
```

3. Update Tauri dialog filter in `handleBrowseClick`:
```typescript
filters: [
  { name: 'Documents (PDF & Office)', extensions: ['pdf', 'docx', 'doc', 'xlsx', 'xls', 'pptx', 'ppt'] },
  { name: 'PDF Files', extensions: ['pdf'] },
  { name: 'Word Documents', extensions: ['docx', 'doc'] },
  { name: 'Excel Spreadsheets', extensions: ['xlsx', 'xls'] },
  { name: 'PowerPoint Presentations', extensions: ['pptx', 'ppt'] }
]
```

4. Update the `get_file_metadata` Tauri invoke to remove the hardcoded `type: 'application/pdf'` —
   detect MIME type from extension:
```typescript
function getMimeType(name: string): string {
  const ext = name.split('.').pop()?.toLowerCase() ?? '';
  const map: Record<string, string> = {
    pdf: 'application/pdf',
    docx: 'application/vnd.openxmlformats-officedocument.wordprocessingml.document',
    doc: 'application/msword',
    xlsx: 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet',
    xls: 'application/vnd.ms-excel',
    pptx: 'application/vnd.openxmlformats-officedocument.presentationml.presentation',
    ppt: 'application/vnd.ms-powerpoint',
  };
  return map[ext] ?? 'application/octet-stream';
}
```

5. Update the hidden file input accept attribute:
```html
accept=".pdf,.docx,.doc,.xlsx,.xls,.pptx,.ppt,application/pdf"
```

6. Update static text in template:
```html
<div class="icon" aria-hidden="true">{dropIcon}</div>
<h3 class="title">{dropTitle}</h3>
<p class="subtitle">PDF, Word, Excel, PowerPoint · or click to browse</p>
```

---

### 2.3 — Context-Aware File Type Switcher (Priority: P1)

**File:** `apps/desktop/src/lib/components/layout/OperationsPanel.svelte`

**Changes:**

1. Add `context` field to `ToolDefinition` interface:
```typescript
interface ToolDefinition {
  id: string;
  title: string;
  description: string;
  icon: string;
  category: 'quick' | 'pages' | 'edit' | 'optimize' | 'security' | 'convert' | 'ai';
  context: 'pdf' | 'office' | 'ai';
  tags: string[];
}
```

2. Add `context: 'pdf'` to ALL existing tools (they all remain PDF context).

3. Add the 3 Office tools to `allTools`:
```typescript
{ id: 'office_to_markdown', title: 'Convert to Markdown', description: 'Convert Word, Excel, or PowerPoint to clean Markdown text', icon: '📑', category: 'convert', context: 'office', tags: ['office', 'markdown', 'docx', 'xlsx', 'pptx', 'convert'] },
{ id: 'office_extract_text', title: 'Extract Text', description: 'Extract all raw text from Office documents into plain text', icon: '📄', category: 'convert', context: 'office', tags: ['office', 'text', 'extract', 'docx', 'xlsx'] },
{ id: 'office_modernize_legacy', title: 'Modernize Legacy Office', description: 'Upgrade .doc/.xls/.ppt to modern .docx/.xlsx/.pptx format', icon: '🔄', category: 'convert', context: 'office', tags: ['legacy', 'doc', 'xls', 'ppt', 'modernize', 'upgrade'] },
```

4. Add state variable for context switcher:
```typescript
let toolContext = $state<'pdf' | 'office' | 'ai'>('pdf');
```

5. Update `filteredTools` to also filter by `toolContext`:
```typescript
let filteredTools = $derived.by(() => {
  const q = searchQuery.trim().toLowerCase();
  let tools = allTools.filter(t => t.context === toolContext);
  if (q) {
    tools = tools.filter(t =>
      t.title.toLowerCase().includes(q) ||
      t.description.toLowerCase().includes(q) ||
      t.tags.some(tag => tag.toLowerCase().includes(q))
    );
  } else if (selectedCategory !== 'all') {
    tools = tools.filter(t => t.category === selectedCategory);
  }
  return tools;
});
```

6. Add auto-context-switch effect (when file is loaded, switch context to match):
```typescript
$effect(() => {
  const paths = appState.documentPaths;
  if (!paths || paths.length === 0) return;
  const officeExts = ['docx','doc','xlsx','xls','pptx','ppt'];
  const allOffice = paths.every(p => officeExts.some(e => p.toLowerCase().endsWith(e)));
  const allPdf = paths.every(p => p.toLowerCase().endsWith('.pdf'));
  if (allOffice && toolContext === 'pdf') toolContext = 'office';
  if (allPdf && toolContext === 'office') toolContext = 'pdf';
});
```

7. Add context bar HTML **before** the search box (inside the `{:else}` branch that shows the directory):
```html
<!-- Context Switcher Bar -->
<div class="context-bar" role="tablist" aria-label="File type context">
  {#each [
    { id: 'pdf', label: 'PDF Tools', icon: '📄' },
    { id: 'office', label: 'Office Tools', icon: '📁' },
    { id: 'ai', label: 'AI Tools', icon: '🤖' }
  ] as ctx}
    <button
      class="context-btn"
      class:active={toolContext === ctx.id}
      role="tab"
      aria-selected={toolContext === ctx.id}
      onclick={() => { toolContext = ctx.id; selectedCategory = 'all'; searchQuery = ''; }}
      id={`context-btn-${ctx.id}`}
    >
      {ctx.icon} {ctx.label}
    </button>
  {/each}
</div>
```

8. Add CSS:
```css
.context-bar {
  display: flex;
  gap: 6px;
  padding: 8px 12px 0;
}
.context-btn {
  flex: 1;
  padding: 6px 8px;
  border-radius: var(--border-radius-sm);
  font-size: 0.8rem;
  font-weight: 500;
  border: 1px solid var(--border-color);
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all var(--transition-fast);
  white-space: nowrap;
}
.context-btn.active {
  background: var(--accent-primary);
  color: #fff;
  border-color: var(--accent-primary);
}
.context-btn:hover:not(.active) {
  background: var(--bg-surface-hover);
  color: var(--text-primary);
}
```

---

### 2.4 — Office Tool Detail Panels (Priority: P1)

**File:** `apps/desktop/src/lib/components/layout/OperationsPanel.svelte`

In `handleRunOperation`, add cases for Office tools (they already have `toolName = 'office_modernize_legacy'`
for the legacy modernize tool — replicate for all three):

```typescript
if (activeTool.id === 'office_to_markdown') toolName = 'office_to_markdown';
if (activeTool.id === 'office_extract_text') toolName = 'office_extract_text';
```

Update the auto-fill output path `$effect` to handle Office outputs:
```typescript
case 'office_to_markdown':  customOutputPath = getOutputPath(`${base}.md`);           break;
case 'office_extract_text': customOutputPath = getOutputPath(`${base}_text.txt`);     break;
case 'office_modernize_legacy': {
  // detect extension from loaded file path
  const srcExt = docPath.split('.').pop()?.toLowerCase() ?? 'docx';
  const modernExt = srcExt === 'doc' ? 'docx' : srcExt === 'xls' ? 'xlsx' : srcExt === 'ppt' ? 'pptx' : srcExt;
  customOutputPath = getOutputPath(`${base}_modern.${modernExt}`);
  break;
}
```

---

### 2.5 — Documents Page Dynamic Heading (Priority: P2)

Search for the component that renders title "Documents" and subtitle "Add PDF files to process".
Run: `grep -r "Add PDF files to process" apps/desktop/src/`

Replace the static heading/subtitle with Svelte reactive logic:
```svelte
<h1 class="page-title">Workspace</h1>
<p class="page-subtitle">
  {#if appState.documents.length === 0}
    Drop PDF or Office files to get started
  {:else}
    {appState.documents.length} document{appState.documents.length > 1 ? 's' : ''} loaded
  {/if}
</p>
```

---

## 3. Acceptance Criteria

1. **Loading State**: Clicking any Run button shows a CSS spinner + "Running..." text. Button is disabled during execution. Elapsed timer ticks below the button. On completion, all resets to normal.
2. **Office Files Accepted**: Dropping `.docx`, `.xlsx`, `.pptx`, `.doc`, `.xls`, or `.ppt` onto the drop zone loads the file without errors.
3. **File Dialog**: Tauri dialog shows Office formats alongside PDF in filter options.
4. **Dynamic Icon**: Drop zone icon changes based on loaded file type (📂/📄/📁/🗂️).
5. **Context Bar**: Operations Panel shows `[📄 PDF Tools] [📁 Office Tools] [🤖 AI]`. Clicking each shows only relevant tools.
6. **Office Tool Panels**: All three Office tools have working detail panels with output paths and Run buttons.
7. **Dynamic Subtitle**: Page subtitle reflects loaded file context.
8. **Zero Regressions**: Existing Playwright desktop E2E tests continue to pass (`cd apps/desktop && pnpm exec playwright test`).
9. **Report**: Author `reports/DESKTOP_UX_OFFICE_PDF_INTEGRATION_REPORT.md`.
10. **PHASES.md**: Update Task 5.9.20 (this spec) from Planned to Completed.

---

## 4. Architecture Constraints (MANDATORY)

- **Svelte 5 Runes ONLY**: Use `$state()`, `$derived()`, `$derived.by()`, `$effect()`, `$props()`. Do NOT use `writable()`, `readable()`, `$:`, or Svelte 4 patterns.
- **No new npm packages**: All changes use existing dependencies (`@tauri-apps/api`, `@tauri-apps/plugin-dialog`).
- **No Rust changes required**: This spec is purely frontend TypeScript/Svelte/CSS.
- **Cargo tests**: Must still pass with `cargo test --workspace -- -j 6` (no Rust changes, this is a sanity guard).
- **Existing job tracking preserved**: `jobsState` integration in sidebar must remain working.
- **Toast system preserved**: `toastState.error(msg)` / `toastState.success(msg)` calls unchanged.
