import { invoke } from '@tauri-apps/api/core';

export type AiProviderMode = 'offlinenlp' | 'llamafile' | 'universal' | 'byok';

export interface AiSettingsConfig {
    mode: AiProviderMode;
    universal_endpoint: string | null;
    byok_provider: string | null;
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
             // ensure we're testing the currently configured values by sending them
             const config: AiSettingsConfig = {
                mode: this.mode,
                universal_endpoint: this.universalEndpoint,
                byok_provider: this.byokProvider,
                api_key: null // doesn't matter for the ping in our mock, but in real life it would
            };
            const result = await invoke<string>('test_ai_endpoint', { config });
            this.testResult = { success: true, message: result };
        } catch (e: any) {
            this.testResult = { success: false, message: e.toString() };
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
