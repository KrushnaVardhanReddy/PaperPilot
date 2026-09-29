<script lang="ts">
  import { appState } from '$lib/state/app.svelte';
  import { onMount } from 'svelte';

  let windowWidth = $state(typeof window !== 'undefined' ? window.innerWidth : 1200);
  let isMobile = $derived(windowWidth < 600);

  onMount(() => {
    function handleResize() { windowWidth = window.innerWidth; }
    window.addEventListener('resize', handleResize);
    return () => window.removeEventListener('resize', handleResize);
  });

  const navItems = [
    { id: 'home', label: 'Home', icon: '🏠' },
    { id: 'documents', label: 'Documents', icon: '📄' },
    { id: 'pipeline', label: 'Pipeline', icon: '🔗' },
    { id: 'settings', label: 'Settings', icon: '⚙️' }
  ];
</script>

{#if isMobile}
  <nav class="bottom-nav" id="mobile-bottom-nav" aria-label="Mobile navigation">
    {#each navItems as item (item.id)}
      <button
        class="bottom-nav-item"
        class:active={appState.activeTab === item.id}
        id={`mobile-nav-${item.id}`}
        onclick={() => appState.setActiveTab(item.id)}
        aria-label={item.label}
        aria-current={appState.activeTab === item.id ? 'page' : undefined}
      >
        <span class="bottom-nav-icon" aria-hidden="true">{item.icon}</span>
        <span class="bottom-nav-label">{item.label}</span>
      </button>
    {/each}
  </nav>
{/if}

<style>
  .bottom-nav {
    position: fixed;
    bottom: 0;
    left: 0;
    right: 0;
    height: 64px;
    background: var(--bg-secondary);
    border-top: 1px solid var(--border-color);
    display: flex;
    align-items: center;
    justify-content: space-around;
    z-index: 100;
    padding: 0 8px;
  }

  .bottom-nav-item {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 2px;
    flex: 1;
    padding: 8px 4px;
    background: none;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    border-radius: var(--border-radius-md);
    transition: color var(--transition-fast), background-color var(--transition-fast);
  }

  .bottom-nav-item:hover {
    color: var(--text-primary);
    background-color: var(--bg-surface);
  }

  .bottom-nav-item.active {
    color: var(--accent-primary);
  }

  .bottom-nav-icon {
    font-size: 1.3rem;
  }

  .bottom-nav-label {
    font-size: 0.65rem;
    font-weight: 500;
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }
</style>