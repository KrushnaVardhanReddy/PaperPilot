import { describe, it, expect } from 'vitest';
import { searchDocumentText } from './workspaceSearch';

describe('workspaceSearch', () => {
    it('returns empty array when query is empty', () => {
        const matches = searchDocumentText(0, 'test.pdf', 'This is some text', '');
        expect(matches).toEqual([]);
    });

    it('returns empty array when text is empty', () => {
        const matches = searchDocumentText(0, 'test.pdf', '', 'query');
        expect(matches).toEqual([]);
    });

    it('finds basic case-insensitive matches by default', () => {
        const matches = searchDocumentText(0, 'test.pdf', 'This is a Query inside a document. Another query here.', 'query');
        expect(matches.length).toBe(2);
        expect(matches[0].snippet).toContain('<mark>Query</mark>');
        expect(matches[1].snippet).toContain('<mark>query</mark>');
    });

    it('respects caseSensitive option', () => {
        const matches = searchDocumentText(0, 'test.pdf', 'This is a Query inside a document. Another query here.', 'Query', { caseSensitive: true });
        expect(matches.length).toBe(1);
        expect(matches[0].snippet).toContain('<mark>Query</mark>');
    });

    it('respects wholeWord option', () => {
        const matches = searchDocumentText(0, 'test.pdf', 'This query inside queries list.', 'query', { wholeWord: true });
        expect(matches.length).toBe(1);
        expect(matches[0].snippet).toContain('<mark>query</mark>');
    });

    it('generates correct snippets with context and ellipsis', () => {
        const text = 'A'.repeat(50) + ' target ' + 'B'.repeat(50);
        const matches = searchDocumentText(0, 'test.docx', text, 'target');
        expect(matches.length).toBe(1);

        const snippet = matches[0].snippet;
        // Verify snippet has ellipsis, the matched text in marks, and context
        expect(snippet.startsWith('...')).toBe(true);
        expect(snippet.endsWith('...')).toBe(true);
        expect(snippet).toContain('<mark>target</mark>');
    });

    it('sets the correct document type based on filename extension', () => {
        const matchesPdf = searchDocumentText(1, 'file.pdf', 'test', 'test');
        expect(matchesPdf[0].docType).toBe('pdf');

        const matchesWord = searchDocumentText(2, 'file.docx', 'test', 'test');
        expect(matchesWord[0].docType).toBe('docx');

        const matchesExcel = searchDocumentText(3, 'file.xlsx', 'test', 'test');
        expect(matchesExcel[0].docType).toBe('xlsx');

        const matchesPpt = searchDocumentText(4, 'file.pptx', 'test', 'test');
        expect(matchesPpt[0].docType).toBe('pptx');

        const matchesOther = searchDocumentText(5, 'file.txt', 'test', 'test');
        expect(matchesOther[0].docType).toBe('other');
    });

    it('escapes HTML to prevent XSS in generated snippets', () => {
        const textWithXss1 = 'Here is a malicious <img src=x onerror=alert(1)> tag payload.';
        const matches1 = searchDocumentText(0, 'test1.pdf', textWithXss1, 'malicious');
        expect(matches1.length).toBe(1);
        expect(matches1[0].snippet).toContain('&lt;img src=x onerror=alert(1)&gt;');
        expect(matches1[0].snippet).not.toContain('<img');

        const textWithXss2 = 'And this is a <script>evil()</script> injection attempt.';
        const matches2 = searchDocumentText(0, 'test2.pdf', textWithXss2, 'injection');
        expect(matches2.length).toBe(1);
        expect(matches2[0].snippet).toContain('&lt;script&gt;evil()&lt;/script&gt;');
        expect(matches2[0].snippet).not.toContain('<script>');
    });
});