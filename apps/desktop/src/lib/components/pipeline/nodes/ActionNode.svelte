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

  const conditionSummary = $derived.by(() => {
    if (!data.condition) return null;
    const cond = data.condition as Record<string, any>;
    const parts: string[] = [];
    if (cond.if_page_count_gt !== undefined && cond.if_page_count_gt !== null && cond.if_page_count_gt !== '') {
      parts.push(`pages > ${cond.if_page_count_gt}`);
    }
    if (cond.if_scanned) parts.push('is scanned');
    if (cond.is_signed) parts.push('is signed');
    if (cond.is_encrypted) parts.push('is encrypted');
    if (cond.text_contains) parts.push(`contains "${cond.text_contains}"`);
    return parts.length > 0 ? parts.join(', ') : null;
  });
</script>

<div
  class="action-node"
  class:selected={isSelected}
  class:running={isRunning}
  class:completed={isCompleted}
  class:has-condition={!!conditionSummary}
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

  {#if conditionSummary}
    <div class="step-condition-badge" title={`Execution Rule: ${conditionSummary}`}>
      <span class="cond-icon">⚡</span>
      <span class="cond-text">IF: {conditionSummary}</span>
    </div>
  {/if}

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

  .action-node.has-condition {
    border-color: rgba(234, 179, 8, 0.55);
  }

  .action-node.has-condition:hover {
    border-color: #facc15;
    box-shadow: 0 0 0 2px rgba(234, 179, 8, 0.25);
  }

  .step-condition-badge {
    margin-top: 8px;
    display: flex;
    align-items: center;
    gap: 4px;
    background: rgba(234, 179, 8, 0.12);
    border: 1px solid rgba(234, 179, 8, 0.35);
    border-radius: 4px;
    padding: 3px 6px;
    font-size: 0.72rem;
    color: #fef08a;
    font-weight: 500;
  }

  .step-condition-badge .cond-icon {
    font-size: 0.75rem;
    color: #facc15;
    flex-shrink: 0;
    line-height: 1;
  }

  .step-condition-badge .cond-text {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  @keyframes pulse {
    0%, 100% { box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.3); }
    50% { box-shadow: 0 0 0 5px rgba(59, 130, 246, 0.3); }
  }
</style>
