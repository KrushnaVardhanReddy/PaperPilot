import { getCurrentWindow } from '@tauri-apps/api/window';

export class AppState {
  activeTab = $state('home');
  isLoading = $state(false);
  theme = $state('dark');
  documents = $state<File[]>([]);
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
    this.documents = [...this.documents, ...files];
  }

  removeDocument(index: number) {
    this.closeTab(index);
    this.openDocIndices = this.openDocIndices.map(i => i > index ? i - 1 : i);
    this.documents = this.documents.filter((_, i) => i !== index);
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
  }
}

export const appState = new AppState();
