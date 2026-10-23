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
        const args = await buildOperationArgs('md_to_pdf', defaultContext);
        expect(args).toEqual({
            input: '/mock/path.md',
            output: '/mock/save_path.pdf',
            preset: 'standard'
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
            output: '/docs/test_pii_report.json'
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
});
