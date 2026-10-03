<script lang="ts">
  import '../app.css';
  import Sidebar from '$lib/components/layout/Sidebar.svelte';
  import BottomNav from '$lib/components/layout/BottomNav.svelte';
  import ToastContainer from '$lib/components/layout/ToastContainer.svelte';
  import Titlebar from '$lib/components/layout/Titlebar.svelte';
  import CommandPalette from '$lib/components/layout/CommandPalette.svelte';
  import { onMount } from 'svelte';
  import { listen } from '@tauri-apps/api/event';
  import { invoke } from '@tauri-apps/api/core';

  import { appState } from '$lib/state/app.svelte';

  let { children } = $props();

  onMount(() => {
    let unlisten: () => void;
    let unlistenSave: () => void;
    let unlistenSettings: () => void;
    let unlistenUndo: () => void;
    let unlistenRedo: () => void;
    let unlistenZoomIn: () => void;
    let unlistenZoomOut: () => void;
    let unlistenFitWidth: () => void;

    // We use an async IIFE because `listen` returns a promise containing the unlisten function
    (async () => {
      try {
        unlisten = await listen('menu-open-file', async () => {
          try {
            const { open } = await import('@tauri-apps/plugin-dialog');
            const selected = await open({
              multiple: false,
              filters: [{ name: 'PDF Documents', extensions: ['pdf'] }]
            });
            if (selected && typeof selected === 'string') {
              const fileName =
                selected.split('/').pop() ||
                selected.split('\\').pop() ||
                'document.pdf';
              const bytes: number[] = await invoke('read_file_bytes', { path: selected });
              const blob = new Blob([new Uint8Array(bytes)], { type: 'application/pdf' });
              const file = new File([blob], fileName, { type: 'application/pdf' });
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
<BottomNav />
<ToastContainer />
