import { invoke } from '@tauri-apps/api/core';
import { libraryState } from '../state/library.svelte';

let pollInterval: ReturnType<typeof setInterval> | null = null;

export async function getWatchStatus() {
  try {
    const status = await invoke('get_watch_status');
    // Assuming status is an array of watched folder objects
    if (Array.isArray(status)) {
        libraryState.watchedFolders = status as any;
    }
  } catch (error) {
    console.error('Failed to get watch status:', error);
  }
}

export function startWatching() {
  if (pollInterval === null) {
    pollInterval = setInterval(getWatchStatus, 5000);
    // Initial fetch
    getWatchStatus();
  }
}

export function stopWatching() {
  if (pollInterval !== null) {
    clearInterval(pollInterval);
    pollInterval = null;
  }
}
