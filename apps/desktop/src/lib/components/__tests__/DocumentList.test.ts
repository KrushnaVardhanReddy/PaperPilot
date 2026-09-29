import { render, screen } from '@testing-library/svelte';
import DocumentList from '$lib/components/ui/DocumentList.svelte';
import { appState } from '$lib/state/app.svelte';

describe('DocumentList', () => {
  beforeEach(() => {
    // Reset app state before each test
    appState.documents = [];
    appState.selectedDocumentIndex = -1;
  });

  it('renders an empty state when no documents are present', () => {
    render(DocumentList);

    expect(screen.getByText('No documents added yet.')).toBeInTheDocument();
  });

  it('renders a list of documents and displays their names', () => {
    // Mock the state by providing objects that match the File interface partially but enough for the app
    // We can cast them as File to satisfy TypeScript if needed, or if appState expects full File objects
    // we use a minimal mock.
    const file1 = new File([''], 'test_doc_1.pdf', { type: 'application/pdf' });
    const file2 = new File([''], 'test_doc_2.pdf', { type: 'application/pdf' });

    // Set the state
    appState.documents = [file1, file2];

    render(DocumentList);

    expect(screen.getByText('test_doc_1.pdf')).toBeInTheDocument();
    expect(screen.getByText('test_doc_2.pdf')).toBeInTheDocument();

    // Check if the empty state is not rendered
    expect(screen.queryByText('No documents added yet.')).not.toBeInTheDocument();
  });
});
