<script lang="ts">
  import { onMount } from 'svelte';
  import { appState } from '$lib/state/app.svelte';
  import { jobsState } from '$lib/state/jobs.svelte';

  const navItems = [
    { id: 'documents', label: 'Documents', icon: '📄' },
    { id: 'pipeline', label: 'Pipeline', icon: '🔗' },
    { id: 'api', label: 'API & Gateway', icon: '⚡' },
    { id: 'settings', label: 'Settings', icon: '⚙️' }
  ];
</script>

<aside class="sidebar collapsed" aria-label="Main navigation">
  <div class="sidebar-header">
    <div class="brand-icon">P</div>
  </div>

  <nav class="sidebar-nav" aria-label="Navigation">
    {#each navItems as item (item.id)}
      {#if item.id !== 'api' || appState.developerMode}
        <button
          class="nav-item"
          class:active={appState.activeTab === item.id}
          onclick={() => appState.setActiveTab(item.id)}
          title={item.label}
          id={`nav-${item.id}`}
        >
          <span class="icon">{item.icon}</span>
        </button>
      {/if}
    {/each}
  </nav>

  <div class="sidebar-footer">
    {#if jobsState.jobs.filter(j => j.status === 'running').length > 0}
      <div class="job-indicator" title="{jobsState.jobs.filter(j => j.status === 'running').length} jobs active">
        <span class="pulse-dot"></span>
        <span class="job-count">{jobsState.jobs.filter(j => j.status === 'running').length}</span>
      </div>
    {/if}
    <div class="version-label" title="v1.0.0">v1</div>
  </div>
</aside>

<style>
  .sidebar { width: var(--layout-sidebar-width, 56px); height: 100%; background-color: var(--surface-0, #0d0d0f); border-right: 1px solid var(--border-color, #2D313C); display: flex; flex-direction: column; align-items: center; padding-top: 16px; padding-bottom: 16px; z-index: 50; flex-shrink: 0; }
  .sidebar-header { margin-bottom: 32px; }
  .brand-icon { width: 32px; height: 32px; background: var(--accent-primary, #5E6AD2); color: white; border-radius: 8px; display: flex; align-items: center; justify-content: center; font-weight: bold; font-size: 1.1rem; }
  .sidebar-nav { display: flex; flex-direction: column; gap: 16px; width: 100%; align-items: center; flex: 1; }
  .nav-item { width: 40px; height: 40px; border-radius: 8px; display: flex; align-items: center; justify-content: center; background: transparent; border: none; cursor: pointer; color: var(--text-secondary, #A1A7B3); transition: all 0.2s; }
  .nav-item:hover { background-color: var(--surface-2, #1c1c1f); color: var(--text-primary, #FFFFFF); }
  .nav-item.active { background-color: var(--surface-3, #242428); color: var(--accent-primary, #5E6AD2); }
  .icon { font-size: 1.25rem; }
  .sidebar-footer { margin-top: auto; display: flex; flex-direction: column; align-items: center; gap: 8px; }
  .job-indicator { display: flex; align-items: center; gap: 4px; background: var(--surface-2); padding: 4px; border-radius: 12px; }
  .pulse-dot { width: 6px; height: 6px; background: var(--watching-green, #22c55e); border-radius: 50%; animation: pulse 2s infinite; }
  .job-count { font-size: 0.65rem; color: var(--text-primary); }
  @keyframes pulse { 0% { transform: scale(1); opacity: 1; } 50% { transform: scale(1.5); opacity: 0.5; } 100% { transform: scale(1); opacity: 1; } }
  .version-label { font-size: 0.6rem; color: var(--text-muted, #6B7280); font-weight: bold; }
</style>
