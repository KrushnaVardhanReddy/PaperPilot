# Task 3.4.14: Desktop Multi-Rule Condition Builder (`ALL` / `ANY` Combinators)

## Objectives
Implement Spec 141 (`specs/141-desktop-multi-rule-condition-builder/spec.md`) in `apps/desktop`, upgrading the single-condition inspector into a full visual Multi-Rule Condition Builder supporting recursive/compound rules (`ALL` / `ANY`).

## Subtasks
- [ ] **Task 141.1: State Model & Serialization**
  - Update `apps/desktop/src/lib/state/pipeline.svelte.ts` to support `WorkflowCondition` (compound `all`, `any`, and flat leaf).
  - Implement bidirectional conversion/normalization helpers between UI rule list and `WorkflowCondition`.
  - Maintain backward compatibility for existing saved workflows.
- [ ] **Task 141.2: ConditionInspector UI Component Upgrade**
  - Update `apps/desktop/src/lib/components/pipeline/ConditionInspector.svelte`:
    - Add `ALL (AND)` vs `ANY (OR)` combinator selector.
    - Add dynamic rule rows with predicate selector and contextual inputs.
    - Add `+ Add Rule` button and per-rule delete (`✕`) button.
    - Ensure clean high-contrast dark theme styling with `--bg-surface`, `--text-primary`, and `color-scheme: dark`.
- [ ] **Task 141.3: StepCard & Canvas Badges**
  - Update `StepCard.svelte` condition summary badge to render concise badges for compound conditions (`⚡ ALL (2 rules)` or `⚡ ANY (3 rules)`).
- [ ] **Task 141.4: Unit Tests & Verification**
  - Add test coverage in `apps/desktop/src/lib/components/__tests__/` verifying:
    - Normalization and serialization of multi-rule conditions.
    - Combinator switching (`all` <-> `any`).
    - Adding and removing rules.
    - Passing existing test suite: `pnpm --dir apps/desktop test`.
