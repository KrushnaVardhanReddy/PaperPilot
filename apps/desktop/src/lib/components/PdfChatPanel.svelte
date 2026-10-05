<script lang="ts">
    import { invoke } from '@tauri-apps/api/core';
    import { toastState } from '$lib/state/toast.svelte';
    import { appState } from '$lib/state/app.svelte';
    import ChatCheatSheet from '$lib/components/ChatCheatSheet.svelte';

    let query = $state('');
    let messages = $state<{role: 'user' | 'assistant', content: string, plan?: any}[]>([]);
    let isThinking = $state(false);
    let isCheatSheetOpen = $state(false);
    let chatInputRef = $state<HTMLInputElement | null>(null);

    const suggestions = [
        "Compress this PDF",
        "Split pages 1 to 5",
        "Rotate 90 degrees",
        "Ask docs: how to sign?"
    ];

    async function handleSend(text?: string) {
        const textToSend = text || query;
        if (!textToSend.trim()) return;

        messages = [...messages, { role: 'user', content: textToSend }];
        query = '';
        isThinking = true;

        try {
            const activeDoc = appState.selectedDocumentIndex !== null ? appState.documents[appState.selectedDocumentIndex] : null;
            const activePath = activeDoc ? ((activeDoc as any)._localPath || activeDoc.name) : undefined;
            const openDocs = appState.documents.map(d => (d as any)._localPath || d.name);

            const plan = await invoke('resolve_natural_language', {
                query: textToSend,
                context: {
                    active_document: activePath || null,
                    open_documents: openDocs
                }
            });
            messages = [...messages, { role: 'assistant', content: "Here's what I can do:", plan }];
        } catch (error) {
            messages = [...messages, { role: 'assistant', content: `Error: ${error}` }];
        } finally {
            isThinking = false;
        }
    }

    async function executePlan(plan: any) {
        try {
            const args: Record<string, any> = {};

            // Map plan inputs to MCP args based on intent
            if (plan.input_files && plan.input_files.length > 0) {
                // If there's an active document, prioritize it unless plan specifies otherwise
                // For now, let's use the first input file or active document
                args.input = plan.input_files[0];
            }

            if (plan.output_file) args.output = plan.output_file;
            if (plan.angles && plan.angles.length > 0) args.angle = plan.angles[0];

            let toolName = "";
            switch (plan.intent) {
                case "Rotate": toolName = "pdf_rotate"; break;
                case "Merge": toolName = "pdf_merge"; args.inputs = plan.input_files; delete args.input; break;
                case "Split": toolName = "pdf_split"; break;
                // Add more mappings
                default:
                    toastState.error(`Unsupported intent via UI: ${plan.intent}`);
                    return;
            }

            toastState.info(`Executing ${plan.intent}...`);
            const result = await invoke('invoke_mcp_tool', {
                toolName,
                arguments: args
            });
            toastState.success(`Action completed successfully!`);

            messages = [...messages, { role: 'assistant', content: `Execution successful: ${JSON.stringify(result)}` }];
        } catch (error) {
            toastState.error(`Failed to execute: ${error}`);
        }
    }

    function handleSelectPrompt(prompt: string) {
        query = prompt;
        if (chatInputRef) {
            setTimeout(() => chatInputRef?.focus(), 0);
        }
    }
</script>

<div class="chat-panel">
    <div class="messages">
        {#if messages.length === 0}
            <div class="welcome">
                <p>Hello! I'm your AI assistant. How can I help you with this document?</p>
                <div class="suggestions">
                    {#each suggestions as suggestion}
                        <button class="suggestion-chip" onclick={() => handleSend(suggestion)}>
                            {suggestion}
                        </button>
                    {/each}
                </div>
            </div>
        {/if}

        {#each messages as msg}
            <div class="message {msg.role}">
                <div class="bubble">
                    {msg.content}
                </div>
                {#if msg.plan}
                    <div class="plan-card">
                        <h4>{msg.plan.intent} Operation</h4>
                        <div class="plan-details">
                            {#if msg.plan.input_files?.length > 0}
                                <div><strong>Inputs:</strong> {msg.plan.input_files.join(', ')}</div>
                            {/if}
                            {#if msg.plan.output_file}
                                <div><strong>Output:</strong> {msg.plan.output_file}</div>
                            {/if}
                            {#if msg.plan.angles?.length > 0}
                                <div><strong>Angle:</strong> {msg.plan.angles.join(', ')}</div>
                            {/if}
                        </div>
                        <button class="execute-btn" onclick={() => executePlan(msg.plan)}>
                            ⚡ Execute Action
                        </button>
                    </div>
                {/if}
            </div>
        {/each}
        {#if isThinking}
            <div class="message assistant thinking">
                <div class="bubble">Thinking...</div>
            </div>
        {/if}
    </div>

    <div class="chat-footer">
        <div class="footer-actions">
            <button class="examples-btn" onclick={() => isCheatSheetOpen = true} title="View examples (press ?)">
                💡 Examples
            </button>
        </div>
        <div class="input-area">
            <input
                type="text"
                bind:this={chatInputRef}
                bind:value={query}
                placeholder="Type a command (press ? for examples)..."
                onkeydown={(e) => {
                    if (e.key === 'Enter') handleSend();
                    if (e.key === '?' && query === '') {
                        e.preventDefault();
                        isCheatSheetOpen = true;
                    }
                }}
            />
            <button onclick={() => handleSend()}>Send</button>
        </div>
    </div>
</div>

<ChatCheatSheet
    bind:isOpen={isCheatSheetOpen}
    onSelectPrompt={handleSelectPrompt}
    onClose={() => isCheatSheetOpen = false}
/>

<style>
    .chat-panel {
        display: flex;
        flex-direction: column;
        height: 100%;
        background: var(--bg-primary);
        font-family: inherit;
    }

    .messages {
        flex: 1;
        overflow-y: auto;
        padding: 1rem;
        display: flex;
        flex-direction: column;
        gap: 1rem;
    }

    .welcome {
        text-align: center;
        color: var(--text-secondary);
        margin-top: 2rem;
    }

    .suggestions {
        display: flex;
        flex-wrap: wrap;
        gap: 0.5rem;
        justify-content: center;
        margin-top: 1rem;
    }

    .suggestion-chip {
        background: var(--bg-surface);
        border: 1px solid var(--border-color);
        border-radius: 16px;
        padding: 0.5rem 1rem;
        font-size: 0.85rem;
        cursor: pointer;
        color: var(--text-primary);
        transition: all 0.2s;
    }

    .suggestion-chip:hover {
        background: var(--bg-surface-hover);
        border-color: var(--accent-primary);
    }

    .message {
        display: flex;
        flex-direction: column;
        max-width: 90%;
    }

    .message.user {
        align-self: flex-end;
    }

    .message.assistant {
        align-self: flex-start;
    }

    .bubble {
        padding: 0.75rem 1rem;
        border-radius: 8px;
        font-size: 0.9rem;
    }

    .message.user .bubble {
        background: var(--accent-primary);
        color: white;
        border-bottom-right-radius: 2px;
    }

    .message.assistant .bubble {
        background: var(--bg-surface);
        border: 1px solid var(--border-color);
        color: var(--text-primary);
        border-bottom-left-radius: 2px;
    }

    .thinking .bubble {
        color: var(--text-muted);
        font-style: italic;
    }

    .plan-card {
        margin-top: 0.5rem;
        background: var(--bg-surface);
        border: 1px solid var(--border-color);
        border-radius: 8px;
        padding: 1rem;
        font-size: 0.85rem;
    }

    .plan-card h4 {
        margin: 0 0 0.5rem 0;
        color: var(--accent-primary);
    }

    .plan-details {
        margin-bottom: 1rem;
        color: var(--text-secondary);
    }

    .execute-btn {
        width: 100%;
        background: var(--accent-primary);
        color: white;
        border: none;
        border-radius: 4px;
        padding: 0.5rem;
        cursor: pointer;
        font-weight: bold;
        transition: opacity 0.2s;
    }

    .execute-btn:hover {
        opacity: 0.9;
    }

    .chat-footer {
        display: flex;
        flex-direction: column;
        border-top: 1px solid var(--border-color);
        background: var(--bg-secondary);
    }

    .footer-actions {
        padding: 0.5rem 1rem 0;
        display: flex;
        justify-content: flex-start;
    }

    .examples-btn {
        background: transparent;
        border: 1px solid var(--border-color);
        color: var(--text-secondary);
        padding: 0.25rem 0.75rem;
        border-radius: 12px;
        font-size: 0.8rem;
        cursor: pointer;
        transition: all 0.2s;
        display: flex;
        align-items: center;
        gap: 0.25rem;
    }

    .examples-btn:hover {
        background: var(--bg-surface);
        color: var(--text-primary);
        border-color: var(--accent-primary);
    }

    .input-area {
        display: flex;
        padding: 0.75rem 1rem 1rem;
    }

    .input-area input {
        flex: 1;
        padding: 0.5rem;
        border: 1px solid var(--border-color);
        border-radius: 4px 0 0 4px;
        background: var(--bg-primary);
        color: var(--text-primary);
    }

    .input-area button {
        padding: 0.5rem 1rem;
        background: var(--accent-primary);
        color: white;
        border: none;
        border-radius: 0 4px 4px 0;
        cursor: pointer;
    }
</style>
