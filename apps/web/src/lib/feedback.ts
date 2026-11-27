/**
 * GitHub Feedback and Issue Reporting URL Generator & Helpers
 */

export const GITHUB_REPO = 'KrushnaVardhanReddy/PaperPilot';
export const GITHUB_BASE_URL = `https://github.com/${GITHUB_REPO}`;
export const GITHUB_ISSUES_NEW = `${GITHUB_BASE_URL}/issues/new`;
export const GITHUB_DISCUSSIONS_URL = `${GITHUB_BASE_URL}/discussions`;

export type FeedbackCategory = 'bug' | 'performance' | 'feature' | 'praise';

export interface FeedbackData {
  category: FeedbackCategory;
  subcategory?: string;
  title: string;
  description: string;
  affectedTool?: string;
  reproductionSteps?: string;
  expectedBehavior?: string;
  errorTrace?: string;
  // Performance-specific
  fileSize?: string;
  processingTime?: string;
  memoryUsage?: string;
  // Feature-specific
  proposedSolution?: string;
  consideredAlternatives?: string;
  // Environment (auto-collected or user-provided)
  appVersion?: string;
  clientType?: string;
  osEnvironment?: string;
}

export function detectEnvironment(clientTypeOverride?: string): {
  appVersion: string;
  clientType: string;
  osEnvironment: string;
} {
  const appVersion = '0.8.2';
  
  let clientType = clientTypeOverride || 'Web (WASM)';
  if (typeof window !== 'undefined' && (window as any).__TAURI_INTERNALS__) {
    clientType = 'Desktop (Tauri)';
  }

  let osEnvironment = 'Unknown OS';
  if (typeof navigator !== 'undefined') {
    const ua = navigator.userAgent;
    const platform = (navigator as any).userAgentData?.platform || navigator.platform || '';
    if (/win/i.test(platform) || /windows/i.test(ua)) {
      osEnvironment = 'Windows';
    } else if (/mac/i.test(platform) || /macintosh/i.test(ua)) {
      osEnvironment = 'macOS';
    } else if (/android/i.test(ua)) {
      osEnvironment = 'Android';
    } else if (/iphone|ipad|ipod/i.test(ua)) {
      osEnvironment = 'iOS';
    } else if (/linux/i.test(platform) || /linux/i.test(ua)) {
      osEnvironment = 'Linux';
    } else {
      osEnvironment = platform || 'Desktop / Browser';
    }
  }

  return { appVersion, clientType, osEnvironment };
}

export function buildGitHubIssueUrl(data: FeedbackData): string {
  const env = detectEnvironment(data.clientType);
  const version = data.appVersion || env.appVersion;
  const surface = data.clientType || env.clientType;
  const osInfo = data.osEnvironment || env.osEnvironment;

  // Handle praise / testimonial / discussions
  if (data.category === 'praise') {
    const praiseTitle = encodeURIComponent(data.title.trim() ? `[Show & Tell]: ${data.title.trim()}` : '[Show & Tell]: Loving PaperPilot!');
    const praiseBody = encodeURIComponent(
      `### What do you love about PaperPilot?\n${data.description || 'PaperPilot has been an awesome local-first PDF tool!'}\n\n` +
      `**Platform:** ${surface} (${osInfo})\n` +
      `**Version:** ${version}\n`
    );
    return `${GITHUB_DISCUSSIONS_URL}/new?category=show-and-tell&title=${praiseTitle}&body=${praiseBody}`;
  }

  if (data.category === 'performance') {
    const params = new URLSearchParams();
    params.set('template', 'performance_issue.yml');
    params.set('title', `[PERF]: ${data.title.trim()}`);
    params.set('description', data.description || '');
    params.set('surface', surface.includes('Desktop') ? 'Desktop App (Tauri)' : 'Browser WASM (usepaperpilot.com / portable HTML)');
    if (data.affectedTool) params.set('tool', data.affectedTool);
    if (data.fileSize) params.set('file_size', data.fileSize);
    if (data.processingTime) params.set('processing_time', data.processingTime);
    if (data.memoryUsage) params.set('memory_usage', data.memoryUsage);
    if (data.reproductionSteps) params.set('reproduction', data.reproductionSteps);
    if (data.errorTrace) params.set('logs', data.errorTrace);
    params.set('version', version);
    params.set('environment', `${osInfo} (${surface})`);
    return `${GITHUB_ISSUES_NEW}?${params.toString()}`;
  }

  if (data.category === 'feature') {
    const params = new URLSearchParams();
    params.set('template', 'feature_request.yml');
    params.set('title', `[FEATURE]: ${data.title.trim()}`);
    params.set('subcategory', data.subcategory || '💡 New Tool Request');
    if (data.affectedTool) params.set('tool', data.affectedTool);
    params.set('problem', data.description || '');
    params.set('proposed_solution', data.proposedSolution || data.description || '');
    params.set('target_surfaces', surface.includes('Desktop') ? 'Desktop App (Tauri)' : 'Browser WASM (Zero-install web)');
    if (data.consideredAlternatives) params.set('alternatives', data.consideredAlternatives);
    return `${GITHUB_ISSUES_NEW}?${params.toString()}`;
  }

  // Default: Bug Report
  const params = new URLSearchParams();
  params.set('template', 'bug_report.yml');
  params.set('title', `[BUG]: ${data.title.trim()}`);
  params.set('subcategory', data.subcategory || '🐞 Functional Bug / Crash');
  params.set('description', data.description || '');
  params.set('surface', surface.includes('Desktop') ? 'Desktop App (Tauri)' : 'Browser WASM (usepaperpilot.com / portable HTML)');
  if (data.affectedTool) params.set('tool', data.affectedTool);
  params.set('reproduction', data.reproductionSteps || '1. Open tool\n2. Perform operation\n3. Observe error');
  params.set('expected', data.expectedBehavior || 'Operation completes successfully with clean zero-panic output.');
  if (data.errorTrace) params.set('logs', data.errorTrace);
  params.set('version', version);
  params.set('environment', `${osInfo} (${surface})`);

  return `${GITHUB_ISSUES_NEW}?${params.toString()}`;
}

export async function openExternalUrl(url: string): Promise<void> {
  if (typeof window === 'undefined') return;

  // In web browser, standard window.open
  window.open(url, '_blank', 'noopener,noreferrer');
}
