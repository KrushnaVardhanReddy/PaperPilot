<script lang="ts">
  import { appState } from '$lib/state/app.svelte';

  let {
    activeTool = null,
    onTabChange = () => {},
    layoutMode = 'studio',
    onLayoutChange = () => {}
  } = $props();

  let openDocsCount = $derived(appState.openDocIndices.length);
  let activeDoc = $derived(appState.selectedDocumentIndex !== null ? appState.documents[appState.selectedDocumentIndex] : null);

  let pdfType = 'PDF/A-2b';
  let sizeStr = $derived(activeDoc ? (activeDoc.size / 1024 / 1024).toFixed(2) + ' MB' : '0 MB');
  let isEncrypted = false;
  let modDate = $derived(activeDoc && activeDoc.lastModified ? new Date(activeDoc.lastModified).toLocaleDateString() : 'N/A');
</script>

<div class="statusbar" aria-label="Status Bar">
  <div class="left">
    <span>Ready</span>
    {#if openDocsCount > 0}
      <span class="divider">|</span>
      <span>{openDocsCount} document{openDocsCount !== 1 ? 's' : ''} open</span>
    {/if}
  </div>

  <div class="right">
    {#if activeDoc}
      <span class="metadata-item" title="Standard">{pdfType}</span>
      <span class="divider">|</span>
      <span class="metadata-item" title="Size">{sizeStr}</span>
      <span class="divider">|</span>
      <span class="metadata-item" title="Security">{isEncrypted ? '🔐 Encrypted' : '🔓 Unsecured'}</span>
      <span class="divider">|</span>
      <span class="metadata-item" title="Modified">{modDate}</span>
    {:else}
      <span>{appState.workspaceLayout} layout</span>
      <span class="divider">|</span>
      <span>v1.0.0</span>
    {/if}
  </div>
</div>

<style>
  .statusbar { height: 24px; background-color: var(--accent-primary, #5E6AD2); color: white; display: flex; align-items: center; justify-content: space-between; padding: 0 16px; font-size: 0.75rem; z-index: 100; }
  .left, .right { display: flex; align-items: center; gap: 8px; }
  .divider { opacity: 0.5; }
  .metadata-item { font-variant-numeric: tabular-nums; }
</style>
