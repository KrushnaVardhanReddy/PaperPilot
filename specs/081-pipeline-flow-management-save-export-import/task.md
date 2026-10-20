# Task: Pipeline Flow Management & Declarative Execution (All 5 Interfaces)

## Status: Part A Merged | Part B & C Ready for Dispatch
Spec: `specs/081-pipeline-flow-management-save-export-import/spec.md`

## Multi-Phase Split Strategy

---

### Part 1: Task 5.9.69A — Desktop State, Storage, Serialization & Presets
- **Status**: ✅ **COMPLETED** (PR #220, commit `460ec1d`)
- **Files**:
  - `apps/desktop/src/lib/state/pipeline.svelte.ts`
  - `apps/desktop/src/lib/state/pipeline.test.ts`
- **Scope Accomplished**:
  - [x] Defined and exported `SavedPipeline` interface (`id`, `name`, `description`, `steps`, `updatedAt`, `isPreset`, `teamId?`).
  - [x] Implemented `DEFAULT_PRESETS` array with 3 standard industry workflows (`preset-legal-discovery`, `preset-clean-scan`, `preset-redact-protect`).
  - [x] Added `savedFlows` reactive array to `PipelineState`, initialized from / synchronized with `localStorage` (`paperpilot_saved_flows`).
  - [x] Implemented `loadFlow(flowId: string): boolean`.
  - [x] Implemented `saveCurrentFlow(name: string, description?: string): SavedPipeline`.
  - [x] Implemented `deleteFlow(flowId: string): boolean` (with preset protection).
  - [x] Implemented `exportFlowJson(flowId?: string): string`.
  - [x] Implemented `importFlowFromJson(jsonString: string): SavedPipeline` with validation.
  - [x] Authored Vitest test suite `apps/desktop/src/lib/state/pipeline.test.ts` (7/7 tests passing).

---

### Part 2: Task 5.9.69B — Flow Selector UI Toolbar & Modal Dialogs (Desktop & Embed)
- **Status**: 📋 **Ready for Dispatch**
- **Prompt**: `prompts/tasks/phase5/P5_9_69B_pipeline_flow_selector_ui.txt`
- **Files**:
  - `apps/desktop/src/lib/components/pipeline/FlowSelector.svelte`
  - `apps/desktop/src/lib/components/pipeline/SaveFlowModal.svelte`
  - `apps/desktop/src/lib/components/pipeline/PipelineView.svelte`
  - `apps/desktop/tests/e2e_pipeline_flow_management.spec.ts`
  - `apps/embed/src/Widget.svelte`
- **Scope**:
  - [ ] Create `FlowSelector.svelte` with preset/saved flows `<select>` dropdown.
  - [ ] Add Save, Export, Import, Delete, and New buttons styled with dark slate glassmorphism.
  - [ ] Create `SaveFlowModal.svelte` with flow name, description, and visual steps badge sequence preview matching `AiSettingsModal.svelte`.
  - [ ] Integrate `<FlowSelector />` directly under the header in `PipelineView.svelte`.
  - [ ] Add preset recipe resolution support in `apps/embed` (`Widget.svelte` pipeline prop accepts preset IDs like `"preset-legal-discovery"`).
  - [ ] Implement automated Playwright E2E test in `apps/desktop/tests/e2e_pipeline_flow_management.spec.ts`.

---

### Part 3: Task 5.9.69C — Penta-Interface Declarative Workflow Engine (CLI, MCP, REST, Core)
- **Status**: 📋 **Planned / Next Phase**
- **Prompt**: `prompts/tasks/phase5/P5_9_69C_workflow_engine_penta_interface.txt`
- **Files**:
  - `paperpilot-core/src/workflow.rs` (Data model: `WorkflowStep`, `SavedWorkflow`, standard preset constants)
  - `paperpilot-pdf/src/operations/workflow.rs` (Sequential in-memory execution pipeline)
  - `paperpilot-cli/src/cli.rs` & `commands/workflow.rs` (`paperpilot workflow --preset <name> --recipe <path>`)
  - `paperpilot-mcp/src/server.rs` (MCP tool `document_workflow`)
  - `paperpilot-gateway/src/handlers.rs` & `server.rs` (`POST /api/v1/workflow/run`, `GET /api/v1/workflow/presets`)
  - `tools/penta-interface-e2e/src/cases/workflow.rs` (20 real assertions across all 5 interfaces)
- **Scope**:
  - [ ] Core Rust serde structs matching `SavedPipeline` schema.
  - [ ] Built-in Rust preset definitions for `legal-discovery`, `clean-scan`, and `redact-protect`.
  - [ ] In-memory chaining runner executing operations in memory without intermediate disk writes.
  - [ ] CLI command `paperpilot workflow` supporting both `--preset` and `--recipe <flow.json>`.
  - [ ] MCP tool `document_workflow` enabling compound multi-step execution in single agent RPC.
  - [ ] REST API endpoints for multipart file and JSON execution.
  - [ ] 20 unmocked test cases in `tools/penta-interface-e2e`.
