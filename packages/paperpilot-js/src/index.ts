export interface PaperPilotOptions {
    endpointUrl?: string;
    apiKey?: string;
}

export interface PaperPilotResponse {
    success?: boolean;
    stdout?: string;
    [key: string]: any;
}

export class PaperPilot {
    private endpointUrl?: string;
    private apiKey?: string;

    constructor(options?: PaperPilotOptions) {
        this.endpointUrl = options?.endpointUrl;
        this.apiKey = options?.apiKey;
    }

    private get isLocal(): boolean {
        return !this.endpointUrl;
    }

    private async execLocal(commandArgs: string[]): Promise<PaperPilotResponse> {
        // Dynamic import to allow browser builds to safely fail or ignore if not using local execution
        const { execFile } = await import('child_process');
        const { promisify } = await import('util');
        const execFileAsync = promisify(execFile);

        const { stdout } = await execFileAsync('paperpilot-cli', commandArgs);
        return { success: true, stdout };
    }

    private async execRemote(endpoint: string, payload: any): Promise<PaperPilotResponse> {
        if (!this.endpointUrl) {
            throw new Error('Remote execution requires an endpointUrl');
        }
        const url = `${this.endpointUrl.replace(/\/$/, '')}${endpoint}`;
        const headers: Record<string, string> = {
            'Content-Type': 'application/json',
        };
        if (this.apiKey) {
            headers['Authorization'] = `Bearer ${this.apiKey}`;
        }

        const response = await fetch(url, {
            method: 'POST',
            headers,
            body: JSON.stringify(payload)
        });

        if (!response.ok) {
            throw new Error(`PaperPilot API error: ${response.status} ${response.statusText}`);
        }
        return response.json();
    }

    public async merge(inputs: string[], options: { output: string }): Promise<PaperPilotResponse> {
        if (this.isLocal) {
            const args = ['merge'];
            for (const i of inputs) {
                args.push('--input', i);
            }
            args.push('--output', options.output);
            return this.execLocal(args);
        }
        return this.execRemote('/api/v1/pdf/merge', {
            inputs: inputs,
            output: options.output
        });
    }

    public async compress(input: string, options: { quality: string, output: string }): Promise<PaperPilotResponse> {
        if (this.isLocal) {
            const args = ['compress', '--input', input, '--quality', options.quality, '--output', options.output];
            return this.execLocal(args);
        }
        return this.execRemote('/api/v1/pdf/compress', {
            input: input,
            quality: options.quality,
            output: options.output
        });
    }

    public async batesStamp(input: string, options: { prefix: string, start: number, output: string }): Promise<PaperPilotResponse> {
        if (this.isLocal) {
            const args = ['bates', '--input', input, '--prefix', options.prefix, '--start', String(options.start), '--output', options.output];
            return this.execLocal(args);
        }
        throw new Error('batesStamp is local-only');
    }

    public async split(input: string, options: { pages: string, output: string }): Promise<PaperPilotResponse> {
        if (this.isLocal) {
            const args = ['split', '--input', input, '--pages', options.pages, '--output', options.output];
            return this.execLocal(args);
        }
        return this.execRemote('/api/v1/pdf/split', {
            input: input,
            pages: options.pages,
            output: options.output
        });
    }
}
