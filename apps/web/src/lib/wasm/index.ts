export interface WasmPdfClient {
    merge(files: Uint8Array[]): Promise<Uint8Array>;
    rotate(file: Uint8Array, angle: number, pages?: string): Promise<Uint8Array>;
    split(file: Uint8Array, ranges: string): Promise<Uint8Array[]>;
    compress(file: Uint8Array): Promise<Uint8Array>;
    encrypt(file: Uint8Array, password: string): Promise<Uint8Array>;
    watermark(file: Uint8Array, text: string): Promise<Uint8Array>;
}

export class WasmPdfClientImpl implements WasmPdfClient {
    private worker: Worker;
    private messageIdCounter = 0;
    private pendingRequests = new Map<number, { resolve: (val: any) => void, reject: (err: any) => void }>();

    constructor() {
        this.worker = new Worker(new URL('./pdfWorker.ts', import.meta.url), { type: 'module' });
        this.worker.onmessage = this.handleMessage.bind(this);
    }

    private handleMessage(e: MessageEvent) {
        const { id, success, result, error } = e.data;
        const req = this.pendingRequests.get(id);
        if (req) {
            this.pendingRequests.delete(id);
            if (success) {
                req.resolve(result);
            } else {
                req.reject(new Error(error));
            }
        }
    }

    private sendRequest<T>(type: string, payload: any): Promise<T> {
        return new Promise((resolve, reject) => {
            const id = this.messageIdCounter++;
            this.pendingRequests.set(id, { resolve, reject });
            this.worker.postMessage({ id, type, payload });
        });
    }

    merge(files: Uint8Array[]): Promise<Uint8Array> {
        return this.sendRequest('merge', { files });
    }

    rotate(file: Uint8Array, angle: number, pages: string = 'all'): Promise<Uint8Array> {
        return this.sendRequest('rotate', { file, angle, pages });
    }

    split(file: Uint8Array, ranges: string): Promise<Uint8Array[]> {
        return this.sendRequest('split', { file, ranges });
    }

    compress(file: Uint8Array): Promise<Uint8Array> {
        return this.sendRequest('compress', { file });
    }

    encrypt(file: Uint8Array, password: string): Promise<Uint8Array> {
        return this.sendRequest('encrypt', { file, password });
    }

    watermark(file: Uint8Array, text: string): Promise<Uint8Array> {
        return this.sendRequest('watermark', { file, text });
    }
}

export const wasmPdfClient = new WasmPdfClientImpl();