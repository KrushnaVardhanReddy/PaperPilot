# Task: Pipeline Flow Management (Free Community vs Enterprise Team Flows)

## Status: Ready for Dispatch
Spec: `specs/081-pipeline-flow-management-save-export-import/spec.md`

## Split Strategy

### Part 1: Task 5.9.69A — State, Storage, Serialization & Presets
- Prompt: `prompts/tasks/phase5/P5_9_69A_pipeline_state_presets_storage.txt`
- Files:
  - `apps/desktop/src/lib/state/pipeline.svelte.ts`
  - `apps/desktop/src/lib/state/pipeline.test.ts`
- Scope:
  - [ ] Define and export `SavedPipeline` interface (`id`, `name`, `description`, `steps`, `updatedAt`, `isPreset`, `teamId?`).
  - [ ] Implement `DEFAULT_PRESETS` array with 3 standard industry workflows (`preset-legal-discovery`, `preset-clean-scan`, `preset-redact-protect`).
  - [ ] Add `savedFlows` reactive array to `PipelineState`, initialized from / synchronized with `localStorage` (`paperpilot_saved_flows`).
  - [ ] Implement `loadFlow(flowId: string): boolean`.
  - [ ] Implement `saveCurrentFlow(name: string, description?: string): SavedPipeline`.
  - [ ] Implement `deleteFlow(flowId: string): boolean` (with preset protection).
  - [ ] Implement `exportFlowJson(flowId?: string): string`.
  - [ ] Implement `importFlowFromJson(jsonString: string): SavedPipeline` with validation.
  - [ ] Author Vitest test suite `apps/desktop/src/lib/state/pipeline.test.ts` covering all methods and roundtrips.

---

### Part 2: Task 5.9.69B — Flow Selector UI Toolbar & Modal Dialogs
- Prompt: `prompts/tasks/phase5/P5_9_69B_pipeline_flow_selector_ui.txt`
- Files:
  - `apps/desktop/src/lib/components/pipeline/FlowSelector.svelte`
  - `apps/desktop/src/lib/components/pipeline/SaveFlowModal.svelte`
  - `apps/desktop/src/lib/components/pipeline/PipelineView.svelte`
  - `apps/desktop/tests/e2e_pipeline_flow_management.spec.ts`
- Scope:
  - [ ] Create `FlowSelector.svelte` with preset/saved flows `<select>` dropdown.
  - [ ] Add Save, Export, Import, Delete, and New buttons.
  - [ ] Create `SaveFlowModal.svelte` with flow name, description, and steps summary preview.
  - [ ] Integrate `<FlowSelector />` into `PipelineView.svelte`.
  - [ ] Implement automated Playwright E2E test in `apps/desktop/tests/e2e_pipeline_flow_management.spec.ts`.
