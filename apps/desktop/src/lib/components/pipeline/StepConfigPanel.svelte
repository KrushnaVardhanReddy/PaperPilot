<script lang="ts">
  import { pipelineState, OPERATION_REGISTRY } from '$lib/state/pipeline.svelte';

  // Derived: the selected step object from global pipelineState
  let selectedStep = $derived(
    pipelineState.selectedStepId ? pipelineState.steps.find(s => s.id === pipelineState.selectedStepId) ?? null : null
  );

  let opDef = $derived(
    selectedStep ? OPERATION_REGISTRY[selectedStep.operationId] : null
  );

  let schema = $derived(
    opDef?.paramSchema ?? []
  );
</script>

{#if selectedStep}
  <div class="config-panel" id="step-config-panel">
    <div class="config-header">
      <div class="step-title-row">
        <span class="step-icon">{selectedStep.icon}</span>
        <span class="step-label">{selectedStep.label}</span>
      </div>
      {#if opDef?.description}
        <p class="step-desc">{opDef.description}</p>
      {/if}
    </div>

    {#if schema.length > 0}
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
                  <option value={String(opt)}>
                    {opt}{field.key === 'angle' ? '°' : ''}
                  </option>
                {/each}
              </select>
            {:else if field.type === 'number' || field.type === 'range'}
              <div class="range-field-row">
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
                {#if field.type === 'range'}
                  <span class="range-badge">{selectedStep.params[field.key] ?? field.min ?? 0}</span>
                {/if}
              </div>
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
    {:else}
      <div class="no-params-card">
        <span class="info-icon">⚡</span>
        <div class="info-text">
          <div class="info-title">Optimized Automatically</div>
          <p>No extra parameters needed for this step. It runs automatically with default pipeline settings.</p>
        </div>
      </div>
    {/if}
  </div>
{/if}

<style>
  .config-panel {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .config-header {
    display: flex;
    flex-direction: column;
    gap: 6px;
    padding-bottom: 14px;
    border-bottom: 1px solid var(--border-color, #334155);
  }

  .step-title-row {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 1.1rem;
    font-weight: 600;
    color: var(--text-primary, #f8fafc);
  }

  .step-icon {
    font-size: 1.3rem;
    line-height: 1;
  }

  .step-desc {
    margin: 0;
    font-size: 0.8rem;
    color: var(--text-secondary, #94a3b8);
    line-height: 1.4;
  }

  .config-fields {
    display: flex;
    flex-direction: column;
    gap: 14px;
  }

  .field-group {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .field-group label {
    font-size: 0.85rem;
    font-weight: 500;
    color: var(--text-secondary, #cbd5e1);
  }

  .field-group input {
    padding: 8px 12px;
    border: 1px solid var(--border-color, #334155);
    border-radius: var(--border-radius-sm, 6px);
    background: var(--bg-secondary, #1e293b);
    color: var(--text-primary, #f8fafc);
    font-size: 0.9rem;
    transition: border-color 0.15s, box-shadow 0.15s;
  }

  .field-group select {
    padding: 8px 12px;
    padding-right: 2.25rem;
    border: 1px solid var(--border-color, #334155);
    border-radius: var(--border-radius-sm, 6px);
    background-color: var(--bg-secondary, #1e293b);
    background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='16' height='16' viewBox='0 0 24 24' fill='none' stroke='%2394a3b8' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'%3E%3Cpolyline points='6 9 12 15 18 9'%3E%3C/polyline%3E%3C/svg%3E");
    background-repeat: no-repeat;
    background-position: right 0.75rem center;
    color: var(--text-primary, #f8fafc);
    font-size: 0.9rem;
    -webkit-appearance: none;
    -moz-appearance: none;
    appearance: none;
    cursor: pointer;
    transition: border-color 0.15s, box-shadow 0.15s;
  }

  .field-group select option {
    background-color: var(--bg-surface, #0f172a);
    color: var(--text-primary, #f8fafc);
    padding: 6px 10px;
  }

  .field-group input:focus,
  .field-group select:focus {
    outline: none;
    border-color: var(--accent-primary, #3b82f6);
    box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.25);
  }

  .range-field-row {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .range-field-row input[type="range"] {
    flex: 1;
    accent-color: var(--accent-primary, #3b82f6);
    padding: 0;
  }

  .range-badge {
    font-size: 0.85rem;
    font-weight: 600;
    color: var(--text-primary, #f8fafc);
    background: var(--bg-surface, #0f172a);
    padding: 2px 8px;
    border-radius: 4px;
    border: 1px solid var(--border-color, #334155);
    min-width: 36px;
    text-align: center;
  }

  .no-params-card {
    display: flex;
    align-items: flex-start;
    gap: 12px;
    padding: 14px;
    background: var(--bg-surface-elevated, #1e293b);
    border: 1px solid var(--border-color, #334155);
    border-radius: var(--border-radius-md, 8px);
  }

  .no-params-card .info-icon {
    font-size: 1.2rem;
    line-height: 1.2;
  }

  .info-title {
    font-size: 0.85rem;
    font-weight: 600;
    color: var(--text-primary, #f8fafc);
    margin-bottom: 2px;
  }

  .no-params-card p {
    margin: 0;
    font-size: 0.8rem;
    color: var(--text-secondary, #94a3b8);
    line-height: 1.4;
  }
</style>
