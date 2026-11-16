import { render, screen, fireEvent, cleanup } from '@testing-library/svelte';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { tick } from 'svelte';
import DocumentList from '$lib/components/ui/DocumentList.svelte';
import { appState } from '$lib/state/app.svelte';
import type { DocumentFacts } from '$lib/utils/documentFilters';
import recordedFacts from '../../../../tests/fixtures/automation/document-facts.json';

// These facts and complete PDFs are exported by the Rust desktop_automation test.
// No PDF operations or IPC responses are replaced in this view-level test.
function loadRealDocuments() {
  const names = ['text.pdf', 'scan.pdf', 'multi.pdf', 'form.pdf', 'signed.pdf', 'unsigned.pdf'];
  const files = names.map(name => new File([Uint8Array.from(readFileSync(resolve('tests/fixtures/automation', name)))], name, { type: 'application/pdf' }));
  appState.addDocuments(files);
  for (const file of files) {
    const facts = (recordedFacts as Record<string, DocumentFacts>)[file.name];
    expect(facts.sizeBytes).toBe(file.size);
    appState.inspections.set(file, { status: 'ready', facts });
  }
  return files;
}

describe('DocumentList', () => {
  beforeEach(() => {
    // Reset app state before each test
    appState.documents = [];
    appState.documentPaths = []; appState.openDocIndices = []; appState.inspections.clear();
    appState.selectedDocumentIndex = null;
  });
  afterEach(cleanup);

  it('renders an empty state when no documents are present', () => {
    render(DocumentList);

    expect(screen.getByText('No documents added yet.')).toBeInTheDocument();
  });

  it('renders a list of documents and displays their names', () => {
    loadRealDocuments();
    render(DocumentList);
    expect(screen.getByText('text.pdf')).toBeInTheDocument();
    expect(screen.getByText('scan.pdf')).toBeInTheDocument();
    expect(screen.queryByText('No documents added yet.')).not.toBeInTheDocument();
  });

  it('counts and filters real scan, signed, form and multi-page facts', async () => {
    loadRealDocuments(); render(DocumentList);
    for (const label of ['All documents (6)', 'Scanned (1)', 'Signed (1)', 'Form fields (2)', 'Multi-page (1)']) expect(screen.getByRole('button', { name: label })).toBeInTheDocument();
    await fireEvent.click(screen.getByRole('button', { name: 'Scanned (1)' }));
    expect(screen.getByText('scan.pdf')).toBeInTheDocument(); expect(screen.queryByText('text.pdf')).not.toBeInTheDocument();
    await fireEvent.click(screen.getByRole('button', { name: 'Signed (1)' }));
    expect(screen.getByText('signed.pdf')).toBeInTheDocument(); expect(screen.queryByText('unsigned.pdf')).not.toBeInTheDocument();
    await fireEvent.click(screen.getByRole('button', { name: 'Form fields (2)' }));
    expect(screen.getByText('signed.pdf')).toBeInTheDocument(); expect(screen.getByText('form.pdf')).toBeInTheDocument();
    await fireEvent.click(screen.getByRole('button', { name: 'Multi-page (1)' }));
    expect(screen.getByText('multi.pdf')).toBeInTheDocument(); expect(screen.queryByText('form.pdf')).not.toBeInTheDocument();
  });

  it('uses original document indices when opening or removing a filtered item', async () => {
    const files = loadRealDocuments(); render(DocumentList);
    await fireEvent.click(screen.getByRole('button', { name: 'Signed (1)' }));
    await fireEvent.click(screen.getByRole('button', { name: 'View signed.pdf' }));
    expect(appState.documents[appState.selectedDocumentIndex!]).toBe(files[4]);
    await fireEvent.click(screen.getByTitle('Remove Document'));
    expect(appState.documents).not.toContain(files[4]); expect(appState.documents).toContain(files[0]);
    expect(appState.inspections.has(files[4])).toBe(false);
    expect(screen.getByRole('button', { name: 'Signed (0)' })).toBeInTheDocument();
    expect(screen.getByText('No documents match this filter.')).toBeInTheDocument();
  });

  it('retains metadata and selection when real documents are reordered', async () => {
    const files = loadRealDocuments(); appState.selectDocument(4); render(DocumentList);
    appState.reorderDocuments(4, 0); await tick();
    expect(appState.selectedDocumentIndex).toBe(0); expect(appState.documents[0]).toBe(files[4]);
    await fireEvent.click(screen.getByRole('button', { name: 'Scanned (1)' }));
    await fireEvent.click(screen.getByRole('button', { name: 'View scan.pdf' }));
    expect(appState.documents[appState.selectedDocumentIndex!]).toBe(files[1]);
    expect(screen.getByRole('button', { name: 'Signed (1)' })).toBeInTheDocument();
  });

  it('keeps a genuinely truncated PDF with unavailable inspection in All documents', () => {
    loadRealDocuments();
    const source = readFileSync(resolve('tests/fixtures/automation/text.pdf'));
    appState.addDocuments([new File([Uint8Array.from(source.subarray(0, 32))], 'truncated.pdf', { type: 'application/pdf' })]);
    render(DocumentList);
    expect(screen.getByRole('button', { name: 'All documents (7)' })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Scanned (1)' })).toBeInTheDocument();
    expect(screen.getByText('truncated.pdf')).toBeInTheDocument();
    expect(screen.getByText(/Details unavailable for 1 document/)).toBeInTheDocument();
  });
});
