<script lang="ts">
  import ClassicGold from '../components/certificate/templates/ClassicGold.svelte';
  import ModernMinimalist from '../components/certificate/templates/ModernMinimalist.svelte';
  import TechHackathon from '../components/certificate/templates/TechHackathon.svelte';
  import AcademicDiploma from '../components/certificate/templates/AcademicDiploma.svelte';
  import BulkCsvGenerator from '../components/certificate/BulkCsvGenerator.svelte';
  import { onMount } from 'svelte';
  import jsPDF from 'jspdf';
  import html2canvas from 'html2canvas';

  let activeTab = $state<'single' | 'bulk'>('single');
  let selectedTemplate = $state('classic');

  let previewPanelWidth = $state(1000);
  let previewPanelRef = $state<HTMLElement | null>(null);

  const BASE_WIDTH = 1122; // A4 Landscape width
  let scaleRatio = $derived(Math.min(1, (previewPanelWidth - 64) / BASE_WIDTH)); // 64px padding

  onMount(() => {
    const observer = new ResizeObserver(entries => {
      if (entries[0]) {
        previewPanelWidth = entries[0].contentRect.width;
      }
    });

    if (previewPanelRef) {
      observer.observe(previewPanelRef);
    }

    return () => observer.disconnect();
  });

  let certificateData = $state({
    recipientName: 'Alex Morgan',
    title: 'Full Stack Rust & WASM Mastery',
    description: 'For successfully completing the intensive curriculum and demonstrating exceptional proficiency in systems programming.',
    date: 'October 2026',
    issuerName: 'PaperPilot Academy',
    issuerTitle: 'Lead Instructor',
    certId: `CERT-${Math.random().toString(36).substr(2, 6).toUpperCase()}`,
    verificationUrl: 'https://usepaperpilot.com/verify',
    sha256Hash: ''
  });

  const getActiveTemplate = () => {
    switch (selectedTemplate) {
      case 'modern': return ModernMinimalist;
      case 'tech': return TechHackathon;
      case 'academic': return AcademicDiploma;
      case 'classic':
      default: return ClassicGold;
    }
  };

  const handlePrint = () => {
    window.print();
  };

  let isGeneratingPdf = $state(false);

  const handleDownloadPdf = async () => {
    isGeneratingPdf = true;
    try {
      // Find the certificate container to screenshot
      const certElement = document.querySelector('.certificate-container') as HTMLElement;
      if (!certElement) throw new Error("Certificate element not found");

      // Momentarily remove scale for clear rendering
      const originalTransform = certElement.parentElement!.style.transform;
      certElement.parentElement!.style.transform = 'none';

      const canvas = await html2canvas(certElement, { scale: 2 });

      certElement.parentElement!.style.transform = originalTransform;

      const imgData = canvas.toDataURL('image/jpeg', 1.0);
      const pdf = new jsPDF({
        orientation: 'landscape',
        unit: 'px',
        format: [canvas.width, canvas.height]
      });
      pdf.addImage(imgData, 'JPEG', 0, 0, canvas.width, canvas.height);
      pdf.save(`certificate_${certificateData.recipientName.replace(/\s+/g, '_')}.pdf`);
    } catch (e) {
      console.error(e);
      alert('Failed to generate PDF.');
    } finally {
      isGeneratingPdf = false;
    }
  };

  const generateHash = async () => {
    // A simple mock hash for demo purposes,
    // in reality would use crypto.subtle.digest
    const text = JSON.stringify(certificateData);
    const encoder = new TextEncoder();
    const data = encoder.encode(text);
    const hashBuffer = await crypto.subtle.digest('SHA-256', data);
    const hashArray = Array.from(new Uint8Array(hashBuffer));
    certificateData.sha256Hash = hashArray.map(b => b.toString(16).padStart(2, '0')).join('');
  };
</script>

<div class="studio-container">
  <div class="header">
    <h2>Certificate & Template Studio</h2>
    <p>100% Client-Side Vector Generation</p>
  </div>

  <div class="tabs">
    <button class:active={activeTab === 'single'} onclick={() => activeTab = 'single'}>Single Certificate</button>
    <button class:active={activeTab === 'bulk'} onclick={() => activeTab = 'bulk'}>Bulk CSV Generator</button>
  </div>

  <div class="studio-workspace">
    <div class="controls-panel noprint">
      {#if activeTab === 'single'}
        <div class="section">
          <h3>1. Select Template</h3>
          <div class="template-selector">
            <button class:selected={selectedTemplate === 'classic'} onclick={() => selectedTemplate = 'classic'}>Classic Gold</button>
            <button class:selected={selectedTemplate === 'modern'} onclick={() => selectedTemplate = 'modern'}>Modern Minimalist</button>
            <button class:selected={selectedTemplate === 'tech'} onclick={() => selectedTemplate = 'tech'}>Tech Hackathon</button>
            <button class:selected={selectedTemplate === 'academic'} onclick={() => selectedTemplate = 'academic'}>Academic Diploma</button>
          </div>
        </div>

        <div class="section">
          <h3>2. Customize Details</h3>
          <div class="form-group">
            <label for="recipient">Recipient Name</label>
            <input id="recipient" type="text" bind:value={certificateData.recipientName} />
          </div>
          <div class="form-group">
            <label for="title">Title</label>
            <input id="title" type="text" bind:value={certificateData.title} />
          </div>
          <div class="form-group">
            <label for="desc">Description</label>
            <textarea id="desc" rows="3" bind:value={certificateData.description}></textarea>
          </div>
          <div class="form-row">
            <div class="form-group half">
              <label for="date">Date</label>
              <input id="date" type="text" bind:value={certificateData.date} />
            </div>
            <div class="form-group half">
              <label for="issuerName">Issuer Name</label>
              <input id="issuerName" type="text" bind:value={certificateData.issuerName} />
            </div>
          </div>
          <div class="form-row">
            <div class="form-group half">
              <label for="issuerTitle">Issuer Title</label>
              <input id="issuerTitle" type="text" bind:value={certificateData.issuerTitle} />
            </div>
            <div class="form-group half">
              <label for="certId">Certificate ID</label>
              <input id="certId" type="text" bind:value={certificateData.certId} />
            </div>
          </div>
        </div>

        <div class="section">
          <h3>3. Actions</h3>
          <div class="action-buttons">
            <button class="btn btn-secondary" onclick={generateHash}>Generate Integrity Hash</button>
            <button class="btn btn-primary" onclick={handleDownloadPdf} disabled={isGeneratingPdf}>
              {isGeneratingPdf ? 'Generating...' : 'Download PDF (Instant)'}
            </button>
            <button class="btn btn-secondary" onclick={handlePrint}>Print / Vector Save</button>
          </div>
        </div>
      {:else}
        <div class="section">
          <h3>1. Select Base Template</h3>
          <div class="template-selector">
            <button class:selected={selectedTemplate === 'classic'} onclick={() => selectedTemplate = 'classic'}>Classic Gold</button>
            <button class:selected={selectedTemplate === 'modern'} onclick={() => selectedTemplate = 'modern'}>Modern Minimalist</button>
            <button class:selected={selectedTemplate === 'tech'} onclick={() => selectedTemplate = 'tech'}>Tech Hackathon</button>
            <button class:selected={selectedTemplate === 'academic'} onclick={() => selectedTemplate = 'academic'}>Academic Diploma</button>
          </div>
        </div>
        <div class="section">
          <h3>2. Process CSV</h3>
          <BulkCsvGenerator template={selectedTemplate} />
        </div>
      {/if}
    </div>

    <div class="preview-panel" bind:this={previewPanelRef}>
      <div class="preview-wrapper" style="width: {BASE_WIDTH}px; transform: scale({scaleRatio}); transform-origin: top center;">
        <!-- Render Active Component Dynamically -->
        {#snippet activeTemplateRenderer()}
          {@const ActiveComponent = getActiveTemplate()}
          <ActiveComponent {...certificateData} />
        {/snippet}
        {@render activeTemplateRenderer()}
      </div>
      {#if activeTab === 'single'}
        <div class="preview-footer noprint">
          <p>Zero-latency live preview. What you see is what gets exported.</p>
        </div>
      {/if}
    </div>
  </div>
</div>

<style>
  .studio-container {
    display: flex;
    flex-direction: column;
    height: 100%;
    max-width: 1600px;
    margin: 0 auto;
    padding: 24px;
    box-sizing: border-box;
  }

  .header {
    margin-bottom: 24px;
  }

  .header h2 {
    margin: 0 0 8px 0;
    font-size: 1.8rem;
    color: var(--text-primary, #333);
  }

  .header p {
    margin: 0;
    color: var(--text-secondary, #666);
  }

  .tabs {
    display: flex;
    gap: 16px;
    margin-bottom: 24px;
    border-bottom: 1px solid var(--border-color, #eee);
    padding-bottom: 8px;
  }

  .tabs button {
    background: none;
    border: none;
    padding: 8px 16px;
    font-size: 1rem;
    font-weight: 600;
    color: var(--text-secondary, #666);
    cursor: pointer;
    border-bottom: 3px solid transparent;
  }

  .tabs button.active {
    color: var(--accent-primary, #3498db);
    border-bottom-color: var(--accent-primary, #3498db);
  }

  .studio-workspace {
    display: flex;
    gap: 32px;
    flex: 1;
    min-height: 0;
  }

  .controls-panel {
    flex: 0 0 400px;
    overflow-y: auto;
    padding-right: 16px;
  }

  .section {
    margin-bottom: 32px;
  }

  .section h3 {
    margin: 0 0 16px 0;
    font-size: 1.1rem;
    border-bottom: 1px solid var(--border-color, #eee);
    padding-bottom: 8px;
  }

  .template-selector {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 12px;
  }

  .template-selector button {
    padding: 12px;
    background: var(--bg-secondary, #f9f9f9);
    border: 1px solid var(--border-color, #ddd);
    border-radius: 6px;
    cursor: pointer;
    font-weight: 500;
    transition: all 0.2s;
  }

  .template-selector button:hover {
    background: #eee;
  }

  .template-selector button.selected {
    background: var(--accent-primary, #3498db);
    color: white;
    border-color: var(--accent-primary, #3498db);
  }

  .form-group {
    margin-bottom: 16px;
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .form-row {
    display: flex;
    gap: 16px;
  }

  .half {
    flex: 1;
  }

  label {
    font-size: 0.85rem;
    font-weight: 600;
    color: var(--text-secondary, #555);
  }

  input, textarea {
    padding: 8px 12px;
    border: 1px solid var(--border-color, #ccc);
    border-radius: 4px;
    font-size: 0.95rem;
    font-family: inherit;
  }

  input:focus, textarea:focus {
    outline: none;
    border-color: var(--accent-primary, #3498db);
  }

  .action-buttons {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .btn {
    padding: 12px 16px;
    border: none;
    border-radius: 6px;
    font-size: 1rem;
    font-weight: 600;
    cursor: pointer;
    text-align: center;
  }

  .btn-primary {
    background: var(--accent-primary, #3498db);
    color: white;
  }

  .btn-secondary {
    background: var(--bg-secondary, #eee);
    color: var(--text-primary, #333);
  }

  .preview-panel {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    background: var(--bg-secondary, #f5f5f5);
    border-radius: 12px;
    padding: 32px;
    overflow-y: auto;
  }

  .preview-wrapper {
    width: 100%;
    max-width: 1000px;
    transition: transform 0.2s;
    /* This ensures it scales down if window is smaller */
  }

  .preview-footer {
    margin-top: 24px;
    color: var(--text-secondary, #666);
    font-size: 0.9rem;
    text-align: center;
  }

  @media print {
    :global(body) {
      background: white !important;
      margin: 0 !important;
      padding: 0 !important;
    }

    .noprint {
      display: none !important;
    }

    .studio-container {
      padding: 0;
      margin: 0;
      max-width: 100%;
      height: 100vh;
      display: block;
    }

    .preview-panel {
      padding: 0;
      background: white;
      border: none;
      display: block;
    }

    .preview-wrapper {
      max-width: none;
      width: 100vw;
      height: 100vh;
      page-break-after: avoid;
    }

    @page {
      size: landscape;
      margin: 0;
    }
  }
</style>
