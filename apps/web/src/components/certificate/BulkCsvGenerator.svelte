<script lang="ts">
  import { onMount } from 'svelte';
  import JSZip from 'jszip';
  import jsPDF from 'jspdf';
  import html2canvas from 'html2canvas';
  import ClassicGold from './templates/ClassicGold.svelte';
  import ModernMinimalist from './templates/ModernMinimalist.svelte';
  import TechHackathon from './templates/TechHackathon.svelte';
  import AcademicDiploma from './templates/AcademicDiploma.svelte';
  import { mount, unmount } from 'svelte';

  let { template = 'classic' } = $props();

  let fileInput: HTMLInputElement;
  let isDragging = $state(false);
  let csvData = $state<string | null>(null);
  let parsedRows = $state<any[]>([]);
  let columns = $state<string[]>([]);

  let mapping = $state({
    recipientName: '',
    title: '',
    date: '',
    certId: ''
  });

  let isGenerating = $state(false);
  let progress = $state(0);
  let total = $state(0);

  // Hidden container to render SVGs/HTML to string
  let renderContainer: HTMLDivElement;

  const handleDragEnter = (e: DragEvent) => { e.preventDefault(); isDragging = true; };
  const handleDragLeave = (e: DragEvent) => { e.preventDefault(); isDragging = false; };
  const handleDragOver = (e: DragEvent) => { e.preventDefault(); };
  const handleDrop = (e: DragEvent) => {
    e.preventDefault();
    isDragging = false;
    if (e.dataTransfer?.files && e.dataTransfer.files.length > 0) {
      processFile(e.dataTransfer.files[0]);
    }
  };

  const handleFileSelect = (e: Event) => {
    const target = e.target as HTMLInputElement;
    if (target.files && target.files.length > 0) {
      processFile(target.files[0]);
    }
  };

  const processFile = (file: File) => {
    if (file.type !== 'text/csv' && !file.name.endsWith('.csv')) {
      alert('Please upload a valid CSV file.');
      return;
    }

    const reader = new FileReader();
    reader.onload = (e) => {
      csvData = e.target?.result as string;
      parseCSV(csvData);
    };
    reader.readAsText(file);
  };

  const parseCSV = (data: string) => {
    const lines = data.split('\n').filter(line => line.trim() !== '');
    if (lines.length < 2) {
      alert('CSV must contain headers and at least one row of data.');
      return;
    }

    columns = lines[0].split(',').map(c => c.trim().replace(/^"|"$/g, ''));
    parsedRows = lines.slice(1).map(line => {
      const values = line.split(',').map(v => v.trim().replace(/^"|"$/g, ''));
      const row: Record<string, string> = {};
      columns.forEach((col, i) => {
        row[col] = values[i] || '';
      });
      return row;
    });

    // Auto-map columns if possible
    mapping.recipientName = columns.find(c => /name|recipient/i.test(c)) || '';
    mapping.title = columns.find(c => /title|course/i.test(c)) || '';
    mapping.date = columns.find(c => /date/i.test(c)) || '';
    mapping.certId = columns.find(c => /id|cert/i.test(c)) || '';
  };

  const getTemplateComponent = () => {
    switch (template) {
      case 'modern': return ModernMinimalist;
      case 'tech': return TechHackathon;
      case 'academic': return AcademicDiploma;
      case 'classic':
      default: return ClassicGold;
    }
  };

  const generateBulk = async () => {
    if (!mapping.recipientName) {
      alert('Please map the Recipient Name column.');
      return;
    }

    isGenerating = true;
    total = parsedRows.length;
    progress = 0;

    const zip = new JSZip();
    const TemplateComponent = getTemplateComponent();

    for (let i = 0; i < parsedRows.length; i++) {
      const row = parsedRows[i];

      const props = {
        recipientName: row[mapping.recipientName] || 'Unknown',
        title: mapping.title ? row[mapping.title] : 'Certificate of Achievement',
        date: mapping.date ? row[mapping.date] : new Date().toLocaleDateString(),
        certId: mapping.certId ? row[mapping.certId] : `CERT-${Math.random().toString(36).substr(2, 6).toUpperCase()}`
      };

      // Create a temporary container
      const tempDiv = document.createElement('div');

      // Need inline styles for standard A4 landscape
      tempDiv.style.width = '1122px'; // A4 landscape at 96 DPI
      tempDiv.style.height = '793px';
      tempDiv.style.position = 'absolute';
      tempDiv.style.left = '-9999px';
      document.body.appendChild(tempDiv);

      // Mount Svelte 5 component
      const component = mount(TemplateComponent, {
        target: tempDiv,
        props
      });

      // Allow DOM to settle and load any fonts/svgs
      await new Promise(r => setTimeout(r, 50));

      // Get the rendered certificate container
      const certElement = tempDiv.firstElementChild as HTMLElement;

      const canvas = await html2canvas(certElement, { scale: 2 });
      const imgData = canvas.toDataURL('image/jpeg', 1.0);
      const pdf = new jsPDF({
        orientation: 'landscape',
        unit: 'px',
        format: [canvas.width, canvas.height]
      });
      pdf.addImage(imgData, 'JPEG', 0, 0, canvas.width, canvas.height);

      // Clean up
      unmount(component);
      document.body.removeChild(tempDiv);

      const pdfArrayBuffer = pdf.output('arraybuffer');
      const fileName = `${props.recipientName.replace(/[^a-z0-9]/gi, '_').toLowerCase()}_certificate.pdf`;
      zip.file(fileName, pdfArrayBuffer);

      progress = i + 1;

      // Yield to main thread for UI updates
      if (i % 10 === 0) {
        await new Promise(r => setTimeout(r, 0));
      }
    }

    try {
      const content = await zip.generateAsync({ type: 'blob' });
      const url = URL.createObjectURL(content);
      const a = document.createElement('a');
      a.href = url;
      a.download = `paperpilot_certificates_${Date.now()}.zip`;
      document.body.appendChild(a);
      a.click();
      document.body.removeChild(a);
      URL.revokeObjectURL(url);
    } catch (err) {
      console.error('Error generating zip:', err);
      alert('Failed to generate zip file.');
    } finally {
      isGenerating = false;
    }
  };

  const reset = () => {
    csvData = null;
    parsedRows = [];
    columns = [];
    isGenerating = false;
    progress = 0;
  };
</script>

<div class="bulk-generator">
  {#if !csvData}
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div
      class="dropzone {isDragging ? 'dragging' : ''}"
      ondragenter={handleDragEnter}
      ondragleave={handleDragLeave}
      ondragover={handleDragOver}
      ondrop={handleDrop}
      onclick={() => fileInput.click()}
      onkeydown={(e) => e.key === 'Enter' && fileInput.click()}
    >
      <div class="icon">📄</div>
      <h3>Drag & Drop CSV File</h3>
      <p>or click to browse</p>
      <input
        type="file"
        accept=".csv"
        bind:this={fileInput}
        onchange={handleFileSelect}
        style="display: none;"
      />
    </div>

    <div class="privacy-note">
      <span class="lock-icon">🔒</span>
      <strong>100% Client-Side</strong> — Roster data is never sent to any server. All processing happens in your browser memory.
    </div>
  {:else}
    <div class="mapping-container">
      <div class="mapping-header">
        <h3>Map Columns</h3>
        <button class="btn-text" onclick={reset}>Choose another file</button>
      </div>

      <p class="subtitle">Found {parsedRows.length} records. Please map your CSV columns to certificate fields.</p>

      <div class="mapping-grid">
        <div class="mapping-row">
          <label for="col-name">Recipient Name <span class="required">*</span></label>
          <select id="col-name" bind:value={mapping.recipientName}>
            <option value="">-- Select Column --</option>
            {#each columns as col}
              <option value={col}>{col}</option>
            {/each}
          </select>
        </div>

        <div class="mapping-row">
          <label for="col-title">Certificate Title</label>
          <select id="col-title" bind:value={mapping.title}>
            <option value="">-- Fixed / Default --</option>
            {#each columns as col}
              <option value={col}>{col}</option>
            {/each}
          </select>
        </div>

        <div class="mapping-row">
          <label for="col-date">Date</label>
          <select id="col-date" bind:value={mapping.date}>
            <option value="">-- Current Date --</option>
            {#each columns as col}
              <option value={col}>{col}</option>
            {/each}
          </select>
        </div>

        <div class="mapping-row">
          <label for="col-id">Certificate ID</label>
          <select id="col-id" bind:value={mapping.certId}>
            <option value="">-- Auto-generate --</option>
            {#each columns as col}
              <option value={col}>{col}</option>
            {/each}
          </select>
        </div>
      </div>

      {#if isGenerating}
        <div class="progress-container">
          <div class="progress-text">Processing {progress} of {total} certificates...</div>
          <div class="progress-bar">
            <div class="progress-fill" style="width: {(progress / total) * 100}%"></div>
          </div>
        </div>
      {:else}
        <button
          class="btn-primary generate-btn"
          onclick={generateBulk}
          disabled={!mapping.recipientName}
        >
          Generate Certificates (.zip)
        </button>
      {/if}
    </div>
  {/if}
</div>

<style>
  .bulk-generator {
    width: 100%;
    max-width: 600px;
    margin: 0 auto;
  }

  .dropzone {
    border: 2px dashed var(--border-color, #ccc);
    border-radius: 8px;
    padding: 40px 20px;
    text-align: center;
    cursor: pointer;
    transition: all 0.2s ease;
    background: var(--bg-secondary, #f9f9f9);
  }

  .dropzone:hover, .dropzone.dragging {
    border-color: var(--accent-primary, #3498db);
    background: rgba(52, 152, 219, 0.05);
  }

  .dropzone .icon {
    font-size: 3rem;
    margin-bottom: 16px;
  }

  .dropzone h3 {
    margin: 0 0 8px 0;
    font-size: 1.2rem;
    color: var(--text-primary, #333);
  }

  .dropzone p {
    margin: 0;
    color: var(--text-secondary, #666);
    font-size: 0.9rem;
  }

  .privacy-note {
    margin-top: 16px;
    padding: 12px 16px;
    background: rgba(46, 204, 113, 0.1);
    color: #27ae60;
    border-radius: 6px;
    font-size: 0.85rem;
    display: flex;
    align-items: center;
    gap: 8px;
    border: 1px solid rgba(46, 204, 113, 0.2);
  }

  .mapping-container {
    background: var(--bg-secondary, #fff);
    border: 1px solid var(--border-color, #eee);
    border-radius: 8px;
    padding: 24px;
    box-shadow: 0 4px 6px rgba(0,0,0,0.05);
  }

  .mapping-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 8px;
  }

  .mapping-header h3 {
    margin: 0;
  }

  .btn-text {
    background: none;
    border: none;
    color: var(--accent-primary, #3498db);
    cursor: pointer;
    font-size: 0.9rem;
  }

  .btn-text:hover {
    text-decoration: underline;
  }

  .subtitle {
    margin: 0 0 24px 0;
    color: var(--text-secondary, #666);
    font-size: 0.9rem;
  }

  .mapping-grid {
    display: flex;
    flex-direction: column;
    gap: 16px;
    margin-bottom: 32px;
  }

  .mapping-row {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .mapping-row label {
    font-size: 0.9rem;
    font-weight: 600;
    color: var(--text-primary, #333);
  }

  .required {
    color: #e74c3c;
  }

  select {
    padding: 8px 12px;
    border: 1px solid var(--border-color, #ddd);
    border-radius: 4px;
    font-size: 1rem;
    background: #fff;
  }

  .generate-btn {
    width: 100%;
    padding: 12px;
    background: var(--accent-primary, #3498db);
    color: white;
    border: none;
    border-radius: 6px;
    font-size: 1rem;
    font-weight: 600;
    cursor: pointer;
    transition: background 0.2s;
  }

  .generate-btn:hover:not(:disabled) {
    background: #2980b9;
  }

  .generate-btn:disabled {
    background: #bdc3c7;
    cursor: not-allowed;
  }

  .progress-container {
    margin-top: 16px;
  }

  .progress-text {
    margin-bottom: 8px;
    font-size: 0.9rem;
    font-weight: 500;
    text-align: center;
  }

  .progress-bar {
    width: 100%;
    height: 12px;
    background: var(--border-color, #eee);
    border-radius: 6px;
    overflow: hidden;
  }

  .progress-fill {
    height: 100%;
    background: var(--accent-primary, #3498db);
    transition: width 0.1s linear;
  }
</style>
