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
    featurePacks: [],
    workspaceLayout: 'studio',
    setWorkspaceLayout: vi.fn(function(this: any, mode: string) {
      this.workspaceLayout = mode;
    }),
    toggleDeveloperMode: vi.fn(async function(this: any, enabled?: boolean) {
      this.developerMode = enabled !== undefined ? enabled : !this.developerMode;
    }),
  }
}));

vi.mock('$lib/utils/tauri', () => ({
  safeInvoke: vi.fn().mockResolvedValue(undefined),
}));

vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn().mockResolvedValue([]),
}));

vi.mock('@tauri-apps/api/event', () => ({
  listen: vi.fn().mockResolvedValue(() => {}),
}));

vi.mock('$lib/state/aiConfig.svelte', () => ({
  aiConfig: {
    mode: 'offlinenlp',
    byokProvider: null,
    testResult: null,
    models: []
  }
}));

describe('SettingsPanel', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    appState.developerMode = false;
  });

  it('changes workspace layout when dropdown is used', async () => {
    render(SettingsPanel);

    // Switch to General & Storage tab explicitly just in case
    const genTab = screen.getByRole('button', { name: /General & Storage/i });
    await fireEvent.click(genTab);

    const layoutSelect = screen.getByLabelText(/Default Workspace Layout/i) as HTMLSelectElement;
    expect(layoutSelect).not.toBeNull();

    // Change to 'hero'
    await fireEvent.change(layoutSelect, { target: { value: 'hero' } });

    expect(appState.setWorkspaceLayout).toHaveBeenCalledWith('hero');
  });


  it('renders developer mode toggle and toggles state', async () => {
    render(SettingsPanel);

    // Switch to Developer tab
    const devTab = screen.getByRole('button', { name: /Developer & Gateway/i });
    await fireEvent.click(devTab);

    const toggle = screen.getByLabelText(/Enable Local REST API & Swagger/i) as HTMLInputElement;
    expect(toggle).not.toBeNull();
    expect(toggle.checked).toBe(false);

    await fireEvent.change(toggle, { target: { checked: true } });

    expect(appState.toggleDeveloperMode).toHaveBeenCalledWith(true);
  });

  it('displays API endpoint status when developer mode is active', async () => {
    appState.developerMode = true;
    render(SettingsPanel);

    // Switch to Developer tab
    const devTab = screen.getByRole('button', { name: /Developer & Gateway/i });
    await fireEvent.click(devTab);

    const status = screen.getByText(/● Running/i);
    expect(status).not.toBeNull();

    const link = screen.getByRole('link', { name: /Open Swagger UI/i });
    expect(link).not.toBeNull();
    expect(link.getAttribute('href')).toContain('7823');
  });
});
