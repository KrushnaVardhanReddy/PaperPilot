import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, fireEvent } from '@testing-library/svelte';
import PdfPreview from './PdfPreview.svelte';
import { appState } from '$lib/state/app.svelte';
import { toastState } from '$lib/state/toast.svelte';

vi.mock('$lib/state/toast.svelte', () => ({
  toastState: { success: vi.fn(), error: vi.fn() }
}));

describe('PdfPreview Mobile UI', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders Export & Share button on mobile and triggers native share', async () => {
    appState.isMobileViewport = true;
    appState.isTauriMobile = true;

    // add dummy document to appState
    appState.addDocuments([new File([''], 'test.pdf')]);
    appState.selectDocument(0);

    // mock navigator share
    const mockShare = vi.fn().mockResolvedValue(undefined);
    Object.defineProperty(global.navigator, 'share', {
      value: mockShare,
      configurable: true
    });

    const { getByText } = render(PdfPreview);
    const shareBtn = getByText('📤 Export & Share');
    expect(shareBtn).toBeTruthy();

    await fireEvent.click(shareBtn);
    expect(mockShare).toHaveBeenCalled();
  });

});
