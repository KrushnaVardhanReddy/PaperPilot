<script lang="ts">
  import { appState } from '$lib/state/app.svelte';
  import { libraryState } from '$lib/state/library.svelte';
  import { startWatching, stopWatching } from '$lib/services/watchedFolders';
  import { onMount, onDestroy } from 'svelte';

  onMount(() => {
    startWatching();
  });

  onDestroy(() => {
    stopWatching();
  });

  let isOpen = $state(false);

  // Close drawer if clicking outside on mobile
  function handleOutsideClick() {
    if (appState.isMobile && isOpen) {
      isOpen = false;
    }
  }

  // Toggle from outside (e.g. toolbar)
  export function toggle() {
    isOpen = !isOpen;
  }
</script>

{#if appState.isMobile && isOpen}
  <div class="library-overlay" onclick={handleOutsideClick} onkeydown={(e) => e.key === 'Escape' && (isOpen = false)} tabindex="0" role="button" aria-label="Close library overlay"></div>
{/if}

<aside
  class="library-panel"
  class:mobile-drawer={appState.isMobile}
  class:open={isOpen}
>
  <div class="library-content">
    <div class="library-section">
      <h3 class="section-title">Recent Files</h3>
      <ul class="file-list">
        {#each libraryState.recentFiles as file}
          <li class="file-item">
            <span class="file-icon">📄</span>
            <span class="file-name">{file.name}</span>
          </li>
        {/each}
      </ul>
    </div>

    <div class="library-section">
      <h3 class="section-title">Smart Categories</h3>
      <ul class="file-list">
        {#each libraryState.smartCategories as cat}
          <li class="file-item">
            <span class="cat-label">{cat.label}</span>
            <span class="cat-count">{cat.count}</span>
            {#if cat.toReview}
              <span class="review-badge" title="To Review">●</span>
            {/if}
          </li>
        {/each}
      </ul>
    </div>

    <div class="library-section">
      <h3 class="section-title">Watched Folders</h3>
      <ul class="file-list">
        {#each libraryState.watchedFolders as folder}
          <li class="file-item folder-item">
            <div class="folder-info">
              <span class="file-name">{folder.path}</span>
              <span class="folder-recipe">{folder.recipe}</span>
            </div>
            <div class="folder-status" class:watching={folder.status === 'watching'} class:error={folder.status === 'error'}>
              {folder.status === 'watching' ? '●' : folder.status === 'error' ? '!' : '⏸'}
            </div>
          </li>
        {/each}
      </ul>
    </div>

    <div class="library-section">
      <h3 class="section-title">Shared With You</h3>
      <ul class="file-list">
        {#each libraryState.sharedFiles as shared}
          <li class="file-item">
            {#if shared.avatarUrl}
              <img src={shared.avatarUrl} alt={shared.sender} class="sender-avatar" />
            {:else}
               <div class="sender-avatar placeholder">{shared.sender.charAt(0)}</div>
            {/if}
            <span class="file-name">{shared.name}</span>
          </li>
        {/each}
      </ul>
    </div>
  </div>
</aside>

<style>
  .library-overlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    background: rgba(0, 0, 0, 0.6);
    z-index: 40;
  }

  .library-panel {
    width: var(--layout-library-width, 240px);
    background-color: var(--bg-surface, #1e1e24);
    border-right: 1px solid var(--border-color, #2a2a35);
    display: flex;
    flex-direction: column;
    height: 100%;
    flex-shrink: 0;
    overflow-y: auto;
    transition: transform 0.3s ease;
  }

  .library-panel.mobile-drawer {
    position: fixed;
    top: 0;
    left: 0;
    height: 100vh;
    z-index: 50;
    transform: translateX(-100%);
    box-shadow: 4px 0 12px rgba(0, 0, 0, 0.5);
  }

  .library-panel.mobile-drawer.open {
    transform: translateX(0);
  }

  .library-content {
    padding: 16px;
    display: flex;
    flex-direction: column;
    gap: 24px;
  }

  .library-section {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .section-title {
    font-size: 11px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--text-muted, #9ca3af);
    margin-bottom: 4px;
  }

  .file-list {
    list-style: none;
    padding: 0;
    margin: 0;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .file-item {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 6px 8px;
    border-radius: 6px;
    cursor: pointer;
    transition: background 0.15s ease;
    font-size: 13px;
    color: var(--text-primary, #ffffff);
  }

  .file-item:hover {
    background-color: var(--bg-surface-hover, rgba(255, 255, 255, 0.1));
  }

  .file-name {
    flex: 1;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .cat-label {
    flex: 1;
  }

  .cat-count {
    font-size: 11px;
    color: var(--text-muted, #9ca3af);
    background: rgba(255, 255, 255, 0.05);
    padding: 2px 6px;
    border-radius: 10px;
  }

  .review-badge {
    color: var(--review-amber, #f59e0b);
    font-size: 12px;
  }

  .folder-item {
    align-items: flex-start;
  }

  .folder-info {
    display: flex;
    flex-direction: column;
    flex: 1;
    overflow: hidden;
  }

  .folder-recipe {
    font-size: 10px;
    color: var(--text-muted, #9ca3af);
  }

  .folder-status {
    font-size: 12px;
  }
  .folder-status.watching {
    color: var(--watching-green, #22c55e);
    animation: pulse 2s infinite;
  }
  .folder-status.error {
    color: #ef4444;
  }

  @keyframes pulse {
    0% { opacity: 1; }
    50% { opacity: 0.4; }
    100% { opacity: 1; }
  }

  .sender-avatar {
    width: 20px;
    height: 20px;
    border-radius: 50%;
    object-fit: cover;
  }

  .sender-avatar.placeholder {
    background: var(--accent-primary, #5e6ad2);
    color: white;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 10px;
    font-weight: 600;
  }
</style>
