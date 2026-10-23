<script lang="ts">
  interface Props {
    title: string;
    report: Record<string, any>;
    outputPath?: string;
  }
  let { title, report, outputPath }: Props = $props();
  let copied = $state(false);

  async function copyJson() {
    await navigator.clipboard.writeText(JSON.stringify(report, null, 2));
    copied = true;
    setTimeout(() => copied = false, 2000);
  }
</script>

<div class="audit-viewer" style="background: var(--bg-surface); border: 1px solid var(--border-color); border-radius: 8px; padding: 12px; margin-top: 12px;">
  <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 8px;">
    <h4 style="margin: 0; font-size: 13px; font-weight: 600;">📊 {title} Findings</h4>
    <button onclick={copyJson} style="font-size: 11px; padding: 4px 8px; border-radius: 4px; border: 1px solid var(--border-color); background: var(--bg-subtle);">
      {copied ? '✓ Copied' : 'Copy JSON'}
    </button>
  </div>
  <pre style="font-size: 11px; background: var(--bg-canvas); padding: 8px; border-radius: 6px; overflow-x: auto; max-height: 200px;">{JSON.stringify(report, null, 2)}</pre>
</div>
