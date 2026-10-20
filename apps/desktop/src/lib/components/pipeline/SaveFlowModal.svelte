<script lang="ts">
    import { slide, fade } from 'svelte/transition';
    import { pipelineState } from '$lib/state/pipeline.svelte';

    export let isOpen = false;
    export let onClose: () => void;
    export let onSaveComplete: (flowId: string) => void = () => {};

    let flowName = '';
    let flowDescription = '';

    function handleSave() {
        if (!flowName.trim()) return;
        const flow = pipelineState.saveCurrentFlow(flowName, flowDescription);
        onClose();
        // Reset state
        flowName = '';
        flowDescription = '';
        onSaveComplete(flow.id);
        // Could show a toast notification here
    }
</script>

{#if isOpen}
    <div class="modal-backdrop" transition:fade={{duration: 200}}>
        <div class="modal-drawer" transition:slide={{axis: 'x', duration: 300}}>
            <div class="modal-header">
                <h2>Save Pipeline Flow</h2>
                <button class="close-btn" aria-label="Close" on:click={onClose}>✕</button>
            </div>

            <div class="modal-content">
                <div class="form-group">
                    <label class="input-label" for="flow-name">Flow Name</label>
                    <input
                        type="text"
                        id="flow-name"
                        class="input-field"
                        bind:value={flowName}
                        placeholder="e.g. Monthly Expense Audit"
                        required
                    />
                </div>

                <div class="form-group">
                    <label class="input-label" for="flow-desc">Description (Optional)</label>
                    <textarea
                        id="flow-desc"
                        class="input-field"
                        bind:value={flowDescription}
                        placeholder="e.g. Scrub PII, flatten, and encrypt with corporate password"
                        rows="3"
                    ></textarea>
                </div>

                <div class="preview-section">
                    <label class="input-label">Step Sequence Preview</label>
                    <div class="step-badges">
                        {#each pipelineState.steps as step, i}
                            <span class="step-badge">[{i + 1}] {step.icon} {step.label}</span>
                            {#if i < pipelineState.steps.length - 1}
                                <span class="step-arrow">➔</span>
                            {/if}
                        {/each}
                    </div>
                </div>
            </div>

            <div class="modal-footer">
                <button class="test-btn" on:click={onClose}>Cancel</button>
                <button
                    class="test-btn primary-test"
                    on:click={handleSave}
                    disabled={!flowName.trim()}
                >
                    Save Pipeline
                </button>
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
        border-left: 1px solid rgba(255, 255, 255, 0.1);
    }

    .modal-header {
        padding: 1.5rem;
        border-bottom: 1px solid rgba(255, 255, 255, 0.1);
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
        display: flex;
        flex-direction: column;
        gap: 1.5rem;
    }

    .form-group {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }

    .input-label {
        font-size: 0.9rem;
        font-weight: 500;
        color: #e2e8f0;
    }

    .input-field {
        background: rgba(0, 0, 0, 0.2);
        border: 1px solid rgba(255, 255, 255, 0.1);
        padding: 0.75rem;
        border-radius: 6px;
        color: white;
        font-family: inherit;
        font-size: 0.9rem;
        transition: border-color 0.2s;
    }

    .input-field:focus {
        outline: none;
        border-color: #3b82f6;
    }

    textarea.input-field {
        resize: vertical;
    }

    .preview-section {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
        margin-top: 1rem;
        padding-top: 1rem;
        border-top: 1px solid rgba(255, 255, 255, 0.1);
    }

    .step-badges {
        display: flex;
        flex-wrap: wrap;
        gap: 0.5rem;
        align-items: center;
        background: rgba(0, 0, 0, 0.2);
        padding: 1rem;
        border-radius: 6px;
        border: 1px solid rgba(255, 255, 255, 0.1);
    }

    .step-badge {
        background: rgba(255, 255, 255, 0.1);
        padding: 0.25rem 0.5rem;
        border-radius: 4px;
        font-size: 0.85rem;
        color: #e2e8f0;
        display: inline-flex;
        align-items: center;
        gap: 0.25rem;
    }

    .step-arrow {
        color: #94a3b8;
        font-size: 0.8rem;
    }

    .modal-footer {
        padding: 1.5rem;
        border-top: 1px solid rgba(255, 255, 255, 0.1);
        display: flex;
        justify-content: flex-end;
        gap: 1rem;
    }

    .test-btn {
        background: rgba(255, 255, 255, 0.05);
        color: #e2e8f0;
        border: 1px solid rgba(255, 255, 255, 0.1);
        padding: 0.75rem 1.5rem;
        border-radius: 6px;
        font-weight: 500;
        cursor: pointer;
        transition: background 0.2s;
    }

    .test-btn:hover:not(:disabled) {
        background: rgba(255, 255, 255, 0.1);
    }

    .test-btn.primary-test {
        background: #2563eb;
        color: white;
        border: 1px solid #3b82f6;
        font-weight: 600;
        box-shadow: 0 2px 8px rgba(37, 99, 235, 0.3);
    }

    .test-btn.primary-test:hover:not(:disabled) {
        background: #1d4ed8;
    }

    .test-btn:disabled {
        opacity: 0.5;
        cursor: not-allowed;
    }
</style>
