import { invoke } from '@tauri-apps/api/core';

export interface Annotation {
  id: string;
  type: 'highlight' | 'underline' | 'strikethrough' | 'note' | 'pen';
  page: number;
  x: number;
  y: number;
  w?: number;
  h?: number;
  color: string;
  content?: string;
  path?: {x: number, y: number}[];
}

export async function saveAnnotations(filePath: string, outputPath: string, annotations: Annotation[]) {
  return invoke('save_annotations', { filePath, outputPath, annotations });
}

export async function savePdfForm(inputPath: string, outputPath: string, values: Record<string, string>): Promise<any> {
    return await invoke('invoke_mcp_tool', {
        toolName: 'pdf_fill_form',
        arguments: {
            input: inputPath,
            output: outputPath,
            values
        }
    });
}
