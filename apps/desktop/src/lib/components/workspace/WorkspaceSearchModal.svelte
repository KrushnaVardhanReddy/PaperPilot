<script lang="ts">
  import { onMount } from 'svelte';
  import { appState } from '$lib/state/app.svelte';
  import { searchDocumentText, type SearchSnippetMatch, type WorkspaceSearchResult } from '$lib/services/workspaceSearch';
  import { safeInvoke } from '$lib/utils/tauri';
  import * as pdfjsLib from 'pdfjs-dist';

  // Reuse Vite worker import trick if PDF.js is used
  import pdfjsWorker from 'pdfjs-dist/build/pdf.worker.mjs?url';
  pdfjsLib.GlobalWorkerOptions.workerSrc = pdfjsWorker;

  export let isOpen = false;

  let searchQuery = '';
  let matchCase = false;
  let wholeWord = false;
  let activeFilter: 'all' | 'pdf' | 'docx' | 'xlsx' = 'all';

  let inputEl: HTMLInputElement;

  let isSearching = false;
  let searchResults: WorkspaceSearchResult | null = null;

  // Cache for document texts
  let docTextCache: Map<number, { text: string, docName: string }> = new Map();
  let searchTimeout: ReturnType<typeof setTimeout>;

  async function extractPdfText(fileUrl: string): Promise<string> {
    const loadingTask = pdfjsLib.getDocument({ url: fileUrl });
    const pdf = await loadingTask.promise;
    let fullText = '';
    for (let i = 1; i <= pdf.numPages; i++) {
        const page = await pdf.getPage(i);
        const textContent = await page.getTextContent();
        const pageText = textContent.items.map((item: any) => item.str).join(' ');
        fullText += pageText + ' \n ';
    }
    return fullText;
  }

  async function extractOfficeText(filePath: string): Promise<string> {
    const tempOut = filePath + '_temp_extract.txt';
    try {
        await safeInvoke('invoke_mcp_tool', {
            tool_name: 'office_extract_text',
            arguments: { input: filePath, output: tempOut }
        });
        const bytes = await safeInvoke('read_file_bytes', { path: tempOut });
        if (bytes) {
           return new TextDecoder().decode(new Uint8Array(bytes));
        }
    } catch (e) {
        console.warn('Failed to extract office text:', e);
    }
    return '';
  }

  async function ensureDocTextCached(docIndex: number, docPath: string, docName: string, fileObject: File): Promise<string> {
    if (docTextCache.has(docIndex)) {
        return docTextCache.get(docIndex)!.text;
    }

    let text = '';
    const ext = docName.split('.').pop()?.toLowerCase() || '';

    if (ext === 'pdf') {
        const fileUrl = URL.createObjectURL(fileObject);
        try {
            text = await extractPdfText(fileUrl);
        } catch (e) {
            console.warn('PDF extraction failed:', e);
        }
        URL.revokeObjectURL(fileUrl);
    } else if (['docx', 'doc', 'xlsx', 'xls', 'pptx', 'ppt'].includes(ext)) {
        text = await extractOfficeText(docPath);
    }

    docTextCache.set(docIndex, { text, docName });
    return text;
  }

  async function performSearch() {
    if (!searchQuery.trim()) {
        searchResults = null;
        isSearching = false;
        return;
    }

    isSearching = true;

    let totalMatches = 0;
    const matchesByDoc = [];

    const openIndices = appState.openDocIndices;

    for (const docIndex of openIndices) {
        const docName = appState.documents[docIndex]?.name;
        const docPath = appState.documentPaths[docIndex];
        const docFile = appState.documents[docIndex];

        if (!docName || !docPath || !docFile) continue;

        const ext = docName.split('.').pop()?.toLowerCase() || '';
        if (activeFilter === 'pdf' && ext !== 'pdf') continue;
        if (activeFilter === 'docx' && !['docx', 'doc'].includes(ext)) continue;
        if (activeFilter === 'xlsx' && !['xlsx', 'xls'].includes(ext)) continue;

        const text = await ensureDocTextCached(docIndex, docPath, docName, docFile);

        const docMatches = searchDocumentText(docIndex, docName, text, searchQuery, {
            caseSensitive: matchCase,
            wholeWord: wholeWord
        });

        if (docMatches.length > 0) {
            matchesByDoc.push({
                docIndex,
                docName,
                matches: docMatches
            });
            totalMatches += docMatches.length;
        }
    }

    searchResults = {
        query: searchQuery,
        totalMatches,
        matchesByDoc
    };

    isSearching = false;
  }

  function handleInput() {
    clearTimeout(searchTimeout);
    searchTimeout = setTimeout(() => {
        performSearch();
    }, 150);
  }

  function handleKeydown(e: KeyboardEvent) {
    if ((e.ctrlKey || e.metaKey) && e.shiftKey && e.key.toLowerCase() === 'f') {
      e.preventDefault();
      isOpen = !isOpen;
      if (isOpen) {
        setTimeout(() => inputEl?.focus(), 50);
      }
      return;
    }

    if (!isOpen) return;

    if (e.key === 'Escape') {
      e.preventDefault();
      isOpen = false;
    }
  }

  function handleFilterClick(filter: typeof activeFilter) {
    activeFilter = filter;
    performSearch();
  }

  function toggleOption(opt: 'case' | 'word') {
    if (opt === 'case') matchCase = !matchCase;
    if (opt === 'word') wholeWord = !wholeWord;
    performSearch();
  }

  function jumpToSnippet(docIndex: number, snippet: SearchSnippetMatch) {
    appState.selectDocument(docIndex);
    // Ideally we would send an event to jump to the specific page/scroll position,
    // but the spec just says "appState.selectDocument(docIndex) and navigates to target".
    isOpen = false;
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if isOpen}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
  <div class="workspace-search-backdrop" onclick={() => isOpen = false} role="presentation">
    <div
      class="workspace-search-modal"
      onclick={(e) => e.stopPropagation()}
      role="dialog"
      aria-label="Workspace Search"
      tabindex="-1"
    >
      <div class="search-header">
        <span class="search-icon">🔍</span>
        <input
          bind:this={inputEl}
          class="search-input"
          placeholder="Search all open documents (Ctrl+Shift+F)..."
          bind:value={searchQuery}
          oninput={handleInput}
        />
        <div class="search-options">
           <button class="option-btn" class:active={matchCase} onclick={() => toggleOption('case')} title="Match Case">Aa</button>
           <button class="option-btn" class:active={wholeWord} onclick={() => toggleOption('word')} title="Whole Word">W</button>
        </div>
        <button class="close-btn" onclick={() => isOpen = false} title="Close (Esc)">×</button>
      </div>

      <div class="filters-bar">
         <span class="match-count">
            {#if isSearching}
              Searching...
            {:else if searchResults}
              {searchResults.totalMatches} matches across {searchResults.matchesByDoc.length} files
            {:else}
              0 matches
            {/if}
         </span>

         <div class="filter-chips">
           <button class="chip" class:active={activeFilter === 'all'} onclick={() => handleFilterClick('all')}>All Files</button>
           <button class="chip" class:active={activeFilter === 'pdf'} onclick={() => handleFilterClick('pdf')}>PDFs</button>
           <button class="chip" class:active={activeFilter === 'docx'} onclick={() => handleFilterClick('docx')}>Word (.docx)</button>
           <button class="chip" class:active={activeFilter === 'xlsx'} onclick={() => handleFilterClick('xlsx')}>Excel (.xlsx)</button>
         </div>
      </div>

      <div class="results-container">
        {#if isSearching}
          <div class="empty-state">Searching...</div>
        {:else if searchResults && searchResults.matchesByDoc.length > 0}
          {#each searchResults.matchesByDoc as docGroup}
            <div class="doc-group">
              <div class="doc-header">
                 <span class="doc-icon">
                    {#if docGroup.docName.endsWith('.pdf')}📄
                    {:else if docGroup.docName.endsWith('.docx') || docGroup.docName.endsWith('.doc')}📝
                    {:else if docGroup.docName.endsWith('.xlsx') || docGroup.docName.endsWith('.xls')}📊
                    {:else}📁{/if}
                 </span>
                 <span class="doc-name">{docGroup.docName}</span>
                 <span class="doc-match-count">{docGroup.matches.length} matches</span>
              </div>
              <div class="doc-matches">
                {#each docGroup.matches as match}
                  <button class="match-item" onclick={() => jumpToSnippet(docGroup.docIndex, match)}>
                     <span class="match-location">{match.location} {match.pageOrSheetNum}</span>
                     <!-- eslint-disable-next-line svelte/no-at-html-tags -->
                     <div class="match-snippet">{@html match.snippet}</div>
                  </button>
                {/each}
              </div>
            </div>
          {/each}
        {:else if searchQuery.trim()}
          <div class="empty-state">No matches found for "{searchQuery}"</div>
        {:else}
          <div class="empty-state">Enter a search query to scan all open tabs.</div>
        {/if}
      </div>
    </div>
  </div>
{/if}

<style>
  .workspace-search-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.55);
    backdrop-filter: blur(4px);
    z-index: 1000;
    display: flex;
    justify-content: center;
    align-items: flex-start;
    padding-top: 10vh;
  }

  .workspace-search-modal {
    width: 100%;
    max-width: 650px;
    background: var(--bg-surface, #1e1e24);
    border: 1px solid var(--border-color, #2a2a35);
    border-radius: var(--border-radius-md, 8px);
    box-shadow: 0 20px 40px rgba(0, 0, 0, 0.4);
    overflow: hidden;
    display: flex;
    flex-direction: column;
    max-height: 75vh;
  }

  .search-header {
    display: flex;
    align-items: center;
    padding: 12px 16px;
    border-bottom: 1px solid var(--border-color, #2a2a35);
    gap: 12px;
  }

  .search-icon {
    font-size: 1rem;
    opacity: 0.7;
  }

  .search-input {
    flex: 1;
    background: transparent;
    border: none;
    color: var(--text-primary, #ffffff);
    font-size: 1rem;
    outline: none;
  }

  .search-options {
    display: flex;
    gap: 4px;
  }

  .option-btn {
    background: transparent;
    border: 1px solid var(--border-color);
    color: var(--text-muted);
    border-radius: 4px;
    padding: 2px 6px;
    font-size: 0.8rem;
    cursor: pointer;
  }

  .option-btn.active {
    background: rgba(94, 106, 210, 0.2);
    border-color: var(--accent-primary, #5e6ad2);
    color: var(--accent-primary, #5e6ad2);
  }

  .close-btn {
    background: transparent;
    border: none;
    color: var(--text-muted);
    font-size: 1.2rem;
    cursor: pointer;
    line-height: 1;
  }
  .close-btn:hover {
    color: var(--text-primary);
  }

  .filters-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 8px 16px;
    background: var(--bg-secondary, #13131a);
    border-bottom: 1px solid var(--border-color, #2a2a35);
    font-size: 0.8rem;
  }

  .match-count {
    color: var(--text-muted);
  }

  .filter-chips {
    display: flex;
    gap: 6px;
  }

  .chip {
    background: var(--bg-surface, #1e1e24);
    border: 1px solid var(--border-color, #2a2a35);
    color: var(--text-secondary);
    border-radius: 12px;
    padding: 2px 10px;
    font-size: 0.75rem;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .chip.active {
    background: var(--accent-primary, #5e6ad2);
    border-color: var(--accent-primary, #5e6ad2);
    color: white;
  }

  .results-container {
    overflow-y: auto;
    padding: 0;
    display: flex;
    flex-direction: column;
  }

  .empty-state {
    padding: 32px;
    text-align: center;
    color: var(--text-muted, #9ca3af);
    font-size: 0.9rem;
  }

  .doc-group {
    border-bottom: 1px solid var(--border-color, #2a2a35);
  }

  .doc-group:last-child {
    border-bottom: none;
  }

  .doc-header {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 16px;
    background: rgba(255, 255, 255, 0.02);
    font-size: 0.85rem;
  }

  .doc-name {
    font-weight: 500;
    color: var(--text-primary);
    flex: 1;
  }

  .doc-match-count {
    color: var(--text-muted);
    font-size: 0.75rem;
  }

  .doc-matches {
    display: flex;
    flex-direction: column;
  }

  .match-item {
    display: flex;
    flex-direction: column;
    padding: 8px 16px 8px 40px;
    background: transparent;
    border: none;
    text-align: left;
    cursor: pointer;
    border-left: 2px solid transparent;
    transition: all 0.1s ease;
  }

  .match-item:hover {
    background: var(--bg-surface-hover, rgba(255,255,255,0.05));
    border-left-color: var(--accent-primary, #5e6ad2);
  }

  .match-location {
    font-size: 0.7rem;
    color: var(--text-muted);
    margin-bottom: 2px;
  }

  .match-snippet {
    font-size: 0.85rem;
    color: var(--text-secondary);
    line-height: 1.4;
  }

  :global(.match-snippet mark) {
    background-color: rgba(255, 215, 0, 0.3);
    color: #fff;
    padding: 0 2px;
    border-radius: 2px;
  }
</style>