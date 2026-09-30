<script lang="ts">
  import '../app.css';
  import Sidebar from '$lib/components/layout/Sidebar.svelte';
  import BottomNav from '$lib/components/layout/BottomNav.svelte';
  import ToastContainer from '$lib/components/layout/ToastContainer.svelte';
  import { onMount } from 'svelte';
  import { listen } from '@tauri-apps/api/event';

  let { children } = $props();

  onMount(() => {
    let unlisten: () => void;

    // We use an async IIFE because `listen` returns a promise containing the unlisten function
    (async () => {
      try {
        unlisten = await listen('menu-open-file', (event) => {
          console.log("Menu action: Open File");
        });
      } catch (err) {
        // If not running in Tauri (e.g. standard browser web mode testing)
        console.warn('Failed to listen to Tauri events. Make sure this is running in a Tauri context.', err);
      }
    })();

    return () => {
      if (unlisten) {
        unlisten();
      }
    };
  });
</script>

<div class="app-container">
  <Sidebar />
  <main class="main-content">
    {@render children()}
  </main>
</div>

<BottomNav />
<ToastContainer />
