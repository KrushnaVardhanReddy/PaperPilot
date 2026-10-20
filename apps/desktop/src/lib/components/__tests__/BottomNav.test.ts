import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import BottomNav from '../layout/BottomNav.svelte';
import { appState } from '$lib/state/app.svelte';

describe('BottomNav', () => {
  it('renders documents, scan, tools and settings nav items on mobile', () => {
    appState.isMobileViewport = true;
    appState.isTauriMobile = true;
    render(BottomNav);
    expect(screen.getByLabelText('Documents')).toBeTruthy();
    expect(screen.getByLabelText('Scan')).toBeTruthy();
    expect(screen.getByLabelText('Tools')).toBeTruthy();
    expect(screen.getByLabelText('Settings')).toBeTruthy();
  });
});
