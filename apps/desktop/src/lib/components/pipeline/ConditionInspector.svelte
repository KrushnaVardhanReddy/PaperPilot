<script module lang="ts">
  export type PredicateType = 'scanned' | 'page_count' | 'text_contains' | 'has_form_fields' | 'is_signed';

  export interface UiRule {
    id: string;
    type: PredicateType;
    comparator?: 'Greater' | 'GreaterOrEqual' | 'Equal' | 'Less' | 'LessOrEqual';
    numValue?: number;
    textValue?: string;
    boolValue?: boolean;
  }
</script>

<script lang="ts">
  import { pipelineState } from '$lib/state/pipeline.svelte';
  import type { WorkflowCondition, WorkflowConditionLeaf, OnNoMatch } from '$lib/state/pipeline.svelte';

  let selectedStep = $derived(
    pipelineState.selectedStepId ? pipelineState.steps.find(s => s.id === pipelineState.selectedStepId) ?? null : null
  );

  let hasCondition = $derived(!!selectedStep?.condition);
  let onNoMatch = $derived(selectedStep?.on_no_match ?? 'skip');

  let combinator = $state<'all' | 'any'>('all');
  let rules = $state<UiRule[]>([]);

  // Parse condition on step selection changes
  $effect(() => {
    if (!selectedStep?.condition) {
      rules = [];
      combinator = 'all';
      return;
    }

    const cond = selectedStep.condition as any;
    let leaves: WorkflowConditionLeaf[] = [];

    if (cond.all) {
      combinator = 'all';
      leaves = cond.all;
    } else if (cond.any) {
      combinator = 'any';
      leaves = cond.any;
    } else {
      combinator = 'all';
      leaves = [cond as WorkflowConditionLeaf];
    }

    const parsedRules: UiRule[] = [];

    for (const leaf of leaves) {
      if (leaf.if_scanned) parsedRules.push({ id: crypto.randomUUID(), type: 'scanned' });
      if (leaf.comparator) parsedRules.push({ id: crypto.randomUUID(), type: 'page_count', comparator: leaf.comparator, numValue: leaf.value });
      if (leaf.text_contains) parsedRules.push({ id: crypto.randomUUID(), type: 'text_contains', textValue: leaf.text_contains });
      if (leaf.has_form_fields) parsedRules.push({ id: crypto.randomUUID(), type: 'has_form_fields' });
      if (leaf.is_signed) parsedRules.push({ id: crypto.randomUUID(), type: 'is_signed' });
    }

    // De-duplicate if single leaf contained multiple fields (backwards compat)
    if (leaves.length === 1 && parsedRules.length > 1) {
       // Single condition with multiple predicates, rules are already extracted
    }

    rules = parsedRules;
  });

  function saveChanges() {
    if (!selectedStep) return;

    if (rules.length === 0) {
      pipelineState.setStepCondition(selectedStep.id, undefined, onNoMatch);
      return;
    }

    let finalCondition: WorkflowCondition | undefined;

    const leaves = rules.map(rule => {
      const leaf: WorkflowConditionLeaf = {};
      if (rule.type === 'scanned') leaf.if_scanned = true;
      if (rule.type === 'page_count') {
        leaf.comparator = rule.comparator;
        leaf.value = rule.numValue;
      }
      if (rule.type === 'text_contains') leaf.text_contains = rule.textValue;
      if (rule.type === 'has_form_fields') leaf.has_form_fields = true;
      if (rule.type === 'is_signed') leaf.is_signed = true;
      return leaf;
    });

    if (leaves.length === 1 && combinator === 'all') {
      finalCondition = leaves[0] as WorkflowCondition;
    } else if (combinator === 'all') {
      finalCondition = { all: leaves as WorkflowCondition[] };
    } else {
      finalCondition = { any: leaves as WorkflowCondition[] };
    }

    pipelineState.setStepCondition(selectedStep.id, finalCondition, onNoMatch);
  }

  function addRule() {
    rules = [...rules, { id: crypto.randomUUID(), type: 'scanned' }];
    saveChanges();
  }

  function removeRule(id: string) {
    rules = rules.filter(r => r.id !== id);
    saveChanges();
  }

  function updateRule(id: string, updates: Partial<UiRule>) {
    rules = rules.map(r => r.id === id ? { ...r, ...updates } : r);
    saveChanges();
  }

  function updateOnNoMatch(value: OnNoMatch) {
    if (!selectedStep) return;
    const cond = selectedStep.condition;
    pipelineState.setStepCondition(selectedStep.id, cond ? cond : undefined, value);
  }

  function clearCondition() {
    if (!selectedStep) return;
    rules = [];
    pipelineState.removeStepCondition(selectedStep.id);
  }
</script>

{#if selectedStep}
  <div class="condition-panel" id="step-condition-panel">
    <div class="condition-header">
      <span>⚡ Step Condition (Optional)</span>
      {#if hasCondition}
        <button class="clear-btn" onclick={clearCondition}>Clear</button>
      {/if}
    </div>

    <div class="condition-fields">
      {#if rules.length > 0}
        <div class="combinator-row">
          <span class="label">Step executes if:</span>
          <select bind:value={combinator} onchange={saveChanges} class="combinator-select">
            <option value="all">ALL rules match (AND)</option>
            <option value="any">ANY rule matches (OR)</option>
          </select>
        </div>
      {/if}

      {#each rules as rule, index (rule.id)}
        <div class="rule-row">
          <select
            value={rule.type}
            onchange={(e) => updateRule(rule.id, { type: (e.target as HTMLSelectElement).value as PredicateType })}
            class="predicate-select"
          >
            <option value="scanned">Only if Scanned Document</option>
            <option value="page_count">Page Count</option>
            <option value="text_contains">Text Contains</option>
            <option value="has_form_fields">Has Form Fields</option>
            <option value="is_signed">Is Digitally Signed</option>
          </select>

          {#if rule.type === 'page_count'}
            <select
              value={rule.comparator ?? 'Greater'}
              onchange={(e) => updateRule(rule.id, { comparator: (e.target as HTMLSelectElement).value as any })}
            >
              <option value="Greater">Greater</option>
              <option value="Less">Less</option>
              <option value="Equal">Equal</option>
            </select>
            <input
              type="number"
              min="1"
              placeholder="Value"
              value={rule.numValue ?? ''}
              oninput={(e) => updateRule(rule.id, { numValue: parseInt((e.target as HTMLInputElement).value) || undefined })}
            />
          {/if}

          {#if rule.type === 'text_contains'}
            <input
              type="text"
              placeholder="e.g. Confidential"
              value={rule.textValue ?? ''}
              oninput={(e) => updateRule(rule.id, { textValue: (e.target as HTMLInputElement).value || undefined })}
            />
          {/if}

          <button class="remove-rule-btn" onclick={() => removeRule(rule.id)} title="Remove Rule">✕</button>
        </div>
      {/each}

      <button type="button" class="add-rule-btn" onclick={addRule}>+ Add Rule</button>

      <div class="on-no-match-section">
        <div class="section-label">If condition not met:</div>
        <div class="radio-group">
          <label class="radio-label">
            <input
              type="radio"
              name="onNoMatch"
              value="skip"
              checked={onNoMatch === 'skip'}
              onchange={() => updateOnNoMatch('skip')}
            />
            Skip step (continue pipeline)
          </label>
          <label class="radio-label">
            <input
              type="radio"
              name="onNoMatch"
              value="archive_original"
              checked={onNoMatch === 'archive_original'}
              onchange={() => updateOnNoMatch('archive_original')}
            />
            Archive original & stop
          </label>
          <label class="radio-label">
            <input
              type="radio"
              name="onNoMatch"
              value="error"
              checked={onNoMatch === 'error'}
              onchange={() => updateOnNoMatch('error')}
            />
            Fail workflow with error
          </label>
        </div>
      </div>
    </div>
  </div>
{/if}

<style>
  .condition-panel {
    background: var(--bg-surface);
    border: 1px solid var(--warning-color-alpha, rgba(245, 158, 11, 0.3));
    border-radius: var(--border-radius-md);
    padding: 16px;
    margin-top: 16px;
  }

  .condition-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    font-weight: 600;
    margin-bottom: 12px;
    color: var(--warning-color, #f59e0b);
  }

  .clear-btn {
    background: none;
    border: 1px solid var(--border-color);
    border-radius: 4px;
    padding: 4px 8px;
    color: var(--text-secondary);
    font-size: 0.8rem;
    cursor: pointer;
  }

  .clear-btn:hover {
    color: var(--text-primary);
    background: var(--bg-secondary);
  }

  .condition-fields {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .checkbox-label {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 0.85rem;
    color: var(--text-primary);
    cursor: pointer;
  }

  .field-row {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 0.85rem;
  }

  .field-row .label {
    min-width: 90px;
    color: var(--text-secondary);
  }

  .field-row input[type="text"],
  .field-row input[type="number"],
  .field-row select {
    padding: 6px 10px;
    border: 1px solid var(--border-color, #333);
    border-radius: var(--border-radius-sm, 4px);
    background: var(--bg-surface, #1e1e24);
    color: var(--text-primary, #ffffff);
    color-scheme: dark;
    font-size: 0.85rem;
    flex: 1;
    outline: none;
    transition: border-color 0.15s ease;
  }

  .field-row select option {
    background: var(--bg-surface, #1e1e24);
    color: var(--text-primary, #ffffff);
  }

  .field-row input[type="text"]:focus,
  .field-row input[type="number"]:focus,
  .field-row select:focus {
    border-color: var(--accent-color, #6366f1);
  }

  .field-row input[type="number"] {
    max-width: 80px;
  }

  .combinator-row {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-bottom: 8px;
  }

  .combinator-row .label {
    font-size: 0.85rem;
    color: var(--text-secondary);
  }

  .combinator-select {
    padding: 6px 10px;
    border: 1px solid var(--border-color, #333);
    border-radius: var(--border-radius-sm, 4px);
    background: var(--bg-surface, #1e1e24);
    color: var(--text-primary, #ffffff);
    color-scheme: dark;
    font-size: 0.85rem;
    outline: none;
    flex: 1;
  }

  .rule-row {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 0.85rem;
    background: rgba(255, 255, 255, 0.02);
    padding: 8px;
    border-radius: 4px;
    border: 1px dashed var(--border-color);
  }

  .rule-row select,
  .rule-row input[type="text"],
  .rule-row input[type="number"] {
    padding: 6px 10px;
    border: 1px solid var(--border-color, #333);
    border-radius: var(--border-radius-sm, 4px);
    background: var(--bg-surface, #1e1e24);
    color: var(--text-primary, #ffffff);
    color-scheme: dark;
    font-size: 0.85rem;
    outline: none;
    transition: border-color 0.15s ease;
  }

  .rule-row select option {
    background: var(--bg-surface, #1e1e24);
    color: var(--text-primary, #ffffff);
  }

  .predicate-select {
    flex: 1;
  }

  .rule-row input[type="number"] {
    width: 60px;
  }

  .rule-row input[type="text"] {
    flex: 1;
  }

  .remove-rule-btn {
    background: none;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    font-size: 1rem;
    padding: 4px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .remove-rule-btn:hover {
    color: var(--error-color, #ef4444);
  }

  .add-rule-btn {
    background: transparent;
    border: 1px dashed var(--border-color);
    color: var(--text-primary);
    padding: 8px;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.85rem;
    text-align: center;
    transition: background 0.15s;
  }

  .add-rule-btn:hover {
    background: rgba(255, 255, 255, 0.05);
    border-style: solid;
  }

  .on-no-match-section {
    margin-top: 8px;
    padding-top: 12px;
    border-top: 1px solid var(--border-color);
  }

  .section-label {
    font-size: 0.85rem;
    font-weight: 600;
    margin-bottom: 8px;
    color: var(--text-secondary);
  }

  .radio-group {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .radio-label {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 0.85rem;
    color: var(--text-primary);
    cursor: pointer;
  }
</style>
