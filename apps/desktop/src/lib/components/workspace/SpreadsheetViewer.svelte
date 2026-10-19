<script lang="ts">
  import { onMount } from 'svelte';
  import * as XLSX from 'xlsx';

  let { file }: { file: File } = $props();

  interface SheetData {
    name: string;
    headers: string[];
    rows: any[][];
  }

  let sheets = $state<SheetData[]>([]);
  let activeSheetIndex = $state(0);
  let isLoading = $state(true);
  let loadError = $state<string | null>(null);
  let searchQuery = $state('');

  let currentSheet = $derived(sheets[activeSheetIndex] ?? null);

  let filteredRows = $derived.by(() => {
    if (!currentSheet) return [];
    if (!searchQuery.trim()) return currentSheet.rows;
    const q = searchQuery.toLowerCase();
    return currentSheet.rows.filter(row =>
      row.some(cell => String(cell ?? '').toLowerCase().includes(q))
    );
  });

  async function parseWorkbook() {
    isLoading = true;
    loadError = null;
    try {
      const buffer = await file.arrayBuffer();
      const workbook = XLSX.read(buffer, { type: 'array' });
      
      const parsedSheets: SheetData[] = [];
      for (const name of workbook.SheetNames) {
        const sheet = workbook.Sheets[name];
        if (!sheet) continue;

        // Convert to array of arrays (AOA)
        const aoa: any[][] = XLSX.utils.sheet_to_json(sheet, { header: 1, defval: '' });
        
        let headers: string[] = [];
        let rows: any[][] = [];

        if (aoa.length > 0) {
          headers = aoa[0].map((h: any, i: number) => String(h || `Column ${i + 1}`));
          rows = aoa.slice(1);
        }

        parsedSheets.push({
          name,
          headers,
          rows
        });
      }

      sheets = parsedSheets;
      activeSheetIndex = 0;
    } catch (err: any) {
      console.error('Failed to parse Excel file:', err);
      loadError = err?.message || 'Failed to parse workbook contents.';
    } finally {
      isLoading = false;
    }
  }

  $effect(() => {
    if (file) {
      parseWorkbook();
    }
  });

  function exportCurrentSheetCsv() {
    if (!currentSheet) return;
    const csvContent = [
      currentSheet.headers.map(h => `"${String(h).replace(/"/g, '""')}"`).join(','),
      ...currentSheet.rows.map(r => r.map(c => `"${String(c).replace(/"/g, '""')}"`).join(','))
    ].join('\n');

    const blob = new Blob([csvContent], { type: 'text/csv;charset=utf-8;' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `${currentSheet.name || 'sheet'}.csv`;
    a.click();
    URL.revokeObjectURL(url);
  }
</script>

<div class="spreadsheet-viewer">
  <!-- Top Bar -->
  <div class="viewer-top-bar">
    <div class="file-info-badge">
      <span class="file-icon">📊</span>
      <div class="file-text">
        <span class="file-name" title={file.name}>{file.name}</span>
        <span class="file-meta">
          {sheets.length} Sheet{sheets.length === 1 ? '' : 's'} · {currentSheet ? `${currentSheet.rows.length} rows` : ''}
        </span>
      </div>
    </div>

    <!-- Search in Sheet -->
    <div class="sheet-search-box">
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <circle cx="11" cy="11" r="8"/>
        <line x1="21" y1="21" x2="16.65" y2="16.65"/>
      </svg>
      <input
        type="text"
        placeholder="Search sheet data..."
        bind:value={searchQuery}
      />
      {#if searchQuery}
        <button class="clear-search-btn" onclick={() => searchQuery = ''}>×</button>
      {/if}
    </div>

    <div class="viewer-actions">
      <button class="btn-action" onclick={exportCurrentSheetCsv} title="Export this sheet to CSV">
        <span>Export CSV</span>
      </button>
    </div>
  </div>

  <!-- Sheet Tabs (bottom or top) -->
  {#if sheets.length > 1}
    <div class="sheet-tabs-bar" role="tablist">
      {#each sheets as sheet, idx}
        <button
          class="sheet-tab-btn"
          class:active={activeSheetIndex === idx}
          onclick={() => { activeSheetIndex = idx; searchQuery = ''; }}
        >
          <span class="sheet-tab-icon">📋</span>
          <span class="sheet-tab-name">{sheet.name}</span>
          <span class="sheet-row-count">({sheet.rows.length})</span>
        </button>
      {/each}
    </div>
  {/if}

  <!-- Data Grid Viewport -->
  <div class="table-container">
    {#if isLoading}
      <div class="loading-state">
        <div class="spinner"></div>
        <p>Loading spreadsheet preview...</p>
      </div>
    {:else if loadError}
      <div class="error-state">
        <span class="error-icon">⚠️</span>
        <h3>Could not preview spreadsheet</h3>
        <p>{loadError}</p>
      </div>
    {:else if currentSheet}
      {#if currentSheet.headers.length === 0 && currentSheet.rows.length === 0}
        <div class="empty-sheet-state">
          <span>This sheet is empty.</span>
        </div>
      {:else}
        <table class="excel-grid">
          <thead>
            <tr>
              <th class="row-num-header">#</th>
              {#each currentSheet.headers as header, colIdx}
                <th>
                  <div class="th-content">
                    <span class="col-letter">{String.fromCharCode(65 + (colIdx % 26))}</span>
                    <span class="col-title" title={header}>{header}</span>
                  </div>
                </th>
              {/each}
            </tr>
          </thead>
          <tbody>
            {#each filteredRows as row, rowIdx}
              <tr>
                <td class="row-num-cell">{rowIdx + 1}</td>
                {#each currentSheet.headers as _, colIdx}
                  <td class="data-cell" title={String(row[colIdx] ?? '')}>
                    {row[colIdx] !== undefined && row[colIdx] !== null ? row[colIdx] : ''}
                  </td>
                {/each}
              </tr>
            {/each}
          </tbody>
        </table>
      {/if}
    {/if}
  </div>

  <!-- Status Bar Footer -->
  <div class="spreadsheet-footer">
    <span>Showing {filteredRows.length} of {currentSheet?.rows.length ?? 0} rows</span>
    {#if searchQuery}
      <span class="search-match-count">Filtered by "{searchQuery}"</span>
    {/if}
  </div>
</div>

<style>
  .spreadsheet-viewer {
    display: flex;
    flex-direction: column;
    flex: 1;
    height: 100%;
    overflow: hidden;
    background: var(--bg-canvas, #0f172a);
    color: var(--text-primary, #f8fafc);
  }

  .viewer-top-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
    padding: 0.6rem 1.25rem;
    background: var(--bg-surface, #1e293b);
    border-bottom: 1px solid var(--border-color, #334155);
  }

  .file-info-badge {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    min-width: 0;
  }

  .file-icon {
    font-size: 1.5rem;
    line-height: 1;
  }

  .file-text {
    display: flex;
    flex-direction: column;
    min-width: 0;
  }

  .file-name {
    font-size: 0.95rem;
    font-weight: 600;
    color: #f1f5f9;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 320px;
  }

  .file-meta {
    font-size: 0.75rem;
    color: #94a3b8;
  }

  .sheet-search-box {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    background: #0f172a;
    border: 1px solid #334155;
    border-radius: 6px;
    padding: 0.35rem 0.65rem;
    width: 260px;
    color: #94a3b8;
  }

  .sheet-search-box input {
    background: transparent;
    border: none;
    outline: none;
    color: #f1f5f9;
    font-size: 0.82rem;
    width: 100%;
  }

  .clear-search-btn {
    background: transparent;
    border: none;
    color: #94a3b8;
    cursor: pointer;
    font-size: 1rem;
    padding: 0;
  }

  .viewer-actions {
    display: flex;
    gap: 0.5rem;
  }

  .btn-action {
    display: flex;
    align-items: center;
    gap: 0.4rem;
    background: #334155;
    border: 1px solid #475569;
    color: #f8fafc;
    padding: 0.4rem 0.85rem;
    border-radius: 6px;
    font-size: 0.82rem;
    font-weight: 500;
    cursor: pointer;
    transition: background 0.15s ease;
  }

  .btn-action:hover {
    background: #475569;
  }

  .sheet-tabs-bar {
    display: flex;
    gap: 0.25rem;
    padding: 0.35rem 1rem 0;
    background: #111827;
    border-bottom: 1px solid #334155;
    overflow-x: auto;
  }

  .sheet-tab-btn {
    display: flex;
    align-items: center;
    gap: 0.4rem;
    background: #1e293b;
    border: 1px solid #334155;
    border-bottom: none;
    border-top-left-radius: 6px;
    border-top-right-radius: 6px;
    color: #94a3b8;
    padding: 0.4rem 0.85rem;
    font-size: 0.8rem;
    cursor: pointer;
    white-space: nowrap;
    transition: all 0.15s ease;
  }

  .sheet-tab-btn:hover {
    color: #f1f5f9;
    background: #273549;
  }

  .sheet-tab-btn.active {
    background: #0f172a;
    color: #38bdf8;
    border-color: #38bdf8 #334155 #0f172a #334155;
    font-weight: 600;
  }

  .sheet-row-count {
    font-size: 0.72rem;
    opacity: 0.7;
  }

  .table-container {
    flex: 1;
    overflow: auto;
    position: relative;
    background: #0b1120;
  }

  .excel-grid {
    width: 100%;
    border-collapse: collapse;
    font-size: 0.82rem;
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
    table-layout: auto;
  }

  .excel-grid thead th {
    position: sticky;
    top: 0;
    z-index: 10;
    background: #1e293b;
    border: 1px solid #334155;
    padding: 0.45rem 0.75rem;
    text-align: left;
    font-weight: 600;
    color: #cbd5e1;
    box-shadow: 0 1px 2px rgba(0, 0, 0, 0.2);
  }

  .th-content {
    display: flex;
    flex-direction: column;
    gap: 0.1rem;
  }

  .col-letter {
    font-size: 0.68rem;
    color: #64748b;
    text-transform: uppercase;
  }

  .col-title {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 200px;
  }

  .row-num-header, .row-num-cell {
    position: sticky;
    left: 0;
    z-index: 5;
    background: #1e293b;
    border: 1px solid #334155;
    width: 45px;
    min-width: 45px;
    text-align: center;
    color: #64748b;
    font-size: 0.75rem;
    user-select: none;
  }

  .excel-grid thead th.row-num-header {
    z-index: 20;
  }

  .excel-grid tbody tr:hover td {
    background: rgba(59, 130, 246, 0.08);
  }

  .data-cell {
    border: 1px solid #1e293b;
    padding: 0.4rem 0.75rem;
    white-space: nowrap;
    max-width: 320px;
    overflow: hidden;
    text-overflow: ellipsis;
    color: #e2e8f0;
  }

  .loading-state, .error-state, .empty-sheet-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    min-height: 250px;
    color: #94a3b8;
    gap: 0.75rem;
  }

  .spinner {
    width: 28px;
    height: 28px;
    border: 3px solid rgba(255, 255, 255, 0.1);
    border-top-color: #38bdf8;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  .spreadsheet-footer {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.4rem 1rem;
    background: #1e293b;
    border-top: 1px solid #334155;
    font-size: 0.75rem;
    color: #94a3b8;
  }

  .search-match-count {
    color: #38bdf8;
  }
</style>
