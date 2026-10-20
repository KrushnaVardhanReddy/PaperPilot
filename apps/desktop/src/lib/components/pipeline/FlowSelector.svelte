<script lang="ts">
    import { pipelineState } from '$lib/state/pipeline.svelte';
    import SaveFlowModal from './SaveFlowModal.svelte';
    import { onMount } from 'svelte';

    let fileInput: HTMLInputElement;
    let isSaveModalOpen = $state(false);
    let selectedFlowId = $state('');

    onMount(() => {
        pipelineState.initStorage();
    });

    function handleFlowChange(event: Event) {
        const target = event.target as HTMLSelectElement;
        const newId = target.value;
        if (newId) {
            pipelineState.loadFlow(newId);
            selectedFlowId = newId;
        } else {
            selectedFlowId = '';
        }
    }

    $effect(() => {
        // We observe pipelineState.steps.length
        const stepsLength = pipelineState.steps.length;

        // Let's do a more robust heuristic:
        // If the current steps don't match the selected preset length, we modified it.
        // Or if it's 0.
        if (stepsLength === 0) {
            selectedFlowId = '';
        } else if (selectedFlowId !== '') {
            const activeFlow = pipelineState.savedFlows.find(f => f.id === selectedFlowId);
            if (activeFlow && activeFlow.steps.length !== stepsLength) {
                // If they added/deleted a step, unselect the preset (i.e. it becomes Custom)
                selectedFlowId = '';
            }
        }
    });

    function handleExport() {
        const json = pipelineState.exportFlowJson();
        const blob = new Blob([json], { type: 'application/json' });
        const url = URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        const name = pipelineState.savedFlows.find(f => f.id === selectedFlowId)?.name || 'custom-pipeline';
        a.download = `${name.toLowerCase().replace(/\s+/g, '-')}.json`;
        document.body.appendChild(a);
        a.click();
        document.body.removeChild(a);
        URL.revokeObjectURL(url);
    }

    function handleFileImport(event: Event) {
        const target = event.target as HTMLInputElement;
        const file = target.files?.[0];
        if (!file) return;

        const reader = new FileReader();
        reader.onload = (e) => {
            try {
                const text = e.target?.result as string;
                const imported = pipelineState.importFlowFromJson(text);
                selectedFlowId = imported.id;
            } catch (err) {
                console.error("Failed to import JSON", err);
                alert("Invalid Pipeline JSON file.");
            }
        };
        reader.readAsText(file);
        target.value = '';
    }

    function handleDelete() {
        if (selectedFlowId) {
            const success = pipelineState.deleteFlow(selectedFlowId);
            if (success) {
                selectedFlowId = '';
                pipelineState.clearPipeline();
            }
        }
    }

    function handleNew() {
        pipelineState.clearPipeline();
        selectedFlowId = '';
    }

    let presetFlows = $derived(pipelineState.savedFlows.filter(f => f.isPreset));
    let userFlows = $derived(pipelineState.savedFlows.filter(f => !f.isPreset));
    let hasSteps = $derived(pipelineState.steps.length > 0);
    let isCustomSelected = $derived(selectedFlowId === '');
    let isUserFlowSelected = $derived(userFlows.some(f => f.id === selectedFlowId));
</script>

<div class="flow-selector-toolbar">
    <div class="selector-group">
        <select class="input-field flow-select" bind:value={selectedFlowId} onchange={handleFlowChange}>
            {#if isCustomSelected && hasSteps}
                <option value="">— Custom / Unsaved Pipeline —</option>
            {/if}
            <option value="" disabled={hasSteps && isCustomSelected}>— Select a Pipeline Flow —</option>

            <optgroup label="Standard Presets">
                {#each presetFlows as flow}
                    <option value={flow.id}>{flow.name} ({flow.steps.length} steps)</option>
                {/each}
            </optgroup>

            {#if userFlows.length > 0}
                <optgroup label="My Saved Flows">
                    {#each userFlows as flow}
                        <option value={flow.id}>{flow.name} ({flow.steps.length} steps)</option>
                    {/each}
                </optgroup>
            {/if}
        </select>
    </div>

    <div class="actions-group">
        <button class="tool-btn" onclick={handleNew} title="Clear and start a new pipeline">
            ✨ New
        </button>

        <div class="divider"></div>

        <button class="tool-btn" disabled={!hasSteps} onclick={() => isSaveModalOpen = true}>
            💾 Save Flow
        </button>

        <button class="tool-btn" disabled={!hasSteps} onclick={handleExport}>
            ⬇️ Export JSON
        </button>

        <button class="tool-btn" onclick={() => fileInput.click()}>
            ⬆️ Import JSON
        </button>
        <input type="file" accept=".json" bind:this={fileInput} onchange={handleFileImport} style="display: none;" />

        {#if isUserFlowSelected}
            <button class="tool-btn danger" onclick={handleDelete} title="Delete this saved flow">
                🗑️ Delete Flow
            </button>
        {/if}
    </div>
</div>

<SaveFlowModal
    isOpen={isSaveModalOpen}
    onClose={() => isSaveModalOpen = false}
    onSaveComplete={(id) => selectedFlowId = id}
/>

<style>
    .flow-selector-toolbar {
        background: rgba(15, 23, 42, 0.6);
        border: 1px solid rgba(255, 255, 255, 0.08);
        border-radius: 8px;
        padding: 8px 16px;
        display: flex;
        align-items: center;
        justify-content: space-between;
        gap: 12px;
        flex-wrap: wrap;
    }

    .selector-group {
        flex: 1;
        min-width: 250px;
    }

    .flow-select {
        width: 100%;
        background: rgba(0, 0, 0, 0.2);
        border: 1px solid rgba(255, 255, 255, 0.1);
        border-radius: 6px;
        color: white;
        padding: 0.5rem 0.75rem;
        font-family: inherit;
        font-size: 0.9rem;
        outline: none;
        transition: border-color 0.2s;
        cursor: pointer;
        appearance: none;
        background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='16' height='16' viewBox='0 0 24 24' fill='none' stroke='%2394a3b8' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'%3E%3Cpolyline points='6 9 12 15 18 9'%3E%3C/polyline%3E%3C/svg%3E");
        background-repeat: no-repeat;
        background-position: right 0.75rem center;
        padding-right: 2.5rem;
    }

    .flow-select:focus {
        border-color: #3b82f6;
    }

    .flow-select optgroup {
        background: #0f172a;
        color: #94a3b8;
        font-weight: 600;
    }

    .flow-select option {
        background: #0f172a;
        color: white;
        font-weight: normal;
    }

    .actions-group {
        display: flex;
        align-items: center;
        gap: 8px;
    }

    .divider {
        width: 1px;
        height: 24px;
        background: rgba(255, 255, 255, 0.1);
        margin: 0 4px;
    }

    .tool-btn {
        background: rgba(255, 255, 255, 0.05);
        border: 1px solid rgba(255, 255, 255, 0.1);
        color: #e2e8f0;
        padding: 0.5rem 0.75rem;
        border-radius: 6px;
        font-size: 0.85rem;
        font-weight: 500;
        cursor: pointer;
        transition: all 0.2s;
        display: flex;
        align-items: center;
        gap: 0.35rem;
    }

    .tool-btn:hover:not(:disabled) {
        background: rgba(255, 255, 255, 0.1);
        border-color: rgba(255, 255, 255, 0.2);
    }

    .tool-btn:disabled {
        opacity: 0.4;
        cursor: not-allowed;
    }

    .tool-btn.danger:hover:not(:disabled) {
        background: rgba(239, 68, 68, 0.15);
        color: #fca5a5;
        border-color: rgba(239, 68, 68, 0.3);
    }
</style>
