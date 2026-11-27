<script lang="ts">
  interface Props {
    isOpen: boolean;
    onClose: () => void;
  }

  let { isOpen = $bindable(false), onClose }: Props = $props();

  const features = [
    {
      title: 'Client-Side WASM Engine',
      description: 'Zero compilation latency, compiling pure Rust into high-performance WebAssembly running in your browser.',
      icon: '⚡',
      link: '#wasm-playground'
    },
    {
      title: '100% Privacy & Zero Uploads',
      description: 'Documents never touch a server. All operations execute strictly on your device hardware with 0 bytes transferred.',
      icon: '🔒',
      link: '#hero-section'
    },
    {
      title: 'Hexagonal Storage Architecture',
      description: 'Modular file persistence ports supporting local disk, cloud buckets, and encrypted client-side memory buffers.',
      icon: '⬡',
      link: 'https://docs.usepaperpilot.com/architecture/storage/'
    },
    {
      title: 'CLI & AI Agent MCP Server',
      description: 'Universal developer tooling with Model Context Protocol (MCP) server for Claude Desktop, Cursor, and automated scripts.',
      icon: '🤖',
      link: 'https://docs.usepaperpilot.com/mcp/'
    }
  ];

  function handleFeatureClick(link: string) {
    onClose();
    if (link.startsWith('#')) {
      const el = document.querySelector(link);
      el?.scrollIntoView({ behavior: 'smooth' });
    } else {
      window.open(link, '_blank', 'noopener,noreferrer');
    }
  }
</script>

{#if isOpen}
  <div class="features-backdrop" onclick={onClose} role="presentation"></div>
  <div class="features-dropdown" role="dialog" aria-label="PaperPilot Features">
    <div class="features-list">
      {#each features as item}
        <button
          type="button"
          class="feature-item"
          onclick={() => handleFeatureClick(item.link)}
        >
          <div class="feature-icon">{item.icon}</div>
          <div class="feature-content">
            <h4 class="feature-title">{item.title}</h4>
            <p class="feature-desc">{item.description}</p>
          </div>
          <span class="feature-arrow">→</span>
        </button>
      {/each}
    </div>
  </div>
{/if}

<style>
  .features-backdrop {
    position: fixed;
    inset: 0;
    z-index: 1000;
  }

  .features-dropdown {
    position: absolute;
    top: 56px;
    left: 0;
    width: 380px;
    background: var(--bg-secondary, #18181b);
    border: 1px solid var(--border-color, #27272a);
    border-radius: 14px;
    box-shadow: 0 20px 40px -10px rgba(0, 0, 0, 0.6);
    z-index: 1001;
    overflow: hidden;
    padding: 8px;
    animation: fadeIn 0.15s ease-out;
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
      transform: translateY(-8px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .features-list {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .feature-item {
    display: flex;
    align-items: flex-start;
    gap: 12px;
    padding: 10px 12px;
    border-radius: 8px;
    background: transparent;
    border: 1px solid transparent;
    text-align: left;
    cursor: pointer;
    transition: all 0.15s ease;
    color: inherit;
  }

  .feature-item:hover {
    background: rgba(255, 255, 255, 0.05);
    border-color: rgba(255, 255, 255, 0.1);
  }

  .feature-icon {
    font-size: 1.25rem;
    padding-top: 2px;
  }

  .feature-content {
    flex: 1;
    min-width: 0;
  }

  .feature-title {
    margin: 0 0 2px 0;
    font-size: 0.88rem;
    font-weight: 600;
    color: var(--text-primary, #f4f4f5);
  }

  .feature-desc {
    margin: 0;
    font-size: 0.75rem;
    color: var(--text-secondary, #a1a1aa);
    line-height: 1.35;
  }

  .feature-arrow {
    color: var(--text-secondary, #71717a);
    font-size: 0.9rem;
    opacity: 0;
    transition: opacity 0.15s;
    margin-top: 2px;
  }

  .feature-item:hover .feature-arrow {
    opacity: 1;
    color: var(--accent-primary, #6366f1);
  }
</style>
