import { describe, it, expect } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/svelte';
import ApiDocsView from './ApiDocsView.svelte';

describe('ApiDocsView', () => {
  it('renders title and gateway input', () => {
    render(ApiDocsView);
    expect(screen.getByText('PaperPilot API & Gateway')).toBeTruthy();
    const input = screen.getByLabelText('Gateway URL') as HTMLInputElement;
    expect(input.value).toBe('http://127.0.0.1:7823');
  });

  it('filters endpoints by search query', async () => {
    render(ApiDocsView);
    const searchInput = screen.getByPlaceholderText('Search endpoints by path or description...') as HTMLInputElement;

    // initially we should have several endpoints visible, including /health
    const healthPathsBefore = screen.getAllByText('/health');
    expect(healthPathsBefore.length).toBeGreaterThan(0);

    // type a query that shouldn't match /health
    await fireEvent.input(searchInput, { target: { value: 'mcp-exec' } });

    // /health should be filtered out
    const healthPathsAfter = screen.queryAllByText('/health');
    expect(healthPathsAfter.length).toBe(0);

    // /api/v1/pdf/mcp-exec should be present
    const execPaths = screen.getAllByText('/api/v1/pdf/mcp-exec');
    expect(execPaths.length).toBeGreaterThan(0);
  });

  it('expands endpoint and shows cURL', async () => {
    render(ApiDocsView);

    // Find the /health endpoint header
    const healthPath = screen.getAllByText('/health')[0];
    const endpointHeader = healthPath.closest('.endpoint-header');
    expect(endpointHeader).toBeTruthy();

    if (endpointHeader) {
       await fireEvent.click(endpointHeader);
       // Now the details should be visible, including a Copy cURL button
       const curlBtn = screen.getByRole('button', { name: /copy curl/i });
       expect(curlBtn).toBeTruthy();

       // Verify cURL content for GET /health
       const pre = screen.getByText(/curl -X GET "http:\/\/127\.0\.0\.1:7823\/health"/);
       expect(pre).toBeTruthy();
    }
  });
});
