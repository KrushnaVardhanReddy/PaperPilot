<script lang="ts">
  import { appState } from '$lib/state/app.svelte';

  const navItems = [
    { id: 'documents', label: 'Documents', icon: '📁' },
    { id: 'pipeline', label: 'Pipeline Studio', icon: '🔗' },
    { id: 'api', label: 'API Gateway', icon: '⚡' },
    { id: 'settings', label: 'Settings', icon: '⚙️' }
  ];
</script>

<aside class="sidebar collapsed" aria-label="Main navigation">
  <div class="sidebar-header">
    <img src="/favicon.png" alt="PaperPilot" class="brand-icon" />
  </div>

  <nav class="sidebar-nav" aria-label="Navigation">
    {#each navItems as item (item.id)}
      {#if item.id !== 'api' || appState.developerMode}
        <button
          class="nav-item"
          class:active={appState.activeTab === item.id}
          onclick={() => {
            if (appState.activeTab === item.id && item.id === 'documents') {
              appState.toggleContextPanel();
            } else {
              appState.setActiveTab(item.id);
            }
          }}
          title={item.label}
          id={`nav-${item.id}`}
          aria-label={item.label}
          aria-current={appState.activeTab === item.id ? 'page' : undefined}
        >
          <span class="nav-icon" aria-hidden="true">{item.icon}</span>
        </button>
      {/if}
    {/each}
  </nav>

  <div class="sidebar-footer">
    <div class="theme-toggle">
      <button
        class="theme-btn"
        title={appState.theme === 'dark' ? 'Switch to Light Mode' : 'Switch to Dark Mode'}
        onclick={() => appState.setTheme(appState.theme === 'dark' ? 'light' : 'dark')}
      >
        <span class="nav-icon">{appState.theme === 'dark' ? '🌙' : '☀️'}</span>
      </button>
    </div>
  </div>
</aside>

<style>
  .sidebar {
    width: 48px;
    background-color: var(--bg-secondary);
    border-right: 1px solid var(--border-color);
    display: flex;
    flex-direction: column;
    height: 100%;
    z-index: 10;
  }

  .sidebar-header {
    padding: 16px 0;
    display: flex;
    justify-content: center;
    border-bottom: 1px solid var(--border-color);
  }

  .brand-icon {
    width: 24px;
    height: 24px;
    border-radius: 4px;
    object-fit: contain;
  }

  .sidebar-nav {
    flex: 1;
    padding: 20px 0;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 12px;
    overflow-y: auto;
  }

  .nav-item {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 36px;
    height: 36px;
    background: transparent;
    border: none;
    border-radius: 8px;
    color: var(--text-secondary);
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .nav-item:hover {
    background-color: var(--bg-surface);
    color: var(--text-primary);
  }

  .nav-item.active {
    background-color: rgba(59, 130, 246, 0.15); /* light accent */
    color: var(--accent-primary);
    border-left: 2px solid var(--accent-primary);
    border-radius: 0 8px 8px 0;
    width: 40px;
    margin-right: -4px;
  }

  .nav-icon {
    font-size: 1.25rem;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .sidebar-footer {
    padding: 16px 0;
    display: flex;
    justify-content: center;
    border-top: 1px solid var(--border-color);
  }

  .theme-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 36px;
    height: 36px;
    background: transparent;
    border: none;
    border-radius: 8px;
    color: var(--text-secondary);
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .theme-btn:hover {
    background-color: var(--bg-surface);
    color: var(--text-primary);
  }
</style>
