import { describe, it, expect } from 'vitest';
import { generateSnippets, type SnippetTarget } from './snippetGenerator';

describe('snippetGenerator', () => {
    it('generates correct snippets for pdf_rotate', () => {
        const target: SnippetTarget = {
            toolId: 'pdf_rotate',
            intent: 'Rotate',
            params: {
                angle: 180,
                pages: '1-5'
            },
            inputFile: 'my_doc.pdf',
            outputFile: 'my_doc_rotated.pdf'
        };

        const snippets = generateSnippets(target);

        expect(snippets.cli).toContain('paperpilot rotate --input "my_doc.pdf" --angle 180 --pages "1-5" --output "my_doc_rotated.pdf"');
        expect(snippets.curl).toContain('curl -X POST http://localhost:8080/api/v1/tools/rotate -F "file=@my_doc.pdf" -F "angle=180"');
        expect(snippets.mcpToolCall).toContain('"name": "pdf_rotate"');
        expect(snippets.mcpToolCall).toContain('"angle": 180');
        expect(snippets.mcpAgentPrompt).toContain('rotate "my_doc.pdf" by 180 degrees');
        expect(snippets.embedJs).toContain("window.PaperPilot.executeTool('pdf_rotate'");
    });

    it('generates correct snippets for pdf_bates', () => {
        const target: SnippetTarget = {
            toolId: 'pdf_bates',
            intent: 'Bates',
            params: {
                prefix: 'CONFIDENTIAL-',
                start_number: 100,
                padding: 4
            },
            inputFile: 'input.pdf',
            outputFile: 'output.pdf'
        };

        const snippets = generateSnippets(target);

        expect(snippets.cli).toContain('paperpilot bates --input "input.pdf" --prefix "CONFIDENTIAL-" --start 100 --padding 4 --output "output.pdf"');
        expect(snippets.mcpToolCall).toContain('"name": "pdf_bates"');
        expect(snippets.mcpAgentPrompt).toContain('prefix "CONFIDENTIAL-"');
    });

    it('generates correct snippets for pdf_watermark', () => {
        const target: SnippetTarget = {
            toolId: 'pdf_watermark',
            intent: 'Watermark',
            params: {
                text: 'DRAFT',
                opacity: 0.5
            },
            inputFile: 'doc.pdf',
            outputFile: 'out.pdf'
        };

        const snippets = generateSnippets(target);

        expect(snippets.cli).toContain('paperpilot watermark --input "doc.pdf" --text "DRAFT" --opacity 0.5 --output "out.pdf"');
        expect(snippets.curl).toContain('-F "text=DRAFT"');
        expect(snippets.mcpToolCall).toContain('"name": "pdf_watermark"');
        expect(snippets.mcpAgentPrompt).toContain('watermark text "DRAFT"');
    });

    it('generates correct snippets for pdf_encrypt', () => {
        const target: SnippetTarget = {
            toolId: 'pdf_encrypt',
            intent: 'Encrypt',
            params: {
                password: 'my-secret-pw'
            },
            inputFile: 'in.pdf',
            outputFile: 'out.pdf'
        };

        const snippets = generateSnippets(target);

        expect(snippets.cli).toContain('paperpilot encrypt --input "in.pdf" --password "my-secret-pw" --output "out.pdf"');
        expect(snippets.mcpToolCall).toContain('"name": "pdf_encrypt"');
        expect(snippets.mcpToolCall).toContain('"user_password": "my-secret-pw"');
        expect(snippets.mcpAgentPrompt).toContain('password "my-secret-pw"');
    });

    it('generates correct snippets for pdf_compress', () => {
        const target: SnippetTarget = {
            toolId: 'pdf_compress',
            intent: 'Compress',
            params: {},
            inputFile: 'big.pdf',
            outputFile: 'small.pdf'
        };

        const snippets = generateSnippets(target);

        expect(snippets.cli).toContain('paperpilot compress --input "big.pdf" --quality 80 --output "small.pdf"');
        expect(snippets.mcpToolCall).toContain('"name": "pdf_compress"');
    });

    it('generates correct snippets for default fallback', () => {
        const target: SnippetTarget = {
            toolId: 'pdf_split',
            intent: 'Split',
            params: {
                ranges: '1-2'
            },
            inputFile: 'book.pdf',
            outputFile: 'page_%d.pdf'
        };

        const snippets = generateSnippets(target);

        expect(snippets.cli).toContain('paperpilot split --input "book.pdf" --output "page_%d.pdf"');
        expect(snippets.curl).toContain('curl -X POST http://localhost:8080/api/v1/tools/split');
        expect(snippets.mcpToolCall).toContain('"name": "pdf_split"');
        expect(snippets.mcpToolCall).toContain('"ranges": "1-2"');
        expect(snippets.mcpAgentPrompt).toContain('Perform Split on "book.pdf"');
        expect(snippets.embedJs).toContain("window.PaperPilot.executeTool('pdf_split'");
    });
});
