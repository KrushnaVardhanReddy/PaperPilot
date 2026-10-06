export interface WasmPdfClient {
    merge(files: Uint8Array[]): Promise<Uint8Array>;
    rotate(file: Uint8Array, angle: number, pages?: string): Promise<Uint8Array>;
    split(file: Uint8Array, ranges: string): Promise<Uint8Array[]>;
    compress(file: Uint8Array): Promise<Uint8Array>;
    encrypt(file: Uint8Array, password: string): Promise<Uint8Array>;
    watermark(file: Uint8Array, text: string): Promise<Uint8Array>;
    delete_pages(file: Uint8Array, pages: string): Promise<Uint8Array>;
    extract_pages(file: Uint8Array, pages: string): Promise<Uint8Array>;
    reorder_pages(file: Uint8Array, new_order: number[]): Promise<Uint8Array>;
    crop(file: Uint8Array, left: number, bottom: number, right: number, top: number): Promise<Uint8Array>;
flatten(file: Uint8Array): Promise<Uint8Array>;
    set_metadata(file: Uint8Array, title?: string, author?: string, subject?: string, keywords?: string): Promise<Uint8Array>;
    images_to_pdf(images: Uint8Array[]): Promise<Uint8Array>;
    extract_images(file: Uint8Array): Promise<Uint8Array[]>;
    pdf_hash(file: Uint8Array): Promise<string>;
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

    delete_pages(file: Uint8Array, pages: string): Promise<Uint8Array> {
        return this.sendRequest('delete_pages', { file, pages });
    }

    extract_pages(file: Uint8Array, pages: string): Promise<Uint8Array> {
        return this.sendRequest('extract_pages', { file, pages });
    }

    reorder_pages(file: Uint8Array, new_order: number[]): Promise<Uint8Array> {
        return this.sendRequest('reorder_pages', { file, new_order });
    }

    crop(file: Uint8Array, left: number, bottom: number, right: number, top: number): Promise<Uint8Array> {
        return this.sendRequest('crop', { file, left, bottom, right, top });
    }

    flatten(file: Uint8Array): Promise<Uint8Array> {
        return this.sendRequest('flatten', { file });
    }

set_metadata(file: Uint8Array, title?: string, author?: string, subject?: string, keywords?: string): Promise<Uint8Array> {
        return this.sendRequest('set_metadata', { file, title, author, subject, keywords });
    }

    images_to_pdf(images: Uint8Array[]): Promise<Uint8Array> {
        return this.sendRequest('images_to_pdf', { images });
    }

    extract_images(file: Uint8Array): Promise<Uint8Array[]> {
        return this.sendRequest('extract_images', { file });
    }

    pdf_hash(file: Uint8Array): Promise<string> {
        return this.sendRequest('pdf_hash', { file });
    }
}

export const wasmPdfClient = new WasmPdfClientImpl();