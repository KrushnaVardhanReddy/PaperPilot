<script lang="ts">
  import '../app.css';
  let { children } = $props();
</script>

<div class="app-container">
  {@render children()}
</div>
