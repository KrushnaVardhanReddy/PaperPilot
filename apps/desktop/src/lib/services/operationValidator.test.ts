import { describe, it, expect } from 'vitest';
import { validatePreconditions } from './operationValidator';

describe('operationValidator', () => {
    it('returns error when compare receives less than 2 documents', () => {
        expect(validatePreconditions('compare', 1)).toEqual({
            valid: false,
            error: 'Need at least 2 documents loaded to compare.'
        });
    });

    it('succeeds when compare receives 2 documents', () => {
        expect(validatePreconditions('compare', 2)).toEqual({ valid: true });
    });

    it('returns error when merge receives less than 2 documents', () => {
        expect(validatePreconditions('merge', 1)).toEqual({
            valid: false,
            error: 'Need at least 2 documents in tabs to merge.'
        });
    });

    it('succeeds when merge receives 2 documents', () => {
        expect(validatePreconditions('merge', 2)).toEqual({ valid: true });
    });

    it('succeeds when creation tools receive 0 documents', () => {
        ['md_to_pdf', 'html_to_pdf', 'img_to_pdf', 'json_to_pdf'].forEach(toolId => {
            expect(validatePreconditions(toolId, 0)).toEqual({ valid: true });
        });
    });

    it('returns error when non-creation tools receive 0 documents', () => {
        expect(validatePreconditions('compress', 0)).toEqual({
            valid: false,
            error: 'No documents available. Add or drop a PDF first.'
        });
    });

    it('succeeds for standard tools with 1 document', () => {
        expect(validatePreconditions('compress', 1)).toEqual({ valid: true });
    });
});
