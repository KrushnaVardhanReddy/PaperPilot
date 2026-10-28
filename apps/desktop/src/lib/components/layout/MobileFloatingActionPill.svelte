<script lang="ts">
  import { appState } from '$lib/state/app.svelte';

  let {
    onTap = () => {},
    isScrolled = false
  }: {
    onTap?: () => void;
    isScrolled?: boolean;
  } = $props();

  let label = "Redact PII (Contract)";
</script>

{#if appState.isMobile}
  <button
    class="mobile-fab"
    class:scrolled={isScrolled}
    onclick={onTap}
    aria-label="AI Recommendations"
  >
    {#if isScrolled}
      <span class="fab-icon glow">✨</span>
    {:else}
      <span class="pill-content">
        <span class="pill-icon">✨</span>
        <span class="pill-text">AI Suggestion: {label}</span>
        <span class="pill-action">[Apply]</span>
      </span>
    {/if}
  </button>
{/if}

<style>
  .mobile-fab {
    position: fixed;
    bottom: calc(80px + var(--safe-area-bottom, 0px));
    left: 50%;
    transform: translateX(-50%);
    background: var(--glass-bg, rgba(30, 30, 35, 0.75));
    backdrop-filter: var(--glass-blur, blur(12px));
    -webkit-backdrop-filter: var(--glass-blur, blur(12px));
    border: 1px solid var(--glass-border, rgba(255, 255, 255, 0.08));
    border-radius: 24px;
    color: var(--text-primary, #ffffff);
    padding: 12px 16px;
    display: flex;
    align-items: center;
    justify-content: center;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
    z-index: 50;
    cursor: pointer;
    transition: all 0.3s cubic-bezier(0.25, 1, 0.5, 1);
    white-space: nowrap;
  }

  .mobile-fab.scrolled {
    bottom: calc(24px + var(--safe-area-bottom, 0px));
    right: 24px;
    left: auto;
    transform: none;
    width: 56px;
    height: 56px;
    border-radius: 50%;
    padding: 0;
  }

  .pill-content {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 13px;
  }

  .pill-icon {
    font-size: 16px;
  }

  .pill-text {
    font-weight: 500;
  }

  .pill-action {
    color: var(--accent-primary, #818cf8);
    font-weight: 600;
  }

  .fab-icon {
    font-size: 24px;
  }

  .fab-icon.glow {
    text-shadow: 0 0 8px rgba(255, 255, 255, 0.5);
    animation: pulse 2s infinite;
  }

  @keyframes pulse {
    0% { opacity: 1; transform: scale(1); }
    50% { opacity: 0.8; transform: scale(1.1); }
    100% { opacity: 1; transform: scale(1); }
  }
</style>
