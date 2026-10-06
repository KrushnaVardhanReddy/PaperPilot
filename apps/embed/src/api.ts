import Widget from './Widget.svelte';
import { getThemeStyles, type ThemeConfig } from './theme';
import { mount as mountSvelte, unmount as unmountSvelte } from 'svelte';

export interface EmbedOptions extends ThemeConfig {
  tools?: string | string[];
  hideBadge?: boolean;
}

export interface EmbedInstance {
  destroy: () => void;
  element: HTMLElement;
}

const instances = new Map<string | HTMLElement, EmbedInstance>();

export function mount(selector: string | HTMLElement, options: EmbedOptions = {}): EmbedInstance {
  let container: HTMLElement | null = null;

  if (typeof selector === 'string') {
    container = document.querySelector(selector);
  } else {
    container = selector;
  }

  if (!container) {
    throw new Error(`PaperPilot: Target container not found for selector ${selector}`);
  }

  // Clear existing content and unmount previous instance if any
  if (instances.has(selector)) {
    unmount(selector);
  }
  container.innerHTML = '';

  const shadow = container.attachShadow({ mode: 'open' });

  // Inject theme styles
  const styleEl = document.createElement('style');
  styleEl.textContent = getThemeStyles({
    brandColor: options.brandColor,
    theme: options.theme,
    logoUrl: options.logoUrl,
  });
  shadow.appendChild(styleEl);

  const toolsString = Array.isArray(options.tools) ? options.tools.join(',') : (options.tools || 'all');

  // Mount Svelte component inside shadow root
  const component = mountSvelte(Widget, {
    target: shadow,
    props: {
      tools: toolsString,
      hideBadge: options.hideBadge === true,
      brandColor: options.brandColor,
      theme: options.theme,
      logoUrl: options.logoUrl,
    },
  });

  const instance: EmbedInstance = {
    destroy: () => {
      unmountSvelte(component);
      shadow.innerHTML = ''; // Clean up
      instances.delete(selector);
    },
    element: container,
  };

  instances.set(selector, instance);
  return instance;
}

export function unmount(selector: string | HTMLElement): void {
  const instance = instances.get(selector);
  if (instance) {
    instance.destroy();
  }
}

export function on(event: string, callback: (data: any) => void): void {
  window.addEventListener(`paperpilot:${event}`, (e: Event) => {
    callback((e as CustomEvent).detail);
  });
}
