<script lang="ts">
  import { wasmPdfClient } from '../lib/wasm';
  import DropZone from '../components/DropZone.svelte';

  let activeTool = $state('merge');
  let isProcessing = $state(false);

  // Tools configuration
  const tools = [
    { id: 'merge', title: 'Merge PDFs', icon: '📑' },
    { id: 'split', title: 'Split PDF', icon: '✂️' },
    { id: 'rotate', title: 'Rotate Pages', icon: '🔄' },
    { id: 'compress', title: 'Compress PDF', icon: '🗜️' },
    { id: 'encrypt', title: 'Encrypt PDF', icon: '🔒' },
    { id: 'watermark', title: 'Watermark', icon: '©️' },
    { id: 'delete', title: 'Delete Pages', icon: '🗑️' },
    { id: 'extract', title: 'Extract Pages', icon: '📤' },
    { id: 'reorder', title: 'Reorder Pages', icon: '🔀' },
    { id: 'crop', title: 'Crop PDF', icon: '📐' },
    { id: 'flatten', title: 'Flatten Forms', icon: '📄' },
    { id: 'metadata', title: 'Edit Metadata', icon: '🏷️' }
  ];

  // Tool specific states
  let mergeFiles: File[] = $state([]);

  let splitFile: File | null = $state(null);
  let splitRanges = $state('');

  let rotateFile: File | null = $state(null);
  let rotateAngle = $state(90);
  let rotatePages = $state('all');

  let compressFile: File | null = $state(null);

  let encryptFile: File | null = $state(null);
  let encryptPassword = $state('');

  let watermarkFile: File | null = $state(null);
  let watermarkText = $state('');

  let deleteFile: File | null = $state(null);
  let deletePagesStr = $state('');

  let extractFile: File | null = $state(null);
  let extractPagesStr = $state('');

  let reorderFile: File | null = $state(null);
  let reorderPagesStr = $state('');

  let cropFile: File | null = $state(null);
  let cropLeft = $state(0);
  let cropBottom = $state(0);
  let cropRight = $state(0);
  let cropTop = $state(0);

  let flattenFile: File | null = $state(null);

  let metadataFile: File | null = $state(null);
  let metaTitle = $state('');
  let metaAuthor = $state('');
  let metaSubject = $state('');
  let metaKeywords = $state('');


  async function readAsUint8Array(file: File): Promise<Uint8Array> {
      const buffer = await file.arrayBuffer();
      return new Uint8Array(buffer);
  }

  function download(data: Uint8Array, filename: string) {
      const blob = new Blob([data.buffer as ArrayBuffer], { type: 'application/pdf' });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = filename;
      document.body.appendChild(a);
      a.click();
      document.body.removeChild(a);
      URL.revokeObjectURL(url);
  }

  // Handlers
  async function handleMerge() {
      if (mergeFiles.length < 2) return;
      isProcessing = true;
      try {
          const buffers = await Promise.all(mergeFiles.map(f => readAsUint8Array(f)));
          const result = await wasmPdfClient.merge(buffers);
          download(result, 'merged.pdf');
      } catch (e: any) {
          alert('Merge failed: ' + e.message);
      } finally {
          isProcessing = false;
      }
  }

  async function handleSplit() {
      if (!splitFile || !splitRanges) return;
      isProcessing = true;
      try {
          const buffer = await readAsUint8Array(splitFile);
          const results = await wasmPdfClient.split(buffer, splitRanges);
          results.forEach((res: Uint8Array, i: number) => {
              download(res, `split_part_${i+1}.pdf`);
          });
      } catch (e: any) {
          alert('Split failed: ' + e.message);
      } finally {
          isProcessing = false;
      }
  }

  async function handleRotate() {
      if (!rotateFile) return;
      isProcessing = true;
      try {
          const buffer = await readAsUint8Array(rotateFile);
          const result = await wasmPdfClient.rotate(buffer, rotateAngle, rotatePages);
          download(result, 'rotated.pdf');
      } catch (e: any) {
          alert('Rotate failed: ' + e.message);
      } finally {
          isProcessing = false;
      }
  }

  async function handleCompress() {
      if (!compressFile) return;
      isProcessing = true;
      try {
          const buffer = await readAsUint8Array(compressFile);
          const result = await wasmPdfClient.compress(buffer);
          download(result, 'compressed.pdf');
      } catch (e: any) {
          alert('Compress failed: ' + e.message);
      } finally {
          isProcessing = false;
      }
  }

  async function handleEncrypt() {
      if (!encryptFile || !encryptPassword) return;
      isProcessing = true;
      try {
          const buffer = await readAsUint8Array(encryptFile);
          const result = await wasmPdfClient.encrypt(buffer, encryptPassword);
          download(result, 'encrypted.pdf');
      } catch (e: any) {
          alert('Encrypt failed: ' + e.message);
      } finally {
          isProcessing = false;
      }
  }

  async function handleWatermark() {
      if (!watermarkFile || !watermarkText) return;
      isProcessing = true;
      try {
          const buffer = await readAsUint8Array(watermarkFile);
          const result = await wasmPdfClient.watermark(buffer, watermarkText);
          download(result, 'watermarked.pdf');
      } catch (e: any) {
          alert('Watermark failed: ' + e.message);
      } finally {
          isProcessing = false;
      }
  }

  async function handleDelete() {
      if (!deleteFile || !deletePagesStr) return;
      isProcessing = true;
      try {
          const buffer = await readAsUint8Array(deleteFile);
          const result = await wasmPdfClient.delete_pages(buffer, deletePagesStr);
          download(result, 'deleted.pdf');
      } catch (e: any) {
          alert('Delete failed: ' + e.message);
      } finally {
          isProcessing = false;
      }
  }

  async function handleExtract() {
      if (!extractFile || !extractPagesStr) return;
      isProcessing = true;
      try {
          const buffer = await readAsUint8Array(extractFile);
          const result = await wasmPdfClient.extract_pages(buffer, extractPagesStr);
          download(result, 'extracted.pdf');
      } catch (e: any) {
          alert('Extract failed: ' + e.message);
      } finally {
          isProcessing = false;
      }
  }

  async function handleReorder() {
      if (!reorderFile || !reorderPagesStr) return;
      isProcessing = true;
      try {
          const buffer = await readAsUint8Array(reorderFile);
          const orderArr = reorderPagesStr.split(',').map(s => parseInt(s.trim())).filter(n => !isNaN(n));
          const result = await wasmPdfClient.reorder_pages(buffer, orderArr);
          download(result, 'reordered.pdf');
      } catch (e: any) {
          alert('Reorder failed: ' + e.message);
      } finally {
          isProcessing = false;
      }
  }

  async function handleCrop() {
      if (!cropFile) return;
      isProcessing = true;
      try {
          const buffer = await readAsUint8Array(cropFile);
          const result = await wasmPdfClient.crop(buffer, cropLeft, cropBottom, cropRight, cropTop);
          download(result, 'cropped.pdf');
      } catch (e: any) {
          alert('Crop failed: ' + e.message);
      } finally {
          isProcessing = false;
      }
  }

  async function handleFlatten() {
      if (!flattenFile) return;
      isProcessing = true;
      try {
          const buffer = await readAsUint8Array(flattenFile);
          const result = await wasmPdfClient.flatten(buffer);
          download(result, 'flattened.pdf');
      } catch (e: any) {
          alert('Flatten failed: ' + e.message);
      } finally {
          isProcessing = false;
      }
  }

  async function handleMetadata() {
      if (!metadataFile) return;
      isProcessing = true;
      try {
          const buffer = await readAsUint8Array(metadataFile);
          const result = await wasmPdfClient.set_metadata(
            buffer,
            metaTitle || undefined,
            metaAuthor || undefined,
            metaSubject || undefined,
            metaKeywords || undefined
          );
          download(result, 'metadata_updated.pdf');
      } catch (e: any) {
          alert('Metadata failed: ' + e.message);
      } finally {
          isProcessing = false;
      }
  }

  // Helpers for merge files
  function removeMergeFile(index: number) {
      mergeFiles = mergeFiles.filter((_, i) => i !== index);
  }

  function moveMergeFileUp(index: number) {
      if (index > 0) {
          const temp = mergeFiles[index];
          mergeFiles[index] = mergeFiles[index - 1];
          mergeFiles[index - 1] = temp;
      }
  }

  function moveMergeFileDown(index: number) {
      if (index < mergeFiles.length - 1) {
          const temp = mergeFiles[index];
          mergeFiles[index] = mergeFiles[index + 1];
          mergeFiles[index + 1] = temp;
      }
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

        <DropZone ondrop={(files: File[]) => mergeFiles = [...mergeFiles, ...files]} multiple={true} />

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
            <DropZone ondrop={(files: File[]) => splitFile = files[0]} multiple={false} />
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
            <DropZone ondrop={(files: File[]) => rotateFile = files[0]} multiple={false} />
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
            <DropZone ondrop={(files: File[]) => compressFile = files[0]} multiple={false} />
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
            <DropZone ondrop={(files: File[]) => encryptFile = files[0]} multiple={false} />
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
            <DropZone ondrop={(files: File[]) => watermarkFile = files[0]} multiple={false} />
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

    {:else if activeTool === 'delete'}
      <div class="tool-pane">
        <h2>Delete Pages</h2>
        <p>Remove specific pages from your document.</p>

        {#if !deleteFile}
            <DropZone ondrop={(files: File[]) => deleteFile = files[0]} multiple={false} />
        {:else}
            <div class="selected-file">
                <span class="file-name">{deleteFile.name}</span>
                <button onclick={() => deleteFile = null}>✕</button>
            </div>

            <div class="input-group">
                <label for="deletePagesStr">Pages to Delete (e.g. 2, 4-6)</label>
                <input id="deletePagesStr" type="text" bind:value={deletePagesStr} placeholder="2, 4-6" />
            </div>

            <button class="action-btn" onclick={handleDelete} disabled={isProcessing || !deletePagesStr}>
                {isProcessing ? 'Processing...' : 'Delete Pages & Download'}
            </button>
        {/if}
      </div>

    {:else if activeTool === 'extract'}
      <div class="tool-pane">
        <h2>Extract Pages</h2>
        <p>Extract specific pages to a new document.</p>

        {#if !extractFile}
            <DropZone ondrop={(files: File[]) => extractFile = files[0]} multiple={false} />
        {:else}
            <div class="selected-file">
                <span class="file-name">{extractFile.name}</span>
                <button onclick={() => extractFile = null}>✕</button>
            </div>

            <div class="input-group">
                <label for="extractPagesStr">Target Pages (e.g. 1-3)</label>
                <input id="extractPagesStr" type="text" bind:value={extractPagesStr} placeholder="1-3" />
            </div>

            <button class="action-btn" onclick={handleExtract} disabled={isProcessing || !extractPagesStr}>
                {isProcessing ? 'Processing...' : 'Extract Pages & Download'}
            </button>
        {/if}
      </div>

    {:else if activeTool === 'reorder'}
      <div class="tool-pane">
        <h2>Reorder Pages</h2>
        <p>Rearrange the pages in your document.</p>

        {#if !reorderFile}
            <DropZone ondrop={(files: File[]) => reorderFile = files[0]} multiple={false} />
        {:else}
            <div class="selected-file">
                <span class="file-name">{reorderFile.name}</span>
                <button onclick={() => reorderFile = null}>✕</button>
            </div>

            <div class="input-group">
                <label for="reorderPagesStr">New Order (comma separated, e.g. 3, 1, 2)</label>
                <input id="reorderPagesStr" type="text" bind:value={reorderPagesStr} placeholder="3, 1, 2" />
            </div>

            <button class="action-btn" onclick={handleReorder} disabled={isProcessing || !reorderPagesStr}>
                {isProcessing ? 'Processing...' : 'Reorder & Download'}
            </button>
        {/if}
      </div>

    {:else if activeTool === 'crop'}
      <div class="tool-pane">
        <h2>Crop PDF</h2>
        <p>Crop the pages in your document.</p>

        {#if !cropFile}
            <DropZone ondrop={(files: File[]) => cropFile = files[0]} multiple={false} />
        {:else}
            <div class="selected-file">
                <span class="file-name">{cropFile.name}</span>
                <button onclick={() => cropFile = null}>✕</button>
            </div>

            <div class="input-group">
                <label for="cropLeft">Left</label>
                <input id="cropLeft" type="number" bind:value={cropLeft} />
            </div>
            <div class="input-group">
                <label for="cropBottom">Bottom</label>
                <input id="cropBottom" type="number" bind:value={cropBottom} />
            </div>
            <div class="input-group">
                <label for="cropRight">Right</label>
                <input id="cropRight" type="number" bind:value={cropRight} />
            </div>
            <div class="input-group">
                <label for="cropTop">Top</label>
                <input id="cropTop" type="number" bind:value={cropTop} />
            </div>

            <button class="action-btn" onclick={handleCrop} disabled={isProcessing}>
                {isProcessing ? 'Processing...' : 'Crop & Download'}
            </button>
        {/if}
      </div>

    {:else if activeTool === 'flatten'}
      <div class="tool-pane">
        <h2>Flatten Forms</h2>
        <p>Remove interactive form elements and flatten them onto the pages.</p>

        {#if !flattenFile}
            <DropZone ondrop={(files: File[]) => flattenFile = files[0]} multiple={false} />
        {:else}
            <div class="selected-file">
                <span class="file-name">{flattenFile.name}</span>
                <button onclick={() => flattenFile = null}>✕</button>
            </div>

            <button class="action-btn" onclick={handleFlatten} disabled={isProcessing}>
                {isProcessing ? 'Processing...' : 'Flatten & Download'}
            </button>
        {/if}
      </div>

    {:else if activeTool === 'metadata'}
      <div class="tool-pane">
        <h2>Edit Metadata</h2>
        <p>Update the metadata of your document.</p>

        {#if !metadataFile}
            <DropZone ondrop={(files: File[]) => metadataFile = files[0]} multiple={false} />
        {:else}
            <div class="selected-file">
                <span class="file-name">{metadataFile.name}</span>
                <button onclick={() => metadataFile = null}>✕</button>
            </div>

            <div class="input-group">
                <label for="metaTitle">Title</label>
                <input id="metaTitle" type="text" bind:value={metaTitle} />
            </div>
            <div class="input-group">
                <label for="metaAuthor">Author</label>
                <input id="metaAuthor" type="text" bind:value={metaAuthor} />
            </div>
            <div class="input-group">
                <label for="metaSubject">Subject</label>
                <input id="metaSubject" type="text" bind:value={metaSubject} />
            </div>
            <div class="input-group">
                <label for="metaKeywords">Keywords</label>
                <input id="metaKeywords" type="text" bind:value={metaKeywords} />
            </div>

            <button class="action-btn" onclick={handleMetadata} disabled={isProcessing}>
                {isProcessing ? 'Processing...' : 'Update Metadata & Download'}
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
    max-height: 80vh;
    overflow-y: auto;
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
