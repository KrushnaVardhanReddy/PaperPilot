import { invoke } from '@tauri-apps/api/core';

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
