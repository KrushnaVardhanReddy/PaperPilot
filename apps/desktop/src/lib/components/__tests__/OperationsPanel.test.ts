import { render, screen, fireEvent, waitFor } from '@testing-library/svelte';
import OperationsPanel from '$lib/components/layout/OperationsPanel.svelte';
import { appState } from '$lib/state/app.svelte';

describe('OperationsPanel', () => {
  beforeEach(() => {
    // Reset app state before each test
    appState.documents = [];
    appState.selectedDocumentIndex = -1;
  });

  it('renders the operations selector', () => {
    render(OperationsPanel);

    // Check if the select element is present
    expect(screen.getByLabelText('Select Action')).toBeInTheDocument();

    // Check if default operation 'merge' params are visible
    expect(screen.getByText('Reorder files to set the merge order:')).toBeInTheDocument();
  });

  it('shows correct parameters when switching operations', async () => {
    render(OperationsPanel);

    const select = screen.getByLabelText('Select Action') as HTMLSelectElement;

    // Default is merge
    expect(select.value).toBe('merge');
    expect(screen.getByText('Reorder files to set the merge order:')).toBeInTheDocument();

    // Switch to split
    await fireEvent.change(select, { target: { value: 'split' } });
    expect(select.value).toBe('split');

    // Check if split params are visible
    await waitFor(() => {
      expect(screen.getByLabelText('Split Points (comma separated):')).toBeInTheDocument();
    });

    // Switch to rotate
    await fireEvent.change(select, { target: { value: 'rotate' } });

    // Check if rotate params are visible
    await waitFor(() => {
      expect(screen.getByLabelText('Rotation Angle:')).toBeInTheDocument();
    });
  });
});
