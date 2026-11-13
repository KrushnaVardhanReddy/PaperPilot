<script lang="ts">
  import { onMount } from 'svelte';
  import * as XLSX from 'xlsx';

  let { file }: { file: File } = $props();

  interface ParagraphBlock {
    type: 'h1' | 'h2' | 'h3' | 'p' | 'list';
    text: string;
    bold?: boolean;
    italic?: boolean;
    underline?: boolean;
    color?: string;
    align?: string;
  }

  interface TableBlock {
    type: 'table';
    rows: string[][];
  }

  type ContentBlock = ParagraphBlock | TableBlock;

  interface DocMetadata {
    title?: string;
    creator?: string;
    lastModifiedBy?: string;
    created?: string;
    modified?: string;
    revision?: string;
  }

  let isLoading = $state(true);
  let loadError = $state<string | null>(null);
  let activeTab = $state<'preview' | 'text' | 'metadata'>('preview');
  let blocks = $state<ContentBlock[]>([]);
  let plainText = $state('');
  let metadata = $state<DocMetadata>({});
  let searchQuery = $state('');
  let wordCount = $state(0);
  let charCount = $state(0);
  let isLegacyDoc = $state(false);

  function decodeXmlEntities(str: string): string {
    return str
      .replace(/&amp;/g, '&')
      .replace(/&lt;/g, '<')
      .replace(/&gt;/g, '>')
      .replace(/&quot;/g, '"')
      .replace(/&apos;/g, "'");
  }

  function extractXmlTag(xml: string, tag: string): string | undefined {
    const match = xml.match(new RegExp(`<${tag}[^>]*>(.*?)</${tag}>`, 's'));
    return match ? decodeXmlEntities(match[1].trim()) : undefined;
  }

  async function parseDocx() {
    isLoading = true;
    loadError = null;
    blocks = [];
    plainText = '';
    metadata = {};
    wordCount = 0;
    charCount = 0;

    const lowerName = file.name.toLowerCase();
    isLegacyDoc = lowerName.endsWith('.doc') && !lowerName.endsWith('.docx');

    try {
      const arrayBuffer = await file.arrayBuffer();
      const uint8Array = new Uint8Array(arrayBuffer);

      // Legacy binary .doc (Word 97-2004) or OpenXML .docx
      // XLSX.CFB expects a Uint8Array or Buffer for type: 'array'
      const cfb = XLSX.CFB.read(uint8Array, { type: 'array' });
      if (!cfb || !cfb.FileIndex) {
        throw new Error('Unable to parse document package structure.');
      }

      // 1. Locate word/document.xml (or document.xml)
      const docEntry =
        XLSX.CFB.find(cfb, 'word/document.xml') ||
        XLSX.CFB.find(cfb, 'document.xml') ||
        cfb.FileIndex.find((f: any) => f.name && (f.name === 'document.xml' || f.name.endsWith('/document.xml')));

      // 2. Parse metadata if docProps/core.xml exists
      const coreEntry =
        XLSX.CFB.find(cfb, 'docProps/core.xml') ||
        XLSX.CFB.find(cfb, 'core.xml') ||
        cfb.FileIndex.find((f: any) => f.name && (f.name === 'core.xml' || f.name.endsWith('/core.xml')));

      if (coreEntry && coreEntry.content) {
        const coreXml = new TextDecoder('utf-8').decode(coreEntry.content);
        metadata = {
          title: extractXmlTag(coreXml, 'dc:title'),
          creator: extractXmlTag(coreXml, 'dc:creator'),
          lastModifiedBy: extractXmlTag(coreXml, 'cp:lastModifiedBy'),
          created: extractXmlTag(coreXml, 'dcterms:created'),
          modified: extractXmlTag(coreXml, 'dcterms:modified'),
          revision: extractXmlTag(coreXml, 'cp:revision'),
        };
      }

      if (!docEntry || !docEntry.content) {
        if (isLegacyDoc) {
          // For legacy .doc, extract readable strings if document.xml is not present
          const textDecoder = new TextDecoder('utf-8', { fatal: false });
          const raw = textDecoder.decode(buffer);
          const readable = raw.replace(/[^\x20-\x7E\t\n\r]/g, ' ').replace(/\s{3,}/g, '\n\n').trim();
          if (readable.length > 50) {
            plainText = readable;
            const pStrings = readable.split('\n\n').filter(p => p.trim());
            blocks = pStrings.map(p => ({ type: 'p', text: p.trim() }));
          } else {
            throw new Error('Legacy .doc binary format contains no OpenXML streams. Convert to .docx or PDF for rich preview.');
          }
        } else {
          throw new Error('Missing word/document.xml in DOCX archive.');
        }
      } else {
        const docXml = new TextDecoder('utf-8').decode(docEntry.content);
        parseDocumentXml(docXml);
      }

      // Compute stats
      const allText = blocks
        .map(b => (b.type === 'table' ? b.rows.map(r => r.join(' ')).join('\n') : b.text))
        .join('\n');
      plainText = allText;
      charCount = allText.length;
      wordCount = allText.trim() ? allText.trim().split(/\s+/).length : 0;
    } catch (err: any) {
      console.error('Docx parse error:', err);
      loadError = err?.message || 'Failed to parse Microsoft Word document.';
    } finally {
      isLoading = false;
    }
  }

  function parseDocumentXml(xml: string) {
    const parsedBlocks: ContentBlock[] = [];

    // Parse elements in document body sequentially: <w:p> paragraphs and <w:tbl> tables
    const bodyMatch = xml.match(/<w:body[^>]*>(.*?)<\/w:body>/s);
    const bodyContent = bodyMatch ? bodyMatch[1] : xml;

    // Match each table or paragraph in order of occurrence
    const elementRegex = /<w:(tbl|p)[\s>].*?<\/w:\1>/gs;
    let match;

    while ((match = elementRegex.exec(bodyContent)) !== null) {
      const tagType = match[1];
      const elementXml = match[0];

      if (tagType === 'tbl') {
        // Table parsing: extract rows <w:tr> and cells <w:tc>
        const rows: string[][] = [];
        const trMatches = elementXml.match(/<w:tr[\s>].*?<\/w:tr>/gs) || [];
        for (const tr of trMatches) {
          const cells: string[] = [];
          const tcMatches = tr.match(/<w:tc[\s>].*?<\/w:tc>/gs) || [];
          for (const tc of tcMatches) {
            const cellTexts = [...tc.matchAll(/<w:t[\s>][^>]*>(.*?)<\/w:t>/gs)].map(m => decodeXmlEntities(m[1]));
            cells.push(cellTexts.join('').trim());
          }
          if (cells.length > 0) {
            rows.push(cells);
          }
        }
        if (rows.length > 0) {
          parsedBlocks.push({ type: 'table', rows });
        }
      } else if (tagType === 'p') {
        // Paragraph parsing
        // Detect heading style in <w:pStyle w:val="..."/>
        const styleMatch = elementXml.match(/<w:pStyle[^>]*w:val="([^"]+)"/i);
        const styleVal = styleMatch ? styleMatch[1].toLowerCase() : '';

        // Detect list numbering <w:numPr>
        const isList = /<w:numPr[\s>]/.test(elementXml) || styleVal.includes('list');

        let blockType: 'h1' | 'h2' | 'h3' | 'p' | 'list' = 'p';
        if (isList) {
          blockType = 'list';
        } else if (styleVal.includes('heading1') || styleVal === '1' || styleVal.includes('title')) {
          blockType = 'h1';
        } else if (styleVal.includes('heading2') || styleVal === '2') {
          blockType = 'h2';
        } else if (styleVal.includes('heading3') || styleVal === '3') {
          blockType = 'h3';
        }

        // Detect alignment
        const jcMatch = elementXml.match(/<w:jc[^>]*w:val="([^"]+)"/i);
        const align = jcMatch ? jcMatch[1].toLowerCase() : undefined;

        // Extract run styles and texts
        const runs = [...elementXml.matchAll(/<w:r[\s>].*?<\/w:r>/gs)];
        let pText = '';
        let hasBold = false;
        let hasItalic = false;
        let hasUnderline = false;

        if (runs.length > 0) {
          for (const rMatch of runs) {
            const rXml = rMatch[0];
            const tMatches = [...rXml.matchAll(/<w:t[\s>][^>]*>(.*?)<\/w:t>/gs)].map(m => decodeXmlEntities(m[1]));
            const runText = tMatches.join('');
            pText += runText;
            if (/<w:b(\/|>|\s)/.test(rXml)) hasBold = true;
            if (/<w:i(\/|>|\s)/.test(rXml)) hasItalic = true;
            if (/<w:u(\/|>|\s)/.test(rXml)) hasUnderline = true;
          }
        } else {
          // Fallback if no <w:r> wrapping
          const tMatches = [...elementXml.matchAll(/<w:t[\s>][^>]*>(.*?)<\/w:t>/gs)].map(m => decodeXmlEntities(m[1]));
          pText = tMatches.join('');
        }

        const trimmed = pText.trim();
        if (trimmed.length > 0) {
          parsedBlocks.push({
            type: blockType,
            text: pText,
            bold: hasBold,
            italic: hasItalic,
            underline: hasUnderline,
            align,
          });
        }
      }
    }

    blocks = parsedBlocks;
  }

  let filteredBlocks = $derived.by(() => {
    if (!searchQuery.trim()) return blocks;
    const q = searchQuery.toLowerCase();
    return blocks.filter(b => {
      if (b.type === 'table') {
        return b.rows.some(r => r.some(cell => cell.toLowerCase().includes(q)));
      }
      return b.text.toLowerCase().includes(q);
    });
  });

  onMount(() => {
    parseDocx();
  });

  $effect(() => {
    if (file) {
      parseDocx();
    }
  });
</script>

<div class="flex flex-col h-full bg-slate-900 text-slate-100 select-text overflow-hidden">
  <!-- Top Navigation Toolbar -->
  <div class="flex items-center justify-between px-4 py-2 border-b border-slate-700 bg-slate-800/80 backdrop-blur shrink-0">
    <!-- File Info & Tabs -->
    <div class="flex items-center gap-3">
      <div class="flex items-center gap-2">
        <span class="text-xl">📝</span>
        <div>
          <span class="font-medium text-sm text-slate-200">{file.name}</span>
          <span class="text-xs text-slate-400 ml-2">{(file.size / 1024).toFixed(1)} KB</span>
        </div>
      </div>

      <div class="h-4 w-px bg-slate-700 mx-2"></div>

      <!-- Tab Buttons -->
      <div class="flex items-center bg-slate-900/60 rounded-lg p-0.5 border border-slate-700/60 text-xs shrink-0 gap-1">
        <button
          onclick={() => activeTab = 'preview'}
          class="px-3 py-1 rounded-md transition-colors font-medium whitespace-nowrap {activeTab === 'preview' ? 'bg-blue-600 text-white shadow-sm' : 'text-slate-400 hover:text-slate-200 hover:bg-slate-800'}"
        >
          Formatted Preview
        </button>
        <button
          onclick={() => activeTab = 'text'}
          class="px-3 py-1 rounded-md transition-colors font-medium whitespace-nowrap {activeTab === 'text' ? 'bg-blue-600 text-white shadow-sm' : 'text-slate-400 hover:text-slate-200 hover:bg-slate-800'}"
        >
          Plain Text
        </button>
        <button
          onclick={() => activeTab = 'metadata'}
          class="px-3 py-1 rounded-md transition-colors font-medium whitespace-nowrap {activeTab === 'metadata' ? 'bg-blue-600 text-white shadow-sm' : 'text-slate-400 hover:text-slate-200 hover:bg-slate-800'}"
        >
          Properties
        </button>
      </div>
    </div>

    <!-- Quick Stats & Search -->
    <div class="flex items-center gap-3">
      {#if !isLoading && !loadError}
        <div class="hidden sm:flex items-center gap-3 text-xs text-slate-400">
          <span>{wordCount.toLocaleString()} words</span>
          <span>•</span>
          <span>{charCount.toLocaleString()} characters</span>
          <span>•</span>
          <span>{blocks.length} elements</span>
        </div>
      {/if}

      <!-- Search Input -->
      <div class="relative w-48 sm:w-64">
        <input
          type="text"
          bind:value={searchQuery}
          placeholder="Search document..."
          class="w-full bg-slate-900/80 border border-slate-700 rounded-lg px-2.5 py-1 text-xs text-slate-200 placeholder-slate-500 focus:outline-none focus:border-primary-500 transition-colors"
        />
        {#if searchQuery}
          <button
            onclick={() => searchQuery = ''}
            class="absolute right-2 top-1/2 -translate-y-1/2 text-slate-400 hover:text-white text-xs"
          >
            ×
          </button>
        {/if}
      </div>
    </div>
  </div>

  <!-- Main Content Area -->
  <div class="flex-1 overflow-auto bg-slate-950 p-6 flex justify-center">
    {#if isLoading}
      <div class="flex flex-col items-center justify-center h-64 gap-3 text-slate-400">
        <div class="animate-spin rounded-full h-8 w-8 border-2 border-primary-500 border-t-transparent"></div>
        <p class="text-sm">Reading Word document structure...</p>
      </div>
    {:else if loadError}
      <div class="flex flex-col items-center justify-center max-w-md mx-auto my-auto p-6 bg-rose-950/30 border border-rose-800/50 rounded-xl text-center">
        <div class="text-rose-400 text-3xl mb-2">⚠️</div>
        <h3 class="text-sm font-semibold text-rose-200 mb-1">Unable to Open Word Document</h3>
        <p class="text-xs text-rose-300/80 mb-4">{loadError}</p>
        <p class="text-xs text-slate-400">
          Tip: You can use the <span class="text-primary-400 font-semibold">Office to PDF</span> operation from the Tools panel to convert this file into a fully rendered PDF.
        </p>
      </div>
    {:else if activeTab === 'preview'}
      <!-- Word Page Canvas Container -->
      <div class="w-full max-w-4xl bg-white text-slate-900 rounded-lg shadow-2xl p-10 sm:p-14 min-h-[800px] border border-slate-200 my-auto">
        {#if blocks.length === 0}
          <div class="text-center py-20 text-slate-400 text-sm">
            This Word document is empty.
          </div>
        {:else}
          <div class="space-y-4">
            {#each filteredBlocks as block}
              {#if block.type === 'h1'}
                <h1 class="text-2xl sm:text-3xl font-bold text-slate-900 tracking-tight mt-6 mb-2 border-b border-slate-200 pb-1"
                    style={block.align ? `text-align: ${block.align}` : ''}>
                  {block.text}
                </h1>
              {:else if block.type === 'h2'}
                <h2 class="text-xl sm:text-2xl font-semibold text-slate-800 tracking-tight mt-5 mb-2"
                    style={block.align ? `text-align: ${block.align}` : ''}>
                  {block.text}
                </h2>
              {:else if block.type === 'h3'}
                <h3 class="text-lg font-medium text-slate-800 tracking-tight mt-4 mb-1"
                    style={block.align ? `text-align: ${block.align}` : ''}>
                  {block.text}
                </h3>
              {:else if block.type === 'list'}
                <div class="flex items-start gap-2 pl-4 text-sm leading-relaxed text-slate-700">
                  <span class="text-primary-600 font-bold select-none">•</span>
                  <p class="{block.bold ? 'font-bold' : ''} {block.italic ? 'italic' : ''} {block.underline ? 'underline' : ''}">
                    {block.text}
                  </p>
                </div>
              {:else if block.type === 'table'}
                <div class="overflow-x-auto my-4 border border-slate-300 rounded">
                  <table class="w-full border-collapse text-xs sm:text-sm text-left">
                    <tbody>
                      {#each block.rows as row, rIdx}
                        <tr class="{rIdx === 0 ? 'bg-slate-100 font-semibold border-b border-slate-300' : 'border-b border-slate-200 hover:bg-slate-50/50'}">
                          {#each row as cell}
                            <td class="p-2 sm:p-2.5 border-r border-slate-200 last:border-r-0 text-slate-800">
                              {cell}
                            </td>
                          {/each}
                        </tr>
                      {/each}
                    </tbody>
                  </table>
                </div>
              {:else}
                <p class="text-sm leading-relaxed text-slate-700 {block.bold ? 'font-bold' : ''} {block.italic ? 'italic' : ''} {block.underline ? 'underline' : ''}"
                   style={block.align ? `text-align: ${block.align}` : ''}>
                  {block.text}
                </p>
              {/if}
            {/each}
          </div>
        {/if}
      </div>

    {:else if activeTab === 'text'}
      <!-- Plain Text Raw View -->
      <div class="w-full max-w-4xl bg-slate-900 border border-slate-800 rounded-lg p-6 font-mono text-xs leading-relaxed text-slate-300 overflow-auto whitespace-pre-wrap select-text shadow-lg">
        {plainText || 'No text extracted.'}
      </div>

    {:else if activeTab === 'metadata'}
      <!-- Document Properties / Metadata View -->
      <div class="w-full max-w-2xl bg-slate-900 border border-slate-800 rounded-xl p-6 shadow-xl my-auto">
        <h3 class="text-sm font-semibold text-slate-200 mb-4 pb-2 border-b border-slate-800 flex items-center gap-2">
          <span>ℹ️</span> Document Properties & Core XML Metadata
        </h3>
        <dl class="space-y-3 text-xs">
          <div class="flex justify-between py-1.5 border-b border-slate-800/60">
            <dt class="text-slate-400">File Name</dt>
            <dd class="text-slate-200 font-mono">{file.name}</dd>
          </div>
          <div class="flex justify-between py-1.5 border-b border-slate-800/60">
            <dt class="text-slate-400">File Size</dt>
            <dd class="text-slate-200">{(file.size / 1024).toFixed(2)} KB ({file.size.toLocaleString()} bytes)</dd>
          </div>
          <div class="flex justify-between py-1.5 border-b border-slate-800/60">
            <dt class="text-slate-400">Title</dt>
            <dd class="text-slate-200">{metadata.title || '(Not specified)'}</dd>
          </div>
          <div class="flex justify-between py-1.5 border-b border-slate-800/60">
            <dt class="text-slate-400">Author / Creator</dt>
            <dd class="text-slate-200">{metadata.creator || '(Not specified)'}</dd>
          </div>
          <div class="flex justify-between py-1.5 border-b border-slate-800/60">
            <dt class="text-slate-400">Last Modified By</dt>
            <dd class="text-slate-200">{metadata.lastModifiedBy || '(Not specified)'}</dd>
          </div>
          <div class="flex justify-between py-1.5 border-b border-slate-800/60">
            <dt class="text-slate-400">Created Date</dt>
            <dd class="text-slate-200">{metadata.created || '(Not specified)'}</dd>
          </div>
          <div class="flex justify-between py-1.5 border-b border-slate-800/60">
            <dt class="text-slate-400">Modified Date</dt>
            <dd class="text-slate-200">{metadata.modified || '(Not specified)'}</dd>
          </div>
          <div class="flex justify-between py-1.5 border-b border-slate-800/60">
            <dt class="text-slate-400">Revision</dt>
            <dd class="text-slate-200">{metadata.revision || '1'}</dd>
          </div>
          <div class="flex justify-between py-1.5 border-b border-slate-800/60">
            <dt class="text-slate-400">Format Standard</dt>
            <dd class="text-slate-200">{isLegacyDoc ? 'Microsoft Word 97-2004 (.doc)' : 'Office Open XML Document (.docx)'}</dd>
          </div>
        </dl>
      </div>
    {/if}
  </div>
</div>

