import { listen } from '@tauri-apps/api/event';
import { isNativeDesktop, requireInvoke } from '$lib/utils/tauri';
import { pipelineState } from './pipeline.svelte';

export interface FolderPreference { id: string; path: string; recipeRef: string; outputDir?: string }
export interface WatchSnapshot {
  id: string; run_id: number; directory: string; output_dir: string; recipe_name: string;
  status: 'watching' | 'stopping' | 'stopped' | 'error'; succeeded: number; failed: number;
  last_processed_file?: string | null; last_processed_at?: string | null; last_error?: string | null;
  last_output?: string | null; last_receipt?: string | null;
}
export interface FolderView extends FolderPreference { runtime?: WatchSnapshot; busy?: boolean; error?: string; lastOutput?: string; lastReceipt?: string }
export interface WatchEvent { kind: string; watcher: WatchSnapshot; file?: string | null; output?: string | null; receipt?: string | null; error?: string | null }
const KEY = 'paperpilot:watched-folders:v1';

export function parseFolderPreferences(raw: string | null): FolderPreference[] {
  if (!raw) return [];
  const value = JSON.parse(raw);
  if (value.version !== 1 || !Array.isArray(value.folders)) throw new Error('Saved folder settings have an unsupported format.');
  const ids = new Set<string>();
  return value.folders.map((f: unknown) => {
    const item = f as FolderPreference;
    if (!item || typeof item.id !== 'string' || !item.id || ids.has(item.id) || typeof item.path !== 'string' || !item.path || typeof item.recipeRef !== 'string' || item.recipeRef.trimStart().startsWith('{') || item.recipeRef.includes('\n') || (item.outputDir !== undefined && typeof item.outputDir !== 'string')) throw new Error('Saved folder settings contain an invalid or duplicate folder.');
    ids.add(item.id);
    return { id: item.id, path: item.path, recipeRef: item.recipeRef, ...(item.outputDir ? { outputDir: item.outputDir } : {}) };
  });
}
export function serializeFolderPreferences(folders: FolderPreference[]): string {
  return JSON.stringify({ version: 1, folders: folders.map(f => ({ id: f.id, path: f.path, recipeRef: f.recipeRef, ...(f.outputDir ? { outputDir: f.outputDir } : {}) })) });
}
export function mergeWatchSnapshot(current: WatchSnapshot | undefined, next: WatchSnapshot): WatchSnapshot {
  if (!current || current.run_id < next.run_id) return next;
  if (current.run_id > next.run_id) return current;
  if (next.succeeded + next.failed < current.succeeded + current.failed) return current;
  const rank = { watching: 1, stopping: 2, stopped: 3, error: 3 };
  if (next.succeeded + next.failed === current.succeeded + current.failed && rank[next.status] < rank[current.status]) return current;
  return next;
}

export class WatchersState {
  folders = $state<FolderView[]>([]);
  error = $state('');
  presets = $state<{ id: string; name: string }[]>([]);
  private revision = 0;
  private persist(folders = this.folders) {
    // Validate existing storage before replacing it; do not silently erase a corrupt list.
    parseFolderPreferences(localStorage.getItem(KEY));
    localStorage.setItem(KEY, serializeFolderPreferences(folders));
  }
  applyEvent(event: WatchEvent) {
    this.revision++;
    const previous = this.folders.find(f => f.id === event.watcher.id);
    if (!previous) this.folders = [...this.folders, { id: event.watcher.id, path: event.watcher.directory, recipeRef: '' }];
    this.folders = this.folders.map(f => {
      if (f.id !== event.watcher.id) return f;
      const runtime = mergeWatchSnapshot(f.runtime, event.watcher);
      if (runtime !== event.watcher) return f;
      return { ...f, runtime, lastOutput: event.output ?? runtime.last_output ?? f.lastOutput, lastReceipt: event.receipt ?? runtime.last_receipt ?? f.lastReceipt, error: event.error ?? '' };
    });
  }
  reconcileSnapshots(snapshots: WatchSnapshot[], revision = this.revision) {
    // A response started before a user action/event cannot erase its new runtime.
    const authoritative = revision === this.revision;
    const ids = new Set(snapshots.map(s => s.id));
    for (const watcher of snapshots) this.applyEvent({ kind: 'status', watcher });
    if (authoritative) {
      this.folders = this.folders.filter(f => ids.has(f.id) || f.recipeRef || f.busy).map(f =>
        !ids.has(f.id) && !f.busy ? { ...f, runtime: undefined } : f
      );
    }
  }
  async connect(): Promise<() => void> {
    this.error = '';
    try { this.folders = parseFolderPreferences(localStorage.getItem(KEY)).map(f => ({ ...f })); }
    catch (e) { this.error = String(e); }
    if (!isNativeDesktop()) return () => {};
    const unsubscribers: (() => void)[] = [];
    let polling = false; let disconnected = false; let timer: ReturnType<typeof setInterval> | undefined;
    try {
      for (const channel of ['watch:file_processed', 'watch:error', 'watch:status']) unsubscribers.push(await listen<WatchEvent>(channel, e => this.applyEvent(e.payload)));
      const revision = this.revision;
      const [snapshots, presets] = await Promise.all([
        requireInvoke<WatchSnapshot[]>('list_watched_folders'), requireInvoke<{ id: string; name: string }[]>('list_watch_recipes')
      ]);
      this.presets = presets;
      // Runtime state is authoritative. Persisted preferences never imply that a daemon is active.
      this.reconcileSnapshots(snapshots, revision);
      timer = setInterval(() => {
        if (polling || disconnected) return;
        polling = true;
        const revision = this.revision;
        void requireInvoke<WatchSnapshot[]>('list_watched_folders').then(items => {
          if (!disconnected) this.reconcileSnapshots(items, revision);
        }).catch(e => { if (!disconnected) this.error = String(e); }).finally(() => polling = false);
      }, 1500);
    } catch (e) { for (const unsubscribe of unsubscribers) unsubscribe(); this.error = String(e); }
    return () => { disconnected = true; if (timer) clearInterval(timer); for (const unsubscribe of unsubscribers) unsubscribe(); };
  }
  add(path: string, recipeRef: string, outputDir?: string) {
    if (recipeRef.trimStart().startsWith('{') || recipeRef.includes('\n')) throw new Error('Choose a saved workflow, preset, or recipe file; inline recipe content is not stored.');
    if (!path.trim() || !recipeRef) throw new Error('Choose a folder and a workflow.');
    const folder: FolderView = { id: crypto.randomUUID(), path: path.trim(), recipeRef, outputDir: outputDir?.trim() || undefined };
    const next = [...this.folders, folder]; this.persist(next); this.folders = next;
  }
  async start(id: string) {
    const folder = this.folders.find(f => f.id === id); if (!folder || folder.busy) return;
    this.revision++;
    this.folders = this.folders.map(f => f.id === id ? { ...f, busy: true, error: '' } : f);
    try {
      let recipe = folder.recipeRef;
      if (recipe.startsWith('flow:')) {
        const flow = pipelineState.savedFlows.find(f => f.id === recipe.slice(5));
        if (!flow) throw new Error('The saved workflow no longer exists. Add this folder again with another workflow.');
        recipe = JSON.stringify({ ...flow, steps: flow.steps.map(({ status: _status, ...step }) => step) });
      }
      if (!recipe) throw new Error('Choose a workflow before starting this folder.');
      const watcher = await requireInvoke<WatchSnapshot>('start_watch_folder', { id, config: { directory: folder.path, recipe, output_dir: folder.outputDir ?? null, settle_delay_ms: 500 } });
      this.applyEvent({ kind: 'started', watcher });
    } catch (e) { this.folders = this.folders.map(f => f.id === id ? { ...f, error: String(e) } : f); }
    finally { this.folders = this.folders.map(f => f.id === id ? { ...f, busy: false } : f); }
  }
  async stop(id: string) {
    const folder = this.folders.find(f => f.id === id); if (!folder || folder.busy) return;
    this.revision++;
    this.folders = this.folders.map(f => f.id === id ? { ...f, busy: true } : f);
    try { this.applyEvent({ kind: 'stopped', watcher: await requireInvoke<WatchSnapshot>('stop_watch_folder', { folderId: id }) }); }
    catch (e) { this.folders = this.folders.map(f => f.id === id ? { ...f, error: String(e) } : f); }
    finally { this.folders = this.folders.map(f => f.id === id ? { ...f, busy: false } : f); }
  }
  async remove(id: string) {
    const folder = this.folders.find(f => f.id === id); if (!folder || folder.busy) return;
    this.revision++;
    this.folders = this.folders.map(f => f.id === id ? { ...f, busy: true } : f);
    try {
      if (folder.runtime) await requireInvoke('remove_watch_folder', { folderId: id });
      this.folders = this.folders.map(f => f.id === id ? { ...f, runtime: undefined } : f);
      const next = this.folders.filter(f => f.id !== id); this.persist(next); this.folders = next;
    } catch (e) { this.folders = this.folders.map(f => f.id === id ? { ...f, error: String(e) } : f); }
    finally { this.folders = this.folders.map(f => f.id === id ? { ...f, busy: false } : f); }
  }
}
export const watchersState = new WatchersState();
