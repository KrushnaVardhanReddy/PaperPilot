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

  // Tool specific states (batch array support)
  let mergeFiles: File[] = $state([]);
  let splitFiles: File[] = $state([]);
  let splitRanges = $state('');

  let rotateFiles: File[] = $state([]);
  let rotateAngle = $state(90);
  let rotatePages = $state('all');

  let compressFiles: File[] = $state([]);

  let encryptFiles: File[] = $state([]);
  let encryptPassword = $state('');

  let watermarkFiles: File[] = $state([]);
  let watermarkText = $state('');

  let deleteFiles: File[] = $state([]);
  let deletePagesStr = $state('');

  let extractFiles: File[] = $state([]);
  let extractPagesStr = $state('');

  let reorderFiles: File[] = $state([]);
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

  let imagesToPdfFiles: File[] = $state([]);
  let extractImagesFile: File | null = $state(null);
  let pdfHashFile: File | null = $state(null);
  let computedPdfHash: string | null = $state(null);

  let renderFile: File | null = $state(null);
  let renderPageIndex: number = $state(0);
  let renderScale: number = $state(2.0);
  let renderOutputUrl: string | null = $state(null);

  let extractTextFile: File | null = $state(null);
  let extractedText: string = $state("");

  let decryptFile: File | null = $state(null);
  let decryptPassword: string = $state("");

  let pageNumbersFile: File | null = $state(null);
  let pageNumbersFormat: string = $state("Page {n} of {total}");
  let pageNumbersPosition: string = $state("bottom-center");

  let headerFooterFile: File | null = $state(null);
  let headerText: string = $state("");
  let footerText: string = $state("");

  let pdfInfoFile: File | null = $state(null);
  let pdfInfoData: string = $state("");

  let ocrFile: File | null = $state(null);
  let ocrData: string = $state("");


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
      if (splitFiles.length === 0 || !splitRanges) return;
      isProcessing = true;
      try {
          for (let fIdx = 0; fIdx < splitFiles.length; fIdx++) {
              const file = splitFiles[fIdx];
              const buffer = await readAsUint8Array(file);
              const results = await wasmPdfClient.split(buffer, splitRanges);
              results.forEach((res: Uint8Array, i: number) => {
                  setTimeout(() => {
                      download(res, `split_part_${i+1}_${file.name}`);
                  }, (fIdx * results.length + i) * 200);
              });
          }
      } catch (e: any) {
          alert('Split failed: ' + e.message);
      } finally {
          isProcessing = false;
      }
  }

  async function handleRotate() {
      if (rotateFiles.length === 0) return;
      isProcessing = true;
      try {
          for (let i = 0; i < rotateFiles.length; i++) {
              const file = rotateFiles[i];
              const buffer = await readAsUint8Array(file);
              const result = await wasmPdfClient.rotate(buffer, rotateAngle, rotatePages);
              setTimeout(() => {
                  download(result, `rotated_${file.name}`);
              }, i * 250);
          }
      } catch (e: any) {
          alert('Rotate failed: ' + e.message);
      } finally {
          isProcessing = false;
      }
  }

  async function handleCompress() {
      if (compressFiles.length === 0) return;
      isProcessing = true;
      try {
          for (let i = 0; i < compressFiles.length; i++) {
              const file = compressFiles[i];
              const buffer = await readAsUint8Array(file);
              const result = await wasmPdfClient.compress(buffer);
              setTimeout(() => {
                  download(result, `compressed_${file.name}`);
              }, i * 250);
          }
      } catch (e: any) {
          alert('Compress failed: ' + e.message);
      } finally {
          isProcessing = false;
      }
  }

  async function handleEncrypt() {
      if (encryptFiles.length === 0 || !encryptPassword) return;
      isProcessing = true;
      try {
          for (let i = 0; i < encryptFiles.length; i++) {
              const file = encryptFiles[i];
              const buffer = await readAsUint8Array(file);
              const result = await wasmPdfClient.encrypt(buffer, encryptPassword);
              setTimeout(() => {
                  download(result, `encrypted_${file.name}`);
              }, i * 250);
          }
      } catch (e: any) {
          alert('Encrypt failed: ' + e.message);
      } finally {
          isProcessing = false;
      }
  }

  async function handleWatermark() {
      if (watermarkFiles.length === 0 || !watermarkText) return;
      isProcessing = true;
      try {
          for (let i = 0; i < watermarkFiles.length; i++) {
              const file = watermarkFiles[i];
              const buffer = await readAsUint8Array(file);
              const result = await wasmPdfClient.watermark(buffer, watermarkText);
              setTimeout(() => {
                  download(result, `watermarked_${file.name}`);
              }, i * 250);
          }
      } catch (e: any) {
          alert('Watermark failed: ' + e.message);
      } finally {
          isProcessing = false;
      }
  }

  async function handleDelete() {
      if (deleteFiles.length === 0 || !deletePagesStr) return;
      isProcessing = true;
      try {
          for (let i = 0; i < deleteFiles.length; i++) {
              const file = deleteFiles[i];
              const buffer = await readAsUint8Array(file);
              const result = await wasmPdfClient.delete_pages(buffer, deletePagesStr);
              setTimeout(() => {
                  download(result, `deleted_${file.name}`);
              }, i * 250);
          }
      } catch (e: any) {
          alert('Delete failed: ' + e.message);
      } finally {
          isProcessing = false;
      }
  }

  async function handleExtract() {
      if (extractFiles.length === 0 || !extractPagesStr) return;
      isProcessing = true;
      try {
          for (let i = 0; i < extractFiles.length; i++) {
              const file = extractFiles[i];
              const buffer = await readAsUint8Array(file);
              const result = await wasmPdfClient.extract_pages(buffer, extractPagesStr);
              setTimeout(() => {
                  download(result, `extracted_${file.name}`);
              }, i * 250);
          }
      } catch (e: any) {
          alert('Extract failed: ' + e.message);
      } finally {
          isProcessing = false;
      }
  }

  async function handleReorder() {
      if (reorderFiles.length === 0 || !reorderPagesStr) return;
      isProcessing = true;
      try {
          const orderArr = reorderPagesStr.split(',').map(s => parseInt(s.trim())).filter(n => !isNaN(n));
          for (let i = 0; i < reorderFiles.length; i++) {
              const file = reorderFiles[i];
              const buffer = await readAsUint8Array(file);
              const result = await wasmPdfClient.reorder_pages(buffer, orderArr);
              setTimeout(() => {
                  download(result, `reordered_${file.name}`);
              }, i * 250);
          }
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
    try {
      isProcessing = true;
      const buffer = new Uint8Array(await metadataFile.arrayBuffer());
      const result = await wasmPdfClient.set_metadata(buffer, metaTitle, metaAuthor, metaSubject, metaKeywords);
      download(result, `metadata_${metadataFile.name}`);
    } catch (e) {
      alert(`Error updating metadata: ${e}`);
    } finally {
      isProcessing = false;
    }
  }

  async function handleImagesToPdf() {
    if (imagesToPdfFiles.length === 0) return;
    try {
      isProcessing = true;
      const buffers = await Promise.all(imagesToPdfFiles.map(async (f) => new Uint8Array(await f.arrayBuffer())));
      const result = await wasmPdfClient.images_to_pdf(buffers);
      download(result, `images_to_pdf.pdf`);
    } catch (e) {
      alert(`Error converting images: ${e}`);
    } finally {
      isProcessing = false;
    }
  }

  async function handleExtractImages() {
    if (!extractImagesFile) return;
    try {
      isProcessing = true;
      const buffer = new Uint8Array(await extractImagesFile.arrayBuffer());
      const result = await wasmPdfClient.extract_images(buffer);
      for (let i = 0; i < result.length; i++) {
        const ext = (result[i][0] === 0xFF && result[i][1] === 0xD8) ? 'jpg' : 'png';
        download(result[i], `image_${i + 1}.${ext}`);
      }
    } catch (e) {
      alert(`Error extracting images: ${e}`);
    } finally {
      isProcessing = false;
    }
  }


  async function handleRenderPage() {
    if (!renderFile) return;
    isProcessing = true;
    try {
      const bytes = new Uint8Array(await renderFile.arrayBuffer());
      const resBytes = await wasmPdfClient.render_page(bytes, renderPageIndex, renderScale);
      const blob = new Blob([resBytes as any], { type: 'image/png' });
      renderOutputUrl = URL.createObjectURL(blob);
    } catch (e) {
      alert("Error: " + e);
    }
    isProcessing = false;
  }

  async function handleExtractText() {
    if (!extractTextFile) return;
    isProcessing = true;
    try {
      const bytes = new Uint8Array(await extractTextFile.arrayBuffer());
      extractedText = await wasmPdfClient.extract_text(bytes);
    } catch (e) {
      alert("Error: " + e);
    }
    isProcessing = false;
  }

  async function handleDecrypt() {
    if (!decryptFile) return;
    isProcessing = true;
    try {
      const bytes = new Uint8Array(await decryptFile.arrayBuffer());
      const resBytes = await wasmPdfClient.decrypt(bytes, decryptPassword);
      download(resBytes, `decrypted_${decryptFile.name}`);
    } catch (e) {
      alert("Error: " + e);
    }
    isProcessing = false;
  }

  async function handlePageNumbers() {
    if (!pageNumbersFile) return;
    isProcessing = true;
    try {
      const bytes = new Uint8Array(await pageNumbersFile.arrayBuffer());
      const resBytes = await wasmPdfClient.page_numbers(bytes, pageNumbersFormat, pageNumbersPosition);
      download(resBytes, `numbered_${pageNumbersFile.name}`);
    } catch (e) {
      alert("Error: " + e);
    }
    isProcessing = false;
  }

  async function handleHeaderFooter() {
    if (!headerFooterFile) return;
    isProcessing = true;
    try {
      const bytes = new Uint8Array(await headerFooterFile.arrayBuffer());
      const resBytes = await wasmPdfClient.header_footer(bytes, headerText, footerText);
      download(resBytes, `header_footer_${headerFooterFile.name}`);
    } catch (e) {
      alert("Error: " + e);
    }
    isProcessing = false;
  }

  async function handlePdfInfo() {
    if (!pdfInfoFile) return;
    isProcessing = true;
    try {
      const bytes = new Uint8Array(await pdfInfoFile.arrayBuffer());
      const jsonStr = await wasmPdfClient.pdf_info(bytes);
      pdfInfoData = JSON.stringify(JSON.parse(jsonStr), null, 2);
    } catch (e) {
      alert("Error: " + e);
    }
    isProcessing = false;
  }

  async function handleOcr() {
    if (!ocrFile) return;
    isProcessing = true;
    try {
      const bytes = new Uint8Array(await ocrFile.arrayBuffer());
      ocrData = await wasmPdfClient.ocr(bytes);
    } catch (e) {
      alert("Error: " + e);
    }
    isProcessing = false;
  }

  async function handlePdfHash() {
    if (!pdfHashFile) return;
    try {
      isProcessing = true;
      const buffer = new Uint8Array(await pdfHashFile.arrayBuffer());
      computedPdfHash = await wasmPdfClient.pdf_hash(buffer);
    } catch (e) {
      alert(`Error hashing pdf: ${e}`);
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
        <p>Extract pages from one or more PDFs.</p>

        <DropZone ondrop={(files: File[]) => splitFiles = [...splitFiles, ...files]} multiple={true} />

        {#if splitFiles.length > 0}
            <div class="file-list">
              {#each splitFiles as file, i}
                <div class="file-item">
                  <span class="file-name">{file.name}</span>
                  <div class="file-controls">
                    <button onclick={() => splitFiles = splitFiles.filter((_, idx) => idx !== i)}>✕</button>
                  </div>
                </div>
              {/each}
            </div>

            <div class="input-group">
                <label for="splitRanges">Page Ranges (e.g. 1-2, 4)</label>
                <input id="splitRanges" type="text" bind:value={splitRanges} placeholder="1-2, 4" />
            </div>

            <button class="action-btn" onclick={handleSplit} disabled={isProcessing}>
                {isProcessing ? 'Processing...' : `Split & Download (${splitFiles.length} file${splitFiles.length > 1 ? 's' : ''})`}
            </button>
        {/if}
      </div>

    {:else if activeTool === 'rotate'}
      <div class="tool-pane">
        <h2>Rotate Pages</h2>
        <p>Rotate pages in one or more PDFs by 90, 180, or 270 degrees.</p>

        <DropZone ondrop={(files: File[]) => rotateFiles = [...rotateFiles, ...files]} multiple={true} />

        {#if rotateFiles.length > 0}
            <div class="file-list">
              {#each rotateFiles as file, i}
                <div class="file-item">
                  <span class="file-name">{file.name}</span>
                  <div class="file-controls">
                    <button onclick={() => rotateFiles = rotateFiles.filter((_, idx) => idx !== i)}>✕</button>
                  </div>
                </div>
              {/each}
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
                {isProcessing ? 'Processing...' : `Rotate & Download (${rotateFiles.length} file${rotateFiles.length > 1 ? 's' : ''})`}
            </button>
        {/if}
      </div>

    {:else if activeTool === 'compress'}
      <div class="tool-pane">
        <h2>Compress PDF</h2>
        <p>Reduce file size across one or multiple documents.</p>

        <DropZone ondrop={(files: File[]) => compressFiles = [...compressFiles, ...files]} multiple={true} />

        {#if compressFiles.length > 0}
            <div class="file-list">
              {#each compressFiles as file, i}
                <div class="file-item">
                  <span class="file-name">{file.name}</span>
                  <div class="file-controls">
                    <button onclick={() => compressFiles = compressFiles.filter((_, idx) => idx !== i)}>✕</button>
                  </div>
                </div>
              {/each}
            </div>

            <button class="action-btn" onclick={handleCompress} disabled={isProcessing}>
                {isProcessing ? 'Processing...' : `Compress & Download (${compressFiles.length} file${compressFiles.length > 1 ? 's' : ''})`}
            </button>
        {/if}
      </div>

    {:else if activeTool === 'encrypt'}
      <div class="tool-pane">
        <h2>Encrypt PDF</h2>
        <p>Add a password to protect your document(s).</p>

        <DropZone ondrop={(files: File[]) => encryptFiles = [...encryptFiles, ...files]} multiple={true} />

        {#if encryptFiles.length > 0}
            <div class="file-list">
              {#each encryptFiles as file, i}
                <div class="file-item">
                  <span class="file-name">{file.name}</span>
                  <div class="file-controls">
                    <button onclick={() => encryptFiles = encryptFiles.filter((_, idx) => idx !== i)}>✕</button>
                  </div>
                </div>
              {/each}
            </div>

            <div class="input-group">
                <label for="encryptPassword">Password</label>
                <input id="encryptPassword" type="password" bind:value={encryptPassword} />
            </div>

            <button class="action-btn" onclick={handleEncrypt} disabled={isProcessing || !encryptPassword}>
                {isProcessing ? 'Processing...' : `Encrypt & Download (${encryptFiles.length} file${encryptFiles.length > 1 ? 's' : ''})`}
            </button>
        {/if}
      </div>

    {:else if activeTool === 'watermark'}
      <div class="tool-pane">
        <h2>Watermark PDF</h2>
        <p>Add a text watermark to your document(s).</p>

        <DropZone ondrop={(files: File[]) => watermarkFiles = [...watermarkFiles, ...files]} multiple={true} />

        {#if watermarkFiles.length > 0}
            <div class="file-list">
              {#each watermarkFiles as file, i}
                <div class="file-item">
                  <span class="file-name">{file.name}</span>
                  <div class="file-controls">
                    <button onclick={() => watermarkFiles = watermarkFiles.filter((_, idx) => idx !== i)}>✕</button>
                  </div>
                </div>
              {/each}
            </div>

            <div class="input-group">
                <label for="watermarkText">Watermark Text</label>
                <input id="watermarkText" type="text" bind:value={watermarkText} />
            </div>

            <button class="action-btn" onclick={handleWatermark} disabled={isProcessing || !watermarkText}>
                {isProcessing ? 'Processing...' : `Watermark & Download (${watermarkFiles.length} file${watermarkFiles.length > 1 ? 's' : ''})`}
            </button>
        {/if}
      </div>

    {:else if activeTool === 'delete'}
      <div class="tool-pane">
        <h2>Delete Pages</h2>
        <p>Remove specific pages from your document(s).</p>

        <DropZone ondrop={(files: File[]) => deleteFiles = [...deleteFiles, ...files]} multiple={true} />

        {#if deleteFiles.length > 0}
            <div class="file-list">
              {#each deleteFiles as file, i}
                <div class="file-item">
                  <span class="file-name">{file.name}</span>
                  <div class="file-controls">
                    <button onclick={() => deleteFiles = deleteFiles.filter((_, idx) => idx !== i)}>✕</button>
                  </div>
                </div>
              {/each}
            </div>

            <div class="input-group">
                <label for="deletePagesStr">Pages to Delete (e.g. 2, 4-6)</label>
                <input id="deletePagesStr" type="text" bind:value={deletePagesStr} placeholder="2, 4-6" />
            </div>

            <button class="action-btn" onclick={handleDelete} disabled={isProcessing || !deletePagesStr}>
                {isProcessing ? 'Processing...' : `Delete Pages & Download (${deleteFiles.length} file${deleteFiles.length > 1 ? 's' : ''})`}
            </button>
        {/if}
      </div>

    {:else if activeTool === 'extract'}
      <div class="tool-pane">
        <h2>Extract Pages</h2>
        <p>Extract specific pages across your document(s).</p>

        <DropZone ondrop={(files: File[]) => extractFiles = [...extractFiles, ...files]} multiple={true} />

        {#if extractFiles.length > 0}
            <div class="file-list">
              {#each extractFiles as file, i}
                <div class="file-item">
                  <span class="file-name">{file.name}</span>
                  <div class="file-controls">
                    <button onclick={() => extractFiles = extractFiles.filter((_, idx) => idx !== i)}>✕</button>
                  </div>
                </div>
              {/each}
            </div>

            <div class="input-group">
                <label for="extractPagesStr">Target Pages (e.g. 1-3)</label>
                <input id="extractPagesStr" type="text" bind:value={extractPagesStr} placeholder="1-3" />
            </div>

            <button class="action-btn" onclick={handleExtract} disabled={isProcessing || !extractPagesStr}>
                {isProcessing ? 'Processing...' : `Extract Pages & Download (${extractFiles.length} file${extractFiles.length > 1 ? 's' : ''})`}
            </button>
        {/if}
      </div>

    {:else if activeTool === 'reorder'}
      <div class="tool-pane">
        <h2>Reorder Pages</h2>
        <p>Rearrange the pages in your document(s).</p>

        <DropZone ondrop={(files: File[]) => reorderFiles = [...reorderFiles, ...files]} multiple={true} />

        {#if reorderFiles.length > 0}
            <div class="file-list">
              {#each reorderFiles as file, i}
                <div class="file-item">
                  <span class="file-name">{file.name}</span>
                  <div class="file-controls">
                    <button onclick={() => reorderFiles = reorderFiles.filter((_, idx) => idx !== i)}>✕</button>
                  </div>
                </div>
              {/each}
            </div>

            <div class="input-group">
                <label for="reorderPagesStr">New Order (comma separated, e.g. 3, 1, 2)</label>
                <input id="reorderPagesStr" type="text" bind:value={reorderPagesStr} placeholder="3, 1, 2" />
            </div>

            <button class="action-btn" onclick={handleReorder} disabled={isProcessing || !reorderPagesStr}>
                {isProcessing ? 'Processing...' : `Reorder & Download (${reorderFiles.length} file${reorderFiles.length > 1 ? 's' : ''})`}
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

    {:else if activeTool === 'images_to_pdf'}
      <div class="tool-pane">
        <h2>Images to PDF</h2>
        <p>Convert multiple images (PNG/JPEG) into a single PDF document.</p>

        {#if imagesToPdfFiles.length === 0}
            <DropZone ondrop={(files: File[]) => imagesToPdfFiles = [...imagesToPdfFiles, ...files]} multiple={true} />
        {:else}
            <div class="file-list">
                {#each imagesToPdfFiles as file, i}
                    <div class="file-item">
                        <span class="file-name">{file.name}</span>
                        <div class="file-controls">
                            <button onclick={() => {
                                const newFiles = [...imagesToPdfFiles];
                                newFiles.splice(i, 1);
                                imagesToPdfFiles = newFiles;
                            }}>✕</button>
                        </div>
                    </div>
                {/each}
            </div>
            <DropZone ondrop={(files: File[]) => imagesToPdfFiles = [...imagesToPdfFiles, ...files]} multiple={true} />
            <button class="action-btn" onclick={handleImagesToPdf} disabled={isProcessing}>
                {isProcessing ? 'Processing...' : 'Convert Images & Download PDF'}
            </button>
        {/if}
      </div>

    {:else if activeTool === 'extract_images'}
      <div class="tool-pane">
        <h2>Extract Images</h2>
        <p>Extract embedded images from a PDF.</p>

        {#if !extractImagesFile}
            <DropZone ondrop={(files: File[]) => extractImagesFile = files[0]} multiple={false} />
        {:else}
            <div class="selected-file">
                <span class="file-name">{extractImagesFile.name}</span>
                <button onclick={() => extractImagesFile = null}>✕</button>
            </div>
            <button class="action-btn" onclick={handleExtractImages} disabled={isProcessing}>
                {isProcessing ? 'Processing...' : 'Extract Images & Download'}
            </button>
        {/if}
      </div>

    {:else if activeTool === 'pdf_hash'}
      <div class="tool-pane">
        <h2>PDF Hash / Checksum</h2>
        <p>Calculate the cryptographic SHA-256 integrity hash of your PDF document locally.</p>

        {#if !pdfHashFile}
            <DropZone ondrop={(files: File[]) => { pdfHashFile = files[0]; computedPdfHash = null; }} multiple={false} />
        {:else}
            <div class="selected-file">
                <span class="file-name">{pdfHashFile.name}</span>
                <button onclick={() => { pdfHashFile = null; computedPdfHash = null; }}>✕</button>
            </div>
            <button class="action-btn" onclick={handlePdfHash} disabled={isProcessing}>
                {isProcessing ? 'Processing...' : 'Calculate Hash'}
            </button>

            {#if computedPdfHash}
                <div class="hash-result" style="margin-top: 24px; padding: 16px; background-color: var(--bg-surface); border-radius: var(--border-radius-md); border: 1px solid var(--border-color);">
                    <h3 style="margin-top: 0; font-size: 1rem; color: var(--text-secondary);">SHA-256 Hash</h3>
                    <div style="display: flex; gap: 8px; align-items: center;">
                        <input type="text" readonly value={computedPdfHash} style="flex: 1; padding: 8px; font-family: monospace; font-size: 0.9rem; background-color: var(--bg-primary); border: 1px solid var(--border-color); border-radius: 4px;" />
                        <button onclick={() => navigator.clipboard.writeText(computedPdfHash || '')} style="padding: 8px 16px; background-color: var(--accent-primary); color: white; border: none; border-radius: 4px; cursor: pointer;">Copy</button>
                    </div>
                </div>
            {/if}
        {/if}
      </div>

    {:else if activeTool === 'render_page'}
      <div class="tool-pane">
        <h2>Render Page</h2>
        <p>Render a PDF page to PNG.</p>
        {#if !renderFile}
            <DropZone ondrop={(files: File[]) => renderFile = files[0]} multiple={false} />
        {:else}
            <div class="selected-file">
                <span class="file-name">{renderFile.name}</span>
                <button onclick={() => renderFile = null}>✕</button>
            </div>
            <div class="input-group">
                <label for="renderPageIndex">Page Index (0-based)</label>
                <input id="renderPageIndex" type="number" bind:value={renderPageIndex} min="0" />
            </div>
            <div class="input-group">
                <label for="renderScale">Scale</label>
                <input id="renderScale" type="number" step="0.1" bind:value={renderScale} />
            </div>
            <button class="action-btn" onclick={handleRenderPage} disabled={isProcessing}>
                {isProcessing ? 'Processing...' : 'Render'}
            </button>
            {#if renderOutputUrl}
                <div style="margin-top: 16px;">
                    <img alt="Rendered PDF Page" src={renderOutputUrl} style="max-width: 100%; border: 1px solid var(--border-color); border-radius: var(--border-radius-md);" />
                </div>
            {/if}
        {/if}
      </div>

    {:else if activeTool === 'extract_text'}
      <div class="tool-pane">
        <h2>Extract Text</h2>
        <p>Extract all text content from the PDF.</p>
        {#if !extractTextFile}
            <DropZone ondrop={(files: File[]) => extractTextFile = files[0]} multiple={false} />
        {:else}
            <div class="selected-file">
                <span class="file-name">{extractTextFile.name}</span>
                <button onclick={() => extractTextFile = null}>✕</button>
            </div>
            <button class="action-btn" onclick={handleExtractText} disabled={isProcessing}>
                {isProcessing ? 'Processing...' : 'Extract'}
            </button>
            {#if extractedText}
                <textarea readonly rows="10" style="width: 100%; margin-top: 16px; font-family: monospace; padding: 8px;">{extractedText}</textarea>
            {/if}
        {/if}
      </div>

    {:else if activeTool === 'decrypt'}
      <div class="tool-pane">
        <h2>Decrypt</h2>
        <p>Decrypt a password-protected PDF.</p>
        {#if !decryptFile}
            <DropZone ondrop={(files: File[]) => decryptFile = files[0]} multiple={false} />
        {:else}
            <div class="selected-file">
                <span class="file-name">{decryptFile.name}</span>
                <button onclick={() => decryptFile = null}>✕</button>
            </div>
            <div class="input-group">
                <label for="decryptPassword">Password</label>
                <input id="decryptPassword" type="password" bind:value={decryptPassword} />
            </div>
            <button class="action-btn" onclick={handleDecrypt} disabled={isProcessing}>
                {isProcessing ? 'Processing...' : 'Decrypt & Download'}
            </button>
        {/if}
      </div>

    {:else if activeTool === 'page_numbers'}
      <div class="tool-pane">
        <h2>Page Numbers</h2>
        <p>Add page numbers to the PDF.</p>
        {#if !pageNumbersFile}
            <DropZone ondrop={(files: File[]) => pageNumbersFile = files[0]} multiple={false} />
        {:else}
            <div class="selected-file">
                <span class="file-name">{pageNumbersFile.name}</span>
                <button onclick={() => pageNumbersFile = null}>✕</button>
            </div>
            <div class="input-group">
                <label for="pageNumbersFormat">Format</label>
                <input id="pageNumbersFormat" type="text" bind:value={pageNumbersFormat} />
            </div>
            <div class="input-group">
                <label for="pageNumbersPosition">Position</label>
                <select id="pageNumbersPosition" bind:value={pageNumbersPosition}>
                    <option value="bottom-center">Bottom Center</option>
                    <option value="bottom-right">Bottom Right</option>
                    <option value="top-center">Top Center</option>
                    <option value="top-right">Top Right</option>
                </select>
            </div>
            <button class="action-btn" onclick={handlePageNumbers} disabled={isProcessing}>
                {isProcessing ? 'Processing...' : 'Add Numbers & Download'}
            </button>
        {/if}
      </div>

    {:else if activeTool === 'header_footer'}
      <div class="tool-pane">
        <h2>Header & Footer</h2>
        <p>Add header and footer text to the PDF.</p>
        {#if !headerFooterFile}
            <DropZone ondrop={(files: File[]) => headerFooterFile = files[0]} multiple={false} />
        {:else}
            <div class="selected-file">
                <span class="file-name">{headerFooterFile.name}</span>
                <button onclick={() => headerFooterFile = null}>✕</button>
            </div>
            <div class="input-group">
                <label for="headerText">Header Text</label>
                <input id="headerText" type="text" bind:value={headerText} />
            </div>
            <div class="input-group">
                <label for="footerText">Footer Text</label>
                <input id="footerText" type="text" bind:value={footerText} />
            </div>
            <button class="action-btn" onclick={handleHeaderFooter} disabled={isProcessing}>
                {isProcessing ? 'Processing...' : 'Add & Download'}
            </button>
        {/if}
      </div>

    {:else if activeTool === 'pdf_info'}
      <div class="tool-pane">
        <h2>PDF Info</h2>
        <p>Get metadata and info about the PDF.</p>
        {#if !pdfInfoFile}
            <DropZone ondrop={(files: File[]) => pdfInfoFile = files[0]} multiple={false} />
        {:else}
            <div class="selected-file">
                <span class="file-name">{pdfInfoFile.name}</span>
                <button onclick={() => pdfInfoFile = null}>✕</button>
            </div>
            <button class="action-btn" onclick={handlePdfInfo} disabled={isProcessing}>
                {isProcessing ? 'Processing...' : 'Get Info'}
            </button>
            {#if pdfInfoData}
                <pre style="margin-top: 16px; padding: 16px; background: var(--bg-surface); border-radius: var(--border-radius-md); overflow-x: auto;">{pdfInfoData}</pre>
            {/if}
        {/if}
      </div>

    {:else if activeTool === 'ocr'}
      <div class="tool-pane">
        <h2>OCR</h2>
        <p>Extract text from images using local WASM OCR.</p>
        {#if !ocrFile}
            <DropZone ondrop={(files: File[]) => ocrFile = files[0]} multiple={false} />
        {:else}
            <div class="selected-file">
                <span class="file-name">{ocrFile.name}</span>
                <button onclick={() => ocrFile = null}>✕</button>
            </div>
            <button class="action-btn" onclick={handleOcr} disabled={isProcessing}>
                {isProcessing ? 'Processing...' : 'Run OCR'}
            </button>
            {#if ocrData}
                <textarea readonly rows="10" style="width: 100%; margin-top: 16px; font-family: monospace; padding: 8px;">{ocrData}</textarea>
            {/if}
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
