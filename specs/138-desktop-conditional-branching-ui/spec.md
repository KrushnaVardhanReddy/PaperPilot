# Spec 138: Desktop Visual Flowchart & Conditional Branching UI (Task 3.4.11)

## Metadata
- **Spec Number**: 138
- **Phase**: Phase 3.4.11 / Task 3.4.11
- **Status**: 📋 In Development
- **Target Branch**: `feat/desktop-conditional-branching-ui-task-3-4-11`
- **Impacted Apps & Packages**: `apps/desktop` (`apps/desktop/src/lib/components/pipeline/*`, `apps/desktop/src/lib/state/pipeline.svelte.ts`)
- **Associated Phase Docs**: `phases/phase-03-desktop-mobile-app.md` (Task 3.4.11), `HANDOFF.md`

---

## 1. Overview & Objective

With the completion of **Spec 136** (Conditional Pipeline Engine) and **Spec 137** (Recursive Multi-Condition Engine), the backend across `paperpilot-core` and `paperpilot-pdf` can parse and execute arbitrary document predicates and branching paths.

**Task 3.4.11 delivers the desktop user interface** in `apps/desktop` allowing users to visually construct, inspect, and execute conditional pipelines without writing TOML or JSON:
1. **Decision Nodes on Canvas**: Adding visual IF/ELSE condition nodes to `PipelineCanvas.svelte` with TRUE (match) and FALSE/ELSE (no-match) branch connectors.
2. **Per-Step Condition Badges**: Card-level indicators on `StepCard.svelte` showing active condition filters (`⚡ IF: Scanned`, `⚡ IF: Pages > 10`) and runtime skipped states (`⏭️ Skipped`).
3. **Condition Inspector Panel**: A docked right-rail drawer or modal popover allowing non-technical users to configure predicates (`is_scanned`, `page_count`, `has_form_fields`, `is_signed`, `text_contains`, `file_size_bytes`) and `on_no_match` actions (`skip`, `archive_original`, `error`).
4. **State Serialization Parity**: Updating `PipelineState` in `apps/desktop/src/lib/state/pipeline.svelte.ts` so saved flows (`localStorage['paperpilot_saved_flows']`) and exported JSON/TOML recipes preserve all conditions and dependencies without data loss.

---

## 2. Component Specifications

### 2.1 State Model (`apps/desktop/src/lib/state/pipeline.svelte.ts`)

Extend `PipelineStep` to support conditions and dependencies:

```typescript
export interface WorkflowConditionLeaf {
  if_scanned?: boolean;
  if_page_count_gt?: number;
  comparator?: 'Greater' | 'GreaterOrEqual' | 'Equal' | 'Less' | 'LessOrEqual';
  value?: number;
  text_contains?: string;
  has_form_fields?: boolean;
  is_signed?: boolean;
  is_encrypted?: boolean;
  file_size_bytes?: {
    eq?: number;
    gt?: number;
    gte?: number;
    lt?: number;
    lte?: number;
    between?: [number, number];
  };
}

export type OnNoMatch = 'skip' | 'archive_original' | 'error';

export interface PipelineStep {
  id: string; // crypto.randomUUID()
  operationId: OperationId;
  label: string;
  icon: string;
  params: Record<string, unknown>;
  // Task 3.4.11 additions:
  condition?: WorkflowConditionLeaf;
  depends_on?: string[];
  on_no_match?: OnNoMatch;
  isDecisionNode?: boolean; // When true, rendered as a diamond decision card
}
```

### 2.2 Visual Canvas (`PipelineCanvas.svelte`)
- Renders standard steps and diamond-shaped decision cards (`.decision-node`).
- Renders connectors between steps. When a step has a condition:
  - Displays a green tag `TRUE` along the forward connection.
  - Displays a subtle purple/gray badge `ELSE (skip)` indicating the bypass route.
- Interactive "+ Add Condition" button on each step card or via "+ Add Step" -> "Decision Node (IF/ELSE)".

### 2.3 Step Card Condition Indicators (`StepCard.svelte`)
- If `step.condition` is present:
  - Renders an amber pill badge: `⚡ IF: [Condition Summary]` (e.g., `⚡ Scanned`, `⚡ Pages > 5`).
- During execution, if a stage is skipped:
  - Renders dimmed status badge with `⏭️ Skipped: [Reason]`.

### 2.4 Condition Inspector Sidebar (`ConditionInspector.svelte`)
- Docked side-panel in `PipelineView.svelte` when a step is selected.
- Sections:
  1. **Document Inspection Checkboxes**:
     - `Is Scanned PDF` (Checkbox)
     - `Page Count` (Dropdown: Greater, Less, Equal, Between + number inputs)
     - `Text Contains` (Input text with case-insensitive placeholder)
     - `Has Form Fields` (Checkbox)
     - `Is Digitally Signed` (Checkbox)
     - `Is Encrypted` (Checkbox)
  2. **On No Match Action**:
     - Radio group: `Skip step (pass original)` | `Archive original & stop` | `Fail workflow with error`
  3. **Clear Condition Button**:
     - Reset condition to unconditional execution.

---

## 3. Acceptance Criteria Checklist

- [ ] `apps/desktop/src/lib/state/pipeline.svelte.ts`:
  - `PipelineStep` serializes and deserializes `condition`, `depends_on`, and `on_no_match`.
  - Exported JSON preserves `condition` blocks compatible with `paperpilot run` and `paperpilot-pdf`.
- [ ] Visual UI Components:
  - `StepCard.svelte` displays condition badges and active status.
  - `PipelineCanvas.svelte` supports adding and displaying conditional steps.
  - `ConditionInspector.svelte` enables interactive configuration of all Spec 136/137 leaf predicates without raw code editing.
- [ ] Tests:
  - Unit tests in `apps/desktop/src/lib/components/__tests__/PipelineCanvas.test.ts` verifying conditional step rendering, badge display, and condition updates.
  - `pnpm --dir apps/desktop test` passes cleanly.
