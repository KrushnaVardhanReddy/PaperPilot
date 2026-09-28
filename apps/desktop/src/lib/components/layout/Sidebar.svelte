<script lang="ts">
  import { appState } from '$lib/state/app.svelte';

  const navItems = [
    { id: 'home', label: 'Home', icon: '🏠' },
    { id: 'documents', label: 'Documents', icon: '📄' },
    { id: 'settings', label: 'Settings', icon: '⚙️' }
  ];
</script>

<aside class="sidebar">
  <div class="sidebar-header">
    <h1 class="brand-title">PaperPilot</h1>
  </div>

  <nav class="sidebar-nav">
    {#each navItems as item (item.id)}
      <button
        class="nav-item"
        class:active={appState.activeTab === item.id}
        onclick={() => appState.setActiveTab(item.id)}
      >
        <span class="nav-icon">{item.icon}</span>
        <span class="nav-label">{item.label}</span>
      </button>
    {/each}
  </nav>

  <div class="sidebar-footer">
    <div class="theme-toggle">
      <button
        class="theme-btn"
        onclick={() => appState.setTheme(appState.theme === 'dark' ? 'light' : 'dark')}
      >
        {appState.theme === 'dark' ? '🌙 Dark Mode' : '☀️ Light Mode'}
      </button>
    </div>
  </div>
</aside>

<style>
  .sidebar {
    width: 260px;
    background-color: var(--bg-secondary);
    border-right: 1px solid var(--border-color);
    display: flex;
    flex-direction: column;
    height: 100%;
  }

  .sidebar-header {
    padding: 24px 20px;
    border-bottom: 1px solid var(--border-color);
  }

  .brand-title {
    font-size: 1.25rem;
    font-weight: 700;
    color: var(--text-primary);
    letter-spacing: -0.025em;
  }

  .sidebar-nav {
    flex: 1;
    padding: 20px 12px;
    display: flex;
    flex-direction: column;
    gap: 4px;
    overflow-y: auto;
  }

  .nav-item {
    display: flex;
    align-items: center;
    padding: 10px 16px;
    background: transparent;
    border: none;
    border-radius: var(--border-radius-md);
    color: var(--text-secondary);
    cursor: pointer;
    transition: all var(--transition-fast);
    text-align: left;
    font-size: 0.95rem;
    font-weight: 500;
  }

  .nav-item:hover {
    background-color: var(--bg-surface);
    color: var(--text-primary);
  }

  .nav-item.active {
    background-color: var(--accent-primary);
    color: #ffffff;
  }

  .nav-icon {
    margin-right: 12px;
    font-size: 1.1rem;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .sidebar-footer {
    padding: 16px 20px;
    border-top: 1px solid var(--border-color);
  }

  .theme-btn {
    width: 100%;
    padding: 8px 12px;
    background-color: var(--bg-surface);
    color: var(--text-secondary);
    border: 1px solid var(--border-color);
    border-radius: var(--border-radius-sm);
    cursor: pointer;
    font-size: 0.85rem;
    transition: all var(--transition-fast);
  }

  .theme-btn:hover {
    background-color: var(--bg-surface-hover);
    color: var(--text-primary);
  }
</style>
