<script lang="ts">
    import { invoke } from '@tauri-apps/api/core';
    import { listen } from '@tauri-apps/api/event';
    import { onDestroy, onMount } from 'svelte';
    import { aiConfig, type ModelInfo } from '$lib/state/aiConfig.svelte';

    interface DownloadProgress {
        model_id: string;
        percent: number;
        bytes_downloaded: number;
        total_bytes: number;
    }

    let activeDownloads = $state<Record<string, DownloadProgress>>({});
    let downloadErrors = $state<Record<string, string>>({});

    let unlisten: () => void;

    onMount(async () => {
        aiConfig.loadModels();
        unlisten = await listen<DownloadProgress>('model-download-progress', (event) => {
            activeDownloads[event.payload.model_id] = event.payload;
            if (event.payload.percent >= 100) {
                 setTimeout(() => {
                     delete activeDownloads[event.payload.model_id];
                     aiConfig.loadModels();
                 }, 1000);
            }
        });
    });

    onDestroy(() => {
        if (unlisten) unlisten();
    });

    async function startDownload(modelId: string) {
        try {
            delete downloadErrors[modelId];
            activeDownloads[modelId] = {
                model_id: modelId,
                percent: 0,
                bytes_downloaded: 0,
                total_bytes: 0
            };
            await invoke('download_model', { modelId });
        } catch (e: any) {
            downloadErrors[modelId] = e.toString();
            delete activeDownloads[modelId];
        }
    }

    async function cancelDownload(modelId: string) {
        try {
            await invoke('cancel_model_download', { modelId });
            delete activeDownloads[modelId];
        } catch (e: any) {
             console.error('Failed to cancel download:', e);
        }
    }

    function formatBytes(bytes: number) {
        if (bytes === 0) return '0 B';
        const k = 1024;
        const sizes = ['B', 'KB', 'MB', 'GB'];
        const i = Math.floor(Math.log(bytes) / Math.log(k));
        return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
    }
</script>

<div class="model-downloader">
    <h3 class="section-title">Local Models</h3>
    <p class="section-desc">Download curated models to run locally. GGUF format.</p>

    <div class="models-list">
        {#if aiConfig.availableModels.length === 0}
            <div class="models-loading">
                <span class="spinner"></span>
                <span>Fetching curated model registry...</span>
            </div>
        {:else}
            {#each aiConfig.availableModels as model (model.id)}
                <div class="model-card">
                    <div class="model-info">
                        <div class="model-header">
                            <h4>{model.name}</h4>
                            <span class="model-size">{formatBytes(model.size_mb * 1024 * 1024)}</span>
                        </div>
                        <p class="model-desc">{model.description}</p>

                        {#if model.is_downloaded && !activeDownloads[model.id]}
                            <div class="status-downloaded">
                                <span class="status-icon">✓</span> Downloaded
                                <span class="active-badge">● Active on Disk</span>
                                <span class="local-path" title={model.local_path}>{model.local_path}</span>
                            </div>
                        {/if}

                        {#if downloadErrors[model.id]}
                             <div class="error-msg">⚠️ {downloadErrors[model.id]}</div>
                        {/if}
                    </div>

                    <div class="model-actions">
                        {#if activeDownloads[model.id]}
                            {@const p = activeDownloads[model.id]}
                            <div class="progress-container">
                                 <div class="progress-text">
                                      <span>{p.percent.toFixed(1)}%</span>
                                      <span>{formatBytes(p.bytes_downloaded)} / {formatBytes(p.total_bytes)}</span>
                                 </div>
                                 <div class="progress-bar">
                                      <div class="progress-fill" style="width: {p.percent}%"></div>
                                 </div>
                                 <button class="cancel-btn" onclick={() => cancelDownload(model.id)}>Cancel</button>
                            </div>
                        {:else if !model.is_downloaded}
                            <button class="download-btn" onclick={() => startDownload(model.id)}>
                                 <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                     <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
                                     <polyline points="7 10 12 15 17 10"></polyline>
                                     <line x1="12" y1="15" x2="12" y2="3"></line>
                                 </svg>
                                 Download
                            </button>
                        {/if}
                    </div>
                </div>
            {/each}
        {/if}
    </div>
</div>

<style>
    .model-downloader {
        margin-top: 1.5rem;
        background: rgba(255, 255, 255, 0.03);
        border: 1px solid rgba(255, 255, 255, 0.1);
        border-radius: 12px;
        padding: 1.25rem;
    }

    .section-title {
        margin: 0 0 0.25rem 0;
        font-size: 1rem;
        font-weight: 600;
        color: #e2e8f0;
    }

    .section-desc {
        margin: 0 0 1rem 0;
        font-size: 0.85rem;
        color: #94a3b8;
    }

    .models-list {
        display: flex;
        flex-direction: column;
        gap: 1rem;
    }

    .model-card {
        background: rgba(0, 0, 0, 0.2);
        border: 1px solid rgba(255, 255, 255, 0.05);
        border-radius: 8px;
        padding: 1rem;
        display: flex;
        flex-direction: column;
        gap: 1rem;
    }

    .model-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 0.25rem;
    }

    .model-header h4 {
        margin: 0;
        font-size: 0.95rem;
        color: #f1f5f9;
    }

    .model-size {
        font-size: 0.8rem;
        color: #94a3b8;
        background: rgba(255, 255, 255, 0.1);
        padding: 0.15rem 0.4rem;
        border-radius: 4px;
    }

    .model-desc {
        margin: 0;
        font-size: 0.85rem;
        color: #94a3b8;
    }

    .status-downloaded {
        margin-top: 0.75rem;
        font-size: 0.85rem;
        color: #10b981;
        display: flex;
        align-items: center;
        gap: 0.5rem;
        flex-wrap: wrap;
    }

    .active-badge {
        background: rgba(16, 185, 129, 0.2);
        color: #34d399;
        font-size: 0.75rem;
        font-weight: 600;
        padding: 2px 8px;
        border-radius: 4px;
        border: 1px solid rgba(16, 185, 129, 0.4);
    }

    .local-path {
        font-family: monospace;
        color: #64748b;
        font-size: 0.75rem;
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
        max-width: 200px;
    }

    .download-btn {
        display: inline-flex;
        align-items: center;
        gap: 0.5rem;
        background: #3b82f6;
        color: white;
        border: none;
        padding: 0.5rem 1rem;
        border-radius: 6px;
        font-size: 0.85rem;
        font-weight: 500;
        cursor: pointer;
        transition: background 0.2s;
    }

    .download-btn:hover {
        background: #2563eb;
    }

    .progress-container {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
        background: rgba(0, 0, 0, 0.3);
        padding: 0.75rem;
        border-radius: 6px;
    }

    .progress-text {
        display: flex;
        justify-content: space-between;
        font-size: 0.8rem;
        color: #cbd5e1;
    }

    .progress-bar {
        height: 6px;
        background: rgba(255, 255, 255, 0.1);
        border-radius: 3px;
        overflow: hidden;
    }

    .progress-fill {
        height: 100%;
        background: #3b82f6;
        transition: width 0.2s ease-out;
    }

    .cancel-btn {
        align-self: flex-end;
        background: transparent;
        color: #ef4444;
        border: 1px solid #ef4444;
        padding: 0.25rem 0.75rem;
        border-radius: 4px;
        font-size: 0.75rem;
        cursor: pointer;
    }

    .cancel-btn:hover {
        background: rgba(239, 68, 68, 0.1);
    }

    .models-loading {
        display: flex;
        align-items: center;
        gap: 0.75rem;
        padding: 1rem;
        font-size: 0.85rem;
        color: #94a3b8;
    }

    .spinner {
        width: 16px;
        height: 16px;
        border: 2px solid rgba(255, 255, 255, 0.1);
        border-top-color: #3b82f6;
        border-radius: 50%;
        animation: spin 0.8s linear infinite;
    }

    @keyframes spin {
        to { transform: rotate(360deg); }
    }

    .error-msg {
        color: #ef4444;
        font-size: 0.8rem;
        margin-top: 0.5rem;
    }
</style>
