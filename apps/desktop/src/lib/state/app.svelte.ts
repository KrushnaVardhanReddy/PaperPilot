export class AppState {
  activeTab = $state('home');
  isLoading = $state(false);
  theme = $state('dark');
  documents = $state<File[]>([]);
  selectedDocumentIndex = $state<number | null>(null);

  constructor() {}

  setActiveTab(tab: string) {
    this.activeTab = tab;
  }

  setLoading(loading: boolean) {
    this.isLoading = loading;
  }

  setTheme(theme: string) {
    this.theme = theme;
  }

  addDocuments(files: File[]) {
    this.documents = [...this.documents, ...files];
  }

  removeDocument(index: number) {
    this.documents = this.documents.filter((_, i) => i !== index);
    if (this.selectedDocumentIndex === index) {
      this.selectedDocumentIndex = null;
    } else if (this.selectedDocumentIndex !== null && this.selectedDocumentIndex > index) {
      this.selectedDocumentIndex -= 1;
    }
  }

  selectDocument(index: number | null) {
    if (index === null || (index >= 0 && index < this.documents.length)) {
      this.selectedDocumentIndex = index;
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
