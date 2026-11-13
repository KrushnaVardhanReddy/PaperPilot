import { appState } from '$lib/state/app.svelte';

export interface OperationContext {
    docName: string;
    docPath: string;
    docDir: string;
    allDocumentPaths: string[];
    allDocuments: { name: string }[];
    getOutputPath: (filename: string) => string;
    customOutputPath: string;
    applyToAllFiles?: boolean;
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
    chunkTargetTokens?: number;
    chunkOverlapTokens?: number;
    chunkStripBoilerplate?: boolean;
    chunkBreakOnHeadings?: boolean;
    targetTokens?: number;
    overlapTokens?: number;
    stripBoilerplate?: boolean;
    breakOnHeadings?: boolean;
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
    renderPage?: number;
    renderDpi?: number;
    invertMode?: string;
    invertPages?: string;
    compareInputB?: string;
    metadataAuthor?: string;
    metadataSubject?: string;
    metadataKeywords?: string;
    tsaServerUrl?: string;
    allowPrinting?: boolean;
    allowCopying?: boolean;
    allowModifying?: boolean;
    sanitizeScripts?: boolean;
    sanitizeMetadata?: boolean;
    sanitizeAttachments?: boolean;
    signPassword?: string;
    signFieldName?: string;
    signReason?: string;
    signLocation?: string;
    hashAlgorithm?: string;
    stationeryPath?: string;
    overlayMode?: string;
    overlayPages?: string;
    overlayOpacity?: number;
    stampImagePath?: string;
    stampX?: number;
    stampY?: number;
    stampWidth?: number;
    stampHeight?: number;
    stampOpacity?: number;
    stampPages?: string;
    piiScanSsn?: boolean;
    piiScanCreditCards?: boolean;
    piiScanEmails?: boolean;
    piiScanPhones?: boolean;
    piiScanIban?: boolean;
    redactQuery?: string;
    redactRect?: string;
    annotationType?: string;
    annotationContent?: string;
    annotationColor?: string;
    annotationRect?: string;
    annotationPage?: number;
    formDataPath?: string;
    formFieldRect?: string;
    formFieldName?: string;
    formFieldType?: string;
    formFieldPage?: number;
    batchCsvPath?: string;
    dpi?: number;
    imageFormat?: string;
    cbzArchiveName?: string;
    pdfaConformance?: string;
    excelSheet?: string;
    hasHeaders?: boolean;
    workspaceQuery?: string;
    workspaceDir?: string;
    semanticQuery?: string;
    semanticLimit?: number;
    classifyDetail?: boolean;
    ragChunkSize?: number;
    workflowPreset?: string;
    workflowRecipe?: string;
    recipeId?: string;
    askPrompt?: string;
    askDryRun?: boolean;
    watcherDir?: string;
    watcherRecipe?: string;
    watcherOutputDir?: string;
    watcherSettleDelay?: number;
    watcherId?: string;
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
        case 'office_modernize_legacy': {
            const srcExt = docPath ? docPath.split('.').pop()?.toLowerCase() ?? 'docx' : 'docx';
            const modernExt = srcExt === 'doc' ? 'docx' : srcExt === 'xls' ? 'xlsx' : srcExt === 'ppt' ? 'pptx' : 'docx';
            args = { input: docPath, output: customOutputPath || getOutputPath(`${docName.replace(/\.[^/.]+$/, "")}_modernized.${modernExt}`) };
            break;
        }
        case 'merge':
        case 'pdf_merge': {
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
            const allImages = (context.allDocumentPaths || [])
                .map((p, i) => p || context.allDocuments?.[i]?.name || '')
                .filter(p => /\.(png|jpg|jpeg|webp|bmp|gif)$/i.test(p));

            let inputPaths: string[] = [];
            if (context.applyToAllFiles !== false && allImages.length > 0) {
                inputPaths = allImages;
            } else if (docPath && /\.(png|jpg|jpeg|webp|bmp|gif)$/i.test(docPath)) {
                inputPaths = [docPath];
            }

            const defaultOutName = inputPaths.length > 1
                ? 'images_combined.pdf'
                : `${docName.replace(/\.[^.]+$/i, '')}_converted.pdf`;
            let outputPath: string | null = customOutputPath || getOutputPath(defaultOutName);

            if (inputPaths.length === 0) {
                try {
                    const { open, save } = await import('@tauri-apps/plugin-dialog');
                    const selected = await open({
                        title: 'Select Images',
                        multiple: true,
                        filters: [{ name: 'Images', extensions: ['png', 'jpg', 'jpeg', 'webp'] }]
                    });
                    if (!selected || selected.length === 0) return null;
                    inputPaths = Array.isArray(selected) ? selected : [selected];

                    if (!outputPath) {
                        const suggested = getOutputPath('images_combined.pdf');
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
        case 'pdf_watermark':
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
        case 'pdf_redact':
            args = {
                input: docPath,
                query: context.redactQuery || searchQuery || 'CONFIDENTIAL',
                rect: context.redactRect || '0,0,100,100',
                regions: context.redactRect || '0,0,100,100',
                output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}_redacted.pdf`)
            };
            break;
        case 'sign':
        case 'pdf_sign':
            args = {
                input: docPath,
                cert: signCertPath,
                password: context.signPassword || context.password || undefined,
                field_name: context.signFieldName || undefined,
                reason: context.signReason || undefined,
                location: context.signLocation || undefined,
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
        case 'pdf_validate':
            args = {
                input: docPath,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.[^.]+$/i, '')}_validate_report.json`)
            };
            break;
        case 'hash':
        case 'pdf_hash':
            args = {
                input: docPath,
                algorithm: context.hashAlgorithm || 'sha256',
                output: customOutputPath || getOutputPath(`${docName.replace(/\.[^.]+$/i, '')}_checksums.txt`)
            };
            break;
        case 'read_form':
        case 'pdf_read_form':
            args = {
                input: docPath,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.[^.]+$/i, '')}_form_fields.json`)
            };
            break;
        case 'flatten':
        case 'pdf_flatten':
            args = {
                input: docPath,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.[^.]+$/i, '')}_flattened.pdf`)
            };
            break;
        case 'fill_form':
        case 'pdf_fill_form':
            args = {
                input: docPath,
                data: context.formDataPath || 'tests/e2e_fixtures/real/form_data.json',
                values: context.formDataPath || 'tests/e2e_fixtures/real/form_data.json',
                output: customOutputPath || getOutputPath(`${docName.replace(/\.[^.]+$/i, '')}_filled.pdf`)
            };
            break;
        case 'create_form_field':
        case 'pdf_create_form_field':
            args = {
                input: docPath,
                name: context.formFieldName || 'ApplicantName',
                field_name: context.formFieldName || 'ApplicantName',
                type: context.formFieldType || 'text',
                field_type: context.formFieldType || 'text',
                page: context.formFieldPage ?? 1,
                rect: context.formFieldRect || '50,50,200,80',
                output: customOutputPath || getOutputPath(`${docName.replace(/\.[^.]+$/i, '')}_with_field.pdf`)
            };
            break;
        case 'batch_fill':
        case 'cli_fill_batch': {
            const defaultBatchDir = getOutputPath(`${docName.replace(/\.[^.]+$/i, '')}_batch_out`);
            const targetDir = customOutputPath || defaultBatchDir;
            args = {
                template: docPath,
                data: context.batchCsvPath || 'tests/e2e_fixtures/real/batch_data.csv',
                output_dir: targetDir,
                name_template: '{index:04}.pdf',
                keep_editable: false
            };
            break;
        }
        case 'bookmarks':
        case 'pdf_bookmarks':
            args = {
                input: docPath,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.[^.]+$/i, '')}_bookmarks.json`)
            };
            break;
        case 'classify':
        case 'classify_type':
        case 'pdf_classify_type':
            args = {
                input: docPath,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.[^.]+$/i, '')}_classified.json`)
            };
            break;
        case 'search':
        case 'pdf_search': {
            const query = (context.searchQuery && context.searchQuery.trim() !== '') ? context.searchQuery.trim() : 'PaperPilot';
            args = {
                input: docPath,
                query,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.[^.]+$/i, '')}_search.json`)
            };
            break;
        }
        case 'workspace_search': {
            const query = (context.workspaceQuery && context.workspaceQuery.trim() !== '')
                ? context.workspaceQuery.trim()
                : (context.searchQuery && context.searchQuery.trim() !== '')
                    ? context.searchQuery.trim()
                    : 'PaperPilot';
            const dir = context.workspaceDir || (docPath && docPath.includes('/') ? docPath.substring(0, docPath.lastIndexOf('/')) : '.');
            args = {
                input: docPath,
                dir,
                query,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.[^.]+$/i, '')}_workspace_search.json`)
            };
            break;
        }
        case 'semantic_search': {
            const query = (context.semanticQuery && context.semanticQuery.trim() !== '')
                ? context.semanticQuery.trim()
                : (context.searchQuery && context.searchQuery.trim() !== '')
                    ? context.searchQuery.trim()
                    : 'document';
            args = {
                input: docPath,
                query,
                limit: context.semanticLimit ?? 5,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.[^.]+$/i, '')}_semantic_search.json`)
            };
            break;
        }
        case 'bates':
        case 'pdf_bates':
            args = {
                input: docPath,
                prefix: batesPrefix || 'CONF-',
                start_number: batesStart || 1,
                start: batesStart || 1,
                padding: batesPadding || 6,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}_bates.pdf`)
            };
            break;
        case 'page_numbers':
        case 'pdf_page_numbers':
            args = {
                input: docPath,
                position: context.pageNumberPosition || 'bottom-center',
                format: context.pageNumberFormat || 'Page {n} of {total}',
                output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}_page_numbered.pdf`)
            };
            break;
        case 'header_footer':
        case 'pdf_header_footer':
            args = {
                input: docPath,
                text: headerText || footerText || 'Header Notice',
                header_left: headerText,
                footer_center: footerText,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}_header.pdf`)
            };
            break;
        case 'render':
        case 'pdf_render':
            args = {
                input: docPath,
                page: context.renderPage ?? 1,
                dpi: context.renderDpi ?? 150.0,
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
        case 'repair':
        case 'pdf_repair':
            args = {
                input: docPath,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}_repaired.pdf`)
            };
            break;
        case 'invert_colors':
        case 'pdf_invert_colors':
            args = {
                input: docPath,
                mode: context.invertMode || 'dark_mode',
                pages: context.invertPages,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}_inverted.pdf`)
            };
            break;
        case 'compare':
        case 'pdf_compare': {
            const secondDoc = context.compareInputB || (context.allDocumentPaths && context.allDocumentPaths.length > 1 ? context.allDocumentPaths[1] : docPath);
            args = {
                input_a: docPath,
                input_b: secondDoc,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}_diff_report.json`)
            };
            break;
        }
        case 'annotate':
        case 'pdf_annotate': {
            const annoType = context.annotationType || 'highlight';
            const annoContent = context.annotationContent || 'Note';
            const annoColor = context.annotationColor || '#FFFF00';
            const pageNum = context.annotationPage || 1;
            const rectParts = (context.annotationRect || '50,50,150,20').split(',').map(s => parseFloat(s.trim()) || 0);
            const x = rectParts[0] || 50;
            const y = rectParts[1] || 50;
            const w = rectParts[2] || 150;
            const h = rectParts[3] || 20;

            const annotations = [{
                id: 'anno-1',
                type: annoType,
                page: pageNum,
                x,
                y,
                w,
                h,
                color: annoColor,
                content: annoContent
            }];

            args = {
                input: docPath,
                annotations,
                data: annotations,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}_annotated.pdf`)
            };
            break;
        }
        case 'encrypt':
        case 'pdf_encrypt':
            args = {
                input: docPath,
                password: password || 'paperpilot',
                output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}_encrypted.pdf`)
            };
            break;
        case 'decrypt':
        case 'pdf_decrypt':
            args = {
                input: docPath,
                password: password || '',
                output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}_decrypted.pdf`)
            };
            break;
        case 'metadata':
        case 'pdf_metadata':
            args = {
                input: docPath,
                title: metadataTitle || docName,
                author: context.metadataAuthor || 'PaperPilot Author',
                subject: context.metadataSubject || 'Document Subject',
                keywords: context.metadataKeywords || 'paperpilot,pdf',
                output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}_metadata.pdf`)
            };
            break;
        case 'sanitize':
        case 'pdf_sanitize':
            args = {
                input: docPath,
                remove_scripts: context.sanitizeScripts !== undefined ? context.sanitizeScripts : true,
                remove_metadata: context.sanitizeMetadata !== undefined ? context.sanitizeMetadata : true,
                remove_attachments: context.sanitizeAttachments !== undefined ? context.sanitizeAttachments : true,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}_sanitized.pdf`)
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
        case 'pdf_extract_text':
            args = {
                input: docPath,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}_text.txt`)
            };
            break;
        case 'ocr':
        case 'pdf_ocr': {
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
        case 'pdf_convert_markdown':
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
        case 'pdf_extract_images':
            args = {
                input: docPath,
                output_dir: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}_images`),
                format: context.imageFormat || 'png'
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
                has_headers: context.hasHeaders !== undefined ? context.hasHeaders : true,
                ...(context.excelSheet ? { sheet: context.excelSheet } : {})
            };
            break;
        case 'excel_to_csv':
            args = {
                input: docPath,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.[^.]+$/i, '')}.csv`),
                ...(context.excelSheet ? { sheet: context.excelSheet } : {})
            };
            break;

        case 'validate_signatures':
        case 'pdf_validate_signatures':
            args = {
                input: docPath,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.[^.]+$/i, '')}_signature_report.json`)
            };
            break;
        case 'document_pii_scan':
        case 'pii_scan':
        case 'pdf_document_pii_scan':
            args = {
                input: docPath,
                scan_ssn: context.piiScanSsn !== undefined ? context.piiScanSsn : true,
                scan_credit_cards: context.piiScanCreditCards !== undefined ? context.piiScanCreditCards : true,
                scan_emails: context.piiScanEmails !== undefined ? context.piiScanEmails : true,
                scan_phones: context.piiScanPhones !== undefined ? context.piiScanPhones : true,
                scan_iban: context.piiScanIban !== undefined ? context.piiScanIban : true,
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
        case 'remove_signatures':
        case 'pdf_remove_signatures':
            args = {
                input: docPath,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.[^.]+$/i, '')}_output.pdf`)
            };
            break;
        case 'stamp':
        case 'pdf_stamp':
            args = {
                input: docPath,
                image_path: context.stampImagePath || undefined,
                x: context.stampX !== undefined ? context.stampX : 50.0,
                y: context.stampY !== undefined ? context.stampY : 50.0,
                width: context.stampWidth !== undefined ? context.stampWidth : 150.0,
                height: context.stampHeight !== undefined ? context.stampHeight : 50.0,
                opacity: context.stampOpacity !== undefined ? context.stampOpacity : 1.0,
                pages: context.stampPages || 'all',
                output: customOutputPath || getOutputPath(`${docName.replace(/\.[^.]+$/i, '')}_stamped.pdf`)
            };
            break;
        case 'overlay_underlay':
        case 'pdf_overlay_underlay':
            args = {
                input: docPath,
                template_path: context.stationeryPath || undefined,
                mode: context.overlayMode || 'underlay',
                pages: context.overlayPages || 'all',
                opacity: context.overlayOpacity !== undefined ? context.overlayOpacity : 1.0,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.[^.]+$/i, '')}_stationery.pdf`)
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
        case 'timestamp':
        case 'pdf_timestamp':
            args = {
                input: docPath,
                server_url: context.tsaServerUrl || 'http://timestamp.digicert.com',
                output: customOutputPath || getOutputPath(`${docName.replace(/\.[^.]+$/i, '')}_timestamped.pdf`)
            };
            break;
        case 'change_permissions':
        case 'pdf_change_permissions':
            args = {
                input: docPath,
                allow_print: context.allowPrinting !== undefined ? context.allowPrinting : false,
                allow_copy: context.allowCopying !== undefined ? context.allowCopying : false,
                allow_modify: context.allowModifying !== undefined ? context.allowModifying : false,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.[^.]+$/i, '')}_permissions.pdf`)
            };
            break;
        case 'pdf_split_scanned_book':
            args = {
                input: docPath,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.[^.]+$/i, '')}_output.pdf`)
            };
            break;
        case 'pdf_comic_archive':
        case 'comic_archive':
            args = {
                input: docPath,
                output: customOutputPath || getOutputPath(context.cbzArchiveName || `${docName.replace(/\.[^.]+$/i, '')}_output.cbz`),
                dpi: context.dpi ?? 150.0
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
        case 'to_epub':
            args = {
                input: docPath,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.[^.]+$/i, '')}_output.epub`)
            };
            break;
        case 'pdf_to_pdf_a':
        case 'to_pdf_a':
        case 'pdf_a':
            args = {
                input: docPath,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.[^.]+$/i, '')}_pdfa.pdf`),
                conformance: context.pdfaConformance || 'PDF/A-1b'
            };
            break;
        case 'pdf_convert_excel':
        case 'convert_excel':
        case 'excel_to_pdf': {
            let inputPath = docPath && (docPath.endsWith('.xlsx') || docPath.endsWith('.xls') || docPath.endsWith('.csv')) ? docPath : null;
            let outputPath = customOutputPath ? customOutputPath : null;

            if (!inputPath || !outputPath) {
                try {
                    const { open, save } = await import('@tauri-apps/plugin-dialog');
                    if (!inputPath) {
                        const selected = await open({
                            title: 'Select Spreadsheet File',
                            multiple: false,
                            filters: [{ name: 'Spreadsheet', extensions: ['xlsx', 'xls', 'csv'] }]
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
        case 'extract_patterns':
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
        case 'pdf_rag_chunks':
        case 'rag_chunks': {
            args = {
                input: docPath,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.[^.]+$/i, '')}_rag_chunks.json`)
            };
            break;
        }
        case 'pdf_chunk':
        case 'chunk': {
            args = {
                input: docPath,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.[^.]+$/i, '')}_chunks.json`),
                target_tokens: context.targetTokens ?? context.chunkTargetTokens ?? 500,
                overlap_tokens: context.overlapTokens ?? context.chunkOverlapTokens ?? 50,
                strip_boilerplate: context.stripBoilerplate ?? context.chunkStripBoilerplate ?? true,
                break_on_headings: context.breakOnHeadings ?? context.chunkBreakOnHeadings ?? true
            };
            break;
        }
        case 'document_workflow':
        case 'pdf_document_workflow':
        case 'workflow': {
            const preset = context.workflowPreset || 'preset-clean-scan';
            args = {
                input: docPath,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.[^.]+$/i, '')}_workflow.pdf`),
                preset,
                ...(context.workflowRecipe ? { recipe: context.workflowRecipe } : {})
            };
            break;
        }
        case 'recipe_run':
        case 'recipe': {
            const recipe = context.recipeId || context.workflowRecipe || context.workflowPreset || 'preset-clean-scan';
            args = {
                input: docPath,
                recipe,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.[^.]+$/i, '')}_recipe.pdf`)
            };
            break;
        }
        case 'cli_ask':
        case 'ask': {
            const prompt = (context.askPrompt && context.askPrompt.trim() !== '') ? context.askPrompt.trim() : 'rotate 90 degrees';
            args = {
                prompt,
                input: docPath,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.[^.]+$/i, '')}_ask_plan.txt`),
                dry_run: context.askDryRun !== undefined ? context.askDryRun : true
            };
            break;
        }
        case 'watcher_start': {
            const dir = context.watcherDir || context.docDir || (docPath && docPath.includes('/') ? docPath.substring(0, docPath.lastIndexOf('/')) : '.');
            const recipe = context.watcherRecipe || context.workflowPreset || 'preset-legal-discovery';
            args = {
                dir,
                recipe,
                ...(context.watcherOutputDir ? { output_dir: context.watcherOutputDir } : {}),
                settle_delay_ms: context.watcherSettleDelay || 500,
                output: customOutputPath || getOutputPath(`${docName.replace(/\.[^.]+$/i, '')}_watcher.json`)
            };
            break;
        }
        case 'watcher_stop': {
            args = {
                watcher_id: context.watcherId || 'default',
                output: customOutputPath || getOutputPath(`${docName.replace(/\.[^.]+$/i, '')}_watcher.json`)
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
