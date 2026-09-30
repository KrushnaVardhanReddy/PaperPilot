<script lang="ts">
  let { pdfDoc } = $props();
  let metadata: any = $state(null);

  $effect(() => {
    if (pdfDoc) {
      pdfDoc.getMetadata().then((data: any) => {
        metadata = data.info;
      }).catch((err: any) => {
        console.error("Error fetching metadata:", err);
      });
    } else {
      metadata = null;
    }
  });
</script>

<div class="info-panel">
  <div class="info-header">
    <h3>Document Info</h3>
  </div>

  <div class="info-content">
    {#if metadata}
      <div class="info-item">
        <span class="info-label">Title</span>
        <span class="info-value">{metadata.Title || 'Unknown'}</span>
      </div>
      <div class="info-item">
        <span class="info-label">Author</span>
        <span class="info-value">{metadata.Author || 'Unknown'}</span>
      </div>
      <div class="info-item">
        <span class="info-label">Producer</span>
        <span class="info-value">{metadata.Producer || 'Unknown'}</span>
      </div>
      <div class="info-item">
        <span class="info-label">Creator</span>
        <span class="info-value">{metadata.Creator || 'Unknown'}</span>
      </div>
      <div class="info-item">
        <span class="info-label">Creation Date</span>
        <span class="info-value">
          {#if metadata.CreationDate}
            {metadata.CreationDate.replace('D:', '').substring(0, 14)}
          {:else}
            Unknown
          {/if}
        </span>
      </div>
    {/if}

    {#if pdfDoc}
      <div class="info-item">
        <span class="info-label">Pages</span>
        <span class="info-value">{pdfDoc.numPages}</span>
      </div>
    {/if}

    {#if !metadata && !pdfDoc}
      <div class="empty-state">
        <p>No document loaded</p>
      </div>
    {/if}
  </div>
</div>

<style>
  .info-panel {
    width: 280px;
    background-color: var(--bg-surface, #ffffff);
    border-left: 1px solid var(--border-color, #e5e7eb);
    display: flex;
    flex-direction: column;
    height: 100%;
  }

  .info-header {
    padding: 1rem 1.5rem;
    border-bottom: 1px solid var(--border-color, #e5e7eb);
  }

  .info-header h3 {
    margin: 0;
    font-size: 1.1rem;
    color: var(--text-primary, #111827);
    font-weight: 600;
  }

  .info-content {
    padding: 1.5rem;
    display: flex;
    flex-direction: column;
    gap: 1.25rem;
    overflow-y: auto;
  }

  .info-item {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .info-label {
    font-size: 0.75rem;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--text-muted, #6b7280);
    font-weight: 600;
  }

  .info-value {
    font-size: 0.95rem;
    color: var(--text-primary, #374151);
    word-break: break-word;
    line-height: 1.4;
  }

  .empty-state {
    text-align: center;
    color: var(--text-muted, #6b7280);
    padding: 2rem 0;
    font-size: 0.9rem;
  }
</style>
