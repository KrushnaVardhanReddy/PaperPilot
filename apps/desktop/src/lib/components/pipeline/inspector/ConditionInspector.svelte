<script lang="ts">
  import { pipelineState } from '$lib/state/pipeline.svelte';
  import type { WorkflowConditionLeaf, PipelineStep } from '$lib/state/pipeline.svelte';

  let selectedStep = $derived(
    pipelineState.selectedStepId ? pipelineState.steps.find(s => s.id === pipelineState.selectedStepId) ?? null : null
  );

  let condition = $derived(selectedStep?.condition as WorkflowConditionLeaf | undefined);

  function updateCondition(key: keyof WorkflowConditionLeaf, value: any) {
    if (!selectedStep) return;

    // We'll assume for this UI that we only create/edit simple Leaf conditions
    let newCondition: WorkflowConditionLeaf = { ...(condition || {}) };

    if (value === null || value === undefined || value === '') {
      delete newCondition[key];
    } else {
      (newCondition as any)[key] = value;
    }

    // Check if empty, if so delete condition entirely
    if (Object.keys(newCondition).length === 0) {
      updateStepCondition(undefined);
    } else {
      updateStepCondition(newCondition);
    }
  }

  function updateStepCondition(newCondition: any) {
    if (!selectedStep) return;
    pipelineState.steps = pipelineState.steps.map(s =>
      s.id === selectedStep!.id ? { ...s, condition: newCondition } : s
    );
  }
</script>

{#if selectedStep}
  <div class="condition-inspector">
    <div class="inspector-header">
      <h4>Execution Condition</h4>
      <p>Only run this step if rules match</p>
    </div>

    <div class="fields-container">
      <div class="field-group">
        <label>
          <input
            type="checkbox"
            checked={condition?.if_scanned === true}
            onchange={(e) => updateCondition('if_scanned', e.currentTarget.checked ? true : undefined)}
          />
          Is Scanned PDF (Needs OCR)
        </label>
      </div>

      <div class="field-group">
        <label>
          <input
            type="checkbox"
            checked={condition?.is_signed === true}
            onchange={(e) => updateCondition('is_signed', e.currentTarget.checked ? true : undefined)}
          />
          Is Digitally Signed
        </label>
      </div>

      <div class="field-group">
        <label>
          <input
            type="checkbox"
            checked={condition?.is_encrypted === true}
            onchange={(e) => updateCondition('is_encrypted', e.currentTarget.checked ? true : undefined)}
          />
          Is Encrypted
        </label>
      </div>

      <div class="field-group">
        <label for="page-count">Page Count Greater Than:</label>
        <input
          id="page-count"
          type="number"
          min="0"
          value={condition?.if_page_count_gt ?? ''}
          oninput={(e) => {
            const val = parseInt(e.currentTarget.value);
            updateCondition('if_page_count_gt', isNaN(val) ? undefined : val);
          }}
        />
      </div>

      <div class="field-group">
        <label for="text-contains">Text Contains:</label>
        <input
          id="text-contains"
          type="text"
          placeholder="e.g. Invoice"
          value={condition?.text_contains ?? ''}
          oninput={(e) => updateCondition('text_contains', e.currentTarget.value || undefined)}
        />
      </div>
    </div>
  </div>
{/if}

<style>
  .condition-inspector {
    background: var(--bg-surface);
    border: 1px solid var(--border-color);
    border-radius: var(--border-radius-md);
    padding: 16px;
    margin-top: 16px;
  }

  .inspector-header h4 {
    margin: 0 0 4px 0;
    font-size: 0.95rem;
    color: var(--text-primary);
  }

  .inspector-header p {
    margin: 0 0 12px 0;
    font-size: 0.8rem;
    color: var(--text-secondary);
  }

  .fields-container {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .field-group {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .field-group label {
    font-size: 0.85rem;
    color: var(--text-primary);
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .field-group input[type="number"],
  .field-group input[type="text"] {
    padding: 8px;
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: var(--border-radius-sm);
    color: var(--text-primary);
    font-size: 0.9rem;
  }

  .field-group input:focus {
    outline: none;
    border-color: var(--accent-primary);
  }
</style>
