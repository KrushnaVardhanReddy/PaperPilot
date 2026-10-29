<script lang="ts">
  import { onMount } from 'svelte';
  import { appState } from '$lib/state/app.svelte';
  import { libraryState } from '$lib/state/library.svelte';
  let { isOpen = false } = $props<{ isOpen?: boolean }>();
  let internalOpen = $state(isOpen);
  onMount(() => {
    const handleToggle = () => { internalOpen = !internalOpen; };
    window.addEventListener('paperpilot:toggle-drawer', handleToggle);
    return () => { window.removeEventListener('paperpilot:toggle-drawer', handleToggle); };
  });
</script>

<div class="library-panel-container" class:mobile-drawer={appState.isMobile} class:open={internalOpen}>
  {#if appState.isMobile && internalOpen}
    <button class="drawer-close" onclick={() => internalOpen = false}>×</button>
  {/if}
  <div class="library-panel-content">
    {#if libraryState.recentFiles.length > 0}
      <div class="library-section">
        <h3 class="section-title">Recent Files</h3>
        <ul class="item-list">
          {#each libraryState.recentFiles as file}
            <li class="item-row">
              <span class="item-label">{file.name}</span>
            </li>
          {/each}
        </ul>
      </div>
    {/if}
    {#if libraryState.smartCategories.length > 0}
      <div class="library-section">
        <h3 class="section-title">Smart Categories</h3>
        <ul class="item-list">
          {#each libraryState.smartCategories as category}
            <li class="item-row">
              <span class="item-label">{category.label}</span>
              <div class="item-badges">
                {#if category.toReview}
                  <span class="badge badge-amber">To Review</span>
                {/if}
                <span class="badge badge-count">{category.count}</span>
              </div>
            </li>
          {/each}
        </ul>
      </div>
    {/if}
    <div class="library-section">
      <h3 class="section-title">Watched Folders</h3>
      {#if libraryState.watchedFolders.length === 0}
        <div class="empty-state">No watched folders configured.</div>
      {:else}
        <ul class="item-list">
          {#each libraryState.watchedFolders as folder}
            <li class="item-row">
              <div class="folder-info">
                <span class="item-label">{folder.path.split('/').pop() || folder.path}</span>
                <span class="item-subtext">{folder.recipe}</span>
              </div>
              <span class="status-indicator" class:watching={folder.status === 'watching'} class:error={folder.status === 'error'}></span>
            </li>
          {/each}
        </ul>
      {/if}
    </div>
    {#if libraryState.sharedFiles.length > 0}
      <div class="library-section">
        <h3 class="section-title">Shared With You</h3>
        <ul class="item-list">
          {#each libraryState.sharedFiles as shared}
            <li class="item-row">
              <div class="shared-info">
                <span class="item-label">{shared.name}</span>
                <span class="item-subtext">from {shared.sender}</span>
              </div>
            </li>
          {/each}
        </ul>
      </div>
    {/if}
  </div>
</div>

<style>
  .library-panel-container { width: var(--layout-library-width); height: 100%; background-color: var(--surface-1); border-right: 1px solid var(--border-color); display: flex; flex-direction: column; overflow-y: auto; transition: transform 0.3s ease; }
  .mobile-drawer { position: fixed; top: 0; left: 0; z-index: 100; width: 280px; height: 100%; transform: translateX(-100%); }
  .mobile-drawer.open { transform: translateX(0); box-shadow: 4px 0 24px rgba(0,0,0,0.5); }
  .drawer-close { position: absolute; top: 16px; right: 16px; background: transparent; border: none; color: var(--text-primary); font-size: 1.5rem; cursor: pointer; }
  .library-panel-content { padding: 16px; display: flex; flex-direction: column; gap: 24px; margin-top: 24px; }
  .library-section { display: flex; flex-direction: column; gap: 8px; }
  .section-title { font-size: 0.75rem; font-weight: 600; text-transform: uppercase; color: var(--text-muted); letter-spacing: 0.05em; margin-bottom: 4px; }
  .item-list { list-style: none; padding: 0; margin: 0; display: flex; flex-direction: column; gap: 2px; }
  .item-row { display: flex; align-items: center; justify-content: space-between; padding: 8px 12px; border-radius: var(--border-radius-sm); cursor: pointer; transition: background-color 0.2s; }
  .item-row:hover { background-color: var(--surface-2); }
  .item-label { font-size: 0.875rem; color: var(--text-primary); }
  .item-subtext { font-size: 0.7rem; color: var(--text-secondary); display: block; }
  .item-badges { display: flex; align-items: center; gap: 6px; }
  .badge { font-size: 0.65rem; padding: 2px 6px; border-radius: 12px; font-weight: 600; }
  .badge-amber { background-color: rgba(245, 158, 11, 0.15); color: var(--review-amber); border: 1px solid rgba(245, 158, 11, 0.3); }
  .badge-count { background-color: var(--surface-3); color: var(--text-secondary); }
  .empty-state { font-size: 0.8rem; color: var(--text-muted); font-style: italic; padding: 8px 12px; }
  .status-indicator { width: 8px; height: 8px; border-radius: 50%; background-color: var(--text-muted); }
  .status-indicator.watching { background-color: var(--watching-green); box-shadow: 0 0 8px rgba(34, 197, 94, 0.4); }
  .status-indicator.error { background-color: #ef4444; }
  .folder-info, .shared-info { display: flex; flex-direction: column; }
</style>
