<script lang="ts">
  import { Handle, Position } from '@xyflow/svelte';
  import { pipelineState } from '$lib/state/pipeline.svelte';
  import type { PipelineStep } from '$lib/state/pipeline.svelte';

  let { data, id, selected } = $props<{
    data: PipelineStep;
    id: string;
    selected?: boolean;
  }>();

</script>

<div
  class="condition-node"
  class:selected={selected}
  role="button"
  tabindex="0"
>
  <Handle type="target" position={Position.Left} />

  <div class="node-content">
    <div class="condition-header">
      <span class="icon">❓</span>
      <span class="label">Condition</span>
      <button
        class="remove-btn"
        onclick={(e) => { e.stopPropagation(); pipelineState.removeStep(id); }}
        title="Remove step"
        disabled={pipelineState.isRunning}
      >×</button>
    </div>
    <div class="condition-summary">
      IF: {data.condition ? Object.keys(data.condition).join(', ') : 'Not set'}
    </div>
  </div>

  <Handle type="source" position={Position.Right} id="true" />
  <Handle type="source" position={Position.Bottom} id="false" />

  <div class="handle-label true-label">True</div>
  <div class="handle-label false-label">False</div>
</div>

<style>
  .condition-node {
    width: 140px;
    padding: 12px;
    background: var(--bg-surface, #1e293b);
    border: 1px solid var(--accent-secondary, #eab308); /* Yellow/Orange border for condition */
    border-radius: 4px;
    cursor: grab;
    transition: border-color 0.15s, box-shadow 0.15s;
    user-select: none;
    position: relative;
    color: var(--text-primary, #f8fafc);
  }

  .condition-node.selected {
    box-shadow: 0 0 0 2px var(--accent-secondary, #eab308);
  }

  .condition-header {
    display: flex;
    align-items: center;
    gap: 6px;
    margin-bottom: 4px;
  }

  .icon {
    font-size: 1.1rem;
  }

  .label {
    font-size: 0.85rem;
    font-weight: 600;
    flex: 1;
  }

  .remove-btn {
    background: none;
    border: none;
    color: var(--text-muted, #94a3b8);
    cursor: pointer;
    font-size: 1rem;
    line-height: 1;
    padding: 0 2px;
  }

  .remove-btn:hover:not(:disabled) {
    color: var(--error-color, #ef4444);
  }

  .condition-summary {
    font-size: 0.75rem;
    color: var(--text-secondary, #cbd5e1);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .handle-label {
    position: absolute;
    font-size: 0.65rem;
    font-weight: bold;
    color: var(--text-secondary, #94a3b8);
  }

  .true-label {
    right: -24px;
    top: 50%;
    transform: translateY(-50%);
  }

  .false-label {
    bottom: -16px;
    left: 50%;
    transform: translateX(-50%);
  }
</style>
