import { render, screen } from '@testing-library/svelte';
import { describe, it, expect } from 'vitest';
import BottomNav from '../layout/BottomNav.svelte';

describe('BottomNav', () => {
  it('renders home, documents and settings nav items', () => {
    // Simulate mobile width
    Object.defineProperty(window, 'innerWidth', { writable: true, configurable: true, value: 375 });
    render(BottomNav);
    expect(screen.getByLabelText('Home')).toBeTruthy();
    expect(screen.getByLabelText('Documents')).toBeTruthy();
    expect(screen.getByLabelText('Settings')).toBeTruthy();
  });
});