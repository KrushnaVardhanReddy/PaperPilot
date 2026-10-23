import { appState } from '$lib/state/app.svelte';

export interface OperationContext {
    docName: string;
    docPath: string;
    docDir: string;
    allDocumentPaths: string[];
    allDocuments: { name: string }[];
    getOutputPath: (filename: string) => string;
    customOutputPath: string;

    // UI control values
    splitPoints: string;
    deletePages?: string;
    extractPages?: string;
  tableFormat?: string;
    rotateAngle: string;
    rotatePages?: string;
    watermarkText: string;
    watermarkOrientation: string;
    watermarkOpacity: number;
    watermarkColor: string;
    watermarkFontSize: number;
    protectPassword?: string;
    password?: string;
    batesPrefix: string;
    batesStart: number;
    batesPadding: number;
    ocrOutputFormat: string;
    ocrLanguage: string;
    cropBox: string;
    pageNumberPosition: string;
    pageNumberFormat: string;
    pageNumberStart: number;
    convertPreset: string;
    reorderList: string;
    signCertPath: string;
    headerText: string;
    footerText: string;
    metadataTitle: string;
    compressQuality: number;
    blankSensitivity?: number;
    searchQuery?: string;
}

export async function buildOperationArgs(toolId: string, context: OperationContext): Promise<Record<string, any> | null> {
    const {
        docName, docPath, getOutputPath, customOutputPath,
        splitPoints, deletePages, extractPages, rotateAngle, watermarkText, watermarkOrientation, watermarkOpacity, watermarkColor,
        password, batesPrefix, batesStart, batesPadding, ocrOutputFormat, cropBox,
        convertPreset, reorderList, signCertPath, headerText, footerText, metadataTitle, compressQuality,
        blankSensitivity, searchQuery
    } = context;

    let args: Record<string, any> = {};

    switch (toolId) {
        case 'office_to_markdown':
        case 'office_extract_text':
        case 'office_modernize_legacy':
            args = { input: docPath, output: customOutputPath || getOutputPath(`${docName.replace(/\.[^/.]+$/, "")}_output`) };
            break;
        case 'merge': {
            let outputPath: string | null = customOutputPath;
            const suggested = getOutputPath(`${docName.replace(/\.pdf$/i, '')}_merged.pdf`);
            if (!outputPath && !appState.isMobile) {
                try {
                    const { save } = await import('@tauri-apps/plugin-dialog');
                    outputPath = await save({
                        title: 'Save Merged PDF As...',
                        defaultPath: suggested,
                        filters: [{ name: 'PDF Document', extensions: ['pdf'] }]
                    });
                } catch (err) {
                    console.warn('Tauri dialog unavailable, falling back:', err);
                }
            }
            const finalPath = outputPath || suggested;
            args = {
                inputs: context.allDocumentPaths.map((p, i) => p || context.allDocuments[i].name),
                output: finalPath
            };
            break;
        }
        case 'split': {
            const defaultSplitDir = getOutputPath(`${docName.replace(/\.pdf$/i, '')}_split`);
            const targetDir = customOutputPath || defaultSplitDir;
            args = {
                input: docPath,
                output_dir: targetDir,
                output: targetDir,
                output_pattern: `${targetDir}/p%d.pdf`,
                ranges: splitPoints
            };
            break;
        }
        case 'md_to_pdf': {
            let selected: any, outputPath: string | null = null;
            try {
                const { open, save } = await import('@tauri-apps/plugin-dialog');
                selected = await open({
                    title: 'Select Markdown File',
                    multiple: false,
                    filters: [{ name: 'Markdown', extensions: ['md', 'markdown'] }]
                });
                if (!selected) return null;
                const suggested = getOutputPath('output.pdf');
                outputPath = await save({
                    title: 'Save PDF As...',
                    defaultPath: suggested,
                    filters: [{ name: 'PDF Document', extensions: ['pdf'] }]
                });
                if (!outputPath) return null;
            } catch(e) {
                return null;
            }
            const inputPath = Array.isArray(selected) ? selected[0] : selected;
            args = {
                input: inputPath,
                output: outputPath,
                preset: convertPreset
            };
            break;
        }
        case 'html_to_pdf': {
            let selected: any, outputPath: string | null = null;
            try {
                const { open, save } = await import('@tauri-apps/plugin-dialog');
                selected = await open({
                    title: 'Select HTML File',
                    multiple: false,
                    filters: [{ name: 'HTML', extensions: ['html', 'htm'] }]
                });
                if (!selected) return null;
                const suggested = getOutputPath('output.pdf');
                outputPath = await save({
                    title: 'Save PDF As...',
                    defaultPath: suggested,
                    filters: [{ name: 'PDF Document', extensions: ['pdf'] }]
                });
                if (!outputPath) return null;
            } catch(e) {
                return null;
            }
            const inputPath = Array.isArray(selected) ? selected[0] : selected;
            args = {
                input: inputPath,
                output: outputPath,
                preset: convertPreset
            };
            break;
        }
        case 'img_to_pdf': {
            let selected: any, outputPath: string | null = null;
            try {
                const { open, save } = await import('@tauri-apps/plugin-dialog');
                selected = await open({
                    title: 'Select Images',
                    multiple: true,
                    filters: [{ name: 'Images', extensions: ['png', 'jpg', 'jpeg'] }]
                });
                if (!selected || selected.length === 0) return null;
                const suggested = getOutputPath('output.pdf');
                outputPath = await save({
                    title: 'Save PDF As...',
                    defaultPath: suggested,
                    filters: [{ name: 'PDF Document', extensions: ['pdf'] }]
                });
                if (!outputPath) return null;
            } catch(e) {
                return null;
            }
            const inputPaths = Array.isArray(selected) ? selected : [selected];
            args = {
                inputs: inputPaths,
                output: outputPath
            };
            break;
        }
        case 'json_to_pdf': {
            let selected: any, outputPath: string | null = null;
            try {
                const { open, save } = await import('@tauri-apps/plugin-dialog');
                selected = await open({
                    title: 'Select Structured JSON File',
                    multiple: false,
                    filters: [{ name: 'JSON Document', extensions: ['json'] }]
                });
                if (!selected) return null;
                const suggested = getOutputPath('synthesized.pdf');
                outputPath = await save({
                    title: 'Save Synthesized PDF As...',
                    defaultPath: suggested,
                    filters: [{ name: 'PDF Document', extensions: ['pdf'] }]
                });
                if (!outputPath) return null;
            } catch(e) {
                return null;
            }
            const inputPath = Array.isArray(selected) ? selected[0] : selected;
            args = {
                input: inputPath,
                output: outputPath
            };
            break;
        }
        case 'compress':
            args = {
                input: docPath,
                quality: compressQuality.toString(),
                output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}_compressed.pdf`)
            };
            break;
        case 'rotate':
            args = {
                input: docPath,
                pages: context.rotatePages && context.rotatePages.trim() !== '' ? context.rotatePages.trim() : 'all',
                angle: parseInt(rotateAngle, 10),
                output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}_rotated.pdf`)
            };
            break;
        case 'watermark':
            args = {
                input: docPath,
                text: watermarkText || 'CONFIDENTIAL',
                angle: watermarkOrientation === 'diagonal' ? 45 : 0,
                opacity: watermarkOpacity / 100,
                color: watermarkColor,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}_watermarked.pdf`)
            };
            break;
        case 'delete_pages':
            args = {
                input: docPath,
                pages: deletePages || '1',
                output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}_deleted.pdf`)
            };
            break;
        case 'reorder_pages':
            args = {
                input: docPath,
                order: reorderList || '1',
                output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}_reordered.pdf`)
            };
            break;
        case 'burst':
            args = {
                input: docPath,
                output_dir: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}_burst`)
            };
            break;
        case 'crop':
            args = {
                input: docPath,
                box: cropBox || '0,0,100,100',
                output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}_cropped.pdf`)
            };
            break;
        case 'redact':
            args = {
                input: docPath,
                regions: '0,0,100,100', // Provide a default/simple region or add an input if needed
                output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}_redacted.pdf`)
            };
            break;
        case 'sign':
            args = {
                input: docPath,
                cert: signCertPath,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}_signed.pdf`)
            };
            break;
        case 'remove_blank':
            args = {
                input: docPath,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}_cleaned.pdf`),
                sensitivity: blankSensitivity !== undefined ? blankSensitivity : 95
            };
            break;
        case 'validate':
        case 'hash':
        case 'read_form':
        case 'bookmarks':
        case 'classify_type':
            args = {
                input: docPath
            };
            break;
        case 'search':
            args = {
                input: docPath,
                query: searchQuery && searchQuery.trim() !== '' ? searchQuery.trim() : 'PaperPilot'
            };
            break;
        case 'bates':
            args = {
                input: docPath,
                prefix: batesPrefix,
                start_number: batesStart,
                padding: batesPadding,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}_bates.pdf`)
            };
            break;
        case 'header_footer':
            args = {
                input: docPath,
                header_left: headerText,
                footer_center: footerText,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}_header.pdf`)
            };
            break;
        case 'render':
            args = {
                input: docPath,
                page: 1,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}_rendered.png`)
            };
            break;
        case 'fill_form':
            args = {
                input: docPath,
                values: {}, // You'd likely want to pass actual values here or add UI for it
                output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}_filled.pdf`)
            };
            break;
        case 'create_form_field':
            args = {
                input: docPath,
                field_name: 'new_field', // Add to UI if needed
                field_type: 'text',
                x: 50.0,
                y: 50.0,
                width: 100.0,
                height: 30.0,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}_field.pdf`)
            };
            break;
        case 'linearize':
            args = {
                input: docPath,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}_linearized.pdf`)
            };
            break;
        case 'annotate':
            args = {
                input: docPath,
                annotations: [], // Placeholder for annotations
                output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}_annotated.pdf`)
            };
            break;
        case 'encrypt':
            args = {
                input: docPath,
                password: password || 'paperpilot',
                output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}_encrypted.pdf`)
            };
            break;
        case 'decrypt':
            args = {
                input: docPath,
                password: password || '',
                output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}_decrypted.pdf`)
            };
            break;
        case 'metadata':
            args = {
                input: docPath,
                title: metadataTitle || docName,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}_metadata.pdf`)
            };
            break;
        case 'extract_pages':
            args = {
                input: docPath,
                pages: extractPages || '1',
                output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}_extracted.pdf`)
            };
            break;
        case 'extract_text':
            args = {
                input: docPath,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}_text.txt`)
            };
            break;
        case 'ocr': {
            const defaultExt = ocrOutputFormat === 'txt' ? 'txt' : 'pdf';
            const targetOut = customOutputPath || (ocrOutputFormat === 'clipboard'
                ? getOutputPath(`temp_ocr_clipboard.txt`)
                : getOutputPath(`${docName.replace(/\.pdf$/i, '')}_ocr.${defaultExt}`));
            args = {
                input: docPath,
                output: targetOut
            };
            break;
        }
        case 'pdf_to_docx':
            args = {
                input: docPath,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}.docx`)
            };
            break;
        case 'pdf_to_xlsx':
            args = {
                input: docPath,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}.xlsx`)
            };
            break;
        case 'pdf_to_markdown':
            args = {
                input: docPath,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}.md`)
            };
            break;
        case 'pdf_to_html':
            args = {
                input: docPath,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}.html`)
            };
            break;
        case 'pdf_to_json':
            args = {
                input: docPath,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}.json`)
            };
            break;
        case 'pdf_to_pptx':
            args = {
                input: docPath,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}.pptx`)
            };
            break;
        case 'extract_images':
            args = {
                input: docPath,
                output_dir: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}_images`)
            };
            break;
        case 'office_to_markdown':
            args = {
                input: docPath,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.[^.]+$/i, '')}.md`)
            };
            break;
        case 'office_extract_text':
            args = {
                input: docPath,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.[^.]+$/i, '')}.txt`)
            };
            break;
        case 'excel_to_json':
            args = {
                input: docPath,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.[^.]+$/i, '')}.json`),
                has_headers: true
            };
            break;
        case 'excel_to_csv':
            args = {
                input: docPath,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.[^.]+$/i, '')}.csv`)
            };
            break;

        case 'pdf_validate_signatures':
            args = {
                input: docPath,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.[^.]+$/i, '')}_signature_report.json`)
            };
            break;
        case 'document_pii_scan':
            args = {
                input: docPath,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.[^.]+$/i, '')}_pii_report.json`)
            };
            break;
        case 'pdf_extract_tables':
            args = {
                input: docPath,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.[^.]+$/i, '')}_tables.${context.tableFormat === 'csv' ? 'csv' : 'json'}`),
                format: context.tableFormat || 'json'
            };
            break;
        default:
            args = {
                input: docPath,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}_output.pdf`)
            };
            break;
    }

    return args;
}
