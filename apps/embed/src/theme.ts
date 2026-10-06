export interface ThemeConfig {
  brandColor?: string;
  theme?: 'light' | 'dark' | 'system';
  logoUrl?: string;
}

export function getThemeStyles(config: ThemeConfig): string {
  const brandColor = config.brandColor || '#3B82F6';
  const isDark = config.theme === 'dark' || (config.theme === 'system' && window.matchMedia('(prefers-color-scheme: dark)').matches);

  const bg = isDark ? '#0f0f13' : '#ffffff';
  const surface = isDark ? '#1a1a2e' : '#f3f4f6';
  const text = isDark ? '#e2e8f0' : '#111827';
  const textMuted = isDark ? '#94a3b8' : '#6b7280';
  const border = isDark ? '#334155' : '#e5e7eb';

  return `
    :host {
      --pp-brand-color: ${brandColor};
      --pp-background: ${bg};
      --pp-surface: ${surface};
      --pp-text: ${text};
      --pp-text-muted: ${textMuted};
      --pp-border: ${border};
      --pp-radius: 12px;
      --pp-font: 'Inter', system-ui, sans-serif;
    }
  `;
}
