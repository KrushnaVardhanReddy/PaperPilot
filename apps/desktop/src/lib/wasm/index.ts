export interface WasmPdfClient {
    merge(files: Uint8Array[], options?: { zeroTrace?: boolean }): Promise<Uint8Array>;
    rotate(file: Uint8Array, angle: number, pages?: string, options?: { zeroTrace?: boolean }): Promise<Uint8Array>;
    split(file: Uint8Array, ranges: string, options?: { zeroTrace?: boolean }): Promise<Uint8Array[]>;
    compress(file: Uint8Array, options?: { zeroTrace?: boolean }): Promise<Uint8Array>;
    encrypt(file: Uint8Array, password: string, options?: { zeroTrace?: boolean }): Promise<Uint8Array>;
    watermark(file: Uint8Array, text: string, options?: { zeroTrace?: boolean }): Promise<Uint8Array>;
    delete_pages(file: Uint8Array, pages: string, options?: { zeroTrace?: boolean }): Promise<Uint8Array>;
    extract_pages(file: Uint8Array, pages: string, options?: { zeroTrace?: boolean }): Promise<Uint8Array>;
    reorder_pages(file: Uint8Array, new_order: number[], options?: { zeroTrace?: boolean }): Promise<Uint8Array>;
    crop(file: Uint8Array, left: number, bottom: number, right: number, top: number, options?: { zeroTrace?: boolean }): Promise<Uint8Array>;
    flatten(file: Uint8Array, options?: { zeroTrace?: boolean }): Promise<Uint8Array>;
    set_metadata(file: Uint8Array, title?: string, author?: string, subject?: string, keywords?: string, options?: { zeroTrace?: boolean }): Promise<Uint8Array>;
    pdf_to_docx(file: Uint8Array, options?: { zeroTrace?: boolean }): Promise<Uint8Array>;
    images_to_pdf(files: Uint8Array[], options?: { zeroTrace?: boolean }): Promise<Uint8Array>;
}

export class WasmPdfClientImpl implements WasmPdfClient {
    runOperation(type: string, payload: any, options?: { zeroTrace?: boolean }): Promise<any> {
        return this.sendRequest(type, payload, options);
    }
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

    private sendRequest<T>(type: string, payload: any, options?: { zeroTrace?: boolean }): Promise<T> {
        return new Promise((resolve, reject) => {
            const id = this.messageIdCounter++;
            if (!options?.zeroTrace) {
                console.debug(`[WASM] Starting ${type} operation`);
            }
            this.pendingRequests.set(id, { 
                resolve: (val) => {
                    if (!options?.zeroTrace) {
                        console.debug(`[WASM] Completed ${type} operation`);
                    }
                    resolve(val);
                }, 
                reject 
            });
            this.worker.postMessage({ id, type, payload, zeroTrace: options?.zeroTrace });
        });
    }

    merge(files: Uint8Array[], options?: { zeroTrace?: boolean }): Promise<Uint8Array> {
        return this.sendRequest('merge', { files }, options);
    }

    rotate(file: Uint8Array, angle: number, pages: string = 'all', options?: { zeroTrace?: boolean }): Promise<Uint8Array> {
        return this.sendRequest('rotate', { file, angle, pages }, options);
    }

    split(file: Uint8Array, ranges: string, options?: { zeroTrace?: boolean }): Promise<Uint8Array[]> {
        return this.sendRequest('split', { file, ranges }, options);
    }

    compress(file: Uint8Array, options?: { zeroTrace?: boolean }): Promise<Uint8Array> {
        return this.sendRequest('compress', { file }, options);
    }

    encrypt(file: Uint8Array, password: string, options?: { zeroTrace?: boolean }): Promise<Uint8Array> {
        return this.sendRequest('encrypt', { file, password }, options);
    }

    watermark(file: Uint8Array, text: string, options?: { zeroTrace?: boolean }): Promise<Uint8Array> {
        return this.sendRequest('watermark', { file, text }, options);
    }

    delete_pages(file: Uint8Array, pages: string, options?: { zeroTrace?: boolean }): Promise<Uint8Array> {
        return this.sendRequest('delete_pages', { file, pages }, options);
    }

    extract_pages(file: Uint8Array, pages: string, options?: { zeroTrace?: boolean }): Promise<Uint8Array> {
        return this.sendRequest('extract_pages', { file, pages }, options);
    }

    reorder_pages(file: Uint8Array, new_order: number[], options?: { zeroTrace?: boolean }): Promise<Uint8Array> {
        return this.sendRequest('reorder_pages', { file, new_order }, options);
    }

    crop(file: Uint8Array, left: number, bottom: number, right: number, top: number, options?: { zeroTrace?: boolean }): Promise<Uint8Array> {
        return this.sendRequest('crop', { file, left, bottom, right, top }, options);
    }

    flatten(file: Uint8Array, options?: { zeroTrace?: boolean }): Promise<Uint8Array> {
        return this.sendRequest('flatten', { file }, options);
    }

    set_metadata(file: Uint8Array, title?: string, author?: string, subject?: string, keywords?: string, options?: { zeroTrace?: boolean }): Promise<Uint8Array> {
        return this.sendRequest('set_metadata', { file, title, author, subject, keywords }, options);
    }

    pdf_to_docx(file: Uint8Array, options?: { zeroTrace?: boolean }): Promise<Uint8Array> {
        return this.sendRequest('pdf_to_docx', { file }, options);
    }

    images_to_pdf(files: Uint8Array[], options?: { zeroTrace?: boolean }): Promise<Uint8Array> {
        return this.sendRequest('images_to_pdf', { files }, options);
    }
}

export const wasmPdfClient = new WasmPdfClientImpl();