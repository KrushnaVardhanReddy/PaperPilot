import { mount, unmount, on, type EmbedOptions } from './api';

const PaperPilotAPI = {
  mount,
  unmount,
  on,
};

// Expose to window
// @ts-ignore
window.PaperPilot = PaperPilotAPI;

// Auto-scan on load
function autoInit() {
  const portals = document.querySelectorAll('[data-paperpilot-portal], #paperpilot-portal, #nobadge-portal');

  portals.forEach(portal => {
    const el = portal as HTMLElement;
    const tools = el.getAttribute('data-tools') || 'all';
    const theme = (el.getAttribute('data-theme') || 'system') as 'light' | 'dark' | 'system';
    const brandColor = el.getAttribute('data-brand-color') || undefined;
    const logoUrl = el.getAttribute('data-logo-url') || undefined;
    const hideBadge = el.getAttribute('data-hide-badge') === 'true';

    mount(el, { tools, theme, brandColor, logoUrl, hideBadge });
  });
}

if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', autoInit);
} else {
  autoInit();
}

export default PaperPilotAPI;
