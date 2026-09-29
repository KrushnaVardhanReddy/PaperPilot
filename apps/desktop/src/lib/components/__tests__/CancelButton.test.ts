import { render, screen, fireEvent, waitFor } from '@testing-library/svelte';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import CancelButton from '../ui/CancelButton.svelte';
import * as tauriApi from '@tauri-apps/api/core';

vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

describe('CancelButton', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders correctly', () => {
    render(CancelButton, { props: { jobId: 'job-123' } });
    expect(screen.getByText('✕ Cancel')).toBeTruthy();
  });

  it('calls invoke and onCancelled when clicked', async () => {
    const onCancelledMock = vi.fn();
    (tauriApi.invoke as any).mockResolvedValueOnce();

    render(CancelButton, { props: { jobId: 'job-123', onCancelled: onCancelledMock } });

    const button = screen.getByRole('button');
    await fireEvent.click(button);

    expect(tauriApi.invoke).toHaveBeenCalledWith('cancel_job', { jobId: 'job-123' });

    await waitFor(() => {
      expect(onCancelledMock).toHaveBeenCalled();
    });
  });

  it('disables button and shows "Cancelling…" while in progress', async () => {
    let resolveInvoke: any;
    const promise = new Promise(resolve => {
      resolveInvoke = resolve;
    });
    (tauriApi.invoke as any).mockReturnValueOnce(promise);

    render(CancelButton, { props: { jobId: 'job-123' } });

    const button = screen.getByRole('button') as HTMLButtonElement;
    await fireEvent.click(button);

    expect(button.disabled).toBe(true);
    expect(screen.getByText('Cancelling…')).toBeTruthy();

    resolveInvoke();

    await waitFor(() => {
      expect(button.disabled).toBe(false);
      expect(screen.getByText('✕ Cancel')).toBeTruthy();
    });
  });

  it('re-enables button if invoke fails', async () => {
    (tauriApi.invoke as any).mockRejectedValueOnce(new Error('Network error'));
    const consoleSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

    render(CancelButton, { props: { jobId: 'job-123' } });

    const button = screen.getByRole('button') as HTMLButtonElement;
    await fireEvent.click(button);

    await waitFor(() => {
      expect(button.disabled).toBe(false);
    });

    consoleSpy.mockRestore();
  });
});
