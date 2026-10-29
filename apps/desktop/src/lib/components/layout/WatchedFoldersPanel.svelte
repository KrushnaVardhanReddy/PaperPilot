<script lang="ts">
  import { onMount } from 'svelte';
  import { open } from '@tauri-apps/plugin-dialog';
  import { watchersState } from '$lib/state/watchers.svelte';
  import { pipelineState } from '$lib/state/pipeline.svelte';
  import { appState } from '$lib/state/app.svelte';
  import { isNativeDesktop, requireInvoke } from '$lib/utils/tauri';
  let path = $state(''); let output = $state(''); let recipe = $state(''); let recipePath = $state(''); let error = $state('');
  let receipt = $state<{ path: string; content: string } | null>(null);
  let native = $derived(isNativeDesktop() && !appState.isTauriMobile);
  let running = $derived(watchersState.folders.filter(f => f.runtime?.status === 'watching').length);
  onMount(() => {
    pipelineState.initStorage(); let disposed = false; let disconnect = () => {};
    void watchersState.connect().then(fn => { if (disposed) fn(); else disconnect = fn; });
    return () => { disposed = true; disconnect(); };
  });
  async function browse(target: 'folder' | 'output' | 'recipe') {
    try {
      const chosen = await open(target === 'recipe' ? { multiple: false, filters: [{ name: 'Workflow', extensions: ['ppflow', 'json', 'toml'] }] } : { directory: true, multiple: false });
      if (typeof chosen === 'string') { if (target === 'folder') path = chosen; else if (target === 'output') output = chosen; else { recipePath = chosen; recipe = 'path'; } }
    } catch (e) { error = String(e); }
  }
  function add() {
    try { watchersState.add(path, recipe === 'path' ? recipePath : recipe, output); path = ''; output = ''; error = ''; }
    catch (e) { error = String(e); }
  }
  async function viewOutput(filePath: string) {
    try {
      const bytes = await requireInvoke<number[]>('read_file_bytes', { path: filePath });
      const file = new File([new Uint8Array(bytes)], filePath.split(/[\\/]/).pop() ?? 'processed.pdf', { type: 'application/pdf' });
      Object.assign(file, { _localPath: filePath }); appState.addDocuments([file]); appState.selectDocument(appState.documents.length - 1); appState.setActiveTab('documents');
    } catch (e) { error = String(e); }
  }
  async function viewReceipt(filePath: string) {
    try {
      const bytes = await requireInvoke<number[]>('read_file_bytes', { path: filePath });
      receipt = { path: filePath, content: JSON.stringify(JSON.parse(new TextDecoder().decode(new Uint8Array(bytes))), null, 2) };
    } catch (e) { error = String(e); }
  }
</script>

<section aria-label="Watched folders" class="watched-folders">
  <h3>Watched folders <span class="pill">{running} running</span></h3>
  <p>Process incoming PDFs with a local workflow. Originals stay in place; outputs default to a separate “processed” folder. Restarting a watcher also processes existing PDFs.</p>
  {#if !native}<p role="status">Folder watching requires the desktop app on a computer.</p>{/if}
  {#if error || watchersState.error}<p class="error" role="alert">{error || watchersState.error}</p>{/if}
  <fieldset disabled={!native}>
    <legend>Add a watched folder</legend>
    <label>Input folder <div class="path"><input aria-label="Input folder" bind:value={path} placeholder="Choose a local folder" /><button type="button" onclick={() => browse('folder')}>Browse</button></div></label>
    <label>Workflow <select aria-label="Watcher workflow" bind:value={recipe}>
      <option value="">Choose a workflow</option>
      {#each watchersState.presets as preset}<option value={preset.id}>{preset.name}</option>{/each}
      {#each pipelineState.savedFlows.filter(f => !f.isPreset) as flow}<option value={`flow:${flow.id}`}>{flow.name} (saved)</option>{/each}
      <option value="path">Local recipe file…</option>
    </select></label>
    {#if recipe === 'path'}<label>Recipe file <div class="path"><input aria-label="Recipe file" bind:value={recipePath} /><button type="button" onclick={() => browse('recipe')}>Browse</button></div></label>{/if}
    <label>Output folder (optional) <div class="path"><input aria-label="Output folder" bind:value={output} placeholder="Input folder / processed" /><button type="button" onclick={() => browse('output')}>Browse</button></div></label>
    <button type="button" disabled={!path.trim() || !recipe || (recipe === 'path' && !recipePath)} onclick={add}>Add folder</button>
  </fieldset>
  {#each watchersState.folders as folder (folder.id)}
    {@const active = folder.runtime?.status === 'watching' || folder.runtime?.status === 'stopping'}
    <article aria-label={`Watcher ${folder.path}`}>
      <strong class="folder-path">{folder.path}</strong>
      <span class="pill">{folder.busy ? 'Updating…' : folder.runtime?.status ?? 'stopped'}</span>
      <p>{folder.runtime?.recipe_name ?? folder.recipeRef} · {folder.runtime?.succeeded ?? 0} processed · {folder.runtime?.failed ?? 0} failed</p>
      {#if folder.runtime?.output_dir}<p>Output: {folder.runtime.output_dir}</p>{/if}
      {#if folder.runtime?.last_processed_file}<p>Last file: {folder.runtime.last_processed_file}<br />{folder.runtime.last_processed_at}</p>{/if}
      {#if folder.error || folder.runtime?.last_error}<p class="error" role="alert">{folder.error || folder.runtime?.last_error}</p>{/if}
      <div class="actions">
        <button type="button" disabled={!native || folder.busy} onclick={() => active ? watchersState.stop(folder.id) : watchersState.start(folder.id)}>{active ? 'Stop watching' : 'Start watching'}</button>
        <button type="button" disabled={!native || folder.busy} onclick={() => watchersState.remove(folder.id)}>Remove</button>
        {#if folder.lastOutput}<button type="button" onclick={() => viewOutput(folder.lastOutput!)}>Open output</button>{/if}
      </div>
      {#if folder.lastReceipt}<p class="receipt">Receipt: {folder.lastReceipt}</p><button type="button" disabled={!native} onclick={() => viewReceipt(folder.lastReceipt!)}>View receipt</button>{/if}
    </article>
  {/each}
  {#if receipt}<aside aria-label="Execution receipt"><h4>Execution receipt</h4><p>{receipt.path}</p><button type="button" onclick={() => receipt = null}>Close receipt</button><pre>{receipt.content}</pre></aside>{/if}
</section>
<style>
  .watched-folders { max-width: 900px; color: var(--text-primary); font-family: var(--font-sans, system-ui); } h3 { display: flex; gap: 12px; align-items: center; }
  p { color: var(--text-secondary); line-height: 1.6; overflow-wrap: anywhere; } fieldset, article { padding: 18px; border: 1px solid var(--border-color); border-radius: 10px; margin-block: 16px; min-width: 0; }
  label { display: flex; flex-direction: column; gap: 7px; margin-bottom: 14px; } .path, .actions { display: flex; flex-wrap: wrap; gap: 8px; } .path input { flex: 1; min-width: 180px; }
  input, select, button { padding: 9px 12px; border: 1px solid var(--border-color, #68718c); border-radius: 6px; color: var(--text-primary); background: var(--bg-surface); }
  button { cursor: pointer; } button:disabled, fieldset:disabled { opacity: .55; } .pill { font-size: .8rem; background: var(--bg-surface); padding: 4px 9px; border-radius: 20px; border: 1px solid var(--border-color); }
  .error { color: var(--error-color, #ffadad); } .folder-path { overflow-wrap: anywhere; } .receipt { font-size: .8rem; }
  button:focus-visible, input:focus-visible, select:focus-visible { outline: 2px solid var(--accent-primary, #9ca4ff); outline-offset: 2px; }
  pre { white-space: pre-wrap; overflow-wrap: anywhere; background: var(--bg-surface); padding: 16px; border: 1px solid var(--border-color); border-radius: 8px; }
</style>
