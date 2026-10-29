<script lang="ts">
  import { Handle, Position } from '@xyflow/svelte';

  let { data } = $props<{
    data: { label: string; icon: string };
  }>();

</script>

<div class="trigger-node">
  <div class="trigger-content">
    <span class="icon">{data.icon}</span>
    <span class="label">{data.label}</span>
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
  }

  .trigger-content {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .icon {
    font-size: 1.2rem;
  }

  .label {
    font-size: 0.85rem;
    font-weight: 600;
  }
</style>
