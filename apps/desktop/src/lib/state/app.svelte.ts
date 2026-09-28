export class AppState {
  activeTab = $state('home');
  isLoading = $state(false);
  theme = $state('dark');
  documents = $state<File[]>([]);

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
  }

  reorderDocuments(fromIndex: number, toIndex: number) {
    const docs = [...this.documents];
    const [movedItem] = docs.splice(fromIndex, 1);
    docs.splice(toIndex, 0, movedItem);
    this.documents = docs;
  }
}

export const appState = new AppState();
