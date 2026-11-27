import { describe, it, expect } from 'vitest';
import {
  buildGitHubIssueUrl,
  detectEnvironment,
  type FeedbackData
} from '$lib/utils/feedback';

describe('GitHub Feedback & Issue Reporting URL Builder', () => {
  it('detects environment correctly', () => {
    const env = detectEnvironment('Desktop (Tauri)');
    expect(env.appVersion).toBe('0.8.2');
    expect(env.clientType).toBe('Desktop (Tauri)');
    expect(typeof env.osEnvironment).toBe('string');
  });

  it('builds valid bug report URL with query parameters', () => {
    const data: FeedbackData = {
      category: 'bug',
      subcategory: '🐞 Functional Bug / Crash',
      title: 'Split operation failed on encrypted PDF',
      description: 'Document gave an unexpected error when processing.',
      affectedTool: 'pdf_split',
      reproductionSteps: '1. Load encrypted.pdf\n2. Click Split\n3. Threw error',
      expectedBehavior: 'Clean error prompt or password request',
      errorTrace: 'Error: Lopdf corrupted xref table',
      appVersion: '0.8.2',
      clientType: 'Desktop (Tauri)',
      osEnvironment: 'Linux (x86_64)'
    };

    const url = buildGitHubIssueUrl(data);
    expect(url).toContain('https://github.com/KrushnaVardhanReddy/PaperPilot/issues/new?');
    
    const parsed = new URL(url);
    expect(parsed.searchParams.get('template')).toBe('bug_report.yml');
    expect(parsed.searchParams.get('title')).toBe('[BUG]: Split operation failed on encrypted PDF');
    expect(parsed.searchParams.get('subcategory')).toBe('🐞 Functional Bug / Crash');
    expect(parsed.searchParams.get('tool')).toBe('pdf_split');
    expect(parsed.searchParams.get('surface')).toBe('Desktop App (Tauri)');
    expect(parsed.searchParams.get('version')).toBe('0.8.2');
    expect(parsed.searchParams.get('logs')).toBe('Error: Lopdf corrupted xref table');
  });

  it('builds valid performance issue URL with benchmark parameters', () => {
    const data: FeedbackData = {
      category: 'performance',
      title: 'High memory usage on 2000 page document',
      description: 'Processing took 4 minutes and spiked RAM to 3.2 GB.',
      affectedTool: 'pdf_ocr',
      fileSize: '350 MB, 2000 pages',
      processingTime: '240s',
      memoryUsage: '3.2 GB',
      reproductionSteps: 'Run OCR on large scanned book',
      appVersion: '0.8.2',
      clientType: 'Desktop (Tauri)',
      osEnvironment: 'macOS Sonoma'
    };

    const url = buildGitHubIssueUrl(data);
    const parsed = new URL(url);
    expect(parsed.searchParams.get('template')).toBe('performance_issue.yml');
    expect(parsed.searchParams.get('title')).toBe('[PERF]: High memory usage on 2000 page document');
    expect(parsed.searchParams.get('tool')).toBe('pdf_ocr');
    expect(parsed.searchParams.get('file_size')).toBe('350 MB, 2000 pages');
    expect(parsed.searchParams.get('processing_time')).toBe('240s');
    expect(parsed.searchParams.get('memory_usage')).toBe('3.2 GB');
    expect(parsed.searchParams.get('version')).toBe('0.8.2');
  });

  it('builds valid feature request URL', () => {
    const data: FeedbackData = {
      category: 'feature',
      subcategory: '💡 New Tool Request',
      title: 'Bates Numbering for Legal Filings',
      description: 'Need sequential prefix/suffix bates stamps across courtroom discovery bundles.',
      proposedSolution: 'Add pdf_bates tool with configurable digits and placement.',
      consideredAlternatives: 'Adobe Acrobat Pro',
      appVersion: '0.8.2',
      clientType: 'Web (WASM)',
      osEnvironment: 'Windows 11'
    };

    const url = buildGitHubIssueUrl(data);
    const parsed = new URL(url);
    expect(parsed.searchParams.get('template')).toBe('feature_request.yml');
    expect(parsed.searchParams.get('title')).toBe('[FEATURE]: Bates Numbering for Legal Filings');
    expect(parsed.searchParams.get('subcategory')).toBe('💡 New Tool Request');
    expect(parsed.searchParams.get('proposed_solution')).toBe('Add pdf_bates tool with configurable digits and placement.');
    expect(parsed.searchParams.get('alternatives')).toBe('Adobe Acrobat Pro');
  });

  it('builds valid GitHub Discussions URL for praise/testimonials', () => {
    const data: FeedbackData = {
      category: 'praise',
      title: 'Replaced Adobe Acrobat for our entire company',
      description: 'Saved our team hours of work with offline air-gapped performance!',
      appVersion: '0.8.2',
      clientType: 'Desktop (Tauri)',
      osEnvironment: 'Linux'
    };

    const url = buildGitHubIssueUrl(data);
    expect(url).toContain('https://github.com/KrushnaVardhanReddy/PaperPilot/discussions/new?');
    expect(url).toContain('category=show-and-tell');
    expect(url).toContain('Replaced%20Adobe%20Acrobat');
  });
});
