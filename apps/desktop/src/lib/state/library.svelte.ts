export interface RecentFile {
  name: string;
  path: string;
  accessedAt: string;
}

export interface LibraryState {
  recentFiles: RecentFile[];
  smartCategories: { label: string; count: number; toReview: boolean }[];
  watchedFolders: { path: string; recipe: string; status: 'watching' | 'paused' | 'error'; lastFile?: string }[];
  sharedFiles: { name: string; sender: string; avatarUrl?: string }[];
}

export class LibraryStore {
  recentFiles = $state<RecentFile[]>([]);
  smartCategories = $state<{ label: string; count: number; toReview: boolean }[]>([]);
  watchedFolders = $state<{ path: string; recipe: string; status: 'watching' | 'paused' | 'error'; lastFile?: string }[]>([]);
  sharedFiles = $state<{ name: string; sender: string; avatarUrl?: string }[]>([]);

  constructor() {
    // Initial mock data if needed, or leave empty
  }
}

export const libraryState = new LibraryStore();
