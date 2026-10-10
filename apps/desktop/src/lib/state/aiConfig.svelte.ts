import { invoke } from '@tauri-apps/api/core';

export type AiProviderMode = 'offlinenlp' | 'llamafile' | 'universal' | 'byok';

export interface AiSettingsConfig {
    mode: AiProviderMode;
    universal_endpoint: string | null;
    byok_provider: string | null;
    active_model: string | null;
    api_key: string | null;
}

export interface ModelInfo {
    id: string;
    name: string;
    description: string;
    size_mb: number;
    download_url: string;
    local_path: string | null;
    is_downloaded: boolean;
}

export class AiConfigState {
    mode = $state<AiProviderMode>('offlinenlp');
    universalEndpoint = $state<string | null>(null);
    byokProvider = $state<string | null>(null);
    activeModel = $state<string | null>('qwen2.5-coder');
    apiKeyPlaceholder = $state<string | null>(null);

    // Non-persistent state for forms
    pendingApiKey = $state<string>('');
    testResult = $state<{success: boolean; message: string} | null>(null);
    testLoading = $state<boolean>(false);

    availableModels = $state<ModelInfo[]>([]);

    constructor() {
        this.loadConfig();
        this.loadModels();
    }

    async loadConfig() {
        try {
            const config = await invoke<AiSettingsConfig>('get_ai_config');
            this.mode = config.mode;
            this.universalEndpoint = config.universal_endpoint;
            this.byokProvider = config.byok_provider;
            if (config.active_model) {
                this.activeModel = config.active_model;
            }
            this.apiKeyPlaceholder = config.api_key;
        } catch (e) {
            console.error('Failed to load AI config:', e);
        }
    }

    async saveConfig() {
        try {
            const config: AiSettingsConfig = {
                mode: this.mode,
                universal_endpoint: this.universalEndpoint,
                byok_provider: this.byokProvider,
                active_model: this.activeModel,
                api_key: this.pendingApiKey ? this.pendingApiKey : null
            };

            await invoke('save_ai_config', { config });
            this.pendingApiKey = ''; // Clear after saving
            await this.loadConfig(); // Reload to get placeholder
        } catch (e) {
            console.error('Failed to save AI config:', e);
        }
    }

    async testConnection() {
        this.testLoading = true;
        this.testResult = null;
        try {
             const config: AiSettingsConfig = {
                mode: this.mode,
                universal_endpoint: this.universalEndpoint,
                byok_provider: this.byokProvider,
                active_model: this.activeModel,
                api_key: null
            };
            console.log('[aiConfig] Testing connection with config:', config);
            const result = await invoke<string>('test_ai_endpoint', { config });
            console.log('[aiConfig] Test result:', result);
            this.testResult = { success: true, message: result };
            
            // Trigger toast notification
            const { toastState } = await import('$lib/state/toast.svelte');
            toastState.success(result, 4500);
        } catch (e: any) {
            console.error('[aiConfig] Test failed:', e);
            const errStr = e?.message || e?.toString() || 'Unknown connection error';
            this.testResult = { success: false, message: errStr };
            
            // Trigger toast notification for error
            const { toastState } = await import('$lib/state/toast.svelte');
            toastState.error(`Test Connection Failed: ${errStr}`, 5000);
        } finally {
            this.testLoading = false;
        }
    }

    async loadModels() {
        try {
             const models = await invoke<ModelInfo[]>('get_available_models');
             this.availableModels = models;
        } catch (e) {
            console.error('Failed to load available models:', e);
        }
    }
}

export const aiConfig = new AiConfigState();
