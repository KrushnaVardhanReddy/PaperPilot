import { describe, it, expect, vi } from 'vitest';
import { buildOperationArgs, type OperationContext } from './operationArgsBuilder';

// Mock the Tauri dialog plugin for tests that import it dynamically
vi.mock('@tauri-apps/plugin-dialog', () => ({
    open: vi.fn().mockResolvedValue(['/mock/path.md']),
    save: vi.fn().mockResolvedValue('/mock/save_path.pdf')
}));

describe('operationArgsBuilder', () => {
    const defaultContext: OperationContext = {
        docName: 'test.pdf',
        docPath: '/docs/test.pdf',
        docDir: '/docs',
        allDocumentPaths: ['/docs/test.pdf'],
        allDocuments: [{ name: 'test.pdf' }],
        getOutputPath: (filename: string) => `/docs/${filename}`,
        customOutputPath: '',
        splitPoints: '1-2',
        rotateAngle: '90',
        watermarkText: 'TEST',
        watermarkOrientation: 'horizontal',
        watermarkOpacity: 50,
        watermarkColor: '#000000',
        watermarkFontSize: 12,
        batesPrefix: 'BATES',
        batesStart: 1,
        batesPadding: 4,
        ocrOutputFormat: 'pdf',
        ocrLanguage: 'eng',
        cropBox: '0,0,100,100',
        pageNumberPosition: 'bottom_right',
        pageNumberFormat: '1',
        pageNumberStart: 1,
        convertPreset: 'standard',
        reorderList: '2,1',
        signCertPath: '/certs/cert.pem',
        headerText: 'Header',
        footerText: 'Footer',
        metadataTitle: 'Title',
        compressQuality: 75
    };

    it('builds args for simple tools (compress)', async () => {
        const args = await buildOperationArgs('compress', defaultContext);
        expect(args).toEqual({
            input: '/docs/test.pdf',
            quality: '75',
            output: '/docs/test_compressed.pdf'
        });
    });

    it('builds args for simple tools with custom output', async () => {
        const args = await buildOperationArgs('compress', { ...defaultContext, customOutputPath: '/custom/out.pdf' });
        expect(args).toEqual({
            input: '/docs/test.pdf',
            quality: '75',
            output: '/custom/out.pdf'
        });
    });

    it('builds args for rotate', async () => {
        const args = await buildOperationArgs('rotate', defaultContext);
        expect(args).toEqual({
            input: '/docs/test.pdf',
            pages: 'all',
            angle: 90,
            output: '/docs/test_rotated.pdf'
        });
    });

    it('builds args for rotate with specific pages', async () => {
        const args = await buildOperationArgs('rotate', { ...defaultContext, rotatePages: '1, 3, 5-7' });
        expect(args).toEqual({
            input: '/docs/test.pdf',
            pages: '1, 3, 5-7',
            angle: 90,
            output: '/docs/test_rotated.pdf'
        });
    });

    it('builds args for creation tools (md_to_pdf) with dialog mock', async () => {
        const args = await buildOperationArgs('md_to_pdf', { ...defaultContext, convertOrientation: 'landscape' });
        expect(args).toEqual({
            input: '/mock/path.md',
            output: '/mock/save_path.pdf',
            preset: 'standard',
            orientation: 'landscape'
        });
    });

    it('builds args for split', async () => {
        const args = await buildOperationArgs('split', defaultContext);
        expect(args).toEqual({
            input: '/docs/test.pdf',
            output_dir: '/docs/test_split',
            output: '/docs/test_split',
            output_pattern: '/docs/test_split/p%d.pdf',
            ranges: '1-2'
        });
    });

    it('builds args for merge (with dialog mock for missing output)', async () => {
        const args = await buildOperationArgs('merge', defaultContext);
        expect(args).toEqual({
            inputs: ['/docs/test.pdf'],
            output: '/mock/save_path.pdf'
        });
    });

    it('handles ocr clipboard option', async () => {
        const args = await buildOperationArgs('ocr', { ...defaultContext, ocrOutputFormat: 'clipboard' });
        expect(args).toEqual({
            input: '/docs/test.pdf',
            output: '/docs/temp_ocr_clipboard.txt'
        });
    });

    it('handles remove_blank sensitivity option', async () => {
        const args = await buildOperationArgs('remove_blank', { ...defaultContext, blankSensitivity: 80 });
        expect(args).toEqual({
            input: '/docs/test.pdf',
            output: '/docs/test_cleaned.pdf',
            sensitivity: 80
        });
    });

    it('handles burst output directory', async () => {
        const args = await buildOperationArgs('burst', defaultContext);
        expect(args).toEqual({
            input: '/docs/test.pdf',
            output_dir: '/docs/test_burst'
        });
    });

    it('handles search query argument', async () => {
        const args = await buildOperationArgs('search', { ...defaultContext, searchQuery: 'Confidential' });
        expect(args).toEqual({
            input: '/docs/test.pdf',
            output: '/docs/test_search.json',
            query: 'Confidential'
        });
    });

    it('returns null if creation tool dialog is canceled', async () => {
        // Redefine mock temporarily for this test
        vi.mocked(await import('@tauri-apps/plugin-dialog')).open.mockResolvedValueOnce(null);
        const args = await buildOperationArgs('md_to_pdf', defaultContext);
        expect(args).toBeNull();
    });

    it('builds args for pdf_validate_signatures', async () => {
        const args = await buildOperationArgs('pdf_validate_signatures', defaultContext);
        expect(args).toEqual({
            input: '/docs/test.pdf',
            output: '/docs/test_signature_report.json'
        });
    });

    it('builds args for document_pii_scan', async () => {
        const args = await buildOperationArgs('document_pii_scan', defaultContext);
        expect(args).toEqual({
            input: '/docs/test.pdf',
            output: '/docs/test_pii_report.json',
            scan_emails: true,
            scan_phones: true,
            scan_ssn: true,
            scan_credit_cards: true,
            scan_iban: true
        });
    });

    it('builds args for pdf_extract_tables', async () => {
        const args = await buildOperationArgs('pdf_extract_tables', defaultContext);
        expect(args).toEqual({
            input: '/docs/test.pdf',
            output: '/docs/test_tables.json',
            format: 'json'
        });
    });

    it('builds args for pdf_extract_tables with csv format', async () => {
        const args = await buildOperationArgs('pdf_extract_tables', { ...defaultContext, tableFormat: 'csv' });
        expect(args).toEqual({
            input: '/docs/test.pdf',
            output: '/docs/test_tables.csv',
            format: 'csv'
        });
    });

    it('builds args for pdf_nup', async () => {
        const args = await buildOperationArgs('pdf_nup', defaultContext);
        expect(args).toEqual({
            input: '/docs/test.pdf',
            output: '/docs/test_nup.pdf',
            nup: 2,
            ordering: 'horizontal',
            margin_pt: 18.0,
            spacing_pt: 10.0,
            draw_borders: false
        });
    });

    it('builds args for pdf_scale_pages', async () => {
        const args = await buildOperationArgs('pdf_scale_pages', defaultContext);
        expect(args).toEqual({
            input: '/docs/test.pdf',
            output: '/docs/test_scaled.pdf',
            target_size: 'A4',
            scale_factor: 1.0,
            maintain_aspect_ratio: true,
            center_content: true
        });
    });

    it('builds args for pdf_remove_signatures', async () => {
        const args = await buildOperationArgs('pdf_remove_signatures', defaultContext);
        expect(args).toEqual({
            input: '/docs/test.pdf',
            output: '/docs/test_output.pdf'
        });
    });

    it('builds args for pdf_stamp', async () => {
        const args = await buildOperationArgs('pdf_stamp', defaultContext);
        expect(args).toEqual({
            input: '/docs/test.pdf',
            output: '/docs/test_stamped.pdf',
            x: 50.0,
            y: 50.0,
            width: 150.0,
            height: 50.0,
            opacity: 1.0,
            pages: 'all'
        });
    });

    it('builds args for pdf_overlay_underlay', async () => {
        const args = await buildOperationArgs('pdf_overlay_underlay', defaultContext);
        expect(args).toEqual({
            input: '/docs/test.pdf',
            output: '/docs/test_stationery.pdf',
            mode: 'underlay',
            pages: 'all',
            opacity: 1.0
        });
    });

    it('builds args for pdf_auto_rotate', async () => {
        const args = await buildOperationArgs('pdf_auto_rotate', defaultContext);
        expect(args).toEqual({
            input: '/docs/test.pdf',
            output: '/docs/test_autorotated.pdf',
            target_orientation: 'portrait'
        });
    });

    it('builds args for pdf_stitch_canvas', async () => {
        const args = await buildOperationArgs('pdf_stitch_canvas', defaultContext);
        expect(args).toEqual({
            input: '/docs/test.pdf',
            output: '/docs/test_stitched.pdf',
            direction: 'vertical',
            spacing: 0.0,
            margin: 0.0
        });
    });

    it('builds args for pdf_attachments', async () => {
        const args = await buildOperationArgs('pdf_attachments', defaultContext);
        expect(args).toEqual({
            input: '/docs/test.pdf',
            output: '/docs/test_attachments.json',
            action: 'list'
        });
    });

    it('builds args for pdf_timestamp', async () => {
        const args = await buildOperationArgs('pdf_timestamp', defaultContext);
        expect(args).toEqual({
            input: '/docs/test.pdf',
            output: '/docs/test_timestamped.pdf',
            server_url: 'http://timestamp.digicert.com'
        });
    });

    it('builds args for pdf_change_permissions', async () => {
        const args = await buildOperationArgs('pdf_change_permissions', defaultContext);
        expect(args).toEqual({
            input: '/docs/test.pdf',
            output: '/docs/test_permissions.pdf',
            allow_print: true,
            allow_copy: true,
            allow_modify: true,
            owner_password: 'paperpilot'
        });
    });

    it('builds args for pdf_split_scanned_book', async () => {
        const args = await buildOperationArgs('pdf_split_scanned_book', defaultContext);
        expect(args).toEqual({
            input: '/docs/test.pdf',
            output: '/docs/test_output.pdf'
        });
    });

    it('builds args for pdf_comic_archive', async () => {
        const args = await buildOperationArgs('pdf_comic_archive', defaultContext);
        expect(args).toEqual({
            input: '/docs/test.pdf',
            output: '/docs/test_output.cbz',
            dpi: 150.0
        });
    });

    it('builds args for pdf_split_by_outline', async () => {
        const args = await buildOperationArgs('pdf_split_by_outline', defaultContext);
        expect(args).toEqual({
            input: '/docs/test.pdf',
            output: '/docs/test_output.zip'
        });
    });

    it('builds args for pdf_merge_alternate', async () => {
        const args = await buildOperationArgs('pdf_merge_alternate', defaultContext);
        expect(args).toEqual({
            input_a: '/docs/test.pdf',
            input_b: '/docs/test.pdf',
            reverse_a: false,
            reverse_b: true,
            step: 1,
            step_b: 1,
            output: '/docs/test_interleaved.pdf'
        });
    });

    it('builds args for pdf_posterize', async () => {
        const args = await buildOperationArgs('pdf_posterize', defaultContext);
        expect(args).toEqual({
            input: '/docs/test.pdf',
            output: '/docs/test_poster.pdf',
            grid: '2x2',
            scale: 1.0,
            target_sheet: 'a4',
            overlap: 14.17,
            guides: true
        });
    });

    it('builds args for pdf_dedup_streams', async () => {
        const args = await buildOperationArgs('pdf_dedup_streams', defaultContext);
        expect(args).toEqual({
            input: '/docs/test.pdf',
            output: '/docs/test_output.pdf'
        });
    });

    it('builds args for pdf_to_epub', async () => {
        const args = await buildOperationArgs('pdf_to_epub', defaultContext);
        expect(args).toEqual({
            input: '/docs/test.pdf',
            output: '/docs/test_output.epub'
        });
    });

    it('builds args for pdf_extract_patterns', async () => {
        const args = await buildOperationArgs('pdf_extract_patterns', defaultContext);
        expect(args).toEqual({
            input: '/docs/test.pdf',
            output: '/docs/test_extract_report.json',
            keywords: [],
            patterns: {},
            case_sensitive: false,
            context_len: 30
        });
    });

    it('builds args for pdf_search', async () => {
        const args = await buildOperationArgs('pdf_search', { ...defaultContext, searchQuery: 'FindMe' });
        expect(args).toEqual({
            input: '/docs/test.pdf',
            query: 'FindMe',
            output: '/docs/test_search.json'
        });
    });

    it('builds args for workspace_search', async () => {
        const args = await buildOperationArgs('workspace_search', { ...defaultContext, workspaceQuery: 'FindInWs' });
        expect(args).toEqual({
            input: '/docs/test.pdf',
            dir: '/docs',
            query: 'FindInWs',
            output: '/docs/test_workspace_search.json'
        });
    });

    it('builds args for semantic_search', async () => {
        const args = await buildOperationArgs('semantic_search', { ...defaultContext, semanticQuery: 'concepts' });
        expect(args).toEqual({
            input: '/docs/test.pdf',
            query: 'concepts',
            limit: 5,
            output: '/docs/test_semantic_search.json'
        });
    });

    it('builds args for pdf_classify_type', async () => {
        const args = await buildOperationArgs('pdf_classify_type', defaultContext);
        expect(args).toEqual({
            input: '/docs/test.pdf',
            output: '/docs/test_classified.json'
        });
    });

    it('builds args for pdf_rag_chunks', async () => {
        const args = await buildOperationArgs('pdf_rag_chunks', defaultContext);
        expect(args).toEqual({
            input: '/docs/test.pdf',
            output: '/docs/test_rag_chunks.json'
        });
    });

    it('builds args for pdf_chunk with defaults and custom parameters', async () => {
        const defaultArgs = await buildOperationArgs('pdf_chunk', defaultContext);
        expect(defaultArgs).toEqual({
            input: '/docs/test.pdf',
            output: '/docs/test_chunks.json',
            target_tokens: 500,
            overlap_tokens: 50,
            strip_boilerplate: true,
            break_on_headings: true
        });

        const customArgs = await buildOperationArgs('pdf_chunk', {
            ...defaultContext,
            chunkTargetTokens: 800,
            chunkOverlapTokens: 100,
            chunkStripBoilerplate: false,
            chunkBreakOnHeadings: false
        });
        expect(customArgs).toEqual({
            input: '/docs/test.pdf',
            output: '/docs/test_chunks.json',
            target_tokens: 800,
            overlap_tokens: 100,
            strip_boilerplate: false,
            break_on_headings: false
        });
    });

    it('builds args for document_workflow', async () => {
        const args = await buildOperationArgs('document_workflow', { ...defaultContext, workflowPreset: 'preset-legal-discovery' });
        expect(args).toEqual({
            input: '/docs/test.pdf',
            output: '/docs/test_workflow.pdf',
            preset: 'preset-legal-discovery'
        });
    });

    it('builds args for recipe_run', async () => {
        const args = await buildOperationArgs('recipe_run', { ...defaultContext, recipeId: 'preset-clean-scan' });
        expect(args).toEqual({
            input: '/docs/test.pdf',
            recipe: 'preset-clean-scan',
            output: '/docs/test_recipe.pdf'
        });
    });

    it('builds args for cli_ask', async () => {
        const args = await buildOperationArgs('cli_ask', { ...defaultContext, askPrompt: 'rotate 90 degrees', askDryRun: true });
        expect(args).toEqual({
            prompt: 'rotate 90 degrees',
            input: '/docs/test.pdf',
            output: '/docs/test_ask_plan.txt',
            dry_run: true
        });
    });

    it('builds args for watcher_start', async () => {
        const args = await buildOperationArgs('watcher_start', { ...defaultContext, watcherDir: '/watch', watcherRecipe: 'preset-legal-discovery' });
        expect(args).toEqual({
            dir: '/watch',
            recipe: 'preset-legal-discovery',
            settle_delay_ms: 500,
            output: '/docs/test_watcher.json'
        });
    });

    it('builds args for watcher_stop', async () => {
        const args = await buildOperationArgs('watcher_stop', { ...defaultContext, watcherId: 'watcher_123' });
        expect(args).toEqual({
            watcher_id: 'watcher_123',
            output: '/docs/test_watcher.json'
        });
    });
});
