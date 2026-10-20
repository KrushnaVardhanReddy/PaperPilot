import { mount, unmount, on, runPipeline } from './api';

const PaperPilotAPI = {
  mount,
  unmount,
  on,
  runPipeline,
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
    const pipelineAttr = el.getAttribute('data-pipeline');

    let pipeline = undefined;
    if (pipelineAttr) {
      try {
        pipeline = JSON.parse(pipelineAttr);
      } catch (e) {
        pipeline = pipelineAttr; // Fallback to raw string
      }
    }

    mount(el, { tools, theme, brandColor, logoUrl, hideBadge, pipeline });
  });
}

if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', autoInit);
} else {
  autoInit();
}

export default PaperPilotAPI;
