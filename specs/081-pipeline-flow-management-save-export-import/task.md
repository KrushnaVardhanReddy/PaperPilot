# Task: Pipeline Flow Management (Save, Switch, Export, Import)

## Status: Planned
Spec: `specs/081-pipeline-flow-management-save-export-import/spec.md`

## Checklist

- [ ] **Data Model & State Updates (`apps/desktop/src/lib/state/pipeline.svelte.ts`)**
  - [ ] Add `SavedPipeline` interface (`id`, `name`, `description`, `steps`, `updatedAt`, `isPreset`).
  - [ ] Add `savedFlows` reactive state array loaded from / synced to `localStorage` (`paperpilot_pipeline_flows`).
  - [ ] Pre-populate system default presets (Legal Discovery, Redact & Protect, Split & Watermark).
  - [ ] Implement `loadFlow(flowId)`, `saveCurrentFlow(name, desc)`, and `deleteFlow(flowId)`.
  - [ ] Implement `exportFlowJson(flowId?)` returning formatted JSON string.
  - [ ] Implement `importFlowFromJson(jsonString)` validating steps array and adding to saved flows.

- [ ] **UI Component (`apps/desktop/src/lib/components/pipeline/FlowSelector.svelte`)**
  - [ ] Active Flow dropdown selector.
  - [ ] Save Flow button + modal for naming / updating flow.
  - [ ] Export Flow button triggering JSON file download.
  - [ ] Import Flow file picker accepting JSON files and loading them.
  - [ ] Delete Flow button (disabled for default presets).

- [ ] **Integration (`apps/desktop/src/lib/components/pipeline/PipelineView.svelte`)**
  - [ ] Mount `FlowSelector` above the Pipeline canvas.
  - [ ] Ensure canvas reactive bindings refresh when switching flows.

- [ ] **Automated Testing**
  - [ ] Add Playwright test in `apps/desktop/tests/pipeline_flow.spec.ts` testing save, export, import, and flow switching.
