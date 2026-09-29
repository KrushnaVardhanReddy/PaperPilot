<script lang="ts">
  import { pipelineState, OPERATION_REGISTRY } from '$lib/state/pipeline.svelte';

  let selectedStepId = $state<string | null>(null);

  // Derived: the selected step object
  let selectedStep = $derived(
    selectedStepId ? pipelineState.steps.find(s => s.id === selectedStepId) ?? null : null
  );

  let schema = $derived(
    selectedStep ? OPERATION_REGISTRY[selectedStep.operationId].paramSchema : []
  );

  export function selectStep(id: string) {
    selectedStepId = selectedStepId === id ? null : id;
  }
</script>

{#if selectedStep && schema.length > 0}
  <div class="config-panel" id="step-config-panel">
    <div class="config-header">
      <span>{selectedStep.icon} Configure: {selectedStep.label}</span>
      <button onclick={() => selectedStepId = null}>×</button>
    </div>
    <div class="config-fields">
      {#each schema as field}
        <div class="field-group">
          <label for={`field-${selectedStep.id}-${field.key}`}>{field.label}</label>
          {#if field.type === 'select'}
            <select
              id={`field-${selectedStep.id}-${field.key}`}
              value={String(selectedStep.params[field.key] ?? '')}
              onchange={(e) => pipelineState.updateStepParam(
                selectedStep!.id, field.key, Number((e.target as HTMLSelectElement).value)
              )}
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
    background: var(--bg-surface);
    border: 1px solid var(--border-color);
    border-radius: var(--border-radius-md);
    padding: 16px;
    margin-top: 16px;
  }

  .config-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    font-weight: 600;
    margin-bottom: 12px;
    color: var(--text-primary);
  }

  .config-header button {
    background: none;
    border: none;
    color: var(--text-muted);
    font-size: 1.2rem;
    cursor: pointer;
  }

  .config-header button:hover {
    color: var(--text-primary);
  }

  .config-fields {
    display: flex;
    flex-direction: column;
    gap: 12px;
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
