import { afterEach, expect, it } from 'vitest';
import { render, screen, cleanup } from '@testing-library/svelte';
import WatchedFoldersPanel from './WatchedFoldersPanel.svelte';
afterEach(cleanup);
it('explains browser unavailability and disables real filesystem actions', () => {
  render(WatchedFoldersPanel);
  expect(screen.getByText('Folder watching requires the desktop app on a computer.')).toBeInTheDocument();
  expect(screen.getByLabelText('Input folder')).toBeDisabled();
  expect(screen.getByRole('button', { name: 'Add folder' })).toBeDisabled();
});
