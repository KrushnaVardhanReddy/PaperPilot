import { describe, it, expect, vi, beforeEach } from 'vitest';
import { PaperPilot } from '../src/index';
import * as child_process from 'child_process';

vi.mock('child_process', () => {
    return {
        execFile: vi.fn((file, args, cb) => {
            cb(null, { stdout: 'success', stderr: '' });
        }),
    };
});

describe('PaperPilot SDK Remote Mode', () => {
    let client: PaperPilot;
    const mockFetch = vi.fn();

    beforeEach(() => {
        vi.clearAllMocks();
        global.fetch = mockFetch;
        mockFetch.mockResolvedValue({
            ok: true,
            json: () => Promise.resolve({ success: true })
        });
        client = new PaperPilot({ endpointUrl: 'http://localhost:7823', apiKey: 'test-key' });
    });

    it('should execute remote merge successfully', async () => {
        await client.merge(['a.pdf', 'b.pdf'], { output: 'out.pdf' });
        expect(mockFetch).toHaveBeenCalledWith('http://localhost:7823/api/v1/pdf/merge', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'Authorization': 'Bearer test-key'
            },
            body: JSON.stringify({ inputs: ['a.pdf', 'b.pdf'], output: 'out.pdf' })
        });
    });

    it('should execute remote compress successfully', async () => {
        await client.compress('in.pdf', { quality: 'medium', output: 'out.pdf' });
        expect(mockFetch).toHaveBeenCalledWith('http://localhost:7823/api/v1/pdf/compress', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'Authorization': 'Bearer test-key'
            },
            body: JSON.stringify({ input: 'in.pdf', quality: 'medium', output: 'out.pdf' })
        });
    });

    it('should execute remote split successfully', async () => {
        await client.split('in.pdf', { pages: '1-3', output: 'out_dir' });
        expect(mockFetch).toHaveBeenCalledWith('http://localhost:7823/api/v1/pdf/split', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'Authorization': 'Bearer test-key'
            },
            body: JSON.stringify({ input: 'in.pdf', pages: '1-3', output: 'out_dir' })
        });
    });

    it('should throw error for remote batesStamp', async () => {
        await expect(client.batesStamp('in.pdf', { prefix: 'DOC-', start: 1, output: 'out.pdf' })).rejects.toThrow('batesStamp is local-only');
    });
});

describe('PaperPilot SDK Local Mode', () => {
    let client: PaperPilot;

    beforeEach(() => {
        vi.clearAllMocks();
        client = new PaperPilot(); // local mode
    });

    it('should execute local merge successfully', async () => {
        await client.merge(['a.pdf', 'b.pdf'], { output: 'out.pdf' });
        expect(child_process.execFile).toHaveBeenCalledWith(
            'paperpilot-cli',
            ['merge', '--input', 'a.pdf', '--input', 'b.pdf', '--output', 'out.pdf'],
            expect.any(Function)
        );
    });

    it('should execute local compress successfully', async () => {
        await client.compress('in.pdf', { quality: 'medium', output: 'out.pdf' });
        expect(child_process.execFile).toHaveBeenCalledWith(
            'paperpilot-cli',
            ['compress', '--input', 'in.pdf', '--quality', 'medium', '--output', 'out.pdf'],
            expect.any(Function)
        );
    });

    it('should execute local batesStamp successfully', async () => {
        await client.batesStamp('in.pdf', { prefix: 'DOC-', start: 1, output: 'out.pdf' });
        expect(child_process.execFile).toHaveBeenCalledWith(
            'paperpilot-cli',
            ['bates', '--input', 'in.pdf', '--prefix', 'DOC-', '--start', '1', '--output', 'out.pdf'],
            expect.any(Function)
        );
    });

    it('should execute local split successfully', async () => {
        await client.split('in.pdf', { pages: '1-3', output: 'out_dir' });
        expect(child_process.execFile).toHaveBeenCalledWith(
            'paperpilot-cli',
            ['split', '--input', 'in.pdf', '--pages', '1-3', '--output', 'out_dir'],
            expect.any(Function)
        );
    });
});
