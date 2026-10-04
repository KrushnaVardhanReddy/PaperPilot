import { invoke } from '@tauri-apps/api/core';

export async function safeInvoke(cmd: string, args?: any): Promise<any> {
    if (typeof window !== 'undefined' && !(window as any).__TAURI_INTERNALS__) {
        console.warn(`[safeInvoke] Tauri not available, skipping '${cmd}'`);
        return null;
    }
    return await invoke(cmd, args);
}
