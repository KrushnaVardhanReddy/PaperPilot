<script lang="ts">
  import { ALL_101_TOOLS } from '../lib/toolsCatalog';
  import { getToolsHandbookUrl } from '../constants';

  interface Props {
    toolId: string;
    toolTitle: string;
    toolIcon?: string;
    toolDescription?: string;
    onclose: () => void;
  }

  let { toolId, toolTitle, toolIcon = '📄', toolDescription = '', onclose }: Props = $props();

  let copied = $state(false);

  const catalogEntry = $derived(
    ALL_101_TOOLS.find((t) => t.wasmToolId === toolId || t.id === toolId)
  );

  const runtime = $derived<'wasm' | 'desktop'>(
    catalogEntry?.runtime === 'desktop' ? 'desktop' : 'wasm'
  );

  const description = $derived(
    catalogEntry?.description ||
      toolDescription ||
      'Process your PDF locally with zero server uploads.'
  );

  const cliCommand = $derived(`paperpilot ${catalogEntry?.id || toolId} input.pdf`);
  const handbookUrl = getToolsHandbookUrl();

  async function copyCliSnippet() {
    try {
      await navigator.clipboard.writeText(cliCommand);
      copied = true;
      setTimeout(() => {
        copied = false;
      }, 1800);
    } catch {
      // Ignore clipboard permission errors
    }
  }
</script>

<aside class="tool-docs-panel" aria-label="Tool documentation">
  <div class="docs-header">
    <div class="docs-title-group">
      <span class="docs-icon">{toolIcon}</span>
      <div class="docs-title-wrap">
        <h3 class="docs-title">{toolTitle}</h3>
        <span class="runtime-badge" class:desktop={runtime === 'desktop'}>
          {runtime === 'desktop' ? 'Desktop' : 'WASM'}
        </span>
      </div>
    </div>
    <button
      type="button"
      class="docs-close-btn"
      onclick={onclose}
      aria-label="Close documentation panel"
      title="Close"
    >
      ×
    </button>
  </div>

  <div class="docs-body">
    <section class="docs-section">
      <h4 class="docs-section-label">Overview</h4>
      <p class="docs-description">{description}</p>
    </section>

    <section class="docs-section">
      <h4 class="docs-section-label">Quick Guide</h4>
      <ol class="docs-steps">
        <li>Drop or select your input file(s)</li>
        <li>Configure tool options</li>
        <li>Run &amp; download the result</li>
      </ol>
    </section>

    <section class="docs-section">
      <div class="privacy-note">
        <span class="privacy-dot" aria-hidden="true"></span>
        <span>
          {runtime === 'desktop'
            ? 'Runs locally via PaperPilot Desktop/CLI'
            : '100% local in-browser WASM'}
        </span>
      </div>
    </section>

    <section class="docs-section">
      <h4 class="docs-section-label">CLI Equivalent</h4>
      <div class="cli-box">
        <code class="cli-code">{cliCommand}</code>
        <button
          type="button"
          class="cli-copy-btn"
          onclick={copyCliSnippet}
          title="Copy CLI command"
        >
          {copied ? 'Copied' : 'Copy'}
        </button>
      </div>
    </section>
  </div>

  <div class="docs-footer">
    <a
      href={handbookUrl}
      target="_blank"
      rel="noopener noreferrer"
      class="full-docs-link"
    >
      Open full docs ↗
    </a>
  </div>
</aside>

<style>
  .tool-docs-panel {
    width: 340px;
    flex-shrink: 0;
    background-color: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: var(--border-radius-lg);
    padding: 18px;
    max-height: 80vh;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    gap: 16px;
    color: var(--text-primary);
  }

  .docs-header {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    gap: 10px;
    padding-bottom: 12px;
    border-bottom: 1px solid var(--border-color);
  }

  .docs-title-group {
    display: flex;
    align-items: center;
    gap: 10px;
    min-width: 0;
  }

  .docs-icon {
    font-size: 1.35rem;
    line-height: 1;
    flex-shrink: 0;
  }

  .docs-title-wrap {
    display: flex;
    flex-direction: column;
    gap: 4px;
    min-width: 0;
  }

  .docs-title {
    margin: 0;
    font-size: 1rem;
    font-weight: 600;
    color: var(--text-primary);
    line-height: 1.25;
  }

  .runtime-badge {
    display: inline-flex;
    align-items: center;
    width: fit-content;
    padding: 2px 8px;
    border-radius: 999px;
    font-size: 0.7rem;
    font-weight: 600;
    letter-spacing: 0.04em;
    text-transform: uppercase;
    background: rgba(59, 130, 246, 0.16);
    color: #60a5fa;
    border: 1px solid rgba(59, 130, 246, 0.35);
  }

  .runtime-badge.desktop {
    background: rgba(139, 92, 246, 0.16);
    color: #a78bfa;
    border-color: rgba(139, 92, 246, 0.35);
  }

  .docs-close-btn {
    background: transparent;
    border: 1px solid var(--border-color);
    color: var(--text-secondary);
    width: 28px;
    height: 28px;
    border-radius: var(--border-radius-sm);
    font-size: 1.1rem;
    line-height: 1;
    cursor: pointer;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    transition: background-color 0.15s ease, color 0.15s ease;
  }

  .docs-close-btn:hover {
    background: rgba(255, 255, 255, 0.08);
    color: var(--text-primary);
  }

  .docs-body {
    display: flex;
    flex-direction: column;
    gap: 16px;
    flex: 1;
  }

  .docs-section-label {
    margin: 0 0 6px 0;
    font-size: 0.72rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: var(--text-secondary);
  }

  .docs-description {
    margin: 0;
    font-size: 0.875rem;
    line-height: 1.5;
    color: var(--text-secondary);
  }

  .docs-steps {
    margin: 0;
    padding-left: 18px;
    display: flex;
    flex-direction: column;
    gap: 6px;
    font-size: 0.85rem;
    line-height: 1.45;
    color: var(--text-primary);
  }

  .privacy-note {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 10px 12px;
    border-radius: var(--border-radius-sm);
    background: rgba(16, 185, 129, 0.1);
    border: 1px solid rgba(16, 185, 129, 0.28);
    font-size: 0.8rem;
    color: #34d399;
    font-weight: 500;
  }

  .privacy-dot {
    width: 7px;
    height: 7px;
    border-radius: 50%;
    background: #10b981;
    flex-shrink: 0;
  }

  .cli-box {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 8px;
    padding: 8px 10px;
    border-radius: var(--border-radius-sm);
    background: rgba(0, 0, 0, 0.35);
    border: 1px solid var(--border-color);
  }

  .cli-code {
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
    font-size: 0.76rem;
    color: #e2e8f0;
    overflow-x: auto;
    white-space: nowrap;
  }

  .cli-copy-btn {
    background: rgba(255, 255, 255, 0.08);
    border: 1px solid var(--border-color);
    color: var(--text-primary);
    padding: 4px 8px;
    border-radius: 4px;
    font-size: 0.72rem;
    font-weight: 500;
    cursor: pointer;
    flex-shrink: 0;
    transition: background-color 0.15s ease;
  }

  .cli-copy-btn:hover {
    background: rgba(255, 255, 255, 0.15);
  }

  .docs-footer {
    padding-top: 12px;
    border-top: 1px solid var(--border-color);
  }

  .full-docs-link {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    font-size: 0.8rem;
    color: var(--primary-color, #3b82f6);
    text-decoration: none;
    font-weight: 500;
  }

  .full-docs-link:hover {
    text-decoration: underline;
  }

  @media (max-width: 1024px) {
    .tool-docs-panel {
      width: 100%;
      max-height: none;
    }
  }
</style>
