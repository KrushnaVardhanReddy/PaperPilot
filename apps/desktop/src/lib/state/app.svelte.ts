import { getCurrentWindow } from '@tauri-apps/api/window';
import { safeInvoke, isNativeDesktop, requireInvoke } from '$lib/utils/tauri';
import { SvelteMap } from 'svelte/reactivity';
import type { DocumentFacts, InspectionState } from '$lib/utils/documentFilters';

const MAX_OPEN_TABS = 8;

export type WorkspaceLayoutMode = 'studio' | 'split' | 'hero';

export class AppState {
  activeTab = $state('documents');
  isLoading = $state(false);
  theme = $state('dark');
  documents = $state<File[]>([]);
  inspections = new SvelteMap<File, InspectionState>();
  private inspectionQueue: File[] = [];
  private inspectionsRunning = 0;
  documentPaths = $state<string[]>([]);  // parallel array: absolute FS path for documents[i]
  selectedDocumentIndex = $state<number | null>(null);
  workspaceLayout = $state<WorkspaceLayoutMode>('studio');

  isMobileViewport = $state(false);
  isTauriMobile = $state(false);
  isMobile = $derived(this.isMobileViewport || this.isTauriMobile);

  // Active Tool selection (used across middle hub, operations panel, toolbar)
  activeTool = $state<any | null>(null);

  // Tab management for multi-document viewer
  openDocIndices = $state<number[]>([]);
  showDiffView = $state(false);

  // Developer Mode
  developerMode = $state(false);
  gatewayRunning = $state(false);
  gatewayPort = $state(7823);

  // Onboarding & Portable Mode
  onboardingCompleted = $state(false);
  isPortableMode = $state(false);
  featurePacks = $state<any[]>([]);

  constructor() {
    if (typeof window !== 'undefined') {
      this.isMobileViewport = window.innerWidth < 768;
      window.addEventListener('resize', () => {
        this.isMobileViewport = window.innerWidth < 768;
      });
      // Try to detect Tauri mobile via userAgent as a fallback if the API isn't ready
      const ua = navigator.userAgent.toLowerCase();
      if (ua.includes('android') || ua.includes('iphone') || ua.includes('ipad')) {
          this.isTauriMobile = true;
      }

      const storedLayout = localStorage.getItem('paperpilot_workspace_layout') as WorkspaceLayoutMode;
      if (storedLayout === 'studio' || storedLayout === 'split' || storedLayout === 'hero') {
        this.workspaceLayout = storedLayout;
      }

      const storedDev = localStorage.getItem('paperpilot:developer-mode');
      if (storedDev === 'true') {
        this.developerMode = true;
        // Don't auto-start here, let the UI or a separate init flow handle it,
        // or actually, we can start it if we are in Tauri
        this.toggleDeveloperMode(true);
      }

      const storedOnboarding = localStorage.getItem('paperpilot_onboarding_completed');
      if (storedOnboarding === 'true') {
        this.onboardingCompleted = true;
      } else {
        this.onboardingCompleted = false;
      }
    }

    this.initPortableState();
  }

  setWorkspaceLayout(mode: WorkspaceLayoutMode) {
    this.workspaceLayout = mode;
    if (typeof window !== 'undefined') {
      localStorage.setItem('paperpilot_workspace_layout', mode);
    }
  }

  async initPortableState() {
    try {
      const portable = await safeInvoke('is_portable_mode_cmd');
      this.isPortableMode = !!portable;
      await this.fetchFeaturePacks();
    } catch (err) {
      console.warn('Failed to fetch portable mode state:', err);
    }
  }

  async fetchFeaturePacks() {
    try {
      const packs: any[] = await safeInvoke('get_feature_packs') || [];
      this.featurePacks = packs;
    } catch (err) {
      console.warn('Failed to fetch feature packs:', err);
    }
  }

  async toggleFeaturePack(packId: string, enabled: boolean) {
    try {
      await safeInvoke('set_feature_pack_enabled', { packId, enabled });
      await this.fetchFeaturePacks();
    } catch (err) {
      console.error('Failed to toggle feature pack:', err);
    }
  }

  completeOnboarding() {
    this.onboardingCompleted = true;
    if (typeof window !== 'undefined') {
      localStorage.setItem('paperpilot_onboarding_completed', 'true');
    }
  }

  async toggleDeveloperMode(enabled?: boolean) {
    if (typeof enabled === 'boolean') {
      this.developerMode = enabled;
    } else {
      this.developerMode = !this.developerMode;
    }

    if (typeof window !== 'undefined') {
      localStorage.setItem('paperpilot:developer-mode', String(this.developerMode));
    }

    if (this.developerMode) {
      await safeInvoke('start_gateway');
      this.gatewayRunning = true;
    } else {
      await safeInvoke('stop_gateway');
      this.gatewayRunning = false;
    }
  }

  setActiveTab(tab: string) {
    this.activeTab = tab;
  }

  openTool(tool: any) {
    this.activeTool = tool;
  }

  closeTool() {
    this.activeTool = null;
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
    for (const file of files) {
      if (file.name.toLowerCase().endsWith('.pdf')) {
        this.inspections.set(file, { status: 'pending' }); this.inspectionQueue.push(file);
      }
    }
    this.pumpInspections();
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
    this.inspections.delete(this.documents[index]);
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

  private pumpInspections() {
    while (this.inspectionsRunning < 2 && this.inspectionQueue.length) {
      const file = this.inspectionQueue.shift()!;
      if (!this.documents.includes(file)) continue;
      if (!isNativeDesktop()) {
        this.inspections.set(file, { status: 'unavailable', error: 'Document details require the desktop app.' }); continue;
      }
      if (file.size > 128 * 1024 * 1024) {
        this.inspections.set(file, { status: 'unavailable', error: 'Document details support PDFs up to 128 MiB.' }); continue;
      }
      this.inspectionsRunning++;
      const request = this.inspections.get(file);
      void (async () => {
        try {
          const path = (file as File & { _localPath?: string })._localPath;
          const args = path ? { path } : { bytes: Array.from(new Uint8Array(await file.arrayBuffer())) };
          const facts = await requireInvoke<DocumentFacts>('inspect_document', args);
          if (this.inspections.get(file) === request) this.inspections.set(file, { status: 'ready', facts });
        } catch (error) {
          if (this.inspections.get(file) === request) this.inspections.set(file, { status: 'error', error: String(error) });
        } finally { this.inspectionsRunning--; this.pumpInspections(); }
      })();
    }
  }

  reorderDocuments(fromIndex: number, toIndex: number) {
    if (fromIndex === toIndex || fromIndex < 0 || toIndex < 0 || fromIndex >= this.documents.length || toIndex >= this.documents.length) return;
    const remap = (index: number) => index === fromIndex ? toIndex :
      fromIndex < toIndex && index > fromIndex && index <= toIndex ? index - 1 :
      fromIndex > toIndex && index >= toIndex && index < fromIndex ? index + 1 : index;
    const docs = [...this.documents];
    const [movedItem] = docs.splice(fromIndex, 1);
    docs.splice(toIndex, 0, movedItem);
    this.documents = docs;

    const paths = [...this.documentPaths];
    const [movedPath] = paths.splice(fromIndex, 1);
    paths.splice(toIndex, 0, movedPath);
    this.documentPaths = paths;
    this.openDocIndices = this.openDocIndices.map(remap);
    if (this.selectedDocumentIndex !== null) this.selectedDocumentIndex = remap(this.selectedDocumentIndex);
  }
}

export const appState = new AppState();
