import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, fireEvent, screen } from '@testing-library/svelte';
import SettingsPanel from './SettingsPanel.svelte';
import { appState } from '$lib/state/app.svelte';
import * as tauriUtils from '$lib/utils/tauri';

vi.mock('$lib/state/app.svelte', () => ({
  appState: {
    developerMode: false,
    gatewayRunning: false,
    gatewayPort: 7823,
    toggleDeveloperMode: vi.fn(async function(this: any, enabled?: boolean) {
      this.developerMode = enabled !== undefined ? enabled : !this.developerMode;
    }),
  }
}));

vi.mock('$lib/utils/tauri', () => ({
  safeInvoke: vi.fn().mockResolvedValue(undefined),
}));

describe('SettingsPanel', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    appState.developerMode = false;
  });

  it('renders developer mode toggle and toggles state', async () => {
    render(SettingsPanel);

    const toggle = screen.getByLabelText(/Enable Local REST API & Swagger/i) as HTMLInputElement;
    expect(toggle).not.toBeNull();
    expect(toggle.checked).toBe(false);

    await fireEvent.change(toggle, { target: { checked: true } });

    expect(appState.toggleDeveloperMode).toHaveBeenCalledWith(true);
  });

  it('displays API endpoint status when developer mode is active', async () => {
    appState.developerMode = true;
    render(SettingsPanel);

    const status = screen.getByText(/● Running/i);
    expect(status).not.toBeNull();

    const link = screen.getByRole('link', { name: /Open Swagger UI/i });
    expect(link).not.toBeNull();
    expect(link.getAttribute('href')).toContain('7823');
  });
});
