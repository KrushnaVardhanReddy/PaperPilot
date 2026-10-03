import { getCurrentWindow } from '@tauri-apps/api/window';

const MAX_OPEN_TABS = 8;

export class AppState {
  activeTab = $state('home');
  isLoading = $state(false);
  theme = $state('dark');
  documents = $state<File[]>([]);
  documentPaths = $state<string[]>([]);  // parallel array: absolute FS path for documents[i]
  selectedDocumentIndex = $state<number | null>(null);

  // Tab management for multi-document viewer
  openDocIndices = $state<number[]>([]);
  showDiffView = $state(false);

  constructor() {}

  setActiveTab(tab: string) {
    this.activeTab = tab;
  }

  toggleDiffView(show?: boolean) {
    this.showDiffView = typeof show === 'boolean' ? show : !this.showDiffView;
  }

  setLoading(loading: boolean) {
    this.isLoading = loading;
  }

  async setTheme(theme: string) {
    this.theme = theme;
    try {
      await getCurrentWindow().setTheme(theme === 'dark' ? 'dark' : 'light');
    } catch (err) {
      console.warn("Could not set native window theme", err);
    }
  }

  addDocuments(files: File[]) {
    const startIdx = this.documents.length;
    this.documents = [...this.documents, ...files];
    // Extract path from the file object (set before addDocuments is called)
    const newPaths = files.map(f => (f as any)._localPath || '');
    console.log('[AppState] addDocuments paths:', newPaths);
    this.documentPaths = [...this.documentPaths, ...newPaths];

    for (let i = 0; i < files.length; i++) {
      if (this.openDocIndices.length >= MAX_OPEN_TABS) {
        // remove the oldest unselected tab
        const oldestIndex = this.openDocIndices.findIndex(idx => idx !== this.selectedDocumentIndex);
        if (oldestIndex !== -1) {
            this.openDocIndices = this.openDocIndices.filter((_, idx) => idx !== oldestIndex);
        }
      }
      this.openDocIndices = [...this.openDocIndices, startIdx + i];
    }
  }

  removeDocument(index: number) {
    this.closeTab(index);
    this.openDocIndices = this.openDocIndices.map(i => i > index ? i - 1 : i);
    this.documents = this.documents.filter((_, i) => i !== index);
    this.documentPaths = this.documentPaths.filter((_, i) => i !== index);
    if (this.selectedDocumentIndex === index) {
      this.selectedDocumentIndex = this.openDocIndices.length > 0 ? this.openDocIndices[this.openDocIndices.length - 1] : null;
    } else if (this.selectedDocumentIndex !== null && this.selectedDocumentIndex > index) {
      this.selectedDocumentIndex -= 1;
    }
  }

  selectDocument(index: number | null) {
    if (index === null) {
      this.selectedDocumentIndex = null;
      return;
    }
    if (index >= 0 && index < this.documents.length) {
      this.selectedDocumentIndex = index;
      if (!this.openDocIndices.includes(index)) {
        if (this.openDocIndices.length >= MAX_OPEN_TABS) {
            // remove the oldest unselected tab
            const oldestIndex = this.openDocIndices.findIndex(idx => idx !== this.selectedDocumentIndex);
            if (oldestIndex !== -1) {
                this.openDocIndices = this.openDocIndices.filter((_, i) => i !== oldestIndex);
            }
        }
        this.openDocIndices = [...this.openDocIndices, index];
      }
    }
  }

  closeTab(index: number) {
    const tabIdx = this.openDocIndices.indexOf(index);
    if (tabIdx !== -1) {
      this.openDocIndices = this.openDocIndices.filter(i => i !== index);
      if (this.selectedDocumentIndex === index) {
        if (this.openDocIndices.length > 0) {
          const nextIdx = Math.min(tabIdx, this.openDocIndices.length - 1);
          this.selectedDocumentIndex = this.openDocIndices[nextIdx];
        } else {
          this.selectedDocumentIndex = null;
        }
      }
    }
  }

  reorderDocuments(fromIndex: number, toIndex: number) {
    const docs = [...this.documents];
    const [movedItem] = docs.splice(fromIndex, 1);
    docs.splice(toIndex, 0, movedItem);
    this.documents = docs;

    const paths = [...this.documentPaths];
    const [movedPath] = paths.splice(fromIndex, 1);
    paths.splice(toIndex, 0, movedPath);
    this.documentPaths = paths;
  }
}

export const appState = new AppState();
