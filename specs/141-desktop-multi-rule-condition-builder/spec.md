# Spec 141: Advanced Multi-Rule Condition Builder (`ALL` / `ANY` Combinators) in Desktop Pipeline Studio

## Metadata
- **Spec Number**: 141
- **Phase**: Phase 3.4.14 / Task 3.4.14
- **Status**: 📋 Ready for Implementation
- **Target Branch**: `feat/desktop-multi-rule-condition-builder-task-3-4-14`
- **Impacted Apps & Packages**: `apps/desktop` (`apps/desktop/src/lib/components/pipeline/*`, `apps/desktop/src/lib/state/pipeline.svelte.ts`)
- **Associated Specs & Phase Docs**: Spec 137 (`specs/137-advanced-multi-condition-engine/spec.md`), Spec 138 (`specs/138-desktop-conditional-branching-ui/spec.md`)

---

## 1. Overview & Objective

Spec 137 introduced recursive boolean combinators (`all`, `any`, `not`) in the `paperpilot-core` and `paperpilot-pdf` runtime engine.
Spec 138 implemented the initial baseline condition inspector in `apps/desktop` with support for a single flat predicate (`WorkflowConditionLeaf`).

**Task 3.4.14 / Spec 141 delivers the full visual Multi-Rule Condition Builder** in `apps/desktop/src/lib/components/pipeline/ConditionInspector.svelte`:
1. **Combinator Switcher**: Toggle between `Match ALL (AND)` and `Match ANY (OR)` logic.
2. **Dynamic Rule List (`+ Add Rule`)**: Add, inspect, and delete individual rule rows dynamically.
3. **Multi-Predicate Row Editor**: Each rule row allows selecting a predicate type:
   - `Scanned Document` (`if_scanned = true / false`)
   - `Page Count` (Comparator: `Greater`, `Less`, `Equal`, `GreaterOrEqual`, `LessOrEqual` + integer value)
   - `Text Contains` (Substring / keyword search)
   - `Has Form Fields` (`has_form_fields = true`)
   - `Digitally Signed` (`is_signed = true`)
   - `Is Encrypted` (`is_encrypted = true`)
   - `File Size` (Comparator: `gt`, `lt`, `gte`, `lte`, `eq` in KB/MB)
4. **Clean Serialization & Backward Compatibility**:
   - Single rule -> serializes as flat `LeafCondition` or single leaf for 100% backward compatibility with existing saved flows.
   - Multiple rules with `ALL` -> `{ all: [rule1, rule2, ...] }`.
   - Multiple rules with `ANY` -> `{ any: [rule1, rule2, ...] }`.
   - Deserialization handles both legacy flat leaf objects and nested `{ all: [...] }` / `{ any: [...] }` structures seamlessly.
5. **On-No-Match Action & Step Badges**:
   - Preserves `on_no_match` actions (`skip`, `archive_original`, `error`).
   - Updates `StepCard.svelte` badge summary to concisely describe compound rules (e.g., `⚡ IF (ALL): Scanned AND Pages > 5` or `⚡ IF (ANY): 2 rules`).

---

## 2. Technical Architecture & State Model

### 2.1 State Definition (`apps/desktop/src/lib/state/pipeline.svelte.ts`)

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
  };
}

export type WorkflowConditionCombinator = 'all' | 'any';

export type WorkflowCondition =
  | { all: WorkflowCondition[] }
  | { any: WorkflowCondition[] }
  | { not: WorkflowCondition }
  | WorkflowConditionLeaf;

export interface PipelineStep {
  id: string;
  operationId: OperationId;
  label: string;
  icon: string;
  params: Record<string, unknown>;
  condition?: WorkflowCondition;
  depends_on?: string[];
  on_no_match?: OnNoMatch;
  status?: 'pending' | 'running' | 'completed' | 'skipped' | 'error';
}
```

### 2.2 Normalization & Helpers

Provide state helpers to normalize any stored condition into an editable UI model:
```typescript
export interface RuleItem {
  id: string;
  type: 'scanned' | 'page_count' | 'text_contains' | 'has_form_fields' | 'is_signed' | 'is_encrypted';
  comparator?: 'Greater' | 'GreaterOrEqual' | 'Equal' | 'Less' | 'LessOrEqual';
  value?: number | string | boolean;
}
```
- `normalizeCondition(cond?: WorkflowCondition): { combinator: 'all' | 'any'; rules: RuleItem[] }`
- `denormalizeRules(combinator: 'all' | 'any', rules: RuleItem[]): WorkflowCondition | undefined`

---

## 3. UI Component Specifications

### 3.1 `ConditionInspector.svelte`
- **Combinator Header**:
  ```svelte
  <div class="combinator-select">
    <span class="label">Execute step if:</span>
    <select bind:value={combinator}>
      <option value="all">ALL rules match (AND)</option>
      <option value="any">ANY rule matches (OR)</option>
    </select>
  </div>
  ```
- **Rules Container**:
  - Each rule rendered in an interactive card with a remove (`✕`) button.
  - Dropdown to pick rule property: `Document Type (Scanned)`, `Page Count`, `Text Layer Contains`, `Form Fields`, `Digital Signatures`.
  - Contextual parameter inputs for the selected property.
- **Actions Bar**:
  - `+ Add Rule` button adds a new rule item.
  - `Clear All` button removes all conditions from the selected step.
- **On No Match Section**:
  - Radio options (`Skip step`, `Archive original & stop`, `Fail workflow with error`).

### 3.2 `StepCard.svelte`
- Helper `getConditionSummary(cond: WorkflowCondition): string`:
  - If leaf: `⚡ Scanned` or `⚡ Pages > 5`
  - If `all`: `⚡ ALL (N rules)`
  - If `any`: `⚡ ANY (N rules)`

---

## 4. Acceptance Criteria Checklist

- [ ] `apps/desktop/src/lib/state/pipeline.svelte.ts`:
  - Type `WorkflowCondition` supports `{ all: [...] }`, `{ any: [...] }`, and legacy flat leaf objects.
  - `setStepCondition` and `removeStepCondition` work seamlessly with compound conditions.
  - JSON serialization / deserialization tests preserve multi-rule structures.
- [ ] `apps/desktop/src/lib/components/pipeline/ConditionInspector.svelte`:
  - Allows toggling `ALL` vs `ANY`.
  - Supports adding multiple rule rows via `+ Add Rule`.
  - Allows removing individual rule rows.
  - Allows configuring each rule predicate (Scanned, Page Count, Text Contains, Form Fields, Digital Signatures).
  - High-contrast dark styling conforming to desktop design system.
- [ ] Unit & Component Tests:
  - Tests in `apps/desktop/src/lib/components/__tests__/PipelineCanvas.test.ts` (or `ConditionInspector.test.ts`) covering:
    - Adding multiple rules with `ALL` combinator.
    - Adding multiple rules with `ANY` combinator.
    - Removing rules.
    - Backward compatibility with legacy single-rule conditions.
  - All desktop tests pass with `pnpm --dir apps/desktop test`.
