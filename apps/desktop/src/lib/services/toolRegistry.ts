import { isImageFile } from '$lib/utils/fileType';

// Explicit mappings for tools with unconventional IDs or legacy names
const TOOL_NAME_ALIASES: Record<string, string> = {
  md_to_pdf: 'pdf_convert_markdown',
  html_to_pdf: 'pdf_convert_html',
  img_to_pdf: 'pdf_images_to_pdf',
  cli_fill_batch: 'batch_fill',
  cli_ask: 'workspace_search',
  document_pii_scan: 'pdf_document_pii_scan',
};

// Known MCP tool names that don't take a 'pdf_' prefix
const UNPREFIXED_TOOLS = new Set([
  'batch_fill',
  'recipe_run',
  'document_workflow',
  'list_workflows',
  'watcher_start',
  'watcher_stop',
  'watcher_list',
  'semantic_search',
  'workspace_search',
  'excel_to_json',
  'excel_to_csv',
  'office_to_markdown',
  'office_extract_text',
  'office_modernize_legacy',
  'json_to_pdf'
]);

export function resolveToolName(toolId: string): string {
  if (TOOL_NAME_ALIASES[toolId]) {
    return TOOL_NAME_ALIASES[toolId];
  }

  if (UNPREFIXED_TOOLS.has(toolId)) {
    return toolId;
  }

  // If the tool ID already starts with standard namespace prefixes, keep it as is
  if (
    toolId.startsWith('pdf_') ||
    toolId.startsWith('office_') ||
    toolId.startsWith('excel_') ||
    toolId.startsWith('json_')
  ) {
    return toolId;
  }

  // Short canonical alias (e.g. 'merge' -> 'pdf_merge', 'sign' -> 'pdf_sign')
  return `pdf_${toolId}`;
}

import type { ToolDefinition } from '../types/tools';

export interface RankedToolsResult {
  recommended: ToolDefinition[];
  available: ToolDefinition[];
  packLocked: { tool: ToolDefinition; missingPack: string }[];
}

export function rankToolsByContext(
  tools: ToolDefinition[],
  documents: File[],
  activePacks: string[]
): RankedToolsResult {
  const recommended: ToolDefinition[] = [];
  const available: ToolDefinition[] = [];
  const packLocked: { tool: ToolDefinition; missingPack: string }[] = [];

  // Track seen canonical names to prevent duplicate logical tools
  const seenRecommended = new Set<string>();
  const seenAvailable = new Set<string>();

  const hasWord = documents.some(f => {
      const name = f.name.toLowerCase();
      return name.endsWith('.docx') || name.endsWith('.doc');
  });

  const hasExcel = documents.some(f => {
      const name = f.name.toLowerCase();
      return name.endsWith('.xlsx') || name.endsWith('.xls') || name.endsWith('.csv');
  });

  const hasJson = documents.some(f => f.name.toLowerCase().endsWith('.json'));
  const hasMarkdown = documents.some(f => f.name.toLowerCase().endsWith('.md') || f.name.toLowerCase().endsWith('.markdown'));
  const hasHtml = documents.some(f => f.name.toLowerCase().endsWith('.html') || f.name.toLowerCase().endsWith('.htm'));
  const hasTextWeb = hasMarkdown || hasHtml;
  const hasImage = documents.some(f => isImageFile(f.name));

  const allPdf = documents.length > 0 && documents.every(f => f.name.toLowerCase().endsWith('.pdf'));

  for (const tool of tools) {
    let isRecommended = false;
    let contextMatches = false;

    if (documents.length === 0) {
      if (['img_to_pdf', 'json_to_pdf', 'md_to_pdf', 'html_to_pdf'].includes(tool.id)) {
        isRecommended = true;
        contextMatches = true;
      }
    } else if (hasImage) {
      if ([
        'img_to_pdf',
        'pdf_images_to_pdf',
        'ocr',
        'pdf_ocr',
        'extract_text',
        'pdf_extract_text',
        'watermark',
        'pdf_watermark'
      ].includes(tool.id)) {
        isRecommended = true;
        contextMatches = true;
      }
    } else if (hasWord) {
      if (['office_to_markdown', 'office_extract_text', 'office_modernize_legacy'].includes(tool.id)) {
        isRecommended = true;
        contextMatches = true;
      }
    } else if (hasExcel) {
      if (['excel_to_json', 'excel_to_csv', 'pdf_convert_excel', 'pdf_extract_tables'].includes(tool.id)) {
        isRecommended = true;
        contextMatches = true;
      }
    } else if (hasTextWeb) {
      if (['md_to_pdf', 'html_to_pdf', 'pdf_convert_markdown', 'pdf_convert_html'].includes(tool.id)) {
        isRecommended = true;
        contextMatches = true;
      }
    } else if (hasJson) {
      if (['json_to_pdf'].includes(tool.id)) {
        isRecommended = true;
        contextMatches = true;
      }
    } else if (allPdf) {
      if (documents.length >= 2) {
        if ([
          'pdf_merge',
          'merge',
          'pdf_merge_alternate',
          'pdf_compare',
          'compare',
          'compress',
          'ocr',
          'pdf_to_docx',
          'rotate'
        ].includes(tool.id)) {
          isRecommended = true;
          contextMatches = true;
        }
      } else if (documents.length === 1) {
        if ([
          'compress',
          'split',
          'rotate',
          'delete_pages',
          'extract_pages',
          'reorder_pages',
          'ocr',
          'pdf_to_docx',
          'watermark',
          'extract_text',
          'pdf_to_epub',
          'document_pii_scan',
          'pdf_sign',
          'sign',
          'pdf_stamp',
          'pdf_extract_tables',
          'pdf_rag_chunks'
        ].includes(tool.id)) {
          isRecommended = true;
          contextMatches = true;
        }
      }
    }

    if (tool.rule?.requiredPack && !activePacks.includes(tool.rule.requiredPack)) {
       // Only add to packLocked if it is relevant to the current context
       if (contextMatches) {
          packLocked.push({ tool, missingPack: tool.rule.requiredPack });
       }
       continue;
    }

    const canonicalId = resolveToolName(tool.id);

    if (isRecommended) {
      // Deduplicate by canonical name (e.g. 'split' and 'pdf_split' are the same tool)
      if (!seenRecommended.has(canonicalId)) {
        seenRecommended.add(canonicalId);
        recommended.push(tool);
      }
    } else {
      if (!seenAvailable.has(canonicalId) && !seenRecommended.has(canonicalId)) {
        seenAvailable.add(canonicalId);
        available.push(tool);
      }
    }
  }

  // Final guard: strip any available entry whose canonical id is already recommended
  const filteredAvailable = available.filter(t => !seenRecommended.has(resolveToolName(t.id)));

  return {
    recommended,
    available: filteredAvailable,
    packLocked
  };
}
