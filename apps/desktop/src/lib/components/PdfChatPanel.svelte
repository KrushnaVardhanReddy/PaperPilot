<script lang="ts">
    import { invoke } from '@tauri-apps/api/core';
    import { toastState } from '$lib/state/toast.svelte';
    import { appState } from '$lib/state/app.svelte';
    import ChatCheatSheet from '$lib/components/ChatCheatSheet.svelte';
    import EditableActionCard from './EditableActionCard.svelte';

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

        async function executePlan(args: Record<string, any>) {
        try {
            let toolName = "";
            let intent = args.intent;
            delete args.intent; // Remove from MCP args

            switch (intent) {
                case "Rotate": toolName = "pdf_rotate"; break;
                case "Merge": toolName = "pdf_merge"; break;
                case "Split": toolName = "pdf_split"; break;
                case "Compress": toolName = "pdf_compress"; break;
                case "Delete": toolName = "pdf_delete_pages"; break;
                case "Reorder": toolName = "pdf_reorder_pages"; break;
                case "Burst": toolName = "pdf_burst"; break;
                case "Crop": toolName = "pdf_crop"; break;
                case "Extract": toolName = "pdf_extract_pages"; break;
                // Add Stirling parity
                case "RemoveBlank": toolName = "pdf_remove_blank"; break;
                case "PageNumbers": toolName = "pdf_page_numbers"; break;

                // Edit & Markup
                case "Watermark": toolName = "pdf_watermark"; break;
                case "Bates": toolName = "pdf_bates"; break;
                case "HeaderFooter": toolName = "pdf_header_footer"; break;
                case "Flatten": toolName = "pdf_flatten"; break;
                case "Annotate": toolName = "pdf_annotate"; break;

                // Security
                case "Encrypt": toolName = "pdf_encrypt"; break;
                case "Decrypt": toolName = "pdf_decrypt"; break;
                case "Redact": toolName = "pdf_redact"; break;
                case "Sign": toolName = "pdf_sign"; break;
                case "Metadata": toolName = "pdf_metadata"; break;
                case "Validate": toolName = "pdf_validate"; break;
                case "Hash": toolName = "pdf_hash"; break;

                // Conversions
                case "ToDocx": toolName = "pdf_to_docx"; break;
                case "ToXlsx": toolName = "pdf_to_xlsx"; break;
                case "ToPptx": toolName = "pdf_to_pptx"; break;
                case "PdfA": toolName = "pdf_to_pdf_a"; break;
                case "ExtractText": toolName = "pdf_extract_text"; break;
                case "ExtractImages": toolName = "pdf_extract_images"; break;
                case "ToMarkdown": toolName = "pdf_convert_markdown"; break;
                case "ToHtml": toolName = "pdf_convert_html"; break;
                case "ImagesToPdf": toolName = "pdf_images_to_pdf"; break;
                case "Render": toolName = "pdf_render"; break;

                // Forms
                case "FormRead": toolName = "pdf_read_form"; break;
                case "FormFill": toolName = "pdf_fill_form"; break;
                case "FormCreate": toolName = "pdf_create_form_field"; break;

                // Extra
                case "Bookmarks": toolName = "pdf_bookmarks"; break;
                case "Search": toolName = "pdf_search"; break;
                case "Ocr": toolName = "pdf_ocr"; break;
                case "Compare": toolName = "pdf_compare"; break; // Needs custom handling if diff view
                case "Linearize": toolName = "pdf_linearize"; break;
                case "Classify": toolName = "pdf_classify_type"; break;

                default:
                    toastState.error(`Unsupported intent via UI: ${intent}`);
                    return;
            }

            toastState.info(`Executing ${intent}...`);
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
                    <EditableActionCard plan={msg.plan} onExecute={executePlan} />
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
