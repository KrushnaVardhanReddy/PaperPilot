import { beforeEach, describe, expect, it } from 'vitest';
import { WatchersState, mergeWatchSnapshot, parseFolderPreferences, serializeFolderPreferences } from './watchers.svelte';
import type { WatchEvent } from './watchers.svelte';
import recorded from '../../../tests/fixtures/automation/watch-events.json';
const events = recorded as WatchEvent[];
const preference = { id: events[0].watcher.id, path: events[0].watcher.directory, recipeRef: 'preset-legal-discovery' };
const KEY = 'paperpilot:watched-folders:v1';

describe('Watcher preferences and actual native event lifecycle', () => {
  beforeEach(() => localStorage.clear());
  it('persists only configuration, excluding runtime, counters and inline recipe contents', () => {
    const raw = serializeFolderPreferences([{ ...preference, runtime: events[0].watcher } as typeof preference]);
    expect(parseFolderPreferences(raw)).toEqual([preference]);
    expect(raw).not.toContain('succeeded'); expect(raw).not.toContain('watching');
    expect(() => parseFolderPreferences(JSON.stringify({ version: 1, folders: [preference, preference] }))).toThrow();
    expect(() => parseFolderPreferences('{bad JSON')).toThrow();
    expect(() => parseFolderPreferences(JSON.stringify({ version: 1, folders: [{ ...preference, recipeRef: '{"api_key":"secret"}' }] }))).toThrow();
  });
  it('merges recorded progress and stop events without regressing counters or status', () => {
    const state = new WatchersState(); state.folders = [preference];
    for (const event of events) state.applyEvent(event);
    expect(state.folders[0].runtime?.status).toBe('stopped'); expect(state.folders[0].runtime?.succeeded).toBe(2);
    state.applyEvent(events[0]); state.applyEvent(events[1]);
    expect(state.folders[0].runtime?.status).toBe('stopped'); expect(state.folders[0].runtime?.succeeded).toBe(2);
    expect(state.folders[0].lastOutput).toBe(events[1].output);
    const restarted = { ...events[0].watcher, run_id: events[0].watcher.run_id + 1, succeeded: 0 };
    expect(mergeWatchSnapshot(state.folders[0].runtime, restarted)).toEqual(restarted);
    expect(mergeWatchSnapshot(restarted, events[2].watcher)).toEqual(restarted);
  });
  it('restores last output and receipt from the native list after reopening the panel', () => {
    const state = new WatchersState(); state.folders = [preference];
    state.reconcileSnapshots([events[1].watcher]);
    expect(state.folders[0].lastOutput).toBe(events[1].output);
    expect(state.folders[0].lastReceipt).toBe(events[1].receipt);
  });
  it('reconciles removals from other local entry points without phantom running state', () => {
    const state = new WatchersState(); state.folders = [preference]; state.applyEvent(events[0]);
    state.reconcileSnapshots([]); expect(state.folders[0].runtime).toBeUndefined();
    const external = new WatchersState(); external.applyEvent(events[0]); external.reconcileSnapshots([]); expect(external.folders).toEqual([]);
    state.applyEvent(events[0]); state.reconcileSnapshots([], -1);
    expect(state.folders[0].runtime?.status).toBe('watching');
  });
  it('restores saved folders stopped and reports missing desktop IPC as an error', async () => {
    localStorage.setItem(KEY, serializeFolderPreferences([preference]));
    const state = new WatchersState(); const disconnect = await state.connect();
    expect(state.folders[0].runtime).toBeUndefined(); await state.start(preference.id);
    expect(state.folders[0].error).toContain('desktop app'); expect(state.folders[0].runtime).toBeUndefined();
    expect(state.folders[0].busy).toBe(false); disconnect();
    await state.remove(preference.id); expect(state.folders).toEqual([]);
    expect(parseFolderPreferences(localStorage.getItem(KEY))).toEqual([]);
  });
  it('preserves a corrupted preference store instead of silently overwriting it', () => {
    localStorage.setItem(KEY, '{bad JSON'); const state = new WatchersState();
    expect(() => state.add(preference.path, preference.recipeRef)).toThrow();
    expect(localStorage.getItem(KEY)).toBe('{bad JSON'); expect(state.folders).toEqual([]);
  });
});
