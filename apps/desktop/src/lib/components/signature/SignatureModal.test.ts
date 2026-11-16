import { render, screen, fireEvent } from '@testing-library/svelte';
import { describe, it, expect, beforeEach, vi } from 'vitest';
import SignatureModal from './SignatureModal.svelte';

describe('SignatureModal Component', () => {
  beforeEach(() => {
    localStorage.clear();
    vi.restoreAllMocks();
  });

  it('renders modal with tabs: Type, Draw, Upload', () => {
    render(SignatureModal, { props: { isOpen: true } });

    expect(screen.getByText('Create & Adopt Signature')).toBeTruthy();
    expect(screen.getByRole('button', { name: /Type/i })).toBeTruthy();
    expect(screen.getByRole('button', { name: /Draw/i })).toBeTruthy();
    expect(screen.getByRole('button', { name: /Upload/i })).toBeTruthy();
    expect(screen.getByRole('button', { name: /Adopt & Sign/i })).toBeTruthy();
  });

  it('renders 4 script font styles and updates typed name preview in Type tab', async () => {
    render(SignatureModal, { props: { isOpen: true, initialTab: 'type' } });

    // Verify 4 script font options exist
    expect(screen.getByText('Caveat Style')).toBeTruthy();
    expect(screen.getByText('Dancing Script')).toBeTruthy();
    expect(screen.getByText('Sacramento Calligraphy')).toBeTruthy();
    expect(screen.getByText('Cedarville Cursive')).toBeTruthy();

    const nameInput = screen.getByLabelText('Full Name');
    expect(nameInput).toBeTruthy();

    // Type a custom name
    await fireEvent.input(nameInput, { target: { value: 'Alex Morgan' } });

    // The live previews should render the typed name
    const matches = screen.getAllByText('Alex Morgan');
    expect(matches.length).toBeGreaterThanOrEqual(4);
  });

  it('switches between Full Signature and Initials modes', async () => {
    render(SignatureModal, { props: { isOpen: true, initialTab: 'type' } });

    const initialsToggle = screen.getByRole('button', { name: /Initials/i });
    await fireEvent.click(initialsToggle);

    // Should switch to initials input
    const initialsInput = screen.getByLabelText('Initials');
    expect(initialsInput).toBeTruthy();

    await fireEvent.input(initialsInput, { target: { value: 'AM' } });
    const matches = screen.getAllByText('AM');
    expect(matches.length).toBeGreaterThanOrEqual(4);
  });

  it('allows selecting ink colors', async () => {
    render(SignatureModal, { props: { isOpen: true, initialTab: 'type' } });

    const blueBtn = screen.getByRole('button', { name: 'Blue' });
    expect(blueBtn).toBeTruthy();
    await fireEvent.click(blueBtn);

    // Selected swatch should show checkmark
    expect(blueBtn.classList.contains('selected')).toBe(true);
  });

  it('switches to Draw tab and clears canvas', async () => {
    render(SignatureModal, { props: { isOpen: true, initialTab: 'draw' } });

    expect(screen.getByLabelText('Signature drawing pad')).toBeTruthy();
    const clearBtn = screen.getByRole('button', { name: /Clear/i });
    expect(clearBtn).toBeTruthy();

    // Clear button is disabled initially when canvas has no strokes
    expect((clearBtn as HTMLButtonElement).disabled).toBe(true);

    const adoptBtn = screen.getByRole('button', { name: /Adopt & Sign/i });
    expect((adoptBtn as HTMLButtonElement).disabled).toBe(true);
  });

  it('switches to Upload tab and shows drag-and-drop file picker', async () => {
    render(SignatureModal, { props: { isOpen: true, initialTab: 'upload' } });

    expect(screen.getByText('Drop your signature image here')).toBeTruthy();
    expect(screen.getByText(/Supports PNG, JPG, or WebP/i)).toBeTruthy();
  });

  it('adopts typed signature, caches to localStorage, and calls onAdopt callback', async () => {
    const onAdoptMock = vi.fn();
    const onCloseMock = vi.fn();

    render(SignatureModal, {
      props: {
        isOpen: true,
        initialTab: 'type',
        onAdopt: onAdoptMock,
        onClose: onCloseMock
      }
    });

    const adoptBtn = screen.getByRole('button', { name: /Adopt & Sign/i });
    await fireEvent.click(adoptBtn);

    expect(onAdoptMock).toHaveBeenCalledTimes(1);
    const adopted = onAdoptMock.mock.calls[0][0];
    expect(adopted.id).toMatch(/^sig_/);
    expect(adopted.dataUrl).toContain('data:image/png');
    expect(onCloseMock).toHaveBeenCalledTimes(1);

    // Verify persisted in localStorage
    const cached = JSON.parse(localStorage.getItem('paperpilot:saved_signatures') || '[]');
    expect(cached.length).toBe(1);
    expect(cached[0].id).toBe(adopted.id);
  });

  it('calls onClose when Cancel button or Close X is clicked', async () => {
    const onCloseMock = vi.fn();

    render(SignatureModal, {
      props: {
        isOpen: true,
        onClose: onCloseMock
      }
    });

    const cancelBtn = screen.getByRole('button', { name: 'Cancel' });
    await fireEvent.click(cancelBtn);
    expect(onCloseMock).toHaveBeenCalledTimes(1);

    const closeXBtn = screen.getByLabelText('Close signature modal');
    await fireEvent.click(closeXBtn);
    expect(onCloseMock).toHaveBeenCalledTimes(2);
  });
});
