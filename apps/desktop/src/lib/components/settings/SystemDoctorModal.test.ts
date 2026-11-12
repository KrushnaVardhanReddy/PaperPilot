import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, fireEvent, screen } from '@testing-library/svelte';
import SystemDoctorModal from './SystemDoctorModal.svelte';
import * as tauriUtils from '$lib/utils/tauri';

vi.mock('$lib/utils/tauri', () => ({
  safeInvoke: vi.fn(),
  isNativeDesktop: vi.fn().mockReturnValue(true),
}));

describe('SystemDoctorModal', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders modal with probe items, badges, and action buttons when open', async () => {
    const mockReport = {
      timestamp: '1794345600',
      status: 'Degraded',
      checks: [
        {
          name: 'Temp Directory: /tmp (writable & ephemeral cleanup verified)',
          passed: true,
          message: 'Temp directory is writable and files can be cleanly deleted.',
          suggestion: null,
        },
        {
          name: 'Gateway Port: 7823 available',
          passed: false,
          message: 'Port 7823 is occupied.',
          suggestion: 'Check conflicting processes.',
        },
        {
          name: 'OCR Models: models/ocr/ppocrv4 (det.onnx, rec.onnx found)',
          passed: false,
          message: 'OCR models missing.',
          suggestion: 'Download OCR models.',
        },
      ],
    };

    vi.mocked(tauriUtils.safeInvoke).mockResolvedValue(mockReport);

    const onClose = vi.fn();
    render(SystemDoctorModal, {
      isOpen: true,
      onClose,
    });

    // Check title
    expect(screen.getByText(/PaperPilot System Doctor/i)).not.toBeNull();

    // Check probe names after async loading resolves
    const tempMatches = await screen.findAllByText(/Temp Directory/i);
    expect(tempMatches.length).toBeGreaterThan(0);
    expect(await screen.findByText(/Gateway Port: 7823 available/i)).not.toBeNull();
    expect(await screen.findByText(/OCR Models: models\/ocr\/ppocrv4/i)).not.toBeNull();

    // Check Badges: Pass, Fail, Warning
    expect(screen.getByText('Pass')).not.toBeNull();
    expect(screen.getByText('Fail')).not.toBeNull();
    expect(screen.getByText('Warning')).not.toBeNull();

    // Check Copy and Export buttons
    const copyBtn = screen.getByRole('button', { name: /Copy JSON/i });
    expect(copyBtn).not.toBeNull();

    const exportBtn = screen.getByRole('button', { name: /Export Report/i });
    expect(exportBtn).not.toBeNull();

    // Check close action
    const closeBtn = screen.getByRole('button', { name: 'Close' });
    await fireEvent.click(closeBtn);
    expect(onClose).toHaveBeenCalled();
  });

  it('copies report JSON to clipboard when Copy JSON is clicked', async () => {
    const mockReport = {
      timestamp: '1794345600',
      status: 'Healthy',
      checks: [
        {
          name: 'Temp Directory: /tmp (writable & ephemeral cleanup verified)',
          passed: true,
          message: 'OK',
          suggestion: null,
        },
      ],
    };

    vi.mocked(tauriUtils.safeInvoke).mockResolvedValue(mockReport);

    const writeTextMock = vi.fn().mockResolvedValue(undefined);
    Object.assign(navigator, {
      clipboard: {
        writeText: writeTextMock,
      },
    });

    render(SystemDoctorModal, {
      isOpen: true,
      onClose: vi.fn(),
    });

    const tempMatches = await screen.findAllByText(/Temp Directory/i);
    expect(tempMatches.length).toBeGreaterThan(0);

    const copyBtn = screen.getByRole('button', { name: /Copy JSON/i });
    await fireEvent.click(copyBtn);

    expect(writeTextMock).toHaveBeenCalled();
    const written = writeTextMock.mock.calls[0][0];
    expect(JSON.parse(written).status).toBe('Healthy');
  });
});
