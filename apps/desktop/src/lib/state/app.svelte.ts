export class AppState {
  activeTab = $state('home');
  isLoading = $state(false);
  theme = $state('dark');

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
}

export const appState = new AppState();
