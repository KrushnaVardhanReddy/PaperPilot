<script lang="ts">
  import { appState } from '$lib/state/app.svelte';
  import DropZone from '$lib/components/ui/DropZone.svelte';
  import { allTools } from '$lib/types/toolList';
  import type { ToolDefinition } from '$lib/types/tools';

  const actionCards: { id: string; title: string; description: string; icon: string; toolId: string }[] = [
    {
      id: 'smart_compress',
      title: 'Smart Compress',
      description: 'Make PDF/Word smaller',
      icon: '🗜️',
      toolId: 'compress'
    },
    {
      id: 'universal_convert',
      title: 'Universal Convert',
      description: 'Word ⇄ PDF ⇄ Excel ⇄ Markdown',
      icon: '🔄',
      toolId: 'pdf_to_docx' // fallback, could open a generic converter modal
    },
    {
      id: 'sign_protect',
      title: 'Sign & Protect',
      description: 'Sign agreements, add password',
      icon: '✍️',
      toolId: 'encrypt'
    },
    {
      id: 'merge_combine',
      title: 'Merge & Combine',
      description: 'Join documents together',
      icon: '📑',
      toolId: 'merge'
    },
    {
      id: 'ai_ocr',
      title: 'AI OCR & Scan',
      description: 'Extract text from images/scans',
      icon: '📷',
      toolId: 'ocr'
    },
    {
      id: 'spreadsheets',
      title: 'Spreadsheets & Data',
      description: 'Inspect & clean CSV/XLSX',
      icon: '📊',
      toolId: 'csv_to_xlsx' // or similar tool
    },
    {
      id: 'redact',
      title: 'Redact & Sanitize',
      description: 'Black out sensitive text, remove metadata',
      icon: '🛡️',
      toolId: 'metadata' // or sanitize
    },
    {
      id: 'automate',
      title: 'Automate Pipeline',
      description: 'Switch to visual DAG workflow builder',
      icon: '🤖',
      toolId: 'pipeline' // Special case: navigate to pipeline
    }
  ];

  function handleActionClick(toolId: string) {
    if (toolId === 'pipeline') {
      appState.setActiveTab('pipeline');
      return;
    }
    const tool = allTools.find(t => t.id === toolId);
    if (tool) {
      appState.openTool(tool);
      // Let the dropzone handle file selection via the active tool context if needed.
    }
  }
</script>

<div class="universal-home">
  <div class="hero-section">
    <h1 class="welcome-header">PaperPilot: Universal Document OS</h1>
    <p class="welcome-subtitle">Open, convert, or automate any document — pure Rust, 100% private on your machine.</p>
  </div>

  <div class="dropzone-container">
    <DropZone
      ondrop={(files) => appState.addDocuments(files)}
      fileContext="mixed"
    />
  </div>

  <div class="action-grid-section">
    <h2 class="section-title">Everyday Universal Actions</h2>
    <div class="action-grid">
      {#each actionCards as card}
        <button class="action-card" onclick={() => handleActionClick(card.toolId)}>
          <div class="card-icon">{card.icon}</div>
          <div class="card-content">
            <h3 class="card-title">{card.title}</h3>
            <p class="card-desc">{card.description}</p>
          </div>
        </button>
      {/each}
    </div>
  </div>
</div>

<style>
  .universal-home {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 48px 24px;
    width: 100%;
    max-width: 900px;
    margin: 0 auto;
    gap: 48px;
  }

  .hero-section {
    text-align: center;
  }

  .welcome-header {
    font-size: 2.5rem;
    font-weight: 700;
    color: var(--text-primary);
    margin-bottom: 12px;
    letter-spacing: -0.02em;
  }

  .welcome-subtitle {
    font-size: 1.1rem;
    color: var(--text-secondary);
    max-width: 600px;
    margin: 0 auto;
    line-height: 1.5;
  }

  .dropzone-container {
    width: 100%;
  }

  .action-grid-section {
    width: 100%;
  }

  .section-title {
    font-size: 1.25rem;
    font-weight: 600;
    color: var(--text-primary);
    margin-bottom: 24px;
  }

  .action-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
    gap: 16px;
  }

  .action-card {
    background: var(--bg-surface);
    border: 1px solid var(--border-color);
    border-radius: var(--border-radius-md);
    padding: 20px;
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    gap: 12px;
    text-align: left;
    cursor: pointer;
    transition: all var(--transition-fast);
  }

  .action-card:hover {
    background: var(--bg-surface-hover);
    border-color: var(--accent-primary);
    transform: translateY(-2px);
  }

  .card-icon {
    font-size: 2rem;
  }

  .card-content {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .card-title {
    font-size: 1rem;
    font-weight: 600;
    color: var(--text-primary);
    margin: 0;
  }

  .card-desc {
    font-size: 0.85rem;
    color: var(--text-secondary);
    margin: 0;
    line-height: 1.4;
  }
</style>
