/**
 * Application Constants & External URL Configuration
 * Update URLs here in one centralized location.
 */

// Primary GitHub Repository & Releases
export const GITHUB_REPO_URL = 'https://github.com/KrushnaVardhanReddy/PaperPilot';
export const GITHUB_RELEASES_URL = `${GITHUB_REPO_URL}/releases`;

// Website & Downloads
export const SITE_URL = 'https://usepaperpilot.com';
export const DOWNLOAD_URL = `${GITHUB_RELEASES_URL}`;
export const DOWNLOAD_LINUX_URL = `${GITHUB_RELEASES_URL}`;
export const DOWNLOAD_WINDOWS_URL = `${GITHUB_RELEASES_URL}`;
export const DOWNLOAD_MACOS_URL = `${GITHUB_RELEASES_URL}`;

// Documentation Portal
export const getDocsBaseUrl = (): string => {
  if (typeof window !== 'undefined' && (window.location.hostname === 'localhost' || window.location.hostname === '127.0.0.1')) {
    return 'http://localhost:4321';
  }
  return 'https://docs.usepaperpilot.com';
};

export const getToolsHandbookUrl = (): string => {
  return `${getDocsBaseUrl()}/reference/50-tools-handbook/`;
};

// Feature Flags
// Automatically displays the badge once total releases downloads reach 1,000+
export const ENABLE_DOWNLOAD_COUNT_BADGE = true;
export const MIN_DOWNLOAD_COUNT_THRESHOLD = 1000;
