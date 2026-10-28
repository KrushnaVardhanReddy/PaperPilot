<script lang="ts">
  import type { PipelineStep, WorkflowConditionLeaf } from '$lib/state/pipeline.svelte';
  import { pipelineState } from '$lib/state/pipeline.svelte';

  let { step, index, isRunning } = $props<{
    step: PipelineStep;
    index: number;
    isRunning: boolean;
  }>();

  function getConditionSummary(cond: WorkflowConditionLeaf): string {
    const parts = [];
    if (cond.if_scanned) parts.push('Scanned');
    if (cond.if_page_count_gt !== undefined) parts.push(`Pages > ${cond.if_page_count_gt}`);
    if (cond.comparator && cond.value !== undefined) parts.push(`Pages ${cond.comparator} ${cond.value}`);
    if (cond.text_contains) parts.push(`Contains "${cond.text_contains}"`);
    if (cond.has_form_fields) parts.push('Has Forms');
    if (cond.is_signed) parts.push('Signed');
    if (cond.is_encrypted) parts.push('Encrypted');

    return parts.length > 0 ? parts.join(', ') : 'Custom';
  }
</script>

<div
  class="step-card"
  id={`pipeline-step-${step.id}`}
  class:selected={pipelineState.selectedStepId === step.id}
  class:running={isRunning}
  class:completed={pipelineState.currentStepIndex !== null &&
                    pipelineState.currentStepIndex > index}
  class:conditional={!!step.condition}
  onclick={() => pipelineState.selectStep(step.id)}
  role="button"
  tabindex="0"
  onkeydown={(e) => { if (e.key === 'Enter' || e.key === ' ') pipelineState.selectStep(step.id); }}
>
  <div class="step-header">
    <span class="step-icon">{step.icon}</span>
    <span class="step-label">{step.label}</span>
    <button
      class="remove-btn"
      id={`remove-step-${step.id}`}
      onclick={(e) => { e.stopPropagation(); pipelineState.removeStep(step.id); }}
      title="Remove step"
      disabled={pipelineState.isRunning}
    >×</button>
  </div>
  <div class="step-index">Step {index + 1}</div>

  {#if step.condition}
    <div class="step-condition-badge" title="Conditional Step">
      ⚡ {getConditionSummary(step.condition)}
    </div>
  {/if}

  {#if step.status === 'skipped'}
    <div class="step-skipped-badge">⏭️ Skipped</div>
  {/if}

  {#if isRunning}
    <div class="step-running-indicator" aria-label="Running">⏳</div>
  {/if}
</div>

<style>
  .step-card {
    width: 130px;
    flex-shrink: 0;
    padding: 12px;
    background: var(--bg-surface);
    border: 1px solid var(--border-color);
    border-radius: var(--border-radius-md);
    cursor: grab;
    transition: border-color 0.15s, box-shadow 0.15s;
    user-select: none;
    position: relative;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .step-card:hover {
    border-color: var(--accent-primary);
    box-shadow: 0 0 0 2px var(--accent-primary-alpha);
  }

  .step-card.selected {
    border-color: var(--accent-primary, #3b82f6);
    background: var(--bg-surface-elevated, #1e293b);
    box-shadow: 0 0 0 2px var(--accent-primary, #3b82f6);
  }

  .step-card.running {
    border-color: var(--accent-primary);
    box-shadow: 0 0 0 2px var(--accent-primary-alpha);
    animation: pulse 1.2s infinite;
  }

  .step-card.completed {
    border-color: var(--success-color, #22c55e);
    opacity: 0.7;
  }

  .step-card.conditional {
    border-color: var(--warning-color, #f59e0b);
    box-shadow: 0 0 4px var(--warning-color-alpha, rgba(245, 158, 11, 0.4));
  }

  .step-card.conditional.selected {
    border-color: var(--warning-color, #f59e0b);
    box-shadow: 0 0 0 2px var(--warning-color, #f59e0b);
  }

  .step-header {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .step-icon {
    font-size: 1.2rem;
    flex-shrink: 0;
  }

  .step-label {
    font-size: 0.85rem;
    font-weight: 600;
    color: var(--text-primary);
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .remove-btn {
    background: none;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    font-size: 1rem;
    line-height: 1;
    padding: 0 2px;
    flex-shrink: 0;
  }

  .remove-btn:hover:not(:disabled) {
    color: var(--error-color, #ef4444);
  }

  .step-index {
    font-size: 0.7rem;
    color: var(--text-muted);
  }

  .step-condition-badge {
    font-size: 0.65rem;
    background: var(--warning-color-alpha, rgba(245, 158, 11, 0.2));
    color: var(--warning-color, #f59e0b);
    padding: 2px 4px;
    border-radius: 4px;
    margin-top: 4px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    text-align: center;
    border: 1px solid var(--warning-color-alpha, rgba(245, 158, 11, 0.4));
  }

  .step-skipped-badge {
    font-size: 0.7rem;
    color: var(--text-muted);
    font-style: italic;
    text-align: center;
    margin-top: 2px;
  }

  .step-running-indicator {
    position: absolute;
    top: 4px;
    right: 4px;
    font-size: 0.75rem;
  }

  @keyframes pulse {
    0%, 100% { box-shadow: 0 0 0 2px var(--accent-primary-alpha); }
    50% { box-shadow: 0 0 0 5px var(--accent-primary-alpha); }
  }
</style>
