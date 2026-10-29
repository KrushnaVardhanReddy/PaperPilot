<script lang="ts">
  import { pipelineState } from '$lib/state/pipeline.svelte';
  import type { WorkflowConditionLeaf, OnNoMatch } from '$lib/state/pipeline.svelte';

  let selectedStep = $derived(
    pipelineState.selectedStepId ? pipelineState.steps.find(s => s.id === pipelineState.selectedStepId) ?? null : null
  );

  let hasCondition = $derived(!!selectedStep?.condition);

  let condition = $derived(selectedStep?.condition ?? {});
  let onNoMatch = $derived(selectedStep?.on_no_match ?? 'skip');

  function updateCondition(updates: Partial<WorkflowConditionLeaf>) {
    if (!selectedStep) return;
    const newCondition = { ...condition, ...updates };
    pipelineState.setStepCondition(selectedStep.id, newCondition, onNoMatch);
  }

  function updateOnNoMatch(value: OnNoMatch) {
    if (!selectedStep) return;
    pipelineState.setStepCondition(selectedStep.id, Object.keys(condition).length > 0 ? condition : undefined, value);
  }

  function clearCondition() {
    if (!selectedStep) return;
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
      <label class="checkbox-label">
        <input
          type="checkbox"
          checked={!!condition.if_scanned}
          onchange={(e) => updateCondition({ if_scanned: (e.target as HTMLInputElement).checked || undefined })}
        />
        Only if Scanned Document
      </label>

      <div class="field-row">
        <span class="label">Page Count:</span>
        <select
          value={condition.comparator ?? ''}
          onchange={(e) => updateCondition({ comparator: (e.target as HTMLSelectElement).value as any || undefined })}
        >
          <option value="">Any</option>
          <option value="Greater">Greater</option>
          <option value="Less">Less</option>
          <option value="Equal">Equal</option>
        </select>
        <input
          type="number"
          min="1"
          placeholder="Value"
          value={condition.value ?? ''}
          oninput={(e) => updateCondition({ value: parseInt((e.target as HTMLInputElement).value) || undefined })}
          disabled={!condition.comparator}
        />
      </div>

      <div class="field-row">
        <span class="label">Text Contains:</span>
        <input
          type="text"
          placeholder="e.g. Confidential"
          value={condition.text_contains ?? ''}
          oninput={(e) => updateCondition({ text_contains: (e.target as HTMLInputElement).value || undefined })}
        />
      </div>

      <label class="checkbox-label">
        <input
          type="checkbox"
          checked={!!condition.has_form_fields}
          onchange={(e) => updateCondition({ has_form_fields: (e.target as HTMLInputElement).checked || undefined })}
        />
        Has Form Fields
      </label>

      <label class="checkbox-label">
        <input
          type="checkbox"
          checked={!!condition.is_signed}
          onchange={(e) => updateCondition({ is_signed: (e.target as HTMLInputElement).checked || undefined })}
        />
        Is Digitally Signed
      </label>

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
