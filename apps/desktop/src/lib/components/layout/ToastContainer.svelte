<script lang="ts">
  import { toastState, type Toast } from '$lib/state/toast.svelte';
  import { fade, slide } from 'svelte/transition';
</script>

<div class="toast-container">
  {#each toastState.toasts as toast (toast.id)}
    <div
      class="toast toast-{toast.type}"
      transition:slide={{ duration: 250 }}
      role="alert"
    >
      <div class="toast-content">
        <span class="toast-message">{toast.message}</span>
      </div>
      <button
        class="toast-close"
        aria-label="Close"
        onclick={() => toastState.remove(toast.id)}
      >
        ×
      </button>
    </div>
  {/each}
</div>

<style>
  .toast-container {
    position: fixed;
    bottom: 24px;
    right: 24px;
    display: flex;
    flex-direction: column;
    gap: 12px;
    z-index: 9999;
    pointer-events: none;
  }

  .toast {
    display: flex;
    align-items: center;
    justify-content: space-between;
    min-width: 300px;
    max-width: 400px;
    padding: 12px 16px;
    background-color: var(--bg-surface);
    color: var(--text-primary);
    border-radius: var(--border-radius-md);
    box-shadow: var(--shadow-lg);
    border-left: 4px solid transparent;
    pointer-events: auto;
    transition: transform var(--transition-normal);
  }

  .toast-info {
    border-left-color: var(--accent-primary);
  }

  .toast-success {
    border-left-color: #10B981; /* Emerald 500 */
  }

  .toast-warning {
    border-left-color: #F59E0B; /* Amber 500 */
  }

  .toast-error {
    border-left-color: #EF4444; /* Red 500 */
  }

  .toast-content {
    flex: 1;
    margin-right: 12px;
  }

  .toast-message {
    font-size: 0.9rem;
    line-height: 1.4;
  }

  .toast-close {
    background: none;
    border: none;
    color: var(--text-muted);
    font-size: 1.2rem;
    cursor: pointer;
    padding: 4px;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: color var(--transition-fast);
  }

  .toast-close:hover {
    color: var(--text-primary);
  }
</style>
