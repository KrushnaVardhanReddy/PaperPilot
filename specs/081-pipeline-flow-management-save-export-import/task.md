# Task: Pipeline Flow Management (Free Community vs Enterprise Team Flows)

## Status: Planned
Spec: `specs/081-pipeline-flow-management-save-export-import/spec.md`

## Checklist

### Phase 1: Free Community Tier (Local Disk & File-based)
- [ ] **Data Model & State Updates (`apps/desktop/src/lib/state/pipeline.svelte.ts`)**
  - [ ] Add `SavedPipeline` interface (`id`, `name`, `description`, `steps`, `updatedAt`, `isPreset`, `teamId?`).
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

### Phase 2: Enterprise Tier (Team Sync & Workspaces)
- [ ] **Gateway Team Pipeline API (`paperpilot-gateway`)**
  - [ ] `GET /api/v1/team/pipelines`: List company-approved shared pipelines.
  - [ ] `POST /api/v1/team/pipelines`: Publish/update shared pipeline (requires Editor role).
  - [ ] Audit log event emitted on pipeline execution (`pipeline.executed`).
- [ ] **Enterprise UI Badge in `FlowSelector.svelte`**
  - [ ] Tab / section for "Team Pipelines (Cloud)".
  - [ ] Visual lock indicator for read-only company flows.

### Verification
- [ ] Add Playwright test in `apps/desktop/tests/pipeline_flow.spec.ts` testing local save, export, import, and flow switching.
