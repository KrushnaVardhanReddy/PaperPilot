import { render, screen } from '@testing-library/svelte';
import { describe, it, expect } from 'vitest';
import ProgressBar from '../ui/ProgressBar.svelte';

describe('ProgressBar', () => {
  it('renders with job name', () => {
    render(ProgressBar, { props: { jobName: 'Merging PDFs', percent: 42 } });
    expect(screen.getByText('Merging PDFs')).toBeTruthy();
    expect(screen.getByText('42%')).toBeTruthy();
  });

  it('clamps percent to 0-100', () => {
    render(ProgressBar, { props: { jobName: 'Test', percent: 150 } });
    expect(screen.getByText('100%')).toBeTruthy();
  });

  it('is hidden when visible=false', () => {
    const { container } = render(ProgressBar, {
      props: { jobName: 'Hidden', percent: 50, visible: false }
    });
    expect(container.querySelector('.progress-wrapper')).toBeNull();
  });

  it('shows page info when provided', () => {
    render(ProgressBar, { props: { jobName: 'Test', percent: 50, currentPage: 3, totalPages: 10 } });
    expect(screen.getByText('Page 3 of 10')).toBeTruthy();
  });
});
