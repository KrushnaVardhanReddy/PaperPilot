# Spec 083: Embed Pipeline Parity — Interactive Step Selector, Customization & Visual Dropzone Polish

## 1. Executive Summary & Vision
PaperPilot's Web Embed distribution (`apps/embed`) enables zero-server, client-side PDF automation inside any web application or third-party host via Shadow DOM. While Phase 5.9.67 established automated pipeline execution (`data-pipeline='[...]'`), it was built as a read-only "drop-to-execute" kiosk.

Comparing the Embed widget to the Desktop application reveals two major functional gaps:
1. **Dropzone Clarity & File Browsing**: The pipeline dropzone currently lacks an explicit, clickable **"Browse Files"** button/link. Although clicking the dropzone container triggers file selection, users cannot visually discern that clicking opens the file dialog, nor can they easily clear/change documents prior to execution.
2. **Interactive Tool & Step Selection**: Unlike Desktop's `PipelineCanvas` and `StepPicker`, the embed widget locks users into predefined steps. When `data-allow-customization="true"` (or when no fixed pipeline is provided), users should be able to:
   - Browse and choose PDF files via an explicit button.
   - Dynamically add steps from available tools (`rotate`, `watermark`, `compress`, `extract`, `split`, etc.).
   - Remove steps (`×`) or edit basic parameters inline.
   - Run the pipeline explicitly on click or auto-execute on drop.

---

## 2. User Experience & Design Requirements

### 2.1 Visual Dropzone Polish
- Prominent upload icon with:
  - `📁 Drag and drop PDF here, or <button type="button" class="pp-btn-browse">Browse Files</button>`
  - File status summary: shows file name and size once selected.
  - Clear / Replace button (`✕ Change File`) so users can reset without page refresh.

### 2.2 Interactive Step Picker & Builder Mode
- Attribute: `data-allow-customization="true"` (boolean attribute on widget).
- When enabled (or when `data-pipeline` is empty/editable):
  - Displays existing steps as interactive chips with a delete (`×`) icon.
  - Adds an `[+ Add Step]` button at the end of the stepper sequence.
  - Clicking `[+ Add Step]` opens a lightweight popover/drawer displaying available in-browser tools (`Rotate`, `Watermark`, `Compress`, `Extract`, `Split`, `Merge`, `Delete`).
  - Selecting a tool appends it to the active pipeline sequence.
- An explicit **"⚡ Execute Pipeline"** action button appears below the stepper once a document is loaded.

---

## 3. Architecture & API Changes

### 3.1 Embed Props & Data Attributes
In `apps/embed/src/Widget.svelte` and `apps/embed/src/api.ts`:

```typescript
export interface EmbedOptions extends ThemeConfig {
  tools?: string | string[];
  hideBadge?: boolean;
  pipeline?: string | PipelineStep[] | null;
  allowCustomization?: boolean; // new prop
  autoExecute?: boolean; // default true for automated pipelines, false when customizable
}
```

HTML markup mapping:
- `data-allow-customization="true|false"`
- `data-auto-execute="true|false"`

### 3.2 Dynamic Step Pipeline State in `Widget.svelte`
- Internal state: `activePipelineSteps = $state<PipelineStep[]>([])`.
- Initialized from `parsedPipeline`.
- If `allowCustomization` is active, mutating `activePipelineSteps` updates the rendered stepper and execution queue.

---

## 4. Verification & Testing
1. **Vitest / Unit Verification**:
   - Verify `parsedPipeline` correctly initializes editable state when `allowCustomization` is true.
   - Verify tool addition and removal from pipeline steps.
2. **Playwright E2E (`apps/embed/tests/pipeline.spec.ts`)**:
   - Assert presence and clickability of the explicit `"Browse Files"` button.
   - Assert adding a step (`+ Add Step`) opens the tool picker and appends to the stepper.
   - Assert deleting a step updates the stepper.
   - Assert execution completes with valid PDF output.
