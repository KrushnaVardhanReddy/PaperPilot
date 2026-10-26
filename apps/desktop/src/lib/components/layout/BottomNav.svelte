<script lang="ts">
  import { appState } from '$lib/state/app.svelte';
  import { onMount } from 'svelte';
  import CameraScannerModal from '$lib/components/scanner/CameraScannerModal.svelte';

  const navItems = [
    { id: 'documents', label: 'Documents', icon: '📄' },
    { id: 'scan', label: 'Scan', icon: '📷' },
    { id: 'tools', label: 'Tools', icon: '⚡' },
    { id: 'settings', label: 'Settings', icon: '⚙️' }
  ];

  let showScanner = $state(false);

  function handleNavClick(id: string) {
    if (id === 'scan') {
      showScanner = true;
    } else if (id === 'tools') {
      appState.setActiveTab('tools');
    } else {
      appState.setActiveTab(id);
    }
  }
</script>

{#if appState.isMobile}
  <nav class="bottom-nav" id="mobile-bottom-nav" aria-label="Mobile navigation">
    {#each navItems as item (item.id)}
      <button
        class="bottom-nav-item"
        class:active={appState.activeTab === item.id && item.id !== 'scan'}
        id={`mobile-nav-${item.id}`}
        onclick={() => handleNavClick(item.id)}
        aria-label={item.label}
        aria-current={appState.activeTab === item.id ? 'page' : undefined}
      >
        <span class="bottom-nav-icon" aria-hidden="true">{item.icon}</span>
        <span class="bottom-nav-label">{item.label}</span>
      </button>
    {/each}
  </nav>

  {#if showScanner}
    <CameraScannerModal onClose={() => showScanner = false} />
  {/if}
{/if}

<style>
  .bottom-nav {
    position: fixed;
    bottom: 0;
    left: 0;
    right: 0;
    height: calc(64px + env(safe-area-inset-bottom, 16px));
    background: var(--bg-secondary);
    border-top: 1px solid var(--border-color);
    display: flex;
    align-items: flex-start;
    justify-content: space-around;
    z-index: 100;
    padding: 8px 8px env(safe-area-inset-bottom, 16px);
    box-sizing: border-box;
  }

  .bottom-nav-item {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 2px;
    flex: 1;
    min-height: 48px;
    min-width: 48px;
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
