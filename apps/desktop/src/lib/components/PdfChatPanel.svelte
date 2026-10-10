<script lang="ts">
    import { invoke } from '@tauri-apps/api/core';
    import { toastState } from '$lib/state/toast.svelte';
    import { appState } from '$lib/state/app.svelte';
    import ChatCheatSheet from '$lib/components/ChatCheatSheet.svelte';
    import EditableActionCard from './EditableActionCard.svelte';

    interface ChatMessage {
        role: 'user' | 'assistant';
        content: string;
        plan?: any;
        result?: any;
    }

    let query = $state('');
    let messages = $state<ChatMessage[]>([]);
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

        const lowerQuery = textToSend.toLowerCase();
        const isDocQuery = lowerQuery.startsWith('how') || lowerQuery.startsWith('what is') || lowerQuery.includes('docs') || lowerQuery.includes('help') || lowerQuery.includes('syntax') || lowerQuery.includes('?');

        if (isDocQuery) {
            try {
                const docAns = await invoke('query_documentation_rag', { query: textToSend }) as any;
                if (docAns && docAns.confidence_score >= 0.39) {
                    messages = [...messages, {
                        role: 'assistant',
                        content: `**📖 Documentation: ${docAns.title}**\n${docAns.explanation}`,
                        result: {
                            is_docs: true,
                            docAns
                        }
                    }];
                    isThinking = false;
                    return;
                }
            } catch (err) {
                // fall back to resolve_natural_language silently
            }
        }

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
                case "ToJson": toolName = "pdf_to_json"; break;
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

            const resultObj: any = typeof result === 'object' && result !== null ? result : { message: String(result), success: true };
            messages = [...messages, { 
                role: 'assistant', 
                content: resultObj.message || 'Action completed successfully.',
                result: resultObj
            }];
        } catch (error) {
            toastState.error(`Failed to execute: ${error}`);
            messages = [...messages, { role: 'assistant', content: `Error: ${error}` }];
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
                {#if msg.result}
                    {#if msg.result.is_docs}
                        <div class="docs-answer-card">
                            <div class="docs-header">
                                <span class="docs-badge">📖 Documentation: {msg.result.docAns.title}</span>
                            </div>
                            <div class="docs-explanation">{msg.result.docAns.explanation}</div>
                            <div class="docs-snippets">
                                <div class="snippet-box">
                                    <span class="snippet-label">CLI</span>
                                    <code class="snippet-value">{msg.result.docAns.cli_example}</code>
                                    <button class="copy-btn" onclick={() => navigator.clipboard.writeText(msg.result.docAns.cli_example)}>[Copy Snippet 📋]</button>
                                </div>
                                <div class="snippet-box">
                                    <span class="snippet-label">cURL</span>
                                    <code class="snippet-value">{msg.result.docAns.curl_example}</code>
                                    <button class="copy-btn" onclick={() => navigator.clipboard.writeText(msg.result.docAns.curl_example)}>[Copy Snippet 📋]</button>
                                </div>
                                <div class="snippet-box">
                                    <span class="snippet-label">MCP</span>
                                    <code class="snippet-value">{msg.result.docAns.mcp_example}</code>
                                    <button class="copy-btn" onclick={() => navigator.clipboard.writeText(msg.result.docAns.mcp_example)}>[Copy Snippet 📋]</button>
                                </div>
                            </div>
                        </div>
                    {:else}
                        <div class="result-card">
                            <div class="result-header">
                                <span class="result-badge">✅ Success</span>
                                <span class="result-title">{msg.result.message || 'Completed'}</span>
                            </div>
                            {#if msg.result.output_path}
                                <div class="result-path-box">
                                    <span class="path-label">Output file:</span>
                                    <code class="path-value">{msg.result.output_path}</code>
                                </div>
                            {/if}
                        </div>
                    {/if}
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
        white-space: pre-wrap;
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

    .result-card {
        margin-top: 0.5rem;
        background: var(--bg-surface);
        border: 1px solid rgba(74, 222, 128, 0.3);
        border-radius: 8px;
        padding: 0.75rem 1rem;
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
        font-size: 0.85rem;
        animation: fadeIn 0.2s ease-out;
    }

    .result-header {
        display: flex;
        align-items: center;
        gap: 0.5rem;
    }

    .result-badge {
        font-size: 0.75rem;
        font-weight: 600;
        background: rgba(74, 222, 128, 0.15);
        color: #4ade80;
        padding: 0.15rem 0.5rem;
        border-radius: 4px;
        border: 1px solid rgba(74, 222, 128, 0.3);
    }

    .result-title {
        font-weight: 500;
        color: var(--text-primary);
    }

    .result-path-box {
        display: flex;
        flex-direction: column;
        gap: 0.2rem;
        background: var(--bg-primary);
        border: 1px solid var(--border-color);
        border-radius: 6px;
        padding: 0.4rem 0.6rem;
    }

    .path-label {
        font-size: 0.7rem;
        color: var(--text-muted);
        text-transform: uppercase;
        letter-spacing: 0.5px;
    }

    .path-value {
        font-family: monospace;
        font-size: 0.75rem;
        color: var(--text-secondary);
        word-break: break-all;
    }

    @keyframes fadeIn {
        from { opacity: 0; transform: translateY(4px); }
        to { opacity: 1; transform: translateY(0); }
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

    .docs-answer-card {
        margin-top: 0.5rem;
        background: var(--bg-surface);
        border: 1px solid var(--accent-primary);
        border-radius: 8px;
        padding: 1rem;
        display: flex;
        flex-direction: column;
        gap: 0.75rem;
        font-size: 0.85rem;
    }
    .docs-header { display: flex; align-items: center; }
    .docs-badge { font-weight: 600; color: var(--accent-primary); font-size: 0.9rem; }
    .docs-explanation { color: var(--text-secondary); }
    .docs-snippets { display: flex; flex-direction: column; gap: 0.5rem; }
    .snippet-box {
        background: var(--bg-primary);
        border: 1px solid var(--border-color);
        border-radius: 6px;
        padding: 0.5rem;
        display: flex;
        flex-direction: column;
        gap: 0.25rem;
        position: relative;
    }
    .snippet-label { font-size: 0.7rem; color: var(--text-muted); font-weight: bold; text-transform: uppercase; }
    .snippet-value { font-family: monospace; font-size: 0.75rem; color: var(--text-primary); word-break: break-all; white-space: pre-wrap; margin-bottom: 0.5rem; }
    .copy-btn {
        align-self: flex-start;
        background: transparent;
        border: 1px solid var(--border-color);
        color: var(--text-secondary);
        padding: 0.2rem 0.5rem;
        border-radius: 4px;
        font-size: 0.7rem;
        cursor: pointer;
        transition: all 0.2s;
    }
    .copy-btn:hover { background: var(--bg-surface); color: var(--text-primary); border-color: var(--accent-primary); }
</style>
