<script lang="ts">
  interface Props {
    jobName: string;
    percent: number; // 0-100
    currentPage?: number;
    totalPages?: number;
    visible?: boolean;
  }

  let { jobName, percent, currentPage, totalPages, visible = true }: Props = $props();

  const clampedPercent = $derived(Math.min(100, Math.max(0, percent)));
</script>

{#if visible}
  <div class="progress-wrapper" role="progressbar" aria-valuenow={clampedPercent} aria-valuemin={0} aria-valuemax={100}>
    <div class="progress-header">
      <span class="progress-job-name">{jobName}</span>
      <span class="progress-percent">{clampedPercent}%</span>
    </div>
    <div class="progress-track">
      <div class="progress-fill" style="width: {clampedPercent}%"></div>
    </div>
    {#if currentPage !== undefined && totalPages !== undefined}
      <div class="progress-detail">Page {currentPage} of {totalPages}</div>
    {/if}
  </div>
{/if}

<style>
  .progress-wrapper {
    width: 100%;
    padding: 0.75rem 0;
  }
  .progress-header {
    display: flex;
    justify-content: space-between;
    margin-bottom: 0.4rem;
    font-size: 0.85rem;
    color: var(--text-secondary, #aaa);
  }
  .progress-job-name {
    font-weight: 500;
    color: var(--text-primary, #eee);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 70%;
  }
  .progress-percent {
    font-variant-numeric: tabular-nums;
  }
  .progress-track {
    height: 6px;
    background: var(--bg-surface-hover, #2a2a3a);
    border-radius: 3px;
    overflow: hidden;
  }
  .progress-fill {
    height: 100%;
    background: linear-gradient(90deg, var(--accent-primary, #6c63ff), var(--accent-hover, #a78bfa));
    border-radius: 3px;
    transition: width 0.25s ease;
  }
  .progress-detail {
    margin-top: 0.3rem;
    font-size: 0.75rem;
    color: var(--text-muted, #666);
  }
</style>
