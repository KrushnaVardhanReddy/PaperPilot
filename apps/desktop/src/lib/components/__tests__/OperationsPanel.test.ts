import { render, screen, fireEvent, waitFor } from '@testing-library/svelte';
import OperationsPanel from '$lib/components/layout/OperationsPanel.svelte';
import { appState } from '$lib/state/app.svelte';

describe('OperationsPanel', () => {
  beforeEach(() => {
    // Reset app state before each test
    appState.documents = [];
    appState.selectedDocumentIndex = -1;
  });

  it('renders the operations search and directory initially', () => {
    render(OperationsPanel);

    // Check if the search input is present
    expect(screen.getByPlaceholderText('Search tools...')).toBeInTheDocument();

    // With context Filter set to 'recommended' initially and no files, it should show creation tools in search-results-list
    // Click "All" context chip to test categories
    const allChip = screen.getByText(/All Tools/);
    fireEvent.click(allChip);

    // Check if category headers are visible after clicking All Tools
    waitFor(() => {
       expect(screen.getAllByText('Quick Actions').length).toBeGreaterThan(0);
       expect(screen.getAllByText('Page Management').length).toBeGreaterThan(0);
    });
  });

  it('shows correct parameters when switching to an operation', async () => {
    render(OperationsPanel);

    // Switch to All Tools to find split/rotate easily
    const allChip = screen.getByText(/All Tools/);
    await fireEvent.click(allChip);

    // Click on the split tool card
    const splitButton = screen.getAllByText('Split PDF')[0].closest('button');
    expect(splitButton).not.toBeNull();
    if (splitButton) {
      await fireEvent.click(splitButton);
    }

    // Check if split params are visible in Inspector Mode
    await waitFor(() => {
      expect(screen.getByLabelText('Split Page Ranges (comma separated):')).toBeInTheDocument();
    });

    // Go back to directory
    const backButton = screen.getByTitle('Back to tool list');
    await fireEvent.click(backButton);

    // Click on the rotate tool card
    const rotateButton = screen.getAllByText('Rotate Pages')[0].closest('button');
    expect(rotateButton).not.toBeNull();
    if (rotateButton) {
      await fireEvent.click(rotateButton);
    }

    // Check if rotate params are visible
    await waitFor(() => {
      expect(screen.getByLabelText('Rotation Angle:')).toBeInTheDocument();
    });
  });
});
