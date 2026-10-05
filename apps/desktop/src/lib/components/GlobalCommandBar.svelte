<script lang="ts">
  import { appState } from '$lib/state/app.svelte';
  import { safeInvoke } from '$lib/utils/tauri';
  import EditableActionCard from './EditableActionCard.svelte';
  import { toastState } from '$lib/state/toast.svelte';

  let inputRef = $state<HTMLInputElement | null>(null);
  let commandText = $state('');

  // NLP plan state
  let resolvedPlan = $state<any>(null);
  let isResolving = $state(false);

  // Autocomplete state
  let showAutocomplete = $state(false);
  let autocompleteQuery = $state('');
  let selectedIndex = $state(0);

  let filteredDocuments = $derived(
    appState.documents.filter(doc =>
      doc.name.toLowerCase().includes(autocompleteQuery.toLowerCase())
    )
  );

  function handleInput(e: Event) {
    const target = e.target as HTMLInputElement;
    commandText = target.value;
    const cursorPosition = target.selectionStart || 0;

    // Check if we are typing after an '@'
    const textBeforeCursor = commandText.slice(0, cursorPosition);
    const match = textBeforeCursor.match(/@([^\s]*)$/);

    if (match) {
      showAutocomplete = true;
      autocompleteQuery = match[1];
      selectedIndex = 0;
    } else {
      showAutocomplete = false;
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (showAutocomplete) {
      if (e.key === 'ArrowDown') {
        e.preventDefault();
        selectedIndex = (selectedIndex + 1) % filteredDocuments.length;
      } else if (e.key === 'ArrowUp') {
        e.preventDefault();
        selectedIndex = (selectedIndex - 1 + filteredDocuments.length) % filteredDocuments.length;
      } else if (e.key === 'Enter') {
        e.preventDefault();
        if (filteredDocuments.length > 0) {
          selectDocument(filteredDocuments[selectedIndex]);
        }
      } else if (e.key === 'Escape') {
        e.preventDefault();
        showAutocomplete = false;
      }
    } else if (e.key === 'Enter') {
      e.preventDefault();
      submitNLPCommand();
    }
  }

  async function submitNLPCommand() {
    if (!commandText.trim() || isResolving) return;

    isResolving = true;
    resolvedPlan = null;

    try {
      // Determine context
      const activeDoc = appState.selectedDocumentIndex !== null ? appState.documents[appState.selectedDocumentIndex] : null;
      let activePath = activeDoc ? ((activeDoc as any)._localPath || activeDoc.name) : undefined;

      const openDocs = appState.documents.map(d => (d as any)._localPath || d.name);

      // If user typed @"some doc" we try to match it and use it as active
      const mentionMatch = commandText.match(/@"([^"]+)"/);
      if (mentionMatch) {
        const docName = mentionMatch[1];
        const matchedDoc = appState.documents.find(d => d.name === docName);
        if (matchedDoc) {
           activePath = (matchedDoc as any)._localPath || matchedDoc.name;
        }
      }

      const plan = await safeInvoke('resolve_natural_language', {
        query: commandText,
        context: {
          active_document: activePath || null,
          open_documents: openDocs
        }
      });
      resolvedPlan = plan;
    } catch (error) {
      toastState.error(`Failed to resolve command: ${error}`);
    } finally {
      isResolving = false;
    }
  }

  function setCommand(text: string) {
    commandText = text;
    if (inputRef) {
      inputRef.focus();
    }
  }

  async function refreshAllDocuments() {
    for (let i = 0; i < appState.documents.length; i++) {
      const file = appState.documents[i];
      const path = appState.documentPaths[i];
      if (!path) continue;

      try {
        const bytes: number[] = await safeInvoke('read_file_bytes', { path });
        const blob = new Blob([new Uint8Array(bytes)], { type: file.type || 'application/pdf' });
        const newFile = new File([blob], file.name, { type: file.type || 'application/pdf' });
        (newFile as any)._localPath = path;
        (newFile as any)._isLoaded = true;
        appState.documents[i] = newFile;
      } catch (err) {
        console.error(`Failed to refresh document ${path}:`, err);
      }
    }
    appState.documents = [...appState.documents];
  }

  async function handleActionExecute(args: Record<string, any>) {
    appState.setLoading(true);
    try {
      const { intent, ...toolArgs } = args;
      const toolName = `pdf_${intent.toLowerCase()}`;

      await safeInvoke('invoke_mcp_tool', {
        toolName: toolName,
        arguments: toolArgs
      });

      await refreshAllDocuments();
      toastState.success(`${intent} operation executed successfully.`);
      resolvedPlan = null;
      commandText = '';
    } catch (err) {
      toastState.error(`Execution failed: ${err}`);
    } finally {
      appState.setLoading(false);
    }
  }

  function handleWindowKeydown(e: KeyboardEvent) {
    if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'k') {
      e.preventDefault();
      if (inputRef) {
        inputRef.focus();
      }
    }
  }

  function selectDocument(doc: File) {
    if (!inputRef) return;

    const cursorPosition = inputRef.selectionStart || 0;
    const textBeforeCursor = commandText.slice(0, cursorPosition);
    const textAfterCursor = commandText.slice(cursorPosition);

    // Replace the '@...' with '@"doc.name" '
    const match = textBeforeCursor.match(/@([^\s]*)$/);
    if (match) {
      const start = textBeforeCursor.lastIndexOf('@');
      const newText = commandText.slice(0, start) + `@"${doc.name}" ` + textAfterCursor;
      commandText = newText;

      // Update input state and cursor
      setTimeout(() => {
        if (inputRef) {
          inputRef.focus();
          const newCursorPos = start + doc.name.length + 4; // 4 is for @"..." + space
          inputRef.setSelectionRange(newCursorPos, newCursorPos);
        }
      }, 0);
    }

    showAutocomplete = false;
  }
</script>

<svelte:window onkeydown={handleWindowKeydown} />

<div class="global-command-bar-wrapper">
  {#if resolvedPlan}
    <div class="plan-preview-popover">
      <div class="plan-header">
        <h4>Action Preview</h4>
        <button class="close-btn" onclick={() => resolvedPlan = null}>×</button>
      </div>
      <EditableActionCard plan={resolvedPlan} onExecute={handleActionExecute} />
    </div>
  {/if}

  <div class="global-command-bar">
    <div class="suggestions">
      <button class="chip" aria-label="Rotate 90°" onclick={() => setCommand("Rotate 90°")}>Rotate 90°</button>
      <button class="chip" aria-label="Compress document" onclick={() => setCommand("Compress document")}>Compress document</button>
      <button class="chip" aria-label="Split pages 1-2" onclick={() => setCommand("Split pages 1-2")}>Split pages 1-2</button>
      <button class="chip" aria-label="Merge open documents" onclick={() => setCommand("Merge open documents")}>Merge open documents</button>
    </div>
    <div class="input-container">
    <input
      type="text"
      id="global-command-input"
      bind:this={inputRef}
      bind:value={commandText}
      oninput={handleInput}
      onkeydown={handleKeydown}
      placeholder={isResolving ? "Resolving action..." : "Type a command or @filename (e.g., '@doc.pdf rotate 90', 'compress all', 'merge open')..."}
      autocomplete="off"
      disabled={isResolving}
    />

    {#if showAutocomplete && filteredDocuments.length > 0}
      <div id="autocomplete-popover" class="autocomplete-popover">
        <ul>
          {#each filteredDocuments as doc, i}
            <li
              class={i === selectedIndex ? 'selected' : ''}
              onclick={() => selectDocument(doc)}
              onmouseenter={() => selectedIndex = i}
            >
              <span class="doc-icon">📄</span>
              <span class="doc-name">{doc.name}</span>
            </li>
          {/each}
        </ul>
      </div>
    {/if}
  </div>
  </div>
</div>

<style>
  .global-command-bar-wrapper {
    position: sticky;
    bottom: 0;
    left: 0;
    right: 0;
    z-index: 50;
    display: flex;
    flex-direction: column;
    align-items: center;
    width: 100%;
  }

  .plan-preview-popover {
    width: 95%;
    max-width: 600px;
    background: var(--bg-surface);
    border: 1px solid var(--border-color);
    border-radius: var(--border-radius-md);
    box-shadow: var(--shadow-lg);
    margin-bottom: 8px;
    padding: 12px;
    animation: slideUp 0.2s ease-out;
  }

  @keyframes slideUp {
    from { opacity: 0; transform: translateY(10px); }
    to { opacity: 1; transform: translateY(0); }
  }

  .plan-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 8px;
  }

  .plan-header h4 {
    margin: 0;
    color: var(--text-primary);
  }

  .close-btn {
    background: none;
    border: none;
    color: var(--text-secondary);
    font-size: 1.2rem;
    cursor: pointer;
  }

  .close-btn:hover {
    color: var(--text-primary);
  }

  .global-command-bar {
    width: calc(100% - 32px);
    background: var(--bg-surface);
    border: 1px solid var(--border-color);
    border-radius: var(--border-radius-md);
    padding: 12px 16px;
    margin: 0 16px 16px 16px;
    box-shadow: var(--shadow-lg);
    /* Glassmorphism */
    background: rgba(34, 37, 46, 0.85);
    backdrop-filter: blur(12px);
    -webkit-backdrop-filter: blur(12px);

    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .suggestions {
    display: flex;
    gap: 8px;
    overflow-x: auto;
    scrollbar-width: none; /* Firefox */
  }

  .suggestions::-webkit-scrollbar {
    display: none; /* Chrome, Safari */
  }

  .chip {
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    color: var(--text-secondary);
    border-radius: var(--border-radius-xl);
    padding: 4px 12px;
    font-size: 0.8rem;
    cursor: pointer;
    white-space: nowrap;
    transition: all var(--transition-fast);
  }

  .chip:hover {
    background: var(--bg-surface-hover);
    color: var(--text-primary);
  }

  .input-container {
    display: flex;
    position: relative;
    width: 100%;
  }

  #global-command-input {
    width: 100%;
    background: var(--surface-1);
    border: 1px solid var(--border-color);
    border-radius: var(--border-radius-sm);
    padding: 10px 14px;
    color: var(--text-primary);
    font-size: 0.95rem;
    outline: none;
    transition: box-shadow var(--transition-fast), border-color var(--transition-fast);
  }

  #global-command-input:focus {
    border-color: var(--accent-primary);
    box-shadow: 0 0 0 2px rgba(94, 106, 210, 0.25);
  }

  #global-command-input::placeholder {
    color: var(--text-muted);
  }

  .autocomplete-popover {
    position: absolute;
    bottom: calc(100% + 8px);
    left: 0;
    width: 300px;
    background: var(--bg-surface);
    border: 1px solid var(--border-color);
    border-radius: var(--border-radius-md);
    box-shadow: var(--shadow-lg);
    z-index: 100;
    overflow: hidden;
  }

  .autocomplete-popover ul {
    list-style: none;
    margin: 0;
    padding: 0;
    max-height: 200px;
    overflow-y: auto;
  }

  .autocomplete-popover li {
    padding: 8px 12px;
    display: flex;
    align-items: center;
    gap: 8px;
    cursor: pointer;
    color: var(--text-secondary);
    border-bottom: 1px solid var(--border-color);
  }

  .autocomplete-popover li:last-child {
    border-bottom: none;
  }

  .autocomplete-popover li.selected {
    background: var(--bg-surface-hover);
    color: var(--text-primary);
  }

  .doc-name {
    font-size: 0.9rem;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
</style>
