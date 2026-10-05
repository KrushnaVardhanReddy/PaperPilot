<script lang="ts">
  import { wasmPdfClient } from '../lib/wasm/index.js';
  import DropZone from '../components/DropZone.svelte';

  let activeTool = $state('merge');
  let isProcessing = $state(false);

  // Merge state
  let mergeFiles: File[] = $state([]);

  // Split state
  let splitFile: File | null = $state(null);
  let splitRanges = $state('1');

  // Rotate state
  let rotateFile: File | null = $state(null);
  let rotateAngle = $state(90);
  let rotatePages = $state('all');

  // Compress state
  let compressFile: File | null = $state(null);

  // Encrypt state
  let encryptFile: File | null = $state(null);
  let encryptPassword = $state('');

  // Watermark state
  let watermarkFile: File | null = $state(null);
  let watermarkText = $state('CONFIDENTIAL');

  const tools = [
    { id: 'merge', title: 'Merge PDFs', icon: '📑' },
    { id: 'split', title: 'Split PDF', icon: '✂️' },
    { id: 'rotate', title: 'Rotate Pages', icon: '🔄' },
    { id: 'compress', title: 'Compress PDF', icon: '🗜️' },
    { id: 'encrypt', title: 'Encrypt PDF', icon: '🔒' },
    { id: 'watermark', title: 'Watermark', icon: '©️' }
  ];

  async function fileToUint8Array(file: File): Promise<Uint8Array> {
    const buffer = await file.arrayBuffer();
    return new Uint8Array(buffer);
  }

  function downloadBlob(data: Uint8Array, filename: string) {
    const blob = new Blob([data as any], { type: 'application/pdf' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = filename;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
  }

  async function handleMerge() {
    if (mergeFiles.length < 2) {
      alert("Please add at least 2 files to merge.");
      return;
    }
    isProcessing = true;
    try {
      const buffers = await Promise.all(mergeFiles.map(fileToUint8Array));
      const result = await wasmPdfClient.merge(buffers);
      downloadBlob(result, 'merged_document.pdf');
    } catch (e: any) {
      alert(`Error: ${e.message || e}`);
    } finally {
      isProcessing = false;
    }
  }

  async function handleSplit() {
    if (!splitFile) return;
    if (!splitRanges.trim()) {
      alert("Please enter split ranges.");
      return;
    }
    isProcessing = true;
    try {
      const buffer = await fileToUint8Array(splitFile);
      const results = await wasmPdfClient.split(buffer, splitRanges);
      results.forEach((res, i) => {
          downloadBlob(res, `split_part_${i+1}.pdf`);
      });
    } catch (e: any) {
      alert(`Error: ${e.message || e}`);
    } finally {
      isProcessing = false;
    }
  }

  async function handleRotate() {
    if (!rotateFile) return;
    isProcessing = true;
    try {
      const buffer = await fileToUint8Array(rotateFile);
      const result = await wasmPdfClient.rotate(buffer, rotateAngle, rotatePages || 'all');
      downloadBlob(result, `rotated_${rotateFile.name}`);
    } catch (e: any) {
      alert(`Error: ${e.message || e}`);
    } finally {
      isProcessing = false;
    }
  }

  async function handleCompress() {
    if (!compressFile) return;
    isProcessing = true;
    try {
      const buffer = await fileToUint8Array(compressFile);
      const result = await wasmPdfClient.compress(buffer);
      const reduction = Math.round(((buffer.length - result.length) / buffer.length) * 100);
      alert(`Compression complete. Size reduced by ${reduction}%`);
      downloadBlob(result, `compressed_${compressFile.name}`);
    } catch (e: any) {
      alert(`Error: ${e.message || e}`);
    } finally {
      isProcessing = false;
    }
  }

  async function handleEncrypt() {
    if (!encryptFile) return;
    if (!encryptPassword) {
      alert("Please enter a password.");
      return;
    }
    isProcessing = true;
    try {
      const buffer = await fileToUint8Array(encryptFile);
      const result = await wasmPdfClient.encrypt(buffer, encryptPassword);
      downloadBlob(result, `encrypted_${encryptFile.name}`);
    } catch (e: any) {
      alert(`Error: ${e.message || e}`);
    } finally {
      isProcessing = false;
    }
  }

  async function handleWatermark() {
    if (!watermarkFile) return;
    if (!watermarkText) {
      alert("Please enter watermark text.");
      return;
    }
    isProcessing = true;
    try {
      const buffer = await fileToUint8Array(watermarkFile);
      const result = await wasmPdfClient.watermark(buffer, watermarkText);
      downloadBlob(result, `watermarked_${watermarkFile.name}`);
    } catch (e: any) {
      alert(`Error: ${e.message || e}`);
    } finally {
      isProcessing = false;
    }
  }

  function moveMergeFileUp(index: number) {
      if (index > 0) {
          const newArr = [...mergeFiles];
          [newArr[index - 1], newArr[index]] = [newArr[index], newArr[index - 1]];
          mergeFiles = newArr;
      }
  }

  function moveMergeFileDown(index: number) {
      if (index < mergeFiles.length - 1) {
          const newArr = [...mergeFiles];
          [newArr[index + 1], newArr[index]] = [newArr[index], newArr[index + 1]];
          mergeFiles = newArr;
      }
  }

  function removeMergeFile(index: number) {
      mergeFiles = mergeFiles.filter((_, i) => i !== index);
  }
</script>

<div class="operations-view">
  <div class="tools-sidebar">
    <h3>Tools</h3>
    <div class="tools-list">
      {#each tools as tool}
        <button
          class="tool-btn"
          class:active={activeTool === tool.id}
          onclick={() => activeTool = tool.id}
        >
          <span class="tool-icon">{tool.icon}</span>
          {tool.title}
        </button>
      {/each}
    </div>
  </div>

  <div class="tool-content">
    {#if activeTool === 'merge'}
      <div class="tool-pane">
        <h2>Merge PDFs</h2>
        <p>Combine multiple PDF files into one.</p>

        <DropZone ondrop={(files) => mergeFiles = [...mergeFiles, ...files]} multiple={true} />

        {#if mergeFiles.length > 0}
          <div class="file-list">
            {#each mergeFiles as file, i}
              <div class="file-item">
                <span class="file-name">{file.name}</span>
                <div class="file-controls">
                    <button onclick={() => moveMergeFileUp(i)} disabled={i === 0}>↑</button>
                    <button onclick={() => moveMergeFileDown(i)} disabled={i === mergeFiles.length - 1}>↓</button>
                    <button onclick={() => removeMergeFile(i)}>✕</button>
                </div>
              </div>
            {/each}
          </div>
          <button class="action-btn" onclick={handleMerge} disabled={isProcessing || mergeFiles.length < 2}>
            {isProcessing ? 'Processing...' : 'Merge & Download'}
          </button>
        {/if}
      </div>

    {:else if activeTool === 'split'}
      <div class="tool-pane">
        <h2>Split PDF</h2>
        <p>Extract pages from a PDF.</p>

        {#if !splitFile}
            <DropZone ondrop={(files) => splitFile = files[0]} multiple={false} />
        {:else}
            <div class="selected-file">
                <span class="file-name">{splitFile.name}</span>
                <button onclick={() => splitFile = null}>✕</button>
            </div>

            <div class="input-group">
                <label for="splitRanges">Page Ranges (e.g. 1-2, 4)</label>
                <input id="splitRanges" type="text" bind:value={splitRanges} placeholder="1-2, 4" />
            </div>

            <button class="action-btn" onclick={handleSplit} disabled={isProcessing}>
                {isProcessing ? 'Processing...' : 'Split & Download'}
            </button>
        {/if}
      </div>

    {:else if activeTool === 'rotate'}
      <div class="tool-pane">
        <h2>Rotate Pages</h2>
        <p>Rotate pages by 90, 180, or 270 degrees.</p>

        {#if !rotateFile}
            <DropZone ondrop={(files) => rotateFile = files[0]} multiple={false} />
        {:else}
            <div class="selected-file">
                <span class="file-name">{rotateFile.name}</span>
                <button onclick={() => rotateFile = null}>✕</button>
            </div>

            <div class="input-group">
                <label for="rotateAngle">Angle</label>
                <select id="rotateAngle" bind:value={rotateAngle}>
                    <option value={90}>90° Clockwise</option>
                    <option value={180}>180°</option>
                    <option value={270}>270° Clockwise (90° CCW)</option>
                </select>
            </div>

            <div class="input-group">
                <label for="rotatePages">Pages (e.g. all, 1, 2-5)</label>
                <input id="rotatePages" type="text" bind:value={rotatePages} placeholder="all" />
            </div>

            <button class="action-btn" onclick={handleRotate} disabled={isProcessing}>
                {isProcessing ? 'Processing...' : 'Rotate & Download'}
            </button>
        {/if}
      </div>

    {:else if activeTool === 'compress'}
      <div class="tool-pane">
        <h2>Compress PDF</h2>
        <p>Reduce file size.</p>

        {#if !compressFile}
            <DropZone ondrop={(files) => compressFile = files[0]} multiple={false} />
        {:else}
            <div class="selected-file">
                <span class="file-name">{compressFile.name}</span>
                <button onclick={() => compressFile = null}>✕</button>
            </div>

            <button class="action-btn" onclick={handleCompress} disabled={isProcessing}>
                {isProcessing ? 'Processing...' : 'Compress & Download'}
            </button>
        {/if}
      </div>

    {:else if activeTool === 'encrypt'}
      <div class="tool-pane">
        <h2>Encrypt PDF</h2>
        <p>Add a password to protect your document.</p>

        {#if !encryptFile}
            <DropZone ondrop={(files) => encryptFile = files[0]} multiple={false} />
        {:else}
            <div class="selected-file">
                <span class="file-name">{encryptFile.name}</span>
                <button onclick={() => encryptFile = null}>✕</button>
            </div>

            <div class="input-group">
                <label for="encryptPassword">Password</label>
                <input id="encryptPassword" type="password" bind:value={encryptPassword} />
            </div>

            <button class="action-btn" onclick={handleEncrypt} disabled={isProcessing || !encryptPassword}>
                {isProcessing ? 'Processing...' : 'Encrypt & Download'}
            </button>
        {/if}
      </div>

    {:else if activeTool === 'watermark'}
      <div class="tool-pane">
        <h2>Watermark PDF</h2>
        <p>Add a text watermark to your document.</p>

        {#if !watermarkFile}
            <DropZone ondrop={(files) => watermarkFile = files[0]} multiple={false} />
        {:else}
            <div class="selected-file">
                <span class="file-name">{watermarkFile.name}</span>
                <button onclick={() => watermarkFile = null}>✕</button>
            </div>

            <div class="input-group">
                <label for="watermarkText">Watermark Text</label>
                <input id="watermarkText" type="text" bind:value={watermarkText} />
            </div>

            <button class="action-btn" onclick={handleWatermark} disabled={isProcessing || !watermarkText}>
                {isProcessing ? 'Processing...' : 'Watermark & Download'}
            </button>
        {/if}
      </div>
    {/if}
  </div>
</div>

<style>
  .operations-view {
    display: flex;
    gap: 32px;
    height: 100%;
    max-width: 1200px;
    margin: 0 auto;
  }

  .tools-sidebar {
    width: 240px;
    flex-shrink: 0;
    background-color: var(--bg-secondary);
    border-radius: var(--border-radius-lg);
    border: 1px solid var(--border-color);
    padding: 16px;
  }

  .tools-sidebar h3 {
    margin-top: 0;
    margin-bottom: 16px;
    color: var(--text-secondary);
    font-size: 0.9rem;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .tools-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .tool-btn {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 12px;
    background: transparent;
    border: 1px solid transparent;
    border-radius: var(--border-radius-md);
    color: var(--text-primary);
    cursor: pointer;
    transition: all var(--transition-fast);
    text-align: left;
    font-size: 0.95rem;
  }

  .tool-btn:hover {
    background-color: var(--bg-surface-hover);
  }

  .tool-btn.active {
    background-color: rgba(79, 70, 229, 0.1);
    color: var(--accent-primary);
    border-color: rgba(79, 70, 229, 0.2);
  }

  .tool-icon {
    font-size: 1.2rem;
  }

  .tool-content {
    flex: 1;
    background-color: var(--bg-secondary);
    border-radius: var(--border-radius-lg);
    border: 1px solid var(--border-color);
    padding: 32px;
    overflow-y: auto;
  }

  .tool-pane h2 {
    margin-top: 0;
    margin-bottom: 8px;
  }

  .tool-pane p {
    color: var(--text-secondary);
    margin-bottom: 24px;
  }

  .file-list {
    margin-top: 24px;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .file-item, .selected-file {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px 16px;
    background-color: var(--bg-surface);
    border: 1px solid var(--border-color);
    border-radius: var(--border-radius-md);
  }

  .file-name {
    font-weight: 500;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .file-controls {
    display: flex;
    gap: 8px;
  }

  .file-controls button, .selected-file button {
    background: transparent;
    border: 1px solid var(--border-color);
    border-radius: 4px;
    padding: 4px 8px;
    color: var(--text-secondary);
    cursor: pointer;
  }

  .file-controls button:hover:not(:disabled), .selected-file button:hover {
      background: var(--bg-surface-hover);
      color: var(--text-primary);
  }

  .file-controls button:disabled {
      opacity: 0.3;
      cursor: not-allowed;
  }

  .action-btn {
    margin-top: 24px;
    width: 100%;
    padding: 12px;
    background-color: var(--accent-primary);
    color: white;
    border: none;
    border-radius: var(--border-radius-md);
    font-size: 1rem;
    font-weight: 600;
    cursor: pointer;
    transition: background-color var(--transition-fast);
  }

  .action-btn:hover:not(:disabled) {
    background-color: var(--accent-hover);
  }

  .action-btn:disabled {
    background-color: var(--bg-surface-hover);
    color: var(--text-muted);
    cursor: not-allowed;
  }

  .input-group {
      margin-top: 16px;
      display: flex;
      flex-direction: column;
      gap: 8px;
  }

  .input-group label {
      font-size: 0.9rem;
      font-weight: 500;
      color: var(--text-secondary);
  }

  .input-group input, .input-group select {
      padding: 10px 12px;
      background: var(--bg-surface);
      border: 1px solid var(--border-color);
      border-radius: var(--border-radius-md);
      color: var(--text-primary);
      font-size: 1rem;
  }

  @media (max-width: 768px) {
    .operations-view {
      flex-direction: column;
    }
    .tools-sidebar {
      width: 100%;
    }
    .tools-list {
      flex-direction: row;
      flex-wrap: wrap;
    }
    .tool-btn {
      flex: 1;
      min-width: 140px;
    }
  }
</style>
