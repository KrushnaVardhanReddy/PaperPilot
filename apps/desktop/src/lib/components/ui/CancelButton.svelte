<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';

  interface Props {
    jobId: string;
    onCancelled?: () => void;
  }

  let { jobId, onCancelled }: Props = $props();
  let cancelling = $state(false);

  async function handleCancel() {
    cancelling = true;
    try {
      await invoke('cancel_job', { jobId });
      onCancelled?.();
    } catch (e) {
      console.error('Failed to cancel job:', e);
    } finally {
      cancelling = false;
    }
  }
</script>

<button
  class="cancel-btn"
  onclick={handleCancel}
  disabled={cancelling}
  aria-label="Cancel job"
>
  {cancelling ? 'Cancelling…' : '✕ Cancel'}
</button>

<style>
  .cancel-btn {
    background: transparent;
    border: 1px solid var(--error-color, #ef4444);
    color: var(--error-color, #ef4444);
    border-radius: 4px;
    padding: 0.25rem 0.75rem;
    font-size: 0.8rem;
    cursor: pointer;
    transition: background 0.15s, color 0.15s;
  }
  .cancel-btn:hover:not(:disabled) {
    background: var(--error-color, #ef4444);
    color: white;
  }
  .cancel-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
</style>
