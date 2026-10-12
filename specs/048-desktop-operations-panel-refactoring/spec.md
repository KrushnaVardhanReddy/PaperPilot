# Spec 048 — Modularization & Refactoring of Desktop OperationsPanel `handleRunOperation`

## 1. Overview & Problem Statement
In `apps/desktop/src/lib/components/layout/OperationsPanel.svelte`, the `handleRunOperation` function spans more than 500 lines of procedural code (lines 281 through 788).

Currently, this single function mixes multiple distinct architectural concerns:
1. **Precondition Guards & Document Validation**: Checking document count requirements (`compare`, `merge` requiring $\ge 2$ documents, file creation tools needing 0 loaded documents).
2. **Tool Name Normalization & Resolution**: Mapping UI tool IDs (`pdf_` prefix rules, `office_` tools, `excel_to_json`, `create_form_field`, etc.) to backend MCP tool names.
3. **Interactive File Pickers & Modal Dialogs**: Direct calls to `@tauri-apps/plugin-dialog` (`open`, `save`) inside switch cases (`md_to_pdf`, `html_to_pdf`, `img_to_pdf`, `merge`).
4. **Massive Parameter Construction Switch**: A 400+ line switch statement building arguments for ~40 distinct PDF, OCR, Office, and Creation operations.
5. **Execution Lifecycle, Timing, & UI Feedback**: Tauri IPC invocation (`invoke('invoke_mcp_tool')`), duration tracking, clipboard copying for OCR format, toast notifications via `toastState`, job queue updating via `jobsState`, and execution history updating via `executionHistory`.

This monolithic structure violates single-responsibility principles, increases cognitive complexity, makes adding new tools error-prone, and hinders automated unit testing of operation argument mappings.

---

## 2. Requirements & Architecture

Refactor `handleRunOperation` into dedicated, modular TypeScript services under `apps/desktop/src/lib/services/`:

### 2.1 `apps/desktop/src/lib/services/toolRegistry.ts`
- Implement a pure function:
  ```typescript
  export function resolveToolName(toolId: string): string;
  ```
- Maps the UI `activeTool.id` to the MCP tool name registered in `paperpilot-mcp`.
- Replaces ad-hoc string manipulations and condition chains.
- Add unit tests in `apps/desktop/src/lib/services/toolRegistry.test.ts`.

### 2.2 `apps/desktop/src/lib/services/operationArgsBuilder.ts`
- Define an `OperationContext` interface encapsulating:
  - Document metadata: `docName`, `docPath`, `docDir`, `allDocumentPaths`, `allDocuments`.
  - Output path configuration: `customOutputPath`, `getOutputPath(filename: string)`.
  - UI control values: `splitPoints`, `rotateAngle`, `watermarkText`, `watermarkAngle`, `watermarkOpacity`, `watermarkColor`, `watermarkFontSize`, `protectPassword`, `batesPrefix`, `batesStartNumber`, `batesDigits`, `ocrOutputFormat`, `ocrLanguage`, `cropBox`, `pageNumberPosition`, `pageNumberFormat`, `pageNumberStart`, `convertPreset`, etc.
- Implement:
  ```typescript
  export async function buildOperationArgs(
    toolId: string,
    context: OperationContext
  ): Promise<Record<string, any> | null>;
  ```
- Moves the massive switch-case logic and `@tauri-apps/plugin-dialog` dialog calls out of `OperationsPanel.svelte`.
- If the user cancels a file dialog picker (`open` or `save`), return `null` so that the caller can abort execution cleanly without throwing an error.

### 2.3 `apps/desktop/src/lib/services/operationExecutor.ts` (Optional/Recommended) or Helper
- Encapsulate pre-execution checks:
  ```typescript
  export function validatePreconditions(toolId: string, documentsCount: number): { valid: boolean; error?: string };
  ```
- Encapsulate execution post-processing (clipboard copying for OCR, toast formatting, duration calculation, and job/history formatting).

### 2.4 Simplified `OperationsPanel.svelte`
- Replace lines 281–788 in `OperationsPanel.svelte` with a clean, readable lifecycle method:
  ```typescript
  async function handleRunOperation() {
    if (isRunning || !activeTool) return;

    const validation = validatePreconditions(activeTool.id, appState.documents.length);
    if (!validation.valid) {
      if (validation.error) toastState.error(validation.error);
      return;
    }

    if (activeTool.id === 'compare') {
      appState.toggleDiffView(true);
      return;
    }

    isRunning = true;
    elapsedSeconds = 0;
    const timer = setInterval(() => { elapsedSeconds++; }, 1000);

    try {
      const context = buildCurrentOperationContext();
      const args = await buildOperationArgs(activeTool.id, context);
      if (!args) return; // User cancelled file picker

      const toolName = resolveToolName(activeTool.id);
      await executeOperation(toolName, activeTool, args);
    } finally {
      isRunning = false;
      clearInterval(timer);
    }
  }
  ```

---

## 3. Validation & Quality Gates
1. Type and Svelte check:
   ```bash
   pnpm --filter desktop check
   ```
   Must pass with 0 errors.
2. Unit tests:
   ```bash
   pnpm --filter desktop test
   ```
   Must pass all existing and new unit tests.
3. Behavior verification:
   Verify that all 40+ tools generate identical MCP arguments as before.
