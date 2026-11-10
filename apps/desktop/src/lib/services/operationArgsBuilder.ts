import { appState } from '$lib/state/app.svelte';

export interface OperationContext {
    docName: string;
    docPath: string;
    docDir: string;
    allDocumentPaths: string[];
    allDocuments: { name: string }[];
    getOutputPath: (filename: string) => string;
    customOutputPath: string;
    action?: string;
    file_name?: string;
    description?: string;
    file_data_base64?: string;

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
    scaleTargetSize?: string;
    scaleFactor?: number;
    reorderList: string;
    signCertPath: string;
    headerText: string;
    footerText: string;
    metadataTitle: string;
    compressQuality: number;
    blankSensitivity?: number;
    searchQuery?: string;
    nupPages?: number;
    nupOrdering?: string;
    nupMargin?: number;
    nupSpacing?: number;
    nupDrawBorders?: boolean;
    extractPatternsKeywords?: string;
    extractPatternsRegex?: string;
    extractPatternsCaseSensitive?: boolean;
    extractPatternsContextLen?: number;
    mergeAlternateDocA?: string;
    mergeAlternateDocB?: string;
    mergeAlternateReverseB?: boolean;
    mergeAlternateStep?: number;
    mergeAlternateStepB?: number;
    bookletBindingEdge?: string;
    bookletSheetOrdering?: string;
    posterGrid?: string;
    posterScale?: number;
    posterTargetSheet?: string;
    posterOverlap?: number;
    posterGuides?: boolean;
    stitchDirection?: string;
    stitchSpacing?: number;
    stitchMargin?: number;
    action?: string;
    file_name?: string;
    description?: string;
    file_data_base64?: string;
    filePath?: string;
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
        case 'office_modernize_legacy':
            args = { input: docPath, output: customOutputPath || getOutputPath(`${docName.replace(/\.[^/.]+$/, "")}_modernized`) };
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
            let inputPath = docPath && (docPath.endsWith('.md') || docPath.endsWith('.markdown')) ? docPath : null;
            let outputPath = customOutputPath ? customOutputPath : null;

            if (!inputPath || !outputPath) {
                try {
                    const { open, save } = await import('@tauri-apps/plugin-dialog');
                    if (!inputPath) {
                        const selected = await open({
                            title: 'Select Markdown File',
                            multiple: false,
                            filters: [{ name: 'Markdown', extensions: ['md', 'markdown'] }]
                        });
                        if (!selected) return null;
                        inputPath = Array.isArray(selected) ? selected[0] : selected;
                    }
                    if (!outputPath) {
                        const suggested = getOutputPath('output.pdf');
                        outputPath = await save({
                            title: 'Save PDF As...',
                            defaultPath: suggested,
                            filters: [{ name: 'PDF Document', extensions: ['pdf'] }]
                        });
                        if (!outputPath) return null;
                    }
                } catch(e) {
                    return null;
                }
            }
            args = {
                input: inputPath,
                output: outputPath,
                preset: convertPreset
            };
            break;
        }
        case 'html_to_pdf': {
            let inputPath = docPath && (docPath.endsWith('.html') || docPath.endsWith('.htm')) ? docPath : null;
            let outputPath = customOutputPath ? customOutputPath : null;

            if (!inputPath || !outputPath) {
                try {
                    const { open, save } = await import('@tauri-apps/plugin-dialog');
                    if (!inputPath) {
                        const selected = await open({
                            title: 'Select HTML File',
                            multiple: false,
                            filters: [{ name: 'HTML', extensions: ['html', 'htm'] }]
                        });
                        if (!selected) return null;
                        inputPath = Array.isArray(selected) ? selected[0] : selected;
                    }
                    if (!outputPath) {
                        const suggested = getOutputPath('output.pdf');
                        outputPath = await save({
                            title: 'Save PDF As...',
                            defaultPath: suggested,
                            filters: [{ name: 'PDF Document', extensions: ['pdf'] }]
                        });
                        if (!outputPath) return null;
                    }
                } catch(e) {
                    return null;
                }
            }
            args = {
                input: inputPath,
                output: outputPath,
                preset: convertPreset
            };
            break;
        }
        case 'img_to_pdf':
        case 'pdf_images_to_pdf': {
            let inputPaths: string[] = [];
            if (docPath && (docPath.match(/\.(png|jpg|jpeg|webp)$/i))) {
                inputPaths = [docPath];
            }
            let outputPath = customOutputPath ? customOutputPath : null;

            if (inputPaths.length === 0 || !outputPath) {
                try {
                    const { open, save } = await import('@tauri-apps/plugin-dialog');
                    if (inputPaths.length === 0) {
                        const selected = await open({
                            title: 'Select Images',
                            multiple: true,
                            filters: [{ name: 'Images', extensions: ['png', 'jpg', 'jpeg', 'webp'] }]
                        });
                        if (!selected || selected.length === 0) return null;
                        inputPaths = Array.isArray(selected) ? selected : [selected];
                    }
                    if (!outputPath) {
                        const suggested = getOutputPath('output.pdf');
                        outputPath = await save({
                            title: 'Save PDF As...',
                            defaultPath: suggested,
                            filters: [{ name: 'PDF Document', extensions: ['pdf'] }]
                        });
                        if (!outputPath) return null;
                    }
                } catch(e) {
                    return null;
                }
            }
            args = {
                inputs: inputPaths,
                output: outputPath
            };
            break;
        }
        case 'json_to_pdf': {
            let inputPath = docPath && docPath.endsWith('.json') ? docPath : null;
            let outputPath = customOutputPath ? customOutputPath : null;

            if (!inputPath || !outputPath) {
                try {
                    const { open, save } = await import('@tauri-apps/plugin-dialog');
                    if (!inputPath) {
                        const selected = await open({
                            title: 'Select Structured JSON File',
                            multiple: false,
                            filters: [{ name: 'JSON Document', extensions: ['json'] }]
                        });
                        if (!selected) return null;
                        inputPath = Array.isArray(selected) ? selected[0] : selected;
                    }
                    if (!outputPath) {
                        const suggested = getOutputPath('synthesized.pdf');
                        outputPath = await save({
                            title: 'Save Synthesized PDF As...',
                            defaultPath: suggested,
                            filters: [{ name: 'PDF Document', extensions: ['pdf'] }]
                        });
                        if (!outputPath) return null;
                    }
                } catch(e) {
                    return null;
                }
            }
            args = {
                input: inputPath,
                output: outputPath
            };
            break;
        }
        case 'compress':
        case 'pdf_compress':
            args = {
                input: docPath,
                quality: compressQuality.toString(),
                output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}_compressed.pdf`)
            };
            break;
        case 'rotate':
        case 'pdf_rotate':
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
        case 'pdf_delete_pages':
            args = {
                input: docPath,
                pages: deletePages || '1',
                output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}_deleted.pdf`)
            };
            break;
        case 'reorder_pages':
        case 'pdf_reorder_pages':
            args = {
                input: docPath,
                order: reorderList || '1',
                output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}_reordered.pdf`)
            };
            break;
        case 'burst':
        case 'pdf_burst':
            args = {
                input: docPath,
                output_dir: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}_burst`)
            };
            break;
        case 'crop':
        case 'pdf_crop':
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
        case 'pdf_remove_blank':
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
        case 'pdf_bookmarks':
        case 'classify_type':
            args = {
                input: docPath,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.[^.]+$/i, '')}_bookmarks.json`)
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
        case 'pdf_linearize':
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
        case 'pdf_extract_pages':
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
        case 'pdf_nup':
            args = {
                input: docPath,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.[^.]+$/i, '')}_nup.pdf`),
                nup: context.nupPages || 2,
                ordering: context.nupOrdering || 'horizontal',
                margin_pt: context.nupMargin ?? 18.0,
                spacing_pt: context.nupSpacing ?? 10.0,
                draw_borders: context.nupDrawBorders ?? false
            };
            break;
        case 'pdf_scale_pages':
            args = {
                input: docPath,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.[^.]+$/i, '')}_scaled.pdf`),
                target_size: context.scaleTargetSize || 'A4',
                scale_factor: context.scaleFactor ?? 1.0,
                maintain_aspect_ratio: true,
                center_content: true
            };
            break;
        case 'pdf_remove_signatures':
            args = {
                input: docPath,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.[^.]+$/i, '')}_output.pdf`)
            };
            break;
        case 'pdf_stamp':
            args = {
                input: docPath,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.[^.]+$/i, '')}_output.pdf`)
            };
            break;
        case 'pdf_overlay_underlay':
            args = {
                input: docPath,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.[^.]+$/i, '')}_output.pdf`)
            };
            break;
        case 'pdf_auto_rotate':
            args = {
                input: docPath,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.[^.]+$/i, '')}_autorotated.pdf`),
                target_orientation: 'portrait'
            };
            break;
        case 'pdf_booklet':
            args = {
                input: docPath,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.[^.]+$/i, '')}_booklet.pdf`),
                binding_edge: context.bookletBindingEdge || 'left',
                sheet_ordering: context.bookletSheetOrdering || 'standard',
                create_sheets: true
            };
            break;
        case 'pdf_stitch_canvas':
            args = {
                input: docPath,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.[^.]+$/i, '')}_stitched.pdf`),
                direction: context.stitchDirection || 'vertical',
                spacing: context.stitchSpacing ?? 0.0,
                margin: context.stitchMargin ?? 0.0
            };
            break;
        case 'pdf_attachments':
            args = {
                input: docPath,
                output: customOutputPath || (context.action === 'add'
                    ? getOutputPath(`${docName.replace(/\.[^.]+$/i, '')}_with_attachment.pdf`)
                    : getOutputPath(`${docName.replace(/\.[^.]+$/i, '')}_attachments.json`)),
                action: context.action || 'list',
                file_name: context.file_name,
                description: context.description,
                file_data_base64: context.file_data_base64,
                file: context.filePath
            };
            break;
        case 'pdf_timestamp':
            args = {
                input: docPath,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.[^.]+$/i, '')}_output.pdf`)
            };
            break;
        case 'pdf_change_permissions':
            args = {
                input: docPath,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.[^.]+$/i, '')}_output.pdf`)
            };
            break;
        case 'pdf_split_scanned_book':
            args = {
                input: docPath,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.[^.]+$/i, '')}_output.pdf`)
            };
            break;
        case 'pdf_comic_archive':
            args = {
                input: docPath,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.[^.]+$/i, '')}_output.cbr`)
            };
            break;
        case 'pdf_split_by_outline':
            args = {
                input: docPath,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.[^.]+$/i, '')}_output.zip`)
            };
            break;
        case 'pdf_merge_alternate': {
            const inputA = context.mergeAlternateDocA || appState.documentPaths[0] || docPath;
            const inputB = context.mergeAlternateDocB || (appState.documentPaths.length > 1 ? appState.documentPaths[1] : docPath);
            const reverseB = context.mergeAlternateReverseB !== undefined ? context.mergeAlternateReverseB : true;
            const step = context.mergeAlternateStep !== undefined ? context.mergeAlternateStep : 1;
            const stepB = context.mergeAlternateStepB !== undefined ? context.mergeAlternateStepB : 1;
            args = {
                input_a: inputA,
                input_b: inputB,
                reverse_a: false,
                reverse_b: reverseB,
                step,
                step_b: stepB,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.[^.]+$/i, '')}_interleaved.pdf`)
            };
            break;
        }
        case 'pdf_posterize':
            args = {
                input: docPath,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.[^.]+$/i, '')}_poster.pdf`),
                grid: context.posterGrid || '2x2',
                scale: context.posterScale ?? 1.0,
                target_sheet: context.posterTargetSheet || 'a4',
                overlap: context.posterOverlap ?? 14.17,
                guides: context.posterGuides ?? true
            };
            break;
        case 'pdf_dedup_streams':
            args = {
                input: docPath,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.[^.]+$/i, '')}_output.pdf`)
            };
            break;
        case 'pdf_to_epub':
            args = {
                input: docPath,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.[^.]+$/i, '')}_output.epub`)
            };
            break;
        case 'pdf_extract_patterns': {
            const keywords = (context.extractPatternsKeywords || '')
                .split(',')
                .map(s => s.trim())
                .filter(Boolean);

            const patterns: Record<string, string> = {};
            if (context.extractPatternsRegex) {
                const lines = context.extractPatternsRegex.split(/[\n,]+/);
                for (const line of lines) {
                    const trimmed = line.trim();
                    if (!trimmed) continue;
                    const eqIdx = trimmed.indexOf('=');
                    if (eqIdx !== -1) {
                        const key = trimmed.slice(0, eqIdx).trim();
                        const val = trimmed.slice(eqIdx + 1).trim();
                        if (key && val) patterns[key] = val;
                    }
                }
            }

            args = {
                input: docPath,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.[^.]+$/i, '')}_extract_report.json`),
                keywords,
                patterns,
                case_sensitive: context.extractPatternsCaseSensitive || false,
                context_len: context.extractPatternsContextLen ?? 30
            };
            break;
        }
        default:
            args = {
                input: docPath,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}_output.pdf`)
            };
            break;
    }

    return args;
}
