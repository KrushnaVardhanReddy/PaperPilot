import { describe, it, expect } from 'vitest';
import { resolveToolName } from './toolRegistry';

describe('toolRegistry', () => {
    it('resolves creation tools correctly', () => {
        expect(resolveToolName('md_to_pdf')).toBe('pdf_convert_markdown');
        expect(resolveToolName('html_to_pdf')).toBe('pdf_convert_html');
        expect(resolveToolName('img_to_pdf')).toBe('pdf_images_to_pdf');
        expect(resolveToolName('json_to_pdf')).toBe('json_to_pdf');
    });

    it('resolves form tools correctly', () => {
        expect(resolveToolName('create_form_field')).toBe('pdf_create_form_field');
        expect(resolveToolName('fill_form')).toBe('pdf_fill_form');
        expect(resolveToolName('read_form')).toBe('pdf_read_form');
    });

    it('returns toolId directly for certain export/office conversions', () => {
        expect(resolveToolName('pdf_to_docx')).toBe('pdf_to_docx');
        expect(resolveToolName('office_to_markdown')).toBe('office_to_markdown');
        expect(resolveToolName('excel_to_json')).toBe('excel_to_json');
        expect(resolveToolName('office_modernize_legacy')).toBe('office_modernize_legacy');
    });

    it('adds pdf_ prefix to standard tools by default', () => {
        expect(resolveToolName('merge')).toBe('pdf_merge');
        expect(resolveToolName('split')).toBe('pdf_split');
        expect(resolveToolName('compress')).toBe('pdf_compress');
    });
});
