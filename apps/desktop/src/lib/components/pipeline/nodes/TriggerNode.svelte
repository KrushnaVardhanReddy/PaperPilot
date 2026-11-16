<script lang="ts">
  import { Handle, Position } from '@xyflow/svelte';

  let { data } = $props<{
    data: { label: string; icon: string; count?: number };
  }>();

</script>

<div class="trigger-node" class:batch={data.count && data.count > 1}>
  <div class="trigger-content">
    <span class="icon">{data.icon}</span>
    <span class="label">{data.label}</span>
    {#if data.count && data.count > 1}
      <span class="count-badge">{data.count}</span>
    {/if}
  </div>
  <Handle type="source" position={Position.Right} />
</div>

<style>
  .trigger-node {
    padding: 10px 16px;
    background: var(--bg-surface-elevated, #334155);
    border: 2px dashed var(--border-color, #475569);
    border-radius: var(--border-radius-full, 9999px);
    color: var(--text-primary, #f8fafc);
    user-select: none;
    cursor: default;
    max-width: 260px;
    transition: all 0.2s ease;
  }

  .trigger-node.batch {
    border-color: var(--accent-primary, #3b82f6);
    background: rgba(59, 130, 246, 0.15);
  }

  .trigger-content {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .icon {
    font-size: 1.2rem;
    line-height: 1;
    flex-shrink: 0;
  }

  .label {
    font-size: 0.85rem;
    font-weight: 600;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .count-badge {
    background: var(--accent-primary, #3b82f6);
    color: #ffffff;
    font-size: 0.7rem;
    font-weight: 700;
    padding: 2px 6px;
    border-radius: 10px;
    line-height: 1;
    flex-shrink: 0;
  }
</style>
