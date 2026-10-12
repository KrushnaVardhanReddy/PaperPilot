<script lang="ts">
  import { appState } from '$lib/state/app.svelte';
  import { safeInvoke } from '$lib/utils/tauri';
  import { onMount, onDestroy } from 'svelte';
  import ProgressBar from '$lib/components/ui/ProgressBar.svelte';

  let selectedPacks = $state<Record<string, boolean>>({});
  let downloadProgress = $state<Record<string, { percent: number; bytes: number; total: number }>>({});
  let isDownloading = $state(false);
  let unlistenProgress: () => void;

  onMount(async () => {
    try {
      const { listen } = await import('@tauri-apps/api/event');
      unlistenProgress = await listen<{ pack_id: string; percent: number; bytes_downloaded: number; total_bytes: number }>('pack-download-progress', (event) => {
        downloadProgress[event.payload.pack_id] = {
          percent: event.payload.percent,
          bytes: event.payload.bytes_downloaded,
          total: event.payload.total_bytes,
        };
      });
    } catch (err) {
      console.warn("Could not bind to Tauri events", err);
    }

    // Initialize selections based on existing config or defaults
    for (const pack of appState.featurePacks) {
      selectedPacks[pack.id] = pack.is_enabled || pack.id === 'core';
    }
  });

  onDestroy(() => {
    if (unlistenProgress) {
      unlistenProgress();
    }
  });

  function applyPreset(preset: 'minimal' | 'productivity' | 'full') {
    for (const pack of appState.featurePacks) {
      if (pack.id === 'core') {
        selectedPacks[pack.id] = true;
      } else if (preset === 'minimal') {
        selectedPacks[pack.id] = false;
      } else if (preset === 'productivity') {
        selectedPacks[pack.id] = pack.id === 'office' || pack.id === 'ocr';
      } else if (preset === 'full') {
        selectedPacks[pack.id] = true;
      }
    }
  }

  async function handleDownloadAndLaunch() {
    isDownloading = true;

    for (const pack of appState.featurePacks) {
      const wantsEnabled = selectedPacks[pack.id];

      if (wantsEnabled && pack.id !== 'core') {
        // Download if not installed
        if (!pack.is_installed) {
          try {
            await safeInvoke('download_feature_pack', { packId: pack.id });
          } catch (e) {
            console.error(`Failed to download pack ${pack.id}:`, e);
          }
        } else {
          // Just enable it
          try {
            await appState.toggleFeaturePack(pack.id, true);
          } catch (e) {
            console.error(`Failed to enable pack ${pack.id}:`, e);
          }
        }
      } else if (!wantsEnabled && pack.id !== 'core') {
        try {
          await appState.toggleFeaturePack(pack.id, false);
        } catch (e) {
          console.error(`Failed to disable pack ${pack.id}:`, e);
        }
      }
    }

    isDownloading = false;
    appState.completeOnboarding();
  }

  function handleSkip() {
    appState.completeOnboarding();
  }
</script>

<div class="modal-overlay">
  <div class="modal-content">
    <div class="header">
      <h2>Welcome to PaperPilot</h2>
      <p>Configure your workspace capabilities.</p>

      <div class="portable-badge {appState.isPortableMode ? 'active' : ''}">
        {#if appState.isPortableMode}
          💾 Portable Mode: Active (data/ detected)
        {:else}
          💻 System Installed Mode
        {/if}
      </div>
    </div>

    <div class="presets">
      <p>Quick Presets:</p>
      <div class="preset-buttons">
        <button class="btn secondary" onclick={() => applyPreset('minimal')} disabled={isDownloading}>⚡ Minimalist</button>
        <button class="btn primary" onclick={() => applyPreset('productivity')} disabled={isDownloading}>💼 Productivity (Recommended)</button>
        <button class="btn secondary" onclick={() => applyPreset('full')} disabled={isDownloading}>🚀 Full Suite</button>
      </div>
    </div>

    <div class="pack-list">
      {#each appState.featurePacks as pack}
        <div class="pack-item {selectedPacks[pack.id] ? 'selected' : ''}">
          <div class="pack-header">
            <label class="checkbox-label">
              <input
                type="checkbox"
                bind:checked={selectedPacks[pack.id]}
                disabled={pack.id === 'core' || isDownloading}
              />
              <span class="pack-name">{pack.name}</span>
            </label>
            <span class="pack-size">{pack.size_mb > 0 ? `~${pack.size_mb} MB` : 'Built-in'}</span>
          </div>

          <p class="pack-desc">{pack.description}</p>

          <div class="pack-tools">
            {#each pack.tools as tool}
              <span class="tool-badge">{tool}</span>
            {/each}
          </div>

          {#if downloadProgress[pack.id] && isDownloading && selectedPacks[pack.id] && !pack.is_installed}
            <div class="progress-container">
              <ProgressBar jobName="Downloading {pack.name}" percent={downloadProgress[pack.id].percent} />
            </div>
          {/if}
        </div>
      {/each}
    </div>

    <div class="actions">
      <button class="btn secondary" onclick={handleSkip} disabled={isDownloading}>Skip for Now</button>
      <button class="btn primary launch-btn" onclick={handleDownloadAndLaunch} disabled={isDownloading}>
        {isDownloading ? 'Downloading...' : 'Download & Launch'}
      </button>
    </div>
  </div>
</div>

<style>
  .modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.75);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 10000;
    backdrop-filter: blur(4px);
  }

  .modal-content {
    background: var(--bg-surface);
    border: 1px solid var(--border-color);
    border-radius: var(--border-radius-lg);
    width: 90%;
    max-width: 800px;
    max-height: 90vh;
    overflow-y: auto;
    padding: 32px;
    display: flex;
    flex-direction: column;
    gap: 24px;
    box-shadow: 0 10px 25px rgba(0, 0, 0, 0.5);
  }

  .header {
    text-align: center;
  }

  .header h2 {
    font-size: 2rem;
    margin-bottom: 8px;
    color: var(--text-primary);
  }

  .header p {
    color: var(--text-secondary);
    margin-bottom: 16px;
  }

  .portable-badge {
    display: inline-block;
    padding: 6px 12px;
    border-radius: 16px;
    font-size: 0.85rem;
    font-weight: 500;
    background: var(--bg-secondary);
    color: var(--text-secondary);
    border: 1px solid var(--border-color);
  }

  .portable-badge.active {
    background: rgba(34, 197, 94, 0.15);
    color: #4ade80;
    border-color: rgba(34, 197, 94, 0.3);
  }

  .presets p {
    font-size: 0.9rem;
    color: var(--text-secondary);
    margin-bottom: 8px;
  }

  .preset-buttons {
    display: flex;
    gap: 12px;
    justify-content: center;
  }

  .pack-list {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .pack-item {
    border: 1px solid var(--border-color);
    border-radius: var(--border-radius-md);
    padding: 16px;
    background: var(--bg-primary);
    transition: border-color var(--transition-fast);
  }

  .pack-item.selected {
    border-color: var(--accent-primary);
    background: var(--bg-surface-hover);
  }

  .pack-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 8px;
  }

  .checkbox-label {
    display: flex;
    align-items: center;
    gap: 8px;
    font-weight: 600;
    font-size: 1.1rem;
    color: var(--text-primary);
    cursor: pointer;
  }

  .pack-size {
    font-size: 0.85rem;
    color: var(--text-muted);
    background: var(--bg-secondary);
    padding: 2px 8px;
    border-radius: 12px;
  }

  .pack-desc {
    font-size: 0.9rem;
    color: var(--text-secondary);
    margin-bottom: 12px;
    line-height: 1.4;
  }

  .pack-tools {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
  }

  .tool-badge {
    font-size: 0.75rem;
    background: var(--bg-secondary);
    color: var(--text-secondary);
    padding: 2px 8px;
    border-radius: 4px;
    border: 1px solid var(--border-color);
  }

  .progress-container {
    margin-top: 16px;
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .actions {
    display: flex;
    justify-content: flex-end;
    gap: 16px;
    margin-top: 16px;
    border-top: 1px solid var(--border-color);
    padding-top: 24px;
  }

  .btn {
    padding: 10px 20px;
    border: none;
    border-radius: var(--border-radius-md);
    font-weight: 500;
    cursor: pointer;
    font-size: 1rem;
    transition: opacity var(--transition-fast);
  }

  .btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn.primary {
    background: var(--accent-primary);
    color: white;
  }

  .btn.secondary {
    background: var(--bg-secondary);
    color: var(--text-primary);
    border: 1px solid var(--border-color);
  }

  .launch-btn {
    min-width: 200px;
  }
</style>
