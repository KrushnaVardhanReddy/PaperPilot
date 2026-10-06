<script lang="ts">
  interface Props {
    tools?: string;
    hideBadge?: boolean;
    brandColor?: string;
    theme?: string;
    logoUrl?: string;
  }

  let { tools = 'all', hideBadge = false, brandColor = '#3B82F6', theme = 'dark', logoUrl }: Props = $props();

  const allTools = [
    { id: 'merge', name: 'Merge PDF' },
    { id: 'compress', name: 'Compress PDF' },
    { id: 'watermark', name: 'Watermark' },
    { id: 'split', name: 'Split PDF' }
  ];

  let availableTools = $derived(
    tools === 'all'
      ? allTools
      : allTools.filter(t => tools.split(',').map(s => s.trim()).includes(t.id))
  );

  function dispatchEvent(eventName: string, detail: any = {}) {
    const event = new CustomEvent(`paperpilot:${eventName}`, {
      detail,
      bubbles: true,
      composed: true
    });
    // @ts-ignore
    document.activeElement?.dispatchEvent(event) || window.dispatchEvent(event);
  }

  $effect(() => {
    dispatchEvent('ready');
  });

  function handleProcessStart(toolId: string) {
    dispatchEvent('process-start', { tool: toolId });
    // Simulate process
    setTimeout(() => {
      dispatchEvent('process-complete', { tool: toolId, outputSizeBytes: 1024 });
    }, 1000);
  }
</script>

<div class="pp-widget" data-theme={theme}>
  {#if logoUrl}
    <div class="pp-header">
      <img src={logoUrl} alt="Logo" class="pp-logo" />
    </div>
  {/if}

  <div class="pp-dropzone">
    <p>Drag and drop PDFs here</p>
  </div>

  <div class="pp-tools">
    {#each availableTools as tool (tool.id)}
      <button class="pp-tool-card" onclick={() => handleProcessStart(tool.id)} data-tool-id={tool.id}>
        {tool.name}
      </button>
    {/each}
  </div>

  {#if !hideBadge}
    <div class="pp-footer">
      <a href="https://usepaperpilot.com" target="_blank" rel="noopener" class="pp-badge">
        ⚡ Powered by PaperPilot
      </a>
    </div>
  {/if}
</div>

<style>
  .pp-widget {
    background-color: var(--pp-background);
    color: var(--pp-text);
    font-family: var(--pp-font);
    border-radius: var(--pp-radius);
    padding: 20px;
    box-sizing: border-box;
    width: 100%;
    border: 1px solid var(--pp-border);
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .pp-header {
    display: flex;
    justify-content: center;
    margin-bottom: 8px;
  }

  .pp-logo {
    max-height: 40px;
    max-width: 100%;
  }

  .pp-dropzone {
    border: 2px dashed var(--pp-border);
    border-radius: var(--pp-radius);
    padding: 32px;
    text-align: center;
    background-color: var(--pp-surface);
    color: var(--pp-text-muted);
    transition: border-color 0.2s;
  }

  .pp-dropzone:hover {
    border-color: var(--pp-brand-color);
  }

  .pp-tools {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(140px, 1fr));
    gap: 12px;
  }

  .pp-tool-card {
    background-color: var(--pp-surface);
    color: var(--pp-text);
    border: 1px solid var(--pp-border);
    border-radius: 8px;
    padding: 16px 12px;
    text-align: center;
    cursor: pointer;
    font-weight: 500;
    transition: all 0.2s;
    font-size: 14px;
    font-family: inherit;
  }

  .pp-tool-card:hover {
    border-color: var(--pp-brand-color);
    background-color: var(--pp-brand-color);
    color: white;
  }

  .pp-footer {
    display: flex;
    justify-content: center;
    margin-top: 8px;
  }

  .pp-badge {
    display: inline-block;
    padding: 6px 12px;
    background-color: var(--pp-surface);
    color: var(--pp-text-muted);
    text-decoration: none;
    font-size: 12px;
    border-radius: 16px;
    border: 1px solid var(--pp-border);
    transition: all 0.2s;
  }

  .pp-badge:hover {
    color: var(--pp-text);
    border-color: var(--pp-text-muted);
  }
</style>
