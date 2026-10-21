# Spec 083: Embed Pipeline 1-to-1 Desktop Parity — Dropzone Polish, Interactive Step Customization, and Flow Management (Save/Export/Import)

## 1. Executive Summary & Vision
PaperPilot's Web Embed distribution (`apps/embed`) enables zero-server, client-side PDF automation inside any web application or third-party host via Shadow DOM. While Phase 5.9.67 and Task 5.9.69B introduced automated pipeline execution and preset mapping, the embed pipeline was previously constrained as a static "drop-to-execute" kiosk.

Spec 083 establishes **1-to-1 Functional Parity between the Embed Pipeline Widget and the Desktop Pipeline Builder**:
1. **Dropzone UX & File Selection**: Clear, visible `📁 Browse Files` button, file status chip summary, and `✕ Clear / Change File` controls.
2. **Interactive Step Management**: When customization is enabled (`data-allow-customization="true"` or editable pipeline), users can view steps, delete steps (`✕`), and dynamically add steps (`+ Add Step`) from available in-browser operations (`Rotate`, `Watermark`, `Compress`, `Extract Pages`, `Split`, `Merge`).
3. **Flow Management (Save, Export & Import)**: Users can save customized workflows to `localStorage`, select between standard presets and saved user flows, export their recipe to a portable `flow.json`, and import external `flow.json` files.
4. **Host Integration Events**: Dispatches custom DOM events (`paperpilot:flow-saved`, `paperpilot:flow-exported`) enabling parent web applications to optionally synchronize recipes to their own backend databases.

---

## 2. User Experience & Design Requirements

### 2.1 Polished Dropzone with Explicit Browse Button
- Prominent upload icon and clear action text:
  - Default: `📁 Drag and drop PDF here, or <button class="pp-btn-browse">Browse Files</button>`
  - Selected State: File badge with document icon, filename, formatted size (e.g. `1.2 MB`), and a `✕ Change File` button.
  - Native file input triggered reliably across desktop browsers and mobile touch devices.

### 2.2 Interactive Stepper & Step Picker
- Stepper displays configured steps with visual connectors (`➔`).
- When customization is enabled:
  - Each step chip features a hoverable delete icon (`✕`) to remove unwanted steps.
  - A `[+ Add Step]` button is displayed at the end of the stepper.
  - Clicking `[+ Add Step]` opens an inline popover or modal picker listing available in-browser operations with icons and descriptions.
  - Clicking an operation appends it with sensible default parameters to the pipeline.

### 2.3 Flow Management Toolbar (Save / Export / Import / Presets)
- Placed directly above or below the pipeline stepper:
  - **Flow Dropdown**: Grouped dropdown `<select>` containing:
    - `<optgroup label="Standard Presets">` (e.g., Legal Discovery, Clean Scan, Redact & Protect)
    - `<optgroup label="My Saved Flows">` (stored in `localStorage['paperpilot_saved_flows']`)
  - **`💾 Save Flow` Button**: Opens a lightweight dialog to enter Flow Name & Description, saving the recipe to `localStorage`.
  - **`⬇️ Export JSON` Button**: Downloads the current pipeline steps as `<flow-name>.json`.
  - **`⬆️ Import JSON` Button**: Lets users upload a `flow.json` created in Desktop or CLI, instantly populating the stepper.

### 2.4 Execution Controls
- When `autoExecute === false` (or when customization is active):
  - Execution does not trigger abruptly on file drop; instead, a prominent `⚡ Run Pipeline` action button is enabled once a document and at least one step exist.
  - Shows progress bar during execution and renders download links with file size badges upon completion.

---

## 3. Architecture & API Changes

### 3.1 Embed Configuration (`apps/embed/src/api.ts` & `Widget.svelte`)
```typescript
export interface EmbedOptions extends ThemeConfig {
  tools?: string | string[];
  hideBadge?: boolean;
  pipeline?: string | PipelineStep[] | null;
  allowCustomization?: boolean; // Enable interactive + Add Step and Flow management
  autoExecute?: boolean;        // Auto-run on drop (default true when customization is off)
  storageKey?: string;         // LocalStorage key for saved flows (default: 'paperpilot_saved_flows')
}
```

HTML Data Attributes:
- `data-allow-customization="true"`
- `data-auto-execute="false"`
- `data-pipeline="preset-clean-scan"`

### 3.2 State Management in `Widget.svelte`
- `customSteps = $state<PipelineStep[]>([])`
- `savedFlows = $state<SavedPipeline[]>([])`
- `selectedFlowId = $state<string>('')`
- Synchronizes with `DEFAULT_PRESETS` and `localStorage['paperpilot_saved_flows']`, maintaining shared recipe compatibility with Desktop (`apps/desktop/src/lib/state/pipeline.svelte.ts`).

---

## 4. Verification & Testing
1. **Unit & Build Integrity**:
   - `npm --prefix apps/embed run build` produces zero errors.
2. **Playwright E2E Suite (`apps/embed/tests/pipeline.spec.ts`)**:
   - Verify explicit `"Browse Files"` button renders and triggers file selection.
   - Verify selecting a preset from the dropdown updates the stepper.
   - Verify adding a step (`+ Add Step`) and removing a step (`✕`).
   - Verify clicking `💾 Save Flow` persists the flow to `localStorage`.
   - Verify exporting JSON triggers download.
   - Verify running the customized pipeline produces the processed PDF.
