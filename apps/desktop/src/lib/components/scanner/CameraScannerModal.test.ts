import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, fireEvent, screen, waitFor } from '@testing-library/svelte';
import CameraScannerModal from './CameraScannerModal.svelte';
import { appState } from '$lib/state/app.svelte';
import * as tauriCore from '@tauri-apps/api/core';

vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn()
}));

// Mock mediaDevices
Object.defineProperty(global.navigator, 'mediaDevices', {
  value: {
    getUserMedia: vi.fn().mockResolvedValue({
      getTracks: () => [{ stop: vi.fn() }]
    })
  }
});

// Mock Canvas implementation enough for testing
HTMLCanvasElement.prototype.getContext = vi.fn().mockReturnValue({
  drawImage: vi.fn(),
  translate: vi.fn(),
  rotate: vi.fn(),
  clearRect: vi.fn(),
  save: vi.fn(),
  restore: vi.fn(),
  putImageData: vi.fn(),
  getImageData: vi.fn().mockReturnValue({ data: [] }),
  createImageData: vi.fn().mockReturnValue({ data: [] })
}) as any;
HTMLCanvasElement.prototype.toDataURL = vi.fn().mockReturnValue('data:image/jpeg;base64,dummy123');

describe('CameraScannerModal (Mobile Spec)', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders modal and accesses camera', async () => {
    const { getByRole } = render(CameraScannerModal, { props: { onClose: vi.fn() } });
    expect(navigator.mediaDevices.getUserMedia).toHaveBeenCalledWith({ video: { facingMode: 'environment' } });
  });

  it('captures multi-images and pipelines them via images_to_pdf with OCR', async () => {
    const { getByRole, getByText, getByLabelText } = render(CameraScannerModal, { props: { onClose: vi.fn() } });

    // simulate taking 2 photos
    const shutter = getByRole('button', { name: /take photo/i });
    await fireEvent.click(shutter);

    // Accept crop modal for first photo
    let acceptCrop = await screen.findByText('✓ Straighten & Use');
    await fireEvent.click(acceptCrop);

    await fireEvent.click(shutter);
    acceptCrop = await screen.findByText('✓ Straighten & Use');
    await fireEvent.click(acceptCrop);

    // we should see the convert button with count 2
    const convertBtn = await screen.findByText('Convert to PDF (2)');
    expect(convertBtn).toBeTruthy();

    // OCR toggle is on by default
    const ocrToggle = getByLabelText(/Make Searchable \(OCR\)/i) as HTMLInputElement;
    expect(ocrToggle.checked).toBe(true);

    // Click convert
    (tauriCore.invoke as any).mockResolvedValue({
      success: true,
      fileName: 'scanned.pdf',
      dataBase64: btoa('dummy_pdf_bytes')
    });

    await fireEvent.click(convertBtn);

    expect(tauriCore.invoke).toHaveBeenCalledWith('process_mobile_buffer', expect.objectContaining({
      operation: 'images_to_pdf',
      params: expect.objectContaining({
        ocr: true,
        images: ['dummy123', 'dummy123']
      })
    }));

    // Document should be added to app state
    expect(appState.documents.length).toBeGreaterThanOrEqual(1);
  });
});
