<script lang="ts">
    import { slide, fade } from 'svelte/transition';
    import { pipelineState } from '$lib/state/pipeline.svelte';
    import { type SnippetTarget, type GeneratedSnippets, generateSnippets } from '$lib/utils/snippetGenerator';

    let {
        isOpen = false,
        onClose,
        target = null
    }: {
        isOpen?: boolean;
        onClose: () => void;
        target?: SnippetTarget | null;
    } = $props();

    let selectedTab = $state<'cli' | 'curl' | 'mcp' | 'js'>('cli');
    let copyText = $state('Copy to Clipboard');
    let timeoutId: number | undefined = undefined;

    const tabs = [
        { id: 'cli' as const, label: 'CLI Command', icon: '💻', desc: 'Native terminal command' },
        { id: 'curl' as const, label: 'cURL / REST API', icon: '⚡', desc: 'Port 7823 Local Gateway' },
        { id: 'mcp' as const, label: 'MCP Agent Prompt', icon: '🤖', desc: 'Claude Desktop / Model Context' },
        { id: 'js' as const, label: 'Embed JS', icon: '🌐', desc: 'Browser WebAssembly SDK' }
    ];

    let recipeJSON = $derived(isOpen && !target ? pipelineState.exportFlowJson() : '{}');
    
    let snippets = $derived.by(() => {
        if (!isOpen || !target) return null;
        try {
            return generateSnippets(target);
        } catch (e) {
            console.error('Failed to generate snippets:', e);
            return null;
        }
    });

    let codeSnippet = $derived.by(() => {
        if (!isOpen) return '';

        if (snippets) {
            switch (selectedTab) {
                case 'cli': return snippets.cli || '';
                case 'curl': return snippets.curl || '';
                case 'mcp': return `${snippets.mcpAgentPrompt || ''}\n\nTool Call:\n${snippets.mcpToolCall || ''}`;
                case 'js': return snippets.embedJs || '';
                default: return '';
            }
        } else {
            switch (selectedTab) {
                case 'cli':
                    return `paperpilot workflow --recipe flow.json --input doc.pdf --output out.pdf`;
                case 'curl':
                    return `curl -X POST http://localhost:7823/api/v1/workflow/run \\\n  -H "Content-Type: application/json" \\\n  -d '${recipeJSON}'`;
                case 'mcp':
                    return `Please invoke the \`document_workflow\` tool using the following recipe:\n\n${recipeJSON}`;
                case 'js':
                    return `window.PaperPilot.runPipeline(${recipeJSON});`;
                default:
                    return '';
            }
        }
    });

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
    <div
        class="modal-backdrop"
        id="export-code-modal"
        transition:fade={{duration: 180}}
        onclick={(e) => {
            if (e.target === e.currentTarget) onClose();
        }}
        role="presentation"
    >
        <div class="modal-drawer" transition:slide={{axis: 'x', duration: 250}}>
            <div class="modal-header">
                <div class="header-titles">
                    <span class="header-badge">&lt;/&gt; Developer Code &amp; API</span>
                    <h2>{target ? (target.intent || 'Export Tool Action') : 'Export Workflow Pipeline'}</h2>
                </div>
                <button class="close-btn" aria-label="Close" onclick={onClose}>✕</button>
            </div>

            <div class="modal-layout">
                <!-- Vertical Tab Navigation Sidebar -->
                <nav class="vertical-tabs" aria-label="Export format tabs">
                    {#each tabs as tab}
                        <button
                            class="vertical-tab-btn"
                            class:active={selectedTab === tab.id}
                            onclick={() => selectedTab = tab.id}
                            type="button"
                        >
                            <span class="tab-icon">{tab.icon}</span>
                            <div class="tab-info">
                                <span class="tab-label">{tab.label}</span>
                                <span class="tab-desc">{tab.desc}</span>
                            </div>
                        </button>
                    {/each}
                </nav>

                <!-- Code Container & Content Area -->
                <div class="code-area">
                    <div class="code-header">
                        <span class="code-format-indicator">
                            {tabs.find(t => t.id === selectedTab)?.label}
                        </span>
                        <button class="copy-btn" onclick={handleCopy}>
                            {copyText}
                        </button>
                    </div>

                    <div class="code-container">
                        <pre><code>{codeSnippet}</code></pre>
                    </div>

                    {#if selectedTab === 'cli' && !target}
                        <p class="help-text">Save the exported JSON to <code>flow.json</code> to run this CLI command.</p>
                    {:else if selectedTab === 'curl'}
                        <p class="help-text">Gateway runs on <code>http://127.0.0.1:7823</code> (enable in Settings &gt; Developer).</p>
                    {:else if selectedTab === 'mcp'}
                        <p class="help-text">Compatible with Anthropic Claude Desktop, Cursor, and standard MCP clients.</p>
                    {/if}
                </div>
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
        background: rgba(0, 0, 0, 0.65);
        backdrop-filter: blur(4px);
        z-index: 99999;
        display: flex;
        justify-content: flex-end;
    }

    .modal-drawer {
        width: 680px;
        max-width: 95vw;
        height: 100%;
        background: var(--bg-secondary, #141416);
        box-shadow: -8px 0 32px rgba(0, 0, 0, 0.5);
        display: flex;
        flex-direction: column;
        color: var(--text-primary, #f8fafc);
        border-left: 1px solid var(--border-color, #2a2a35);
        z-index: 100000;
    }

    .modal-header {
        padding: 1.25rem 1.5rem;
        border-bottom: 1px solid var(--border-color, #2a2a35);
        display: flex;
        justify-content: space-between;
        align-items: center;
        background: var(--bg-surface, #1e1e24);
    }

    .header-titles {
        display: flex;
        flex-direction: column;
        gap: 0.25rem;
    }

    .header-badge {
        font-size: 0.75rem;
        font-weight: 600;
        color: var(--accent-primary, #6366f1);
        text-transform: uppercase;
        letter-spacing: 0.5px;
    }

    .modal-header h2 {
        margin: 0;
        font-size: 1.15rem;
        font-weight: 600;
        color: var(--text-primary, #ffffff);
    }

    .close-btn {
        background: transparent;
        border: 1px solid transparent;
        color: var(--text-muted, #94a3b8);
        font-size: 1.2rem;
        cursor: pointer;
        padding: 4px 8px;
        border-radius: 6px;
        transition: all 0.15s ease;
    }

    .close-btn:hover {
        color: var(--text-primary, #ffffff);
        background: var(--bg-surface-hover, rgba(255, 255, 255, 0.08));
        border-color: var(--border-color, #2a2a35);
    }

    .modal-layout {
        display: flex;
        flex: 1;
        overflow: hidden;
    }

    /* Vertical Tabs Sidebar */
    .vertical-tabs {
        width: 220px;
        min-width: 200px;
        background: var(--bg-primary, #0f172a);
        border-right: 1px solid var(--border-color, #2a2a35);
        padding: 1rem 0.75rem;
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }

    .vertical-tab-btn {
        display: flex;
        align-items: flex-start;
        gap: 0.75rem;
        padding: 0.75rem 0.85rem;
        background: transparent;
        border: 1px solid transparent;
        border-radius: 8px;
        cursor: pointer;
        text-align: left;
        transition: all 0.15s ease;
        color: var(--text-secondary, #94a3b8);
    }

    .vertical-tab-btn:hover {
        background: var(--bg-surface-hover, rgba(255, 255, 255, 0.05));
        color: var(--text-primary, #f8fafc);
    }

    .vertical-tab-btn.active {
        background: var(--bg-surface, #1e1e24);
        border-color: var(--accent-primary, #6366f1);
        color: var(--text-primary, #ffffff);
        box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
    }

    .tab-icon {
        font-size: 1.2rem;
        line-height: 1.2;
    }

    .tab-info {
        display: flex;
        flex-direction: column;
        gap: 2px;
    }

    .tab-label {
        font-size: 0.85rem;
        font-weight: 600;
    }

    .tab-desc {
        font-size: 0.7rem;
        color: var(--text-muted, #64748b);
        line-height: 1.2;
    }

    .vertical-tab-btn.active .tab-desc {
        color: var(--accent-primary, #818cf8);
    }

    /* Code View Area */
    .code-area {
        flex: 1;
        display: flex;
        flex-direction: column;
        padding: 1.25rem;
        background: var(--bg-secondary, #141416);
        overflow: hidden;
        gap: 0.75rem;
    }

    .code-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
    }

    .code-format-indicator {
        font-size: 0.8rem;
        font-weight: 600;
        color: var(--text-muted, #94a3b8);
        text-transform: uppercase;
        letter-spacing: 0.5px;
    }

    .copy-btn {
        background: var(--accent-primary, #6366f1);
        border: none;
        color: #ffffff;
        padding: 0.4rem 0.9rem;
        border-radius: 6px;
        font-size: 0.8rem;
        font-weight: 500;
        cursor: pointer;
        transition: opacity 0.15s ease;
    }

    .copy-btn:hover {
        opacity: 0.9;
    }

    .code-container {
        flex: 1;
        background: var(--bg-primary, #090d16);
        border: 1px solid var(--border-color, #2a2a35);
        border-radius: 8px;
        overflow: hidden;
        display: flex;
    }

    pre {
        margin: 0;
        padding: 1rem;
        overflow: auto;
        flex: 1;
    }

    code {
        font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", monospace;
        font-size: 0.85rem;
        color: var(--accent-active, #c7d2fe);
        line-height: 1.6;
        white-space: pre-wrap;
        word-break: break-all;
    }

    .help-text {
        font-size: 0.8rem;
        color: var(--text-muted, #94a3b8);
        margin: 0;
        line-height: 1.4;
    }

    .help-text code {
        color: var(--text-primary, #e2e8f0);
        background: var(--bg-surface, #1e1e24);
        padding: 0.15rem 0.35rem;
        border-radius: 4px;
        font-size: 0.75rem;
    }
</style>
