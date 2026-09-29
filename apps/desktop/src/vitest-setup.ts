import '@testing-library/jest-dom';
import { vi } from 'vitest';

// Mock Web Animations API since jsdom doesn't support it
if (typeof window !== 'undefined') {
  window.HTMLElement.prototype.animate = vi.fn().mockImplementation(() => ({
    onfinish: vi.fn(),
    cancel: vi.fn(),
    play: vi.fn(),
    pause: vi.fn(),
    reverse: vi.fn(),
    finish: vi.fn()
  }));
}
