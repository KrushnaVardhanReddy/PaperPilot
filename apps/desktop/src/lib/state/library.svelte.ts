import { safeInvoke } from '$lib/utils/tauri';

export interface RecentFile {
  name: string;
  path: string;
  lastOpened: number;
}

export interface SmartCategory {
  label: string;
  count: number;
  toReview: boolean;
}

export interface WatchedFolder {
  path: string;
  recipe: string;
  status: 'watching' | 'paused' | 'error';
  lastFile?: string;
}

export interface SharedFile {
  name: string;
  sender: string;
  avatarUrl?: string;
}

export class LibraryState {
  recentFiles = $state<RecentFile[]>([]);
  smartCategories = $state<SmartCategory[]>([]);
  watchedFolders = $state<WatchedFolder[]>([]);
  sharedFiles = $state<SharedFile[]>([]);

  constructor() {
    this.startWatchPolling();
  }

  startWatchPolling() {
    if (typeof window !== 'undefined') {
      setInterval(async () => {
        try {
          const statusList = await safeInvoke('get_watch_status');
          if (statusList && Array.isArray(statusList)) {
            this.watchedFolders = statusList;
          }
        } catch (err) {}
      }, 5000);
    }
  }
}

export const libraryState = new LibraryState();
