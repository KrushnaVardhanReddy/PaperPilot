<script lang="ts">
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { emit } from '@tauri-apps/api/event';
  import { onMount } from 'svelte';
  import { appState } from '$lib/state/app.svelte';

  let activeMenu: string | null = $state(null);

  onMount(() => {
    const handleClickOutside = (e: MouseEvent) => {
      if (!(e.target as Element).closest('.menu-item')) activeMenu = null;
    };
    document.addEventListener('click', handleClickOutside);

    // Global Cmd+K / Ctrl+K listener for the global command bar
    const handleKeyDown = (e: KeyboardEvent) => {
      if ((e.ctrlKey || e.metaKey) && e.key === 'k') {
        e.preventDefault();
        // Fire custom event to open the command palette
        window.dispatchEvent(new CustomEvent('open-command-palette'));
      }
    };
    document.addEventListener('keydown', handleKeyDown);

    return () => {
      document.removeEventListener('click', handleClickOutside);
      document.removeEventListener('keydown', handleKeyDown);
    };
  });

  function toggleMenu(menu: string, e: MouseEvent) {
    e.stopPropagation();
    activeMenu = activeMenu === menu ? null : menu;
  }

  async function triggerEvent(eventName: string) {
    activeMenu = null;
    await emit(eventName);
  }

  function handleSearchClick() {
     window.dispatchEvent(new CustomEvent('open-command-palette'));
  }
</script>

<div class="titlebar" data-tauri-drag-region>
  <div class="titlebar-menus" data-tauri-drag-region>
    <div class="brand" data-tauri-drag-region>
      <img src="/favicon.png" alt="PaperPilot" class="brand-icon" />
      <span>PaperPilot</span>
    </div>

    <div class="menu-item" class:active={activeMenu === 'file'}>
      <button class="menu-btn" onclick={(e) => toggleMenu('file', e)}>File</button>
      {#if activeMenu === 'file'}
        <div class="dropdown">
          <button onclick={() => triggerEvent('menu-open-file')}>Open <span class="shortcut">Ctrl+O</span></button>
          <button onclick={() => triggerEvent('menu-save-annotations')}>Save Annotations <span class="shortcut">Ctrl+S</span></button>
          <hr />
          <button onclick={() => triggerEvent('menu-settings')}>Settings <span class="shortcut">Ctrl+,</span></button>
          <hr />
          <button onclick={() => getCurrentWindow().close()}>Exit</button>
        </div>
      {/if}
    </div>

    <div class="menu-item" class:active={activeMenu === 'edit'}>
      <button class="menu-btn" onclick={(e) => toggleMenu('edit', e)}>Edit</button>
      {#if activeMenu === 'edit'}
        <div class="dropdown">
          <button onclick={() => triggerEvent('menu-undo')}>Undo <span class="shortcut">Ctrl+Z</span></button>
          <button onclick={() => triggerEvent('menu-redo')}>Redo <span class="shortcut">Ctrl+Shift+Z</span></button>
        </div>
      {/if}
    </div>

    <div class="menu-item" class:active={activeMenu === 'view'}>
      <button class="menu-btn" onclick={(e) => toggleMenu('view', e)}>View</button>
      {#if activeMenu === 'view'}
        <div class="dropdown">
          <button onclick={() => triggerEvent('menu-zoom-in')}>Zoom In <span class="shortcut">Ctrl+=</span></button>
          <button onclick={() => triggerEvent('menu-zoom-out')}>Zoom Out <span class="shortcut">Ctrl+-</span></button>
          <button onclick={() => triggerEvent('menu-fit-width')}>Fit Width <span class="shortcut">Ctrl+0</span></button>
          <hr />
          <button onclick={() => triggerEvent('menu-toggle-diff')}>Compare Documents (Visual Diff) <span class="shortcut">Ctrl+D</span></button>
        </div>
      {/if}
    </div>

    <div class="menu-item" class:active={activeMenu === 'window'}>
      <button class="menu-btn" onclick={(e) => toggleMenu('window', e)}>Window</button>
      {#if activeMenu === 'window'}
        <div class="dropdown">
          <button onclick={() => getCurrentWindow().minimize()}>Minimize</button>
          <button onclick={() => getCurrentWindow().toggleMaximize()}>Maximize</button>
          <button onclick={() => getCurrentWindow().close()}>Close</button>
        </div>
      {/if}
    </div>
  </div>

  <div class="titlebar-center" data-tauri-drag-region>
    <!-- Mode Switcher -->
    <div class="mode-switcher">
      <button
        class="mode-btn {appState.activeTab === 'documents' ? 'active' : ''}"
        onclick={() => appState.setActiveTab('documents')}
      >
        📄 Document Studio
      </button>
      <button
        class="mode-btn {appState.activeTab === 'pipeline' ? 'active' : ''}"
        onclick={() => appState.setActiveTab('pipeline')}
      >
        🔗 Automations
      </button>
    </div>

    <!-- Global Search -->
    <button class="global-search-bar" onclick={handleSearchClick}>
      <span class="search-icon">🔍</span>
      <span class="search-text">Search 75 tools or documents (Ctrl+K)...</span>
    </button>
  </div>

  <div class="window-controls">
    <button class="control-btn" aria-label="Minimize window" onclick={() => getCurrentWindow().minimize()}>
      <svg width="10" height="10" viewBox="0 0 10 10"><rect x="1" y="4" width="8" height="1" fill="currentColor"/></svg>
    </button>
    <button class="control-btn" aria-label="Maximize window" onclick={() => getCurrentWindow().toggleMaximize()}>
      <svg width="10" height="10" viewBox="0 0 10 10"><rect x="1.5" y="1.5" width="7" height="7" fill="none" stroke="currentColor" stroke-width="1"/></svg>
    </button>
    <button class="control-btn close" aria-label="Close window" onclick={() => getCurrentWindow().close()}>
      <svg width="10" height="10" viewBox="0 0 10 10"><path d="M2,2 L8,8 M8,2 L2,8" stroke="currentColor" stroke-width="1" fill="none"/></svg>
    </button>
  </div>
</div>

<style>
  .titlebar {
    height: 38px;
    background-color: var(--bg-secondary);
    border-bottom: 1px solid var(--border-color);
    display: flex;
    justify-content: space-between;
    align-items: center;
    width: 100%;
    user-select: none;
    flex-shrink: 0;
    font-size: 13px;
    position: relative;
  }

  .titlebar-menus {
    display: flex;
    align-items: center;
    padding-left: 8px;
    flex: 1;
  }

  .titlebar-center {
    display: flex;
    align-items: center;
    gap: 16px;
    position: absolute;
    left: 50%;
    transform: translateX(-50%);
  }

  .mode-switcher {
    display: flex;
    background: var(--bg-surface);
    border: 1px solid var(--border-color);
    border-radius: 6px;
    padding: 2px;
  }

  .mode-btn {
    background: transparent;
    border: none;
    color: var(--text-secondary);
    padding: 4px 12px;
    font-size: 12px;
    border-radius: 4px;
    cursor: pointer;
    transition: all 0.2s;
  }

  .mode-btn:hover {
    color: var(--text-primary);
  }

  .mode-btn.active {
    background: var(--bg-primary);
    color: var(--text-primary);
    box-shadow: 0 1px 2px rgba(0,0,0,0.1);
  }

  .global-search-bar {
    display: flex;
    align-items: center;
    gap: 8px;
    background: var(--bg-surface);
    border: 1px solid var(--border-color);
    border-radius: 6px;
    padding: 4px 12px;
    height: 26px;
    width: 300px;
    color: var(--text-muted);
    cursor: pointer;
    transition: all 0.2s;
  }

  .global-search-bar:hover {
    background: var(--bg-surface-hover);
    border-color: var(--text-secondary);
    color: var(--text-secondary);
  }

  .search-text {
    font-size: 12px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .brand {
    display: flex;
    align-items: center;
    gap: 8px;
    font-weight: 600;
    margin-right: 16px;
    margin-left: 8px;
    color: var(--text-primary);
  }

  .brand-icon {
    width: 16px;
    height: 16px;
    border-radius: 3px;
    object-fit: contain;
  }

  .menu-item {
    position: relative;
    height: 100%;
    display: flex;
    align-items: center;
  }

  .menu-btn {
    background: transparent;
    border: none;
    color: var(--text-primary);
    padding: 0 10px;
    height: 24px;
    border-radius: 4px;
    cursor: default;
    font-family: inherit;
    font-size: 13px;
    margin: 0 2px;
  }

  .menu-btn:hover, .menu-item.active .menu-btn {
    background-color: var(--bg-surface-hover);
  }

  .dropdown {
    position: absolute;
    top: 100%;
    left: 0;
    background-color: var(--bg-surface);
    border: 1px solid var(--border-color);
    border-radius: 4px;
    box-shadow: var(--shadow-md);
    min-width: 200px;
    padding: 4px 0;
    z-index: 9999;
    display: flex;
    flex-direction: column;
  }

  .dropdown button {
    background: transparent;
    border: none;
    color: var(--text-primary);
    padding: 6px 16px;
    text-align: left;
    cursor: default;
    font-family: inherit;
    font-size: 13px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    width: 100%;
  }

  .dropdown button:hover {
    background-color: var(--bg-surface-hover);
  }

  .dropdown hr {
    margin: 4px 0;
    border: none;
    border-top: 1px solid var(--border-color);
  }

  .shortcut {
    color: var(--text-muted);
    font-size: 12px;
  }

  .window-controls {
    display: flex;
    height: 38px;
    flex: 1;
    justify-content: flex-end;
    z-index: 9999;
  }

  .control-btn {
    background: transparent;
    border: none;
    color: var(--text-primary);
    width: 46px;
    height: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
    cursor: default;
  }

  .control-btn:hover {
    background-color: var(--bg-surface-hover);
  }

  .control-btn.close:hover {
    background-color: #E81123;
    color: white;
  }

  @media (max-width: 1000px) {
    .titlebar-center {
      display: none;
    }
  }

  @media (max-width: 768px) {
    .titlebar,
    .window-controls {
      display: none !important;
    }
  }
</style>
