<script lang="ts">
  import type { PipelineStep } from '$lib/state/pipeline.svelte';
  import { pipelineState } from '$lib/state/pipeline.svelte';

  let { step, index, isRunning } = $props<{
    step: PipelineStep;
    index: number;
    isRunning: boolean;
  }>();
</script>

<div
  class="step-card"
  id={`pipeline-step-${step.id}`}
  class:running={isRunning}
  class:completed={pipelineState.currentStepIndex !== null &&
                    pipelineState.currentStepIndex > index}
>
  <div class="step-header">
    <span class="step-icon">{step.icon}</span>
    <span class="step-label">{step.label}</span>
    <button
      class="remove-btn"
      id={`remove-step-${step.id}`}
      onclick={() => pipelineState.removeStep(step.id)}
      title="Remove step"
      disabled={pipelineState.isRunning}
    >×</button>
  </div>
  <div class="step-index">Step {index + 1}</div>
  {#if isRunning}
    <div class="step-running-indicator" aria-label="Running">⏳</div>
  {/if}
</div>

<style>
  .step-card {
    width: 120px;
    flex-shrink: 0;
    padding: 12px;
    background: var(--bg-surface);
    border: 1px solid var(--border-color);
    border-radius: var(--border-radius-md);
    cursor: grab;
    transition: border-color 0.15s, box-shadow 0.15s;
    user-select: none;
    position: relative;
  }

  .step-card:hover {
    border-color: var(--accent-primary);
    box-shadow: 0 0 0 2px var(--accent-primary-alpha);
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
    margin-top: 6px;
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
