<script lang="ts">
  import '../app.css';
  import Sidebar from '$lib/components/layout/Sidebar.svelte';
  import BottomNav from '$lib/components/layout/BottomNav.svelte';
  import ToastContainer from '$lib/components/layout/ToastContainer.svelte';
  import Titlebar from '$lib/components/layout/Titlebar.svelte';
  import CommandPalette from '$lib/components/layout/CommandPalette.svelte';
  import ModularSetupModal from '$lib/components/onboarding/ModularSetupModal.svelte';
  import WorkspaceSearchModal from '$lib/components/workspace/WorkspaceSearchModal.svelte';
  import ExportCodeModal from '$lib/components/pipeline/ExportCodeModal.svelte';
  import { onMount } from 'svelte';
  import { listen } from '@tauri-apps/api/event';
  import { invoke } from '@tauri-apps/api/core';
  import { getCurrentWindow } from '@tauri-apps/api/window';

  import { appState } from '$lib/state/app.svelte';

  let { children } = $props();

  onMount(() => {
    if (typeof window !== 'undefined') {
      (window as any).appState = appState;
    }
    let unlisten: () => void;
    let unlistenSave: () => void;
    let unlistenSettings: () => void;
    let unlistenUndo: () => void;
    let unlistenRedo: () => void;
    let unlistenZoomIn: () => void;
    let unlistenZoomOut: () => void;
    let unlistenFitWidth: () => void;
    let unlistenDragDrop: (() => void) | undefined;

    // Listen to native Tauri OS drag-drop events
    (async () => {
      try {
        const currentWin = getCurrentWindow();
        if (currentWin && typeof currentWin.onDragDropEvent === 'function') {
          unlistenDragDrop = await currentWin.onDragDropEvent(async (event) => {
            if (event.payload.type === 'drop' && event.payload.paths && event.payload.paths.length > 0) {
              const { getMimeType } = await import('$lib/utils/fileType');
              const filesArray: File[] = [];
              for (const filePath of event.payload.paths) {
                try {
                  const meta = await invoke<{ name: string; size_bytes: number; path: string }>('get_file_metadata', { path: filePath });
                  const mime = getMimeType(meta.name);
                  const blob = new Blob([], { type: mime });
                  const file = new File([blob], meta.name, { type: mime });
                  Object.defineProperty(file, 'size', { value: meta.size_bytes, writable: false });
                  (file as any)._localPath = meta.path;
                  (file as any)._isLoaded = false;
                  filesArray.push(file);
                } catch (metaErr) {
                  console.error('Failed to get metadata for dropped file:', filePath, metaErr);
                }
              }
              if (filesArray.length > 0) {
                const startIndex = appState.documents.length;
                appState.addDocuments(filesArray);
                appState.selectDocument(startIndex);
              }
            }
          });
        }
      } catch (err) {
        console.warn('Native drag-drop event registration skipped:', err);
      }
    })();

    // We use an async IIFE because `listen` returns a promise containing the unlisten function
    (async () => {
      try {
        unlisten = await listen('menu-open-file', async () => {
          try {
            const { open } = await import('@tauri-apps/plugin-dialog');
            const selected = await open({
              multiple: false,
              filters: [
                { name: 'All Supported Documents', extensions: ['pdf', 'docx', 'doc', 'xlsx', 'xls', 'pptx', 'ppt', 'csv'] },
                { name: 'PDF Documents', extensions: ['pdf'] },
                { name: 'Excel Spreadsheets', extensions: ['xlsx', 'xls', 'csv'] },
                { name: 'Word Documents', extensions: ['docx', 'doc'] },
                { name: 'PowerPoint Presentations', extensions: ['pptx', 'ppt'] }
              ]
            });
            if (selected && typeof selected === 'string') {
              const fileName =
                selected.split('/').pop() ||
                selected.split('\\').pop() ||
                'document.pdf';
              const { getMimeType } = await import('$lib/utils/fileType');
              const mime = getMimeType(fileName);
              const bytes: number[] = await invoke('read_file_bytes', { path: selected });
              const blob = new Blob([new Uint8Array(bytes)], { type: mime });
              const file = new File([blob], fileName, { type: mime });
              (file as any)._localPath = selected;
              appState.addDocuments([file]);
              appState.selectDocument(appState.documents.length - 1);
            }
          } catch (err) {
            console.error('Failed to open file dialog:', err);
          }
        });

        unlistenSave = await listen('menu-save-annotations', () => {
          window.dispatchEvent(new CustomEvent('paperpilot:save-annotations'));
        });

        let unlistenCloseAll = await listen('menu-close-all', () => {
          appState.clearAllDocuments();
        });

        unlistenSettings = await listen('menu-settings', () => {
          appState.setActiveTab('settings');
        });

        unlistenUndo = await listen('menu-undo', () => {
          window.dispatchEvent(new CustomEvent('paperpilot:undo'));
        });

        unlistenRedo = await listen('menu-redo', () => {
          window.dispatchEvent(new CustomEvent('paperpilot:redo'));
        });

        unlistenZoomIn = await listen('menu-zoom-in', () => {
          window.dispatchEvent(new CustomEvent('paperpilot:zoom-in'));
        });

        unlistenZoomOut = await listen('menu-zoom-out', () => {
          window.dispatchEvent(new CustomEvent('paperpilot:zoom-out'));
        });

        unlistenFitWidth = await listen('menu-fit-width', () => {
          window.dispatchEvent(new CustomEvent('paperpilot:fit-width'));
        });
      } catch (err) {
        // If not running in Tauri (e.g. standard browser web mode testing)
        console.warn('Failed to listen to Tauri events. Make sure this is running in a Tauri context.', err);
      }
    })();

    return () => {
      if (unlisten) unlisten();
      if (unlistenSave) unlistenSave();
      if (unlistenSettings) unlistenSettings();
      if (unlistenUndo) unlistenUndo();
      if (unlistenRedo) unlistenRedo();
      if (unlistenZoomIn) unlistenZoomIn();
      if (unlistenZoomOut) unlistenZoomOut();
      if (unlistenFitWidth) unlistenFitWidth();
      if (unlistenDragDrop) unlistenDragDrop();
    };
  });

  // Sync theme with document class
  $effect(() => {
    if (typeof document !== 'undefined') {
      if (appState.theme === 'light') {
        document.documentElement.classList.add('light');
        document.documentElement.classList.remove('dark');
      } else {
        document.documentElement.classList.add('dark');
        document.documentElement.classList.remove('light');
      }
    }
  });
</script>

<div class="app-container">
  <Titlebar />
  <div class="main-wrapper">
    <Sidebar />
    <main class="main-content">
      {@render children()}
    </main>
  </div>
</div>

<CommandPalette />
<WorkspaceSearchModal />
<BottomNav />
<ToastContainer />
<ExportCodeModal
  isOpen={appState.exportCodeModalOpen}
  target={appState.exportCodeTarget}
  onClose={() => appState.closeExportCode()}
/>

{#if !appState.onboardingCompleted}
  <ModularSetupModal />
{/if}
