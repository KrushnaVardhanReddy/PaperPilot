<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { safeInvoke } from '$lib/utils/tauri';
  import { slide } from 'svelte/transition';

  let { isOpen = $bindable(false) } = $props<{ isOpen: boolean }>();

  type JobStatus = 'Pending' | 'Running' | 'Completed' | 'Failed';

  interface JobRecord {
    id: string;
    operation: string;
    status: JobStatus;
    input_path?: string;
    output_path?: string;
    parameters_json: string;
    error_message?: string;
    progress_percentage: number;
    created_at: string;
    updated_at: string;
  }

  let jobs = $state<JobRecord[]>([]);
  let activeFilter = $state<'All' | 'Running' | 'Completed' | 'Failed'>('All');
  let isLoading = $state(false);
  let pollInterval: ReturnType<typeof setInterval>;

  async function fetchJobs() {
    isLoading = true;
    try {
      const data = await safeInvoke('get_jobs_history', { limit: 50 });
      if (data) {
        jobs = data as JobRecord[];
      }
    } catch (err) {
      console.error("Failed to fetch job history:", err);
    } finally {
      isLoading = false;
    }
  }

  function startPolling() {
    fetchJobs();
    pollInterval = setInterval(fetchJobs, 5000);
  }

  function stopPolling() {
    if (pollInterval) clearInterval(pollInterval);
  }

  $effect(() => {
    if (isOpen) {
      startPolling();
    } else {
      stopPolling();
    }
  });

  onDestroy(() => {
    stopPolling();
  });

  let filteredJobs = $derived(
    activeFilter === 'All'
      ? jobs
      : jobs.filter(j => j.status === activeFilter)
  );

  function getStatusIcon(status: JobStatus) {
    switch (status) {
      case 'Completed': return '✅';
      case 'Failed': return '❌';
      case 'Running': return '⏳';
      default: return '🕒';
    }
  }

  function formatTime(isoStr: string) {
    const d = new Date(isoStr);
    return d.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
  }
  async function clearJobs() {
    try {
      await safeInvoke('clear_jobs_history', {});
      jobs = [];
    } catch (err) {
      console.error("Failed to clear job history:", err);
    }
  }
</script>

{#if isOpen}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
  <div class="drawer-backdrop" onclick={() => isOpen = false} role="presentation">
    <div
      class="drawer-panel"
      transition:slide={{ axis: 'x', duration: 300 }}
      onclick={(e) => e.stopPropagation()}
      role="dialog"
      aria-label="Job Queue"
    >
      <div class="drawer-header">
        <h3>Background Jobs</h3>
        <button class="close-btn" onclick={() => isOpen = false} title="Close">×</button>
      </div>

      <div class="drawer-actions">
        <div class="filter-tabs">
          <button class:active={activeFilter === 'All'} onclick={() => activeFilter = 'All'}>All</button>
          <button class:active={activeFilter === 'Running'} onclick={() => activeFilter = 'Running'}>Running</button>
          <button class:active={activeFilter === 'Completed'} onclick={() => activeFilter = 'Completed'}>Completed</button>
          <button class:active={activeFilter === 'Failed'} onclick={() => activeFilter = 'Failed'}>Failed</button>
        </div>
        <div class="header-btns">
          <button class="action-btn" onclick={clearJobs} title="Clear history" disabled={jobs.length === 0}>
            🗑️
          </button>
          <button class="refresh-btn" onclick={fetchJobs} disabled={isLoading} title="Refresh">
            {isLoading ? '...' : '↻'}
          </button>
        </div>
      </div>

      <div class="job-list">
        {#if filteredJobs.length === 0}
          <div class="empty-state">No jobs found.</div>
        {/if}
        {#each filteredJobs as job (job.id)}
          <div class="job-item">
            <div class="job-header">
              <span class="job-op">{job.operation}</span>
              <span class="job-time">{formatTime(job.created_at)}</span>
            </div>
            <div class="job-status">
              <span class={`status-pill status-${job.status.toLowerCase()}`}>
                {getStatusIcon(job.status)} {job.status}
              </span>
              {#if job.status === 'Running'}
                <div class="progress-bar-bg">
                  <div class="progress-bar-fill" style="width: {job.progress_percentage}%"></div>
                </div>
                <span class="progress-text">{job.progress_percentage}%</span>
              {/if}
            </div>
            {#if job.status === 'Failed' && job.error_message}
              <div class="job-error">{job.error_message}</div>
            {/if}
          </div>
        {/each}
      </div>
    </div>
  </div>
{/if}

<style>
  .drawer-backdrop {
    position: fixed;
    top: 0;
    right: 0;
    bottom: 0;
    left: 0;
    background: rgba(0, 0, 0, 0.4);
    z-index: 9999;
    display: flex;
    justify-content: flex-end;
  }

  .drawer-panel {
    width: 350px;
    background: var(--bg-surface, #1e1e24);
    border-left: 1px solid var(--border-color, #2a2a35);
    display: flex;
    flex-direction: column;
    height: 100%;
    box-shadow: -4px 0 15px rgba(0,0,0,0.3);
  }

  .drawer-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 16px;
    border-bottom: 1px solid var(--border-color, #2a2a35);
  }

  .drawer-header h3 {
    margin: 0;
    color: var(--text-primary, #fff);
  }

  .close-btn {
    background: transparent;
    border: none;
    color: var(--text-muted, #9ca3af);
    font-size: 1.5rem;
    cursor: pointer;
  }

  .drawer-actions {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px 16px;
    background: var(--bg-secondary, #13131a);
    border-bottom: 1px solid var(--border-color, #2a2a35);
  }

  .filter-tabs {
    display: flex;
    gap: 4px;
  }

  .filter-tabs button {
    background: transparent;
    border: 1px solid transparent;
    color: var(--text-secondary, #d1d5db);
    padding: 4px 8px;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.8rem;
  }

  .filter-tabs button.active {
    background: rgba(94, 106, 210, 0.2);
    color: var(--accent-primary, #5e6ad2);
    border-color: var(--accent-primary, #5e6ad2);
  }

  .header-btns {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .action-btn {
    background: transparent;
    border: none;
    color: var(--text-secondary);
    cursor: pointer;
    font-size: 0.9rem;
    padding: 2px 4px;
    border-radius: 4px;
    transition: opacity 0.2s;
  }

  .action-btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .action-btn:not(:disabled):hover {
    background: rgba(239, 68, 68, 0.15);
  }

  .refresh-btn {
    background: transparent;
    border: none;
    color: var(--text-secondary);
    cursor: pointer;
    font-size: 1.1rem;
  }

  .job-list {
    flex: 1;
    overflow-y: auto;
    padding: 12px;
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .empty-state {
    text-align: center;
    color: var(--text-muted);
    margin-top: 32px;
    font-size: 0.9rem;
  }

  .job-item {
    background: rgba(255,255,255,0.03);
    border: 1px solid var(--border-color, #2a2a35);
    border-radius: 8px;
    padding: 12px;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .job-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .job-op {
    font-weight: 600;
    color: var(--text-primary);
    font-size: 0.9rem;
  }

  .job-time {
    font-size: 0.75rem;
    color: var(--text-muted);
  }

  .job-status {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .status-pill {
    font-size: 0.75rem;
    padding: 2px 6px;
    border-radius: 12px;
    display: inline-flex;
    align-items: center;
    gap: 4px;
    font-weight: 500;
  }

  .status-running {
    background: rgba(94, 106, 210, 0.15);
    color: #8b96f5;
  }

  .status-completed {
    background: rgba(34, 197, 94, 0.15);
    color: #4ade80;
  }

  .status-failed {
    background: rgba(239, 68, 68, 0.15);
    color: #f87171;
  }

  .status-pending {
    background: rgba(156, 163, 175, 0.15);
    color: #d1d5db;
  }

  .progress-bar-bg {
    flex: 1;
    height: 4px;
    background: var(--border-color);
    border-radius: 2px;
    overflow: hidden;
  }

  .progress-bar-fill {
    height: 100%;
    background: var(--accent-primary, #5e6ad2);
    transition: width 0.3s ease;
  }

  .progress-text {
    font-size: 0.7rem;
    color: var(--text-secondary);
  }

  .job-error {
    font-size: 0.8rem;
    color: #f87171;
    background: rgba(239, 68, 68, 0.05);
    padding: 6px 8px;
    border-radius: 4px;
    border-left: 2px solid #ef4444;
  }
</style>
