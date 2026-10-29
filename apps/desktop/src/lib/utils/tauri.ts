import { invoke } from '@tauri-apps/api/core';

export async function safeInvoke(cmd: string, args?: any): Promise<any> {
    if (typeof window !== 'undefined' && !(window as any).__TAURI_INTERNALS__) {
        console.warn(`[safeInvoke] Tauri not available, skipping '${cmd}'`);
        return null;
    }
    return await invoke(cmd, args);
}

/** Commands that promise work must fail explicitly when no native engine exists. */
export function isNativeDesktop(): boolean {
    return typeof window !== 'undefined' && !!(window as any).__TAURI_INTERNALS__;
}
export async function requireInvoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
    if (!isNativeDesktop()) throw new Error('This action requires the PaperPilot desktop app.');
    return invoke<T>(cmd, args);
}
