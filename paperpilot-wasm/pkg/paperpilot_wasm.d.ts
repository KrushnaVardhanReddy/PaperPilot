/* tslint:disable */
/* eslint-disable */

/**
 * Chroma subsampling format
 */
export enum ChromaSampling {
    /**
     * Both vertically and horizontally subsampled.
     */
    Cs420 = 0,
    /**
     * Horizontally subsampled.
     */
    Cs422 = 1,
    /**
     * Not subsampled.
     */
    Cs444 = 2,
    /**
     * Monochrome.
     */
    Cs400 = 3,
}

/**
 * A line of text that has been detected, but not recognized.
 *
 * This contains information about the location of the text, but not the
 * string contents.
 */
export class DetectedLine {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    rotatedRect(): RotatedRect;
    words(): RotatedRect[];
}

/**
 * A pre-processed image that can be passed as input to `OcrEngine.loadImage`.
 */
export class Image {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Return the number of channels in the image.
     */
    channels(): number;
    /**
     * Return the image data in row-major, channels-last order.
     */
    data(): Uint8Array;
    /**
     * Return the height of the image.
     */
    height(): number;
    /**
     * Return the width of the image.
     */
    width(): number;
}

/**
 * OcrEngine is the main API for performing OCR in WebAssembly.
 */
export class OcrEngine {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Detect text in an image.
     *
     * Returns a list of lines that were found. These can be passed to
     * `recognizeText` identify the characters.
     */
    detectText(image: Image): DetectedLine[];
    /**
     * Detect and recognize text in an image.
     *
     * Returns a list of `TextLine` objects that can be used to query the text
     * and bounding boxes of each line.
     */
    getTextLines(image: Image): TextLine[];
    /**
     * Detect and recognize text in an image.
     *
     * Returns a single string containing all the text found in reading order.
     */
    getText(image: Image): string;
    /**
     * Prepare an image for analysis by the OCR engine.
     *
     * The image is an array of pixels in row-major, channels last order. This
     * matches the format of the
     * [ImageData](https://developer.mozilla.org/en-US/docs/Web/API/ImageData)
     * API. Supported channel combinations are RGB and RGBA. The number of
     * channels is inferred from the length of `data`.
     */
    loadImage(width: number, height: number, data: Uint8Array): Image;
    /**
     * Construct a new `OcrEngine` using the models and other settings given
     * by `init`.
     *
     * To detect text in an image, `init` must have a detection model set.
     * To recognize text, `init` must have a recognition model set.
     */
    constructor(init: OcrEngineInit);
    /**
     * Recognize text that was previously detected with `detectText`.
     *
     * Returns a list of `TextLine` objects that can be used to query the text
     * and bounding boxes of each line.
     */
    recognizeText(image: Image, lines: DetectedLine[]): TextLine[];
}

/**
 * Options for constructing an [OcrEngine].
 */
export class OcrEngineInit {
    free(): void;
    [Symbol.dispose](): void;
    constructor();
    /**
     * Load a model for text detection.
     */
    setDetectionModel(data: Uint8Array): void;
    /**
     * Load a model for text recognition.
     */
    setRecognitionModel(data: Uint8Array): void;
}

export class PdfInspector {
    free(): void;
    [Symbol.dispose](): void;
    contains_text(text: string): boolean;
    form_fields(): any;
    get_text(page: number): string;
    is_encrypted(): boolean;
    constructor(bytes: Uint8Array);
    page_count(): number;
    page_dimensions(page: number): any;
    sha256_hex(): string;
}

export class RotatedRect {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Return the coordinates of the axis-aligned bounding rectangle of this
     * rotated rect.
     *
     * The result is a `[left, top, right, bottom]` array of coordinates.
     */
    boundingRect(): Float32Array;
    /**
     * Return an array of the X and Y coordinates of corners of this rectangle,
     * arranged as `[x0, y0, ... x3, y3]`.
     */
    corners(): Float32Array;
}

/**
 * A sequence of `TextWord`s that were recognized, forming a line.
 */
export class TextLine {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    text(): string;
    words(): TextWord[];
}

/**
 * Bounding box and text of a word that was recognized.
 */
export class TextWord {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Return the oriented bounding rectangle containing the characters in
     * this word.
     */
    rotatedRect(): RotatedRect;
    text(): string;
}

export class WasmPdfEngine {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Compress PDF by removing unneeded metadata, deflating streams, and cleaning cross-references
     */
    static compress(input_bytes: Uint8Array, quality?: string | null): Uint8Array;
    static crop(input_bytes: Uint8Array, left: number, bottom: number, right: number, top: number): Uint8Array;
    static decrypt(input_bytes: Uint8Array, password: string): Uint8Array;
    static delete_pages(input_bytes: Uint8Array, pages: string): Uint8Array;
    /**
     * Encrypt PDF with user password (standard 128/256-bit)
     */
    static encrypt(input_bytes: Uint8Array, password: string): Uint8Array;
    static extract_images(input_bytes: Uint8Array): Array<any>;
    static extract_pages(input_bytes: Uint8Array, pages: string): Uint8Array;
    static extract_text(input_bytes: Uint8Array): string;
    static flatten(input_bytes: Uint8Array): Uint8Array;
    static header_footer(input_bytes: Uint8Array, header: string, footer: string): Uint8Array;
    static images_to_pdf(image_buffers: Array<any>): Uint8Array;
    /**
     * Convert JSON document into a PDF Uint8Array
     */
    static json_to_pdf(json_str: string): Uint8Array;
    static list_tools(): string;
    /**
     * Merge multiple PDF buffers (Array of Uint8Array) into a single PDF Uint8Array
     */
    static merge(buffers: Array<any>): Uint8Array;
    static ocr(image_or_pdf_bytes: Uint8Array): string;
    static page_numbers(input_bytes: Uint8Array, format: string, position: string): Uint8Array;
    static pdf_attachments(input_bytes: Uint8Array, options_json: string): Uint8Array;
    static pdf_auto_rotate(input_bytes: Uint8Array, options_json: string): Uint8Array;
    static pdf_booklet(input_bytes: Uint8Array, options_json: string): Uint8Array;
    static pdf_change_permissions(input_bytes: Uint8Array, options_json: string): Uint8Array;
    static pdf_comic_archive(input_bytes: Uint8Array, options_json: string): Uint8Array;
    static pdf_hash(input_bytes: Uint8Array): string;
    static pdf_info(input_bytes: Uint8Array): string;
    static pdf_nup(input_bytes: Uint8Array, options_json: string): Uint8Array;
    static pdf_overlay_underlay(input_bytes: Uint8Array, options_json: string): Uint8Array;
    static pdf_remove_signatures(input_bytes: Uint8Array, options_json: string): Uint8Array;
    static pdf_scale_pages(input_bytes: Uint8Array, options_json: string): Uint8Array;
    static pdf_split_by_outline(input_bytes: Uint8Array, options_json: string): Uint8Array;
    static pdf_split_scanned_book(input_bytes: Uint8Array, options_json: string): Uint8Array;
    static pdf_stamp(input_bytes: Uint8Array, options_json: string): Uint8Array;
    static pdf_stitch_canvas(input_bytes: Uint8Array, options_json: string): Uint8Array;
    static pdf_timestamp(input_bytes: Uint8Array, options_json: string): Uint8Array;
    static pdf_to_docx(input_bytes: Uint8Array): Uint8Array;
    /**
     * List all registered tools and their availability in the WASM engine
     */
    static pdf_validate_signatures(input_bytes: Uint8Array, options_json: string): Uint8Array;
    static pii_scan(input_bytes: Uint8Array, options_json: string): string;
    static render_page(input_bytes: Uint8Array, page_index: number, scale: number): Uint8Array;
    static reorder_pages(input_bytes: Uint8Array, new_order: Uint32Array): Uint8Array;
    /**
     * Rotate specific pages ("all", "1,2", or "1") by angle (90, 180, 270)
     */
    static rotate(input_bytes: Uint8Array, angle: number, pages: string): Uint8Array;
    static sanitize(input_bytes: Uint8Array, options_json: string): Uint8Array;
    static set_metadata(input_bytes: Uint8Array, title?: string | null, author?: string | null, subject?: string | null, keywords?: string | null): Uint8Array;
    /**
     * Split PDF by page ranges (e.g. "1-2, 3-4") returning an Array of Uint8Arrays
     */
    static split(input_bytes: Uint8Array, ranges: string): Array<any>;
    /**
     * Add watermark text across pages
     */
    static watermark(input_bytes: Uint8Array, text: string, angle?: number | null, opacity?: number | null): Uint8Array;
    static workflow(input_bytes: Uint8Array, recipe_json: string): Uint8Array;
    static workflow_presets(): string;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_detectedline_free: (a: number, b: number) => void;
    readonly __wbg_image_free: (a: number, b: number) => void;
    readonly __wbg_ocrengine_free: (a: number, b: number) => void;
    readonly __wbg_ocrengineinit_free: (a: number, b: number) => void;
    readonly __wbg_pdfinspector_free: (a: number, b: number) => void;
    readonly __wbg_rotatedrect_free: (a: number, b: number) => void;
    readonly __wbg_textline_free: (a: number, b: number) => void;
    readonly __wbg_textword_free: (a: number, b: number) => void;
    readonly __wbg_wasmpdfengine_free: (a: number, b: number) => void;
    readonly detectedline_rotatedRect: (a: number) => number;
    readonly detectedline_words: (a: number, b: number) => void;
    readonly image_channels: (a: number) => number;
    readonly image_data: (a: number, b: number) => void;
    readonly image_height: (a: number) => number;
    readonly image_width: (a: number) => number;
    readonly ocrengine_detectText: (a: number, b: number, c: number) => void;
    readonly ocrengine_getText: (a: number, b: number, c: number) => void;
    readonly ocrengine_getTextLines: (a: number, b: number, c: number) => void;
    readonly ocrengine_loadImage: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
    readonly ocrengine_new: (a: number, b: number) => void;
    readonly ocrengine_recognizeText: (a: number, b: number, c: number, d: number, e: number) => void;
    readonly ocrengineinit_new: () => number;
    readonly ocrengineinit_setDetectionModel: (a: number, b: number, c: number, d: number) => void;
    readonly ocrengineinit_setRecognitionModel: (a: number, b: number, c: number, d: number) => void;
    readonly pdfinspector_contains_text: (a: number, b: number, c: number) => number;
    readonly pdfinspector_form_fields: (a: number, b: number) => void;
    readonly pdfinspector_get_text: (a: number, b: number, c: number) => void;
    readonly pdfinspector_is_encrypted: (a: number) => number;
    readonly pdfinspector_new: (a: number, b: number, c: number) => void;
    readonly pdfinspector_page_count: (a: number) => number;
    readonly pdfinspector_page_dimensions: (a: number, b: number, c: number) => void;
    readonly pdfinspector_sha256_hex: (a: number, b: number) => void;
    readonly rotatedrect_boundingRect: (a: number, b: number) => void;
    readonly rotatedrect_corners: (a: number, b: number) => void;
    readonly textline_text: (a: number, b: number) => void;
    readonly textline_words: (a: number, b: number) => void;
    readonly textword_rotatedRect: (a: number) => number;
    readonly textword_text: (a: number, b: number) => void;
    readonly wasmpdfengine_compress: (a: number, b: number, c: number, d: number, e: number) => void;
    readonly wasmpdfengine_crop: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
    readonly wasmpdfengine_decrypt: (a: number, b: number, c: number, d: number, e: number) => void;
    readonly wasmpdfengine_delete_pages: (a: number, b: number, c: number, d: number, e: number) => void;
    readonly wasmpdfengine_encrypt: (a: number, b: number, c: number, d: number, e: number) => void;
    readonly wasmpdfengine_extract_images: (a: number, b: number, c: number) => void;
    readonly wasmpdfengine_extract_pages: (a: number, b: number, c: number, d: number, e: number) => void;
    readonly wasmpdfengine_extract_text: (a: number, b: number, c: number) => void;
    readonly wasmpdfengine_flatten: (a: number, b: number, c: number) => void;
    readonly wasmpdfengine_header_footer: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
    readonly wasmpdfengine_images_to_pdf: (a: number, b: number) => void;
    readonly wasmpdfengine_json_to_pdf: (a: number, b: number, c: number) => void;
    readonly wasmpdfengine_list_tools: (a: number) => void;
    readonly wasmpdfengine_merge: (a: number, b: number) => void;
    readonly wasmpdfengine_ocr: (a: number, b: number, c: number) => void;
    readonly wasmpdfengine_page_numbers: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
    readonly rust_zstd_wasm_shim_calloc: (a: number, b: number) => number;
    readonly rust_zstd_wasm_shim_free: (a: number) => void;
    readonly rust_zstd_wasm_shim_malloc: (a: number) => number;
    readonly rust_zstd_wasm_shim_memcmp: (a: number, b: number, c: number) => number;
    readonly rust_zstd_wasm_shim_memcpy: (a: number, b: number, c: number) => number;
    readonly rust_zstd_wasm_shim_memmove: (a: number, b: number, c: number) => number;
    readonly rust_zstd_wasm_shim_memset: (a: number, b: number, c: number) => number;
    readonly rust_zstd_wasm_shim_qsort: (a: number, b: number, c: number, d: number) => void;
    readonly wasmpdfengine_pdf_attachments: (a: number, b: number, c: number, d: number, e: number) => void;
    readonly wasmpdfengine_pdf_auto_rotate: (a: number, b: number, c: number, d: number, e: number) => void;
    readonly wasmpdfengine_pdf_booklet: (a: number, b: number, c: number, d: number, e: number) => void;
    readonly wasmpdfengine_pdf_change_permissions: (a: number, b: number, c: number, d: number, e: number) => void;
    readonly wasmpdfengine_pdf_comic_archive: (a: number, b: number, c: number, d: number, e: number) => void;
    readonly wasmpdfengine_pdf_hash: (a: number, b: number, c: number) => void;
    readonly wasmpdfengine_pdf_info: (a: number, b: number, c: number) => void;
    readonly wasmpdfengine_pdf_nup: (a: number, b: number, c: number, d: number, e: number) => void;
    readonly wasmpdfengine_pdf_overlay_underlay: (a: number, b: number, c: number, d: number, e: number) => void;
    readonly wasmpdfengine_pdf_remove_signatures: (a: number, b: number, c: number, d: number, e: number) => void;
    readonly wasmpdfengine_pdf_scale_pages: (a: number, b: number, c: number, d: number, e: number) => void;
    readonly wasmpdfengine_pdf_split_by_outline: (a: number, b: number, c: number, d: number, e: number) => void;
    readonly wasmpdfengine_pdf_split_scanned_book: (a: number, b: number, c: number, d: number, e: number) => void;
    readonly wasmpdfengine_pdf_stamp: (a: number, b: number, c: number, d: number, e: number) => void;
    readonly wasmpdfengine_pdf_stitch_canvas: (a: number, b: number, c: number, d: number, e: number) => void;
    readonly wasmpdfengine_pdf_timestamp: (a: number, b: number, c: number, d: number, e: number) => void;
    readonly wasmpdfengine_pdf_to_docx: (a: number, b: number, c: number) => void;
    readonly wasmpdfengine_pdf_validate_signatures: (a: number, b: number, c: number, d: number, e: number) => void;
    readonly wasmpdfengine_pii_scan: (a: number, b: number, c: number, d: number, e: number) => void;
    readonly wasmpdfengine_render_page: (a: number, b: number, c: number, d: number, e: number) => void;
    readonly wasmpdfengine_reorder_pages: (a: number, b: number, c: number, d: number, e: number) => void;
    readonly wasmpdfengine_rotate: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
    readonly wasmpdfengine_sanitize: (a: number, b: number, c: number, d: number, e: number) => void;
    readonly wasmpdfengine_set_metadata: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number) => void;
    readonly wasmpdfengine_split: (a: number, b: number, c: number, d: number, e: number) => void;
    readonly wasmpdfengine_watermark: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
    readonly wasmpdfengine_workflow: (a: number, b: number, c: number, d: number, e: number) => void;
    readonly wasmpdfengine_workflow_presets: (a: number) => void;
    readonly __wbindgen_export: (a: number) => void;
    readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
    readonly __wbindgen_export2: (a: number, b: number, c: number) => void;
    readonly __wbindgen_export3: (a: number, b: number) => number;
    readonly __wbindgen_export4: (a: number, b: number, c: number, d: number) => number;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
