<script lang="ts">
  import '../app.css';
  import Sidebar from '$lib/components/layout/Sidebar.svelte';
  import BottomNav from '$lib/components/layout/BottomNav.svelte';
  import ToastContainer from '$lib/components/layout/ToastContainer.svelte';
  let { children } = $props();
</script>

<div class="app-container">
  <Sidebar />
  <main class="main-content">
    {@render children()}
  </main>
</div>

<BottomNav />
<ToastContainer />
