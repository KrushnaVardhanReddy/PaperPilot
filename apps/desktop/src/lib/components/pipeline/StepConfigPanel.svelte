<script lang="ts">
  import { pipelineState, OPERATION_REGISTRY } from '$lib/state/pipeline.svelte';

  // Derived: the selected step object from global pipelineState
  let selectedStep = $derived(
    pipelineState.selectedStepId ? pipelineState.steps.find(s => s.id === pipelineState.selectedStepId) ?? null : null
  );

  let schema = $derived(
    selectedStep ? OPERATION_REGISTRY[selectedStep.operationId].paramSchema : []
  );
</script>

{#if selectedStep && schema.length > 0}
  <div class="config-panel" id="step-config-panel">
    <div class="config-header">
      <span>{selectedStep.icon} {selectedStep.label}</span>
    </div>
    <div class="config-fields">
      {#each schema as field}
        <div class="field-group">
          <label for={`field-${selectedStep.id}-${field.key}`}>{field.label}</label>
          {#if field.type === 'select'}
            <select
              id={`field-${selectedStep.id}-${field.key}`}
              value={String(selectedStep.params[field.key] ?? '')}
              onchange={(e) => {
                const val = (e.target as HTMLSelectElement).value;
                const num = Number(val);
                const finalVal = !isNaN(num) && val.trim() !== '' && typeof field.options?.[0] === 'number' ? num : val;
                pipelineState.updateStepParam(selectedStep!.id, field.key, finalVal);
              }}
            >
              {#each (field.options ?? []) as opt}
                <option value={String(opt)}>{opt}</option>
              {/each}
            </select>
          {:else if field.type === 'number' || field.type === 'range'}
            <input
              id={`field-${selectedStep.id}-${field.key}`}
              type={field.type === 'range' ? 'range' : 'number'}
              min={field.min}
              max={field.max}
              step={field.step}
              value={Number(selectedStep.params[field.key] ?? field.min ?? 0)}
              oninput={(e) => pipelineState.updateStepParam(
                selectedStep!.id, field.key, Number((e.target as HTMLInputElement).value)
              )}
            />
          {:else}
            <input
              id={`field-${selectedStep.id}-${field.key}`}
              type={field.type}
              placeholder={field.placeholder}
              value={String(selectedStep.params[field.key] ?? '')}
              oninput={(e) => pipelineState.updateStepParam(
                selectedStep!.id, field.key, (e.target as HTMLInputElement).value
              )}
            />
          {/if}
        </div>
      {/each}
    </div>
  </div>
{/if}

<style>
  .config-panel {
    /* Removed padding, margin, border as it's now inside the inspector drawer */
  }

  .config-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    font-weight: 600;
    margin-bottom: 16px;
    color: var(--text-primary);
    font-size: 1.05rem;
  }

  .config-fields {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .field-group {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .field-group label {
    font-size: 0.85rem;
    color: var(--text-secondary);
  }

  .field-group input,
  .field-group select {
    padding: 8px;
    border: 1px solid var(--border-color);
    border-radius: var(--border-radius-sm);
    background: var(--bg-secondary);
    color: var(--text-primary);
    font-size: 0.9rem;
  }

  .field-group input:focus,
  .field-group select:focus {
    outline: none;
    border-color: var(--accent-primary);
  }
</style>
