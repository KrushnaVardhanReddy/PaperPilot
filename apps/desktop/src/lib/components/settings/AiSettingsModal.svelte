<script lang="ts">
    import { createEventDispatcher } from 'svelte';
    import { fade, slide } from 'svelte/transition';
    import { aiConfig, type AiProviderMode } from '$lib/state/aiConfig.svelte';
    import ModelDownloader from './ModelDownloader.svelte';

    export let isOpen = false;
    const dispatch = createEventDispatcher();

    function close() {
        isOpen = false;
        dispatch('close');
    }

    function handleModeChange(mode: AiProviderMode) {
        aiConfig.mode = mode;
        aiConfig.saveConfig();
    }
</script>

{#if isOpen}
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="modal-backdrop" transition:fade={{duration: 200}} onclick={close}>
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div class="modal-drawer" transition:slide={{duration: 300, axis: 'x'}} onclick={(e) => e.stopPropagation()}>
            <div class="modal-header">
                <h2>AI Engine Settings</h2>
                <button class="close-btn" onclick={close}>✕</button>
            </div>

            <div class="modal-content">
                <div class="mode-selector">
                    <label class="mode-option" class:active={aiConfig.mode === 'offlinenlp'}>
                        <input type="radio" name="ai-mode" value="offlinenlp"
                               checked={aiConfig.mode === 'offlinenlp'}
                               onchange={() => handleModeChange('offlinenlp')} />
                        <div class="mode-info">
                            <span class="mode-title">⚡ Offline NLP Mode (Default)</span>
                            <span class="mode-desc">100% on-device embedded ONNX model. &lt;1.1MB, zero setup, airplane-ready.</span>
                        </div>
                    </label>

                    <label class="mode-option" class:active={aiConfig.mode === 'llamafile'}>
                        <input type="radio" name="ai-mode" value="llamafile"
                               checked={aiConfig.mode === 'llamafile'}
                               onchange={() => handleModeChange('llamafile')} />
                        <div class="mode-info">
                            <span class="mode-title">🚀 Offline Llamafile</span>
                            <span class="mode-desc">Standalone local LLM via OS background process.</span>
                        </div>
                    </label>

                    <label class="mode-option" class:active={aiConfig.mode === 'universal'}>
                        <input type="radio" name="ai-mode" value="universal"
                               checked={aiConfig.mode === 'universal'}
                               onchange={() => handleModeChange('universal')} />
                        <div class="mode-info">
                            <span class="mode-title">🌐 Universal Endpoint</span>
                            <span class="mode-desc">Connect to local/LAN Ollama, LM Studio, or vLLM.</span>
                        </div>
                    </label>

                    <label class="mode-option" class:active={aiConfig.mode === 'byok'}>
                        <input type="radio" name="ai-mode" value="byok"
                               checked={aiConfig.mode === 'byok'}
                               onchange={() => handleModeChange('byok')} />
                        <div class="mode-info">
                            <span class="mode-title">🔑 Commercial BYOK Cloud</span>
                            <span class="mode-desc">OpenAI, Anthropic, or Gemini. Securely stored in OS Keyring.</span>
                        </div>
                    </label>
                </div>

                {#if aiConfig.mode === 'llamafile'}
                    <div class="config-section" transition:slide>
                        <ModelDownloader />
                        <div class="test-row">
                            <button class="test-btn" onclick={() => aiConfig.testConnection()} disabled={aiConfig.testLoading}>
                                {aiConfig.testLoading ? 'Verifying...' : '⚡ Test Local Model Connection'}
                            </button>
                        </div>
                    </div>
                {/if}

                {#if aiConfig.mode === 'universal'}
                    <div class="config-section" transition:slide>
                        <label class="input-label" for="universal-endpoint">Endpoint URL</label>
                        <input type="text" id="universal-endpoint" class="input-field"
                               placeholder="http://localhost:11434"
                               bind:value={aiConfig.universalEndpoint}
                               onblur={() => aiConfig.saveConfig()} />

                        <button class="test-btn" onclick={() => aiConfig.testConnection()} disabled={aiConfig.testLoading}>
                             {aiConfig.testLoading ? 'Testing...' : 'Test Connection'}
                        </button>
                    </div>
                {/if}

                {#if aiConfig.mode === 'byok'}
                    <div class="config-section" transition:slide>
                        <label class="input-label" for="byok-provider">Provider</label>
                        <select id="byok-provider" class="input-field" bind:value={aiConfig.byokProvider} onchange={() => aiConfig.saveConfig()}>
                            <option value="openai">OpenAI (GPT-4o)</option>
                            <option value="anthropic">Anthropic (Claude 3.5)</option>
                            <option value="gemini">Google (Gemini)</option>
                        </select>

                        <label class="input-label" for="api-key">API Key</label>
                        <div class="input-desc">Saved securely to the OS Keyring.</div>
                        <input type="password" id="api-key" class="input-field"
                               placeholder={aiConfig.apiKeyPlaceholder || "sk-..."}
                               bind:value={aiConfig.pendingApiKey}
                               onblur={() => aiConfig.saveConfig()} />

                        <button class="test-btn" onclick={() => aiConfig.testConnection()} disabled={aiConfig.testLoading}>
                             {aiConfig.testLoading ? 'Testing...' : 'Test Connection'}
                        </button>
                    </div>
                {/if}

                {#if aiConfig.testResult}
                     <div class="test-result" class:success={aiConfig.testResult.success} class:error={!aiConfig.testResult.success}>
                          {aiConfig.testResult.success ? '🟢' : '🔴'} {aiConfig.testResult.message}
                     </div>
                {/if}

            </div>
        </div>
    </div>
{/if}

<style>
    .modal-backdrop {
        position: fixed;
        top: 0;
        left: 0;
        width: 100vw;
        height: 100vh;
        background: rgba(0, 0, 0, 0.6);
        backdrop-filter: blur(4px);
        z-index: 1000;
        display: flex;
        justify-content: flex-end;
    }

    .modal-drawer {
        width: 450px;
        height: 100%;
        background: #0f172a;
        box-shadow: -4px 0 24px rgba(0, 0, 0, 0.5);
        display: flex;
        flex-direction: column;
        color: #f8fafc;
        border-left: 1px solid rgba(255,255,255,0.1);
    }

    .modal-header {
        padding: 1.5rem;
        border-bottom: 1px solid rgba(255,255,255,0.1);
        display: flex;
        justify-content: space-between;
        align-items: center;
    }

    .modal-header h2 {
        margin: 0;
        font-size: 1.25rem;
        font-weight: 600;
    }

    .close-btn {
        background: transparent;
        border: none;
        color: #94a3b8;
        font-size: 1.25rem;
        cursor: pointer;
        transition: color 0.2s;
    }

    .close-btn:hover {
        color: white;
    }

    .modal-content {
        padding: 1.5rem;
        overflow-y: auto;
        flex: 1;
    }

    .mode-selector {
        display: flex;
        flex-direction: column;
        gap: 0.75rem;
        margin-bottom: 2rem;
    }

    .mode-option {
        display: flex;
        align-items: flex-start;
        gap: 1rem;
        padding: 1rem;
        border: 1px solid rgba(255,255,255,0.1);
        border-radius: 8px;
        cursor: pointer;
        transition: all 0.2s;
        background: rgba(255,255,255,0.02);
    }

    .mode-option:hover {
        background: rgba(255,255,255,0.05);
    }

    .mode-option.active {
        border-color: #3b82f6;
        background: rgba(59, 130, 246, 0.1);
    }

    .mode-option input[type="radio"] {
        margin-top: 0.25rem;
        accent-color: #3b82f6;
    }

    .mode-info {
        display: flex;
        flex-direction: column;
        gap: 0.25rem;
    }

    .mode-title {
        font-weight: 600;
        font-size: 0.95rem;
    }

    .mode-desc {
        font-size: 0.8rem;
        color: #94a3b8;
        line-height: 1.4;
    }

    .config-section {
        margin-top: 1.5rem;
        padding-top: 1.5rem;
        border-top: 1px solid rgba(255,255,255,0.1);
        display: flex;
        flex-direction: column;
        gap: 1rem;
    }

    .input-label {
        font-size: 0.9rem;
        font-weight: 500;
        color: #e2e8f0;
    }

    .input-desc {
        font-size: 0.75rem;
        color: #10b981;
        margin-top: -0.5rem;
    }

    .input-field {
        background: rgba(0,0,0,0.2);
        border: 1px solid rgba(255,255,255,0.1);
        padding: 0.75rem;
        border-radius: 6px;
        color: white;
        font-family: inherit;
        font-size: 0.9rem;
    }

    .input-field:focus {
        outline: none;
        border-color: #3b82f6;
    }

    select.input-field {
        appearance: none;
        background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='16' height='16' viewBox='0 0 24 24' fill='none' stroke='%2394a3b8' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'%3E%3Cpolyline points='6 9 12 15 18 9'%3E%3C/polyline%3E%3C/svg%3E");
        background-repeat: no-repeat;
        background-position: right 0.75rem center;
        padding-right: 2.5rem;
    }

    .test-btn {
        background: rgba(255,255,255,0.05);
        color: #e2e8f0;
        border: 1px solid rgba(255,255,255,0.1);
        padding: 0.75rem;
        border-radius: 6px;
        font-weight: 500;
        cursor: pointer;
        transition: background 0.2s;
        margin-top: 0.5rem;
    }

    .test-btn:hover:not(:disabled) {
        background: rgba(255,255,255,0.1);
    }

    .test-btn:disabled {
        opacity: 0.5;
        cursor: not-allowed;
    }

    .test-result {
        margin-top: 1rem;
        padding: 0.75rem;
        border-radius: 6px;
        font-size: 0.85rem;
        display: flex;
        align-items: center;
        gap: 0.5rem;
    }

    .test-result.success {
        background: rgba(16, 185, 129, 0.1);
        border: 1px solid rgba(16, 185, 129, 0.2);
        color: #34d399;
    }

    .test-result.error {
        background: rgba(239, 68, 68, 0.1);
        border: 1px solid rgba(239, 68, 68, 0.2);
        color: #f87171;
    }
</style>
