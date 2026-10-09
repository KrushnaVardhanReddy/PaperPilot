<script lang="ts">
  import { toastState, type Toast } from '$lib/state/toast.svelte';
  import { fade, slide } from 'svelte/transition';

  let expandedDetails: Record<string, boolean> = $state({});

  function toggleDetails(id: string) {
    expandedDetails[id] = !expandedDetails[id];
  }

  function copyToClipboard(text: string) {
    navigator.clipboard.writeText(text).catch(err => {
      console.error('Failed to copy text: ', err);
    });
  }
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
        {#if toast.actionHint}
          <div class="toast-hint">{toast.actionHint}</div>
        {/if}
        {#if toast.technicalDetails}
          <div class="toast-technical">
            <button class="details-toggle" onclick={() => toggleDetails(toast.id)}>
              Details {expandedDetails[toast.id] ? '▴' : '▾'}
            </button>
            {#if expandedDetails[toast.id]}
              <div class="details-block-wrapper" transition:slide={{ duration: 200 }}>
                <pre class="details-block">{toast.technicalDetails}</pre>
                <button class="copy-btn" onclick={() => copyToClipboard(toast.technicalDetails || '')} aria-label="Copy technical details">
                  Copy
                </button>
              </div>
            {/if}
          </div>
        {/if}
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
    font-weight: 600;
  }

  .toast-hint {
    font-size: 0.8rem;
    color: var(--text-secondary);
    margin-top: 4px;
    line-height: 1.3;
  }

  .toast-technical {
    margin-top: 8px;
  }

  .details-toggle {
    background: none;
    border: none;
    color: var(--text-muted);
    font-size: 0.75rem;
    cursor: pointer;
    padding: 0;
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .details-toggle:hover {
    color: var(--text-primary);
  }

  .details-block-wrapper {
    position: relative;
    margin-top: 4px;
  }

  .details-block {
    background: var(--bg-primary);
    color: var(--text-secondary);
    padding: 8px;
    border-radius: 4px;
    font-size: 0.75rem;
    white-space: pre-wrap;
    word-break: break-all;
    max-height: 150px;
    overflow-y: auto;
    border: 1px solid var(--border-color);
  }

  .copy-btn {
    position: absolute;
    top: 4px;
    right: 4px;
    background: var(--bg-surface);
    border: 1px solid var(--border-color);
    color: var(--text-primary);
    font-size: 0.65rem;
    padding: 2px 6px;
    border-radius: 4px;
    cursor: pointer;
    opacity: 0.8;
  }

  .copy-btn:hover {
    opacity: 1;
    background: var(--bg-surface-hover);
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
