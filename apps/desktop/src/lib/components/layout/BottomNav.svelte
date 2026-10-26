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
    bottom: calc(16px + env(safe-area-inset-bottom, 12px));
    left: 16px;
    right: 16px;
    max-width: 480px;
    margin: 0 auto;
    height: 64px;
    background: rgba(22, 26, 37, 0.88);
    backdrop-filter: blur(20px);
    -webkit-backdrop-filter: blur(20px);
    border: 1px solid rgba(255, 255, 255, 0.12);
    border-radius: 32px;
    display: flex;
    align-items: center;
    justify-content: space-around;
    z-index: 100;
    padding: 0 12px;
    box-sizing: border-box;
    box-shadow: 0 12px 32px -4px rgba(0, 0, 0, 0.6), 0 4px 12px rgba(0, 0, 0, 0.4);
  }

  .bottom-nav-item {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 3px;
    flex: 1;
    height: 48px;
    padding: 4px 0;
    background: none;
    border: none;
    color: var(--text-muted, #9ca3af);
    cursor: pointer;
    border-radius: 24px;
    transition: color 0.15s ease, background-color 0.15s ease, transform 0.15s ease;
  }

  .bottom-nav-item:hover,
  .bottom-nav-item:active {
    color: var(--text-primary, #ffffff);
    background-color: rgba(255, 255, 255, 0.08);
  }

  .bottom-nav-item.active {
    color: var(--accent-primary, #6366f1);
    background-color: rgba(99, 102, 241, 0.15);
  }

  .bottom-nav-icon {
    font-size: 1.25rem;
    line-height: 1;
  }

  .bottom-nav-label {
    font-size: 0.62rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }
</style>
