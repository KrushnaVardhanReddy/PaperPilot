// @ts-ignore
import init, { WasmPdfEngine } from 'paperpilot-wasm';

let initPromise: Promise<any> | null = null;

async function ensureInit() {
    if (!initPromise) {
        initPromise = init();
    }
    await initPromise;
}

self.onmessage = async (e: MessageEvent) => {
    const { id, type, payload } = e.data;

    try {
        await ensureInit();

        let result: any;

        switch (type) {
            case 'merge':
                result = WasmPdfEngine.merge(payload.files);
                break;
            case 'rotate':
                result = WasmPdfEngine.rotate(payload.file, payload.angle, payload.pages);
                break;
            case 'split':
                result = WasmPdfEngine.split(payload.file, payload.ranges);
                break;
            case 'compress':
                result = WasmPdfEngine.compress(payload.file);
                break;
            case 'encrypt':
                result = WasmPdfEngine.encrypt(payload.file, payload.password);
                break;
            case 'watermark':
                result = WasmPdfEngine.watermark(payload.file, payload.text);
                break;
            case 'delete_pages':
                result = WasmPdfEngine.delete_pages(payload.file, payload.pages);
                break;
            case 'extract_pages':
                result = WasmPdfEngine.extract_pages(payload.file, payload.pages);
                break;
            case 'reorder_pages':
                result = WasmPdfEngine.reorder_pages(payload.file, new Uint32Array(payload.new_order));
                break;
            case 'crop':
                result = WasmPdfEngine.crop(payload.file, payload.left, payload.bottom, payload.right, payload.top);
                break;
            case 'flatten':
                result = WasmPdfEngine.flatten(payload.file);
                break;
            case 'set_metadata':
                result = WasmPdfEngine.set_metadata(payload.file, payload.title, payload.author, payload.subject, payload.keywords);
                break;
            case 'pdf_to_docx':
                result = WasmPdfEngine.pdf_to_docx(payload.file);
                break;
            default:
                throw new Error(`Unknown operation type: ${type}`);
        }

        self.postMessage({
            id,
            success: true,
            result
        });
    } catch (error: any) {
        self.postMessage({
            id,
            success: false,
            error: error.message || error.toString()
        });
    }
};