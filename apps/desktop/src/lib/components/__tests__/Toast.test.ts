import { render, screen } from '@testing-library/svelte';
import ToastContainer from '$lib/components/layout/ToastContainer.svelte';
import { toastState } from '$lib/state/toast.svelte';

describe('ToastContainer', () => {
  beforeEach(() => {
    // Clear toasts before each test
    toastState.toasts = [];
  });

  it('renders a success toast with correct message and class', async () => {
    render(ToastContainer);

    toastState.success('Test success message', 0); // 0 duration so it doesn't auto-dismiss

    // Using findByText as we might need to wait a tick for runes to render
    const toastMessage = await screen.findByText('Test success message');
    expect(toastMessage).toBeInTheDocument();

    // The closest .toast element should have .toast-success
    const toastDiv = toastMessage.closest('.toast');
    expect(toastDiv).toHaveClass('toast-success');
  });

  it('renders an error toast with correct message and class', async () => {
    render(ToastContainer);

    toastState.error('Test error message', 0);

    const toastMessage = await screen.findByText('Test error message');
    expect(toastMessage).toBeInTheDocument();

    const toastDiv = toastMessage.closest('.toast');
    expect(toastDiv).toHaveClass('toast-error');
  });
});
