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

  it('does not apply zoom transform to header', () => {
    appState.isMobileViewport = true;
    appState.isTauriMobile = true;
    appState.addDocuments([new File([''], 'test.pdf')]);
    appState.selectDocument(0);

    const { container } = render(PdfPreview);
    const header = container.querySelector('.preview-header') as HTMLElement;
    const thumbnailsGrid = container.querySelector('.thumbnails-grid') as HTMLElement;

    // Header should not have a transform
    expect(header.style.transform).toBe('');
    // Grid should have transform initial state
    expect(thumbnailsGrid.style.transform).toContain('scale(1)');
  });

  it('double tap toggles zoom scale between 1.0 and 2.5', async () => {
    appState.isMobileViewport = true;
    appState.isTauriMobile = true;

    const { container } = render(PdfPreview);
    const previewContent = container.querySelector('.preview-content') as HTMLElement;
    const thumbnailsGrid = container.querySelector('.thumbnails-grid') as HTMLElement;

    vi.useFakeTimers();

    // First tap
    await fireEvent.pointerDown(previewContent, { pointerId: 1, clientX: 100, clientY: 100 });

    // Fast second tap
    vi.setSystemTime(new Date(Date.now() + 50));
    await fireEvent.pointerDown(previewContent, { pointerId: 1, clientX: 100, clientY: 100 });

    // Svelte state updates asynchronously after pointerdown handling depending on render cycle, but vitest flush can help or immediate read if reactive DOM updates are synchronous. Svelte 5 runes usually need microtask flush or await tick().
    // fireEvent handles some of this, but let's check style.
    expect(thumbnailsGrid.style.transform).toContain('scale(2.5)');

    // Need a tiny delay so it's considered a new tap sequence (or mock Date.now)
    vi.setSystemTime(new Date(Date.now() + 400));

    // Double tap again to reset
    await fireEvent.pointerDown(previewContent, { pointerId: 1, clientX: 100, clientY: 100 });
    vi.setSystemTime(new Date(Date.now() + 50));
    await fireEvent.pointerDown(previewContent, { pointerId: 1, clientX: 100, clientY: 100 });

    expect(thumbnailsGrid.style.transform).toContain('scale(1)');
    vi.useRealTimers();
  });

  it('single finger move when zoomed in updates pan translation coordinates', async () => {
    appState.isMobileViewport = true;
    appState.isTauriMobile = true;

    const { container } = render(PdfPreview);
    const previewContent = container.querySelector('.preview-content') as HTMLElement;
    const thumbnailsGrid = container.querySelector('.thumbnails-grid') as HTMLElement;

    vi.useFakeTimers();

    // Double tap to zoom in
    await fireEvent.pointerDown(previewContent, { pointerId: 1, clientX: 100, clientY: 100 });
    vi.setSystemTime(new Date(Date.now() + 50));
    await fireEvent.pointerDown(previewContent, { pointerId: 1, clientX: 100, clientY: 100 });

    expect(thumbnailsGrid.style.transform).toContain('scale(2.5)');

    // Fast forward to avoid double tap on next touch
    vi.setSystemTime(new Date(Date.now() + 400));

    // Now zoomed in, pan
    await fireEvent.pointerDown(previewContent, { pointerId: 1, clientX: 150, clientY: 150 });
    await fireEvent.pointerMove(previewContent, { pointerId: 1, clientX: 200, clientY: 250 });

    expect(thumbnailsGrid.style.transform).toContain('translate(50px, 100px)');

    vi.useRealTimers();
  });

  it('single finger horizontal swipe when zoomed out triggers navigation', async () => {
    appState.isMobileViewport = true;
    appState.isTauriMobile = true;

    // Mock navigator.vibrate
    const mockVibrate = vi.fn();
    Object.defineProperty(global.navigator, 'vibrate', {
      value: mockVibrate,
      configurable: true
    });

    const { container } = render(PdfPreview);
    const previewContent = container.querySelector('.preview-content') as HTMLElement;

    // Zoom is 1.0 (zoomed out). Swipe right
    await fireEvent.pointerDown(previewContent, { pointerId: 1, clientX: 100, clientY: 100 });
    await fireEvent.pointerUp(previewContent, { pointerId: 1, clientX: 200, clientY: 100 });

    expect(toastState.success).toHaveBeenCalledWith('Swiped Right');
    expect(mockVibrate).toHaveBeenCalledWith(10);
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
