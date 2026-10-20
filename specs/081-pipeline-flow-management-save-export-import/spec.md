# Spec 081: Pipeline Flow Management (Save, Multi-Flow Switcher, Export & Import)

## 1. Context & Motivation
PaperPilot features a multi-step visual pipeline runner in Desktop (`apps/desktop`) and Web Embed (`apps/embed`). Users often construct complex custom document automation pipelines (e.g. Legal Discovery Bates Stamping, Financial Redaction & Encryption, Split & Fan-Out Watermarking). 

Currently, steps configured in the visual canvas are volatile and cleared when the application resets. Users need the ability to:
1. Select from multiple pre-configured industry standard flows or user-saved flows.
2. Save custom workflows with names and descriptions.
3. Switch seamlessly between different pipeline flows.
4. Export pipelines to portable `.json` configuration files for sharing across teams.
5. Import `.json` pipeline configuration files into the local builder.

---

## 2. Technical Architecture & Data Model

### Pipeline Flow JSON Schema (`SavedPipeline`)
```typescript
export interface SavedPipeline {
  id: string;               // e.g. "flow-uuid" or "preset-legal-bates"
  name: string;             // User-visible flow title
  description?: string;     // Purpose / notes
  steps: PipelineStep[];    // Array of configured steps with tool operationId & params
  updatedAt: string;        // ISO 8601 timestamp
  isPreset?: boolean;       // System default vs user-created
}
```

### Flow Presets (Out-of-the-Box)
1. **Legal Discovery & Bates Stamping**:
   - `pdf_bates` (Prefix: `CASE-2024-`, start: 1)
   - `pdf_watermark` (Text: `CONFIDENTIAL`, opacity: 0.3)
   - `pdf_linearize` (Optimize for fast web viewing)
2. **Redact & Password Protect**:
   - `pdf_redact` (Terms: `SSN, confidential, invoice`)
   - `pdf_flatten` (Flatten interactive form fields & annotations)
   - `pdf_encrypt` (Password encryption)
3. **Split & Fan-Out Watermark**:
   - `pdf_split` (Split by page intervals)
   - `pdf_watermark` (Watermark each split output)

---

## 3. Component Design (`apps/desktop/src/lib/components/pipeline/FlowSelector.svelte`)

### UI Requirements
1. **Flow Dropdown Selector**:
   - Displays presets and custom saved flows with step counts (e.g., `Legal Discovery (3 steps)`).
   - Selecting a flow immediately populates the canvas and config panels.
2. **Action Toolbar**:
   - `💾 Save Flow`: Opens modal prompting for Flow Name & Description. Saves to `localStorage`.
   - `📤 Export`: Downloads active flow as `<flow-name>.json`.
   - `📥 Import`: Hidden file input allowing user to upload `.json` flow. Validates schema and activates imported flow.
   - `🗑️ Delete`: Deletes user-saved flow (presets protected).

---

## 4. Verification & Testing
- Unit test for `PipelineState` flow persistence, serialization, and import validation.
- End-to-end Playwright test verifying saving a 3-step pipeline, exporting to JSON, clearing, importing the JSON back, and executing against a fixture PDF.
