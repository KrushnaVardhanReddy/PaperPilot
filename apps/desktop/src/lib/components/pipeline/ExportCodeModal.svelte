<script lang="ts">
    import { slide, fade } from 'svelte/transition';
    import { pipelineState } from '$lib/state/pipeline.svelte';
    import { type SnippetTarget, type GeneratedSnippets, generateSnippets } from '$lib/utils/snippetGenerator';

    export let isOpen = false;
    export let onClose: () => void;
    export let target: SnippetTarget | null = null;

    let selectedTab = 'curl';
    let copyText = 'Copy to Clipboard';
    let timeoutId: number;

    const tabs = [
        { id: 'curl', label: 'cURL (REST API)' },
        { id: 'cli', label: 'CLI Command' },
        { id: 'mcp', label: 'MCP Agent Prompt' },
        { id: 'js', label: 'Embed JS' }
    ];

    $: recipeJSON = isOpen && !target ? pipelineState.exportFlowJson() : '{}';
    $: snippets = isOpen && target ? generateSnippets(target) : null;

    $: codeSnippet = (() => {
        if (!isOpen) return '';

        if (snippets) {
            switch (selectedTab) {
                case 'curl': return snippets.curl;
                case 'cli': return snippets.cli;
                case 'mcp': return `${snippets.mcpAgentPrompt}\n\nTool Call:\n${snippets.mcpToolCall}`;
                case 'js': return snippets.embedJs;
                default: return '';
            }
        } else {
            switch (selectedTab) {
                case 'curl':
                    return `curl -X POST http://localhost:8080/api/v1/workflow/run \\
  -H "Content-Type: application/json" \\
  -d '${recipeJSON}'`;
                case 'cli':
                    return `paperpilot workflow --recipe flow.json --input doc.pdf --output out.pdf`;
                case 'mcp':
                    return `Please invoke the \`document_workflow\` tool using the following recipe:

${recipeJSON}`;
                case 'js':
                    return `window.PaperPilot.runPipeline(${recipeJSON});`;
                default:
                    return '';
            }
        }
    })();

    async function handleCopy() {
        try {
            await navigator.clipboard.writeText(codeSnippet);
            copyText = 'Copied!';
            if (timeoutId) clearTimeout(timeoutId);
            timeoutId = window.setTimeout(() => {
                copyText = 'Copy to Clipboard';
            }, 2000);
        } catch (err) {
            console.error('Failed to copy text: ', err);
            copyText = 'Failed to copy';
        }
    }
</script>

{#if isOpen}
    <div class="modal-backdrop" id="export-code-modal" transition:fade={{duration: 200}}>
        <div class="modal-drawer" transition:slide={{axis: 'x', duration: 300}}>
            <div class="modal-header">
                <h2>{target ? 'Export Action as Code / API' : 'Export as Code / API'}</h2>
                <button class="close-btn" aria-label="Close" on:click={onClose}>✕</button>
            </div>

            <div class="modal-content">
                <div class="tabs">
                    {#each tabs as tab}
                        <button
                            class="tab-btn"
                            class:active={selectedTab === tab.id}
                            on:click={() => selectedTab = tab.id}
                        >
                            {tab.label}
                        </button>
                    {/each}
                </div>

                <div class="code-container">
                    <pre><code>{codeSnippet}</code></pre>
                    <button class="copy-btn" on:click={handleCopy}>{copyText}</button>
                </div>

                {#if selectedTab === 'cli' && !target}
                    <p class="help-text">Save the exported JSON to <code>flow.json</code> to run this CLI command.</p>
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
        z-index: 9999; /* Ensure backdrop is on top of everything */
        display: flex;
        justify-content: flex-end;
    }

    .modal-drawer {
        width: 550px;
        max-width: 90vw;
        height: 100%;
        background: #0f172a;
        box-shadow: -4px 0 24px rgba(0, 0, 0, 0.5);
        display: flex;
        flex-direction: column;
        color: #f8fafc;
        border-left: 1px solid rgba(255, 255, 255, 0.1);
        z-index: 1001; /* Ensure drawer is on top */
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
        gap: 1rem;
    }

    .tabs {
        display: flex;
        gap: 0.5rem;
        background: rgba(0, 0, 0, 0.2);
        padding: 0.5rem;
        border-radius: 8px;
        border: 1px solid rgba(255, 255, 255, 0.1);
        overflow-x: auto;
    }

    .tab-btn {
        flex: 1;
        background: transparent;
        border: none;
        color: #94a3b8;
        padding: 0.5rem;
        border-radius: 4px;
        font-size: 0.85rem;
        font-weight: 500;
        cursor: pointer;
        transition: all 0.2s;
        white-space: nowrap;
    }

    .tab-btn:hover {
        color: #e2e8f0;
        background: rgba(255, 255, 255, 0.05);
    }

    .tab-btn.active {
        color: white;
        background: rgba(255, 255, 255, 0.1);
        box-shadow: 0 1px 3px rgba(0,0,0,0.1);
    }

    .code-container {
        position: relative;
        background: rgba(0, 0, 0, 0.3);
        border: 1px solid rgba(255, 255, 255, 0.1);
        border-radius: 8px;
        flex: 1;
        display: flex;
        flex-direction: column;
        overflow: hidden;
    }

    pre {
        margin: 0;
        padding: 2.5rem 1rem 1rem 1rem; /* Give space for copy button */
        overflow: auto;
        flex: 1;
    }

    code {
        font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;
        font-size: 0.85rem;
        color: #a78bfa; /* Light purple for syntax */
        line-height: 1.5;
        white-space: pre-wrap;
        word-break: break-all;
    }

    .copy-btn {
        position: absolute;
        top: 0.5rem;
        right: 0.5rem;
        background: rgba(255, 255, 255, 0.1);
        border: 1px solid rgba(255, 255, 255, 0.2);
        color: white;
        padding: 0.25rem 0.75rem;
        border-radius: 4px;
        font-size: 0.75rem;
        cursor: pointer;
        transition: all 0.2s;
    }

    .copy-btn:hover {
        background: rgba(255, 255, 255, 0.2);
    }

    .help-text {
        font-size: 0.85rem;
        color: #94a3b8;
        margin: 0;
    }

    .help-text code {
        color: #e2e8f0;
        background: rgba(0, 0, 0, 0.2);
        padding: 0.1rem 0.3rem;
        border-radius: 3px;
    }
</style>
