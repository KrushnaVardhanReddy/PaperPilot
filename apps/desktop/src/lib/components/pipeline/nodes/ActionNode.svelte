<script lang="ts">
  import { Handle, Position } from '@xyflow/svelte';
  import { pipelineState } from '$lib/state/pipeline.svelte';
  import type { PipelineStep } from '$lib/state/pipeline.svelte';

  let { data, id, selected } = $props<{
    data: PipelineStep;
    id: string;
    selected?: boolean;
  }>();

  const isSelected = $derived(
    selected || pipelineState.selectedStepId === id
  );

  const isRunning = $derived(
    pipelineState.isRunning && pipelineState.steps[pipelineState.currentStepIndex ?? -1]?.id === id
  );

  const isCompleted = $derived(
    pipelineState.currentStepIndex !== null &&
    pipelineState.steps.findIndex(s => s.id === id) < pipelineState.currentStepIndex
  );
</script>

<div
  class="action-node"
  class:selected={isSelected}
  class:running={isRunning}
  class:completed={isCompleted}
  role="button"
  tabindex="0"
  onclick={() => pipelineState.selectStep(id)}
  onkeydown={(e) => { if (e.key === 'Enter' || e.key === ' ') pipelineState.selectStep(id); }}
>
  <Handle type="target" position={Position.Left} />

  <div class="step-header">
    <span class="step-icon">{data.icon}</span>
    <span class="step-label">{data.label}</span>
    <button
      class="remove-btn"
      onclick={(e) => { e.stopPropagation(); pipelineState.removeStep(id); }}
      title="Remove step"
      disabled={pipelineState.isRunning}
    >×</button>
  </div>
  {#if isRunning}
    <div class="step-running-indicator" aria-label="Running">⏳</div>
  {/if}

  <Handle type="source" position={Position.Right} />
</div>

<style>
  .action-node {
    min-width: 150px;
    max-width: 220px;
    width: auto;
    padding: 12px 14px;
    background: var(--bg-surface, #1e293b);
    border: 1px solid var(--border-color, #334155);
    border-radius: var(--border-radius-md, 8px);
    cursor: pointer;
    transition: border-color 0.15s, box-shadow 0.15s, background-color 0.15s;
    user-select: none;
    position: relative;
    color: var(--text-primary, #f8fafc);
  }

  .action-node:hover {
    border-color: var(--accent-primary, #3b82f6);
    box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.3);
  }

  .action-node.selected {
    border-color: var(--accent-primary, #3b82f6);
    background: var(--bg-surface-elevated, #1e293b);
    box-shadow: 0 0 0 2px var(--accent-primary, #3b82f6);
  }

  .action-node.running {
    border-color: var(--accent-primary, #3b82f6);
    box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.3);
    animation: pulse 1.2s infinite;
  }

  .action-node.completed {
    border-color: var(--success-color, #22c55e);
    opacity: 0.8;
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
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .remove-btn {
    background: none;
    border: none;
    color: var(--text-muted, #94a3b8);
    cursor: pointer;
    font-size: 1rem;
    line-height: 1;
    padding: 0 2px;
    flex-shrink: 0;
  }

  .remove-btn:hover:not(:disabled) {
    color: var(--error-color, #ef4444);
  }

  .step-running-indicator {
    position: absolute;
    top: -10px;
    right: -10px;
    font-size: 1rem;
  }

  @keyframes pulse {
    0%, 100% { box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.3); }
    50% { box-shadow: 0 0 0 5px rgba(59, 130, 246, 0.3); }
  }
</style>
