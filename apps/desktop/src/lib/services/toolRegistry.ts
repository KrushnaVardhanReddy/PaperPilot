export function resolveToolName(toolId: string): string {
    if (toolId === 'md_to_pdf') return 'pdf_convert_markdown';
    if (toolId === 'html_to_pdf') return 'pdf_convert_html';
    if (toolId === 'img_to_pdf') return 'pdf_images_to_pdf';
    if (toolId === 'json_to_pdf') return 'json_to_pdf';

    // Forms and conversions mapping
    if (['create_form_field', 'fill_form', 'read_form'].includes(toolId)) {
        return `pdf_${toolId}`;
    }

    if ([
        'pdf_to_docx', 'pdf_to_xlsx', 'pdf_to_pptx', 'pdf_to_pdf_a', 'pdf_to_markdown', 'pdf_to_html', 'pdf_to_json',
        'pdf_convert_excel', 'office_modernize_legacy', 'office_to_markdown', 'office_extract_text',
        'excel_to_json', 'excel_to_csv'
    ].includes(toolId)) {
        return toolId;
    }

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

  const hasExcel = documents.some(f => {
      const name = f.name.toLowerCase();
      return name.endsWith('.xlsx') || name.endsWith('.xls') || name.endsWith('.csv');
  });

  const allPdf = documents.length > 0 && documents.every(f => f.name.toLowerCase().endsWith('.pdf'));

  for (const tool of tools) {
    let isRecommended = false;
    let contextMatches = false;

    if (documents.length === 0) {
      if (['img_to_pdf', 'json_to_pdf', 'md_to_pdf'].includes(tool.id)) {
        isRecommended = true;
        contextMatches = true;
      }
    } else if (hasExcel) {
      if (['excel_to_json', 'excel_to_csv'].includes(tool.id)) {
        isRecommended = true;
        contextMatches = true;
      }
    } else if (allPdf) {
      if (documents.length >= 2) {
        if (['merge', 'compare'].includes(tool.id) || ['compress', 'ocr', 'pdf_to_docx', 'rotate'].includes(tool.id)) {
          isRecommended = true;
          contextMatches = true;
        }
      } else if (documents.length === 1) {
        if (['compress', 'split', 'rotate', 'ocr', 'pdf_to_docx'].includes(tool.id)) {
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

    if (isRecommended) {
      // Avoid duplicates
      if (!recommended.find(t => t.id === tool.id)) {
         recommended.push(tool);
      }
    } else {
      if (!available.find(t => t.id === tool.id) && !recommended.find(t => t.id === tool.id)) {
         available.push(tool);
      }
    }
  }

  // Ensure available doesn't contain recommended
  const filteredAvailable = available.filter(t => !recommended.find(rt => rt.id === t.id));

  return {
    recommended,
    available: filteredAvailable,
    packLocked
  };
}
