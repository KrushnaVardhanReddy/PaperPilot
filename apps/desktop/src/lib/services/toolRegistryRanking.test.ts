import { describe, it, expect } from 'vitest';
import { rankToolsByContext } from './toolRegistry';
import { allTools } from '../types/toolList';

describe('Tool Registry Ranking Engine', () => {
    it('prioritizes excel_to_json for Excel documents', () => {
        const fakeFile = { name: 'financials.xlsx', size: 1024 } as unknown as File;
        const result = rankToolsByContext(allTools, [fakeFile], ['office']);

        const recommendedIds = result.recommended.map(t => t.id);
        expect(recommendedIds).toContain('excel_to_json');
        expect(recommendedIds).toContain('excel_to_csv');
        // Does not recommend merge (minDocuments: 2) or basic PDF tools if not PDF
        expect(recommendedIds).not.toContain('merge');
    });

    it('prioritizes merge and batch tools when >=2 PDFs are present', () => {
        const f1 = { name: 'page1.pdf', size: 1024 } as unknown as File;
        const f2 = { name: 'page2.pdf', size: 1024 } as unknown as File;
        const result = rankToolsByContext(allTools, [f1, f2], ['core', 'office', 'ocr']);

        const recommendedIds = result.recommended.map(t => t.id);
        expect(recommendedIds).toContain('merge');
        expect(recommendedIds).toContain('compare');
        expect(recommendedIds).toContain('compress');
        expect(recommendedIds).toContain('ocr');

        // Split should NOT be recommended for multiple files (maxDocuments: 1)
        expect(recommendedIds).not.toContain('split');
    });

    it('flags required packs when absent for relevant context tools', () => {
        const fakeFile = { name: 'data.xlsx', size: 1024 } as unknown as File;
        // office pack is missing
        const result = rankToolsByContext(allTools, [fakeFile], ['core']);

        // Since office pack is required for excel_to_json, it should be in packLocked
        const lockedIds = result.packLocked.map(l => l.tool.id);
        expect(lockedIds).toContain('excel_to_json');
        expect(lockedIds).toContain('excel_to_csv');
        // office_to_markdown is not a recommended tool for xlsx only context, so it won't be flagged in packLocked

        // They should not appear in recommended if pack is missing
        expect(result.recommended.map(t => t.id)).not.toContain('excel_to_json');
    });

    it('recommends creation tools when no files are open', () => {
        const result = rankToolsByContext(allTools, [], []);
        const recommendedIds = result.recommended.map(t => t.id);

        expect(recommendedIds).toContain('img_to_pdf');
        expect(recommendedIds).toContain('json_to_pdf');
        expect(recommendedIds).toContain('md_to_pdf');
    });
});
