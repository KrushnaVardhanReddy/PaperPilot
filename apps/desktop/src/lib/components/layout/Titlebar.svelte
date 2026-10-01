<script lang="ts">
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { emit } from '@tauri-apps/api/event';
  import { onMount } from 'svelte';

  let activeMenu: string | null = $state(null);

  onMount(() => {
    const handleClickOutside = (e: MouseEvent) => {
      if (!(e.target as Element).closest('.menu-item')) activeMenu = null;
    };
    document.addEventListener('click', handleClickOutside);
    return () => document.removeEventListener('click', handleClickOutside);
  });

  function toggleMenu(menu: string, e: MouseEvent) {
    e.stopPropagation();
    activeMenu = activeMenu === menu ? null : menu;
  }

  async function triggerEvent(eventName: string) {
    activeMenu = null;
    await emit(eventName);
  }
</script>

<div class="titlebar" data-tauri-drag-region>
  <div class="titlebar-menus" data-tauri-drag-region>
    <div class="brand" data-tauri-drag-region>PaperPilot</div>

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
  }

  .titlebar-menus {
    display: flex;
    align-items: center;
    padding-left: 8px;
  }

  .brand {
    font-weight: 600;
    margin-right: 16px;
    margin-left: 8px;
    color: var(--text-primary);
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
    margin-left: auto;
    flex-shrink: 0;
    z-index: 9999;
    position: fixed;
    right: 0;
    top: 0;
    background: var(--bg-secondary);
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
</style>
