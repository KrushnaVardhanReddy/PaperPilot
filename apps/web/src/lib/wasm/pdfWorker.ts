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
            case 'images_to_pdf':
                result = WasmPdfEngine.images_to_pdf(payload.images);
                break;
            case 'extract_images':
                result = WasmPdfEngine.extract_images(payload.file);
                break;
            case 'pdf_hash':
                result = WasmPdfEngine.pdf_hash(payload.file);
                break;
            
            case 'render_page':
                result = WasmPdfEngine.render_page(payload.file, payload.page_index, payload.scale);
                break;
            case 'extract_text':
                result = WasmPdfEngine.extract_text(payload.file);
                break;
            case 'decrypt':
                result = WasmPdfEngine.decrypt(payload.file, payload.password);
                break;
            case 'page_numbers':
                result = WasmPdfEngine.page_numbers(payload.file, payload.format, payload.position);
                break;
            case 'header_footer':
                result = WasmPdfEngine.header_footer(payload.file, payload.header, payload.footer);
                break;
            case 'pdf_info':
                result = WasmPdfEngine.pdf_info(payload.file);
                break;
            case 'ocr':
                result = WasmPdfEngine.ocr(payload.file);
                break;
            case 'json_to_pdf':
                result = WasmPdfEngine.json_to_pdf(payload.json_str);
                break;
            case 'pdf_nup':
                // @ts-ignore
                result = WasmPdfEngine.pdf_nup(payload.file, JSON.stringify(payload.options || {}));
                break;
            case 'pdf_booklet':
                // @ts-ignore
                result = WasmPdfEngine.pdf_booklet(payload.file, JSON.stringify(payload.options || {}));
                break;
            case 'pdf_scale_pages':
                // @ts-ignore
                result = WasmPdfEngine.pdf_scale_pages(payload.file, JSON.stringify(payload.options || {}));
                break;
            case 'pdf_auto_rotate':
                // @ts-ignore
                result = WasmPdfEngine.pdf_auto_rotate(payload.file, JSON.stringify(payload.options || {}));
                break;
            case 'pdf_stamp':
                // @ts-ignore
                result = WasmPdfEngine.pdf_stamp(payload.file, JSON.stringify(payload.options || {}));
                break;
            case 'pdf_overlay_underlay':
                // @ts-ignore
                result = WasmPdfEngine.pdf_overlay_underlay(payload.file, JSON.stringify(payload.options || {}));
                break;
            case 'pdf_remove_signatures':
                // @ts-ignore
                result = WasmPdfEngine.pdf_remove_signatures(payload.file, JSON.stringify(payload.options || {}));
                break;
            case 'pdf_validate_signatures':
                // @ts-ignore
                result = WasmPdfEngine.pdf_validate_signatures(payload.file, JSON.stringify(payload.options || {}));
                break;
            case 'sanitize':
                // @ts-ignore
                result = WasmPdfEngine.sanitize(payload.file, JSON.stringify(payload.options || {}));
                break;
            case 'pii_scan':
                // @ts-ignore
                result = WasmPdfEngine.pii_scan(payload.file, JSON.stringify(payload.options || {}));
                break;
            case 'pdf_to_docx':
                // @ts-ignore
                result = WasmPdfEngine.pdf_to_docx(payload.file);
                break;
            case 'pdf_to_gif':
                // @ts-ignore
                result = WasmPdfEngine.pdf_to_gif(payload.file, JSON.stringify(payload.options || {}));
                break;
            case 'showcase_gif':
                // @ts-ignore
                result = WasmPdfEngine.showcase_gif(payload.file, JSON.stringify(payload.options || {}));
                break;
            case 'images_to_gif':
                // @ts-ignore
                result = WasmPdfEngine.images_to_gif(payload.file, JSON.stringify(payload.options || {}));
                break;
            case 'img_optimize_for_llm':
                // @ts-ignore
                result = WasmPdfEngine.img_optimize_for_llm(payload.file, JSON.stringify(payload.options || {}));
                break;
            case 'img_deskew':
                // @ts-ignore
                result = WasmPdfEngine.img_deskew(payload.file, JSON.stringify(payload.options || {}));
                break;
            case 'img_convert':
                // @ts-ignore
                result = WasmPdfEngine.img_convert(payload.file, JSON.stringify(payload.options || {}));
                break;
            case 'img_crop_to_content':
                // @ts-ignore
                result = WasmPdfEngine.img_crop_to_content(payload.file, JSON.stringify(payload.options || {}));
                break;
            case 'img_redact':
                // @ts-ignore
                result = WasmPdfEngine.img_redact(payload.file, JSON.stringify(payload.options || {}));
                break;
            case 'pdf_stitch_canvas':
                // @ts-ignore
                result = WasmPdfEngine.pdf_stitch_canvas(payload.file, JSON.stringify(payload.options || {}));
                break;
            case 'pdf_attachments':
                // @ts-ignore
                result = WasmPdfEngine.pdf_attachments(payload.file, JSON.stringify(payload.options || {}));
                break;
            case 'pdf_timestamp':
                // @ts-ignore
                result = WasmPdfEngine.pdf_timestamp(payload.file, JSON.stringify(payload.options || {}));
                break;
            case 'pdf_change_permissions':
                // @ts-ignore
                result = WasmPdfEngine.pdf_change_permissions(payload.file, JSON.stringify(payload.options || {}));
                break;
            case 'pdf_split_scanned_book':
                // @ts-ignore
                result = WasmPdfEngine.pdf_split_scanned_book(payload.file, JSON.stringify(payload.options || {}));
                break;
            case 'pdf_comic_archive':
                // @ts-ignore
                result = WasmPdfEngine.pdf_comic_archive(payload.file, JSON.stringify(payload.options || {}));
                break;
            case 'pdf_split_by_outline':
                // @ts-ignore
                result = WasmPdfEngine.pdf_split_by_outline(payload.file, JSON.stringify(payload.options || {}));
                break;
            case 'pdf_merge_alternate':
                // @ts-ignore
                result = WasmPdfEngine.pdf_merge_alternate(payload.primary, payload.secondary);
                break;
            case 'pdf_posterize':
                // @ts-ignore
                result = WasmPdfEngine.pdf_posterize(payload.file, JSON.stringify(payload.options || {}));
                break;
            case 'pdf_dedup_streams':
                // @ts-ignore
                result = WasmPdfEngine.pdf_dedup_streams(payload.file, JSON.stringify(payload.options || {}));
                break;
            case 'pdf_to_epub':
                // @ts-ignore
                result = WasmPdfEngine.pdf_to_epub(payload.file, JSON.stringify(payload.options || {}));
                break;
            case 'pdf_extract_patterns':
                // @ts-ignore
                result = WasmPdfEngine.pdf_extract_patterns(payload.file, JSON.stringify(payload.options || {}));
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