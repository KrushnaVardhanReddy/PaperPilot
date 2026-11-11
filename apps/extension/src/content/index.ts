/**
 * PaperPilot Browser Companion - Content Script
 * Injected into web pages to capture clean articles, strip advertisements,
 * and communicate with service worker and popup.
 */

import { extractArticleMarkdown } from './articleReader';

// Check if current page is viewing a raw PDF
function isPdfViewerPage(): boolean {
  if (document.contentType === 'application/pdf') return true;
  if (window.location.pathname.toLowerCase().endsWith('.pdf')) return true;
  if (document.querySelector('embed[type="application/pdf"]')) return true;
  return false;
}

// Display subtle feedback toast when user captures content
function showCaptureToast(message: string) {
  const existing = document.getElementById('paperpilot-capture-toast');
  if (existing) existing.remove();

  const toast = document.createElement('div');
  toast.id = 'paperpilot-capture-toast';
  toast.textContent = message;
  toast.style.position = 'fixed';
  toast.style.top = '16px';
  toast.style.right = '16px';
  toast.style.zIndex = '999999';
  toast.style.padding = '8px 16px';
  toast.style.borderRadius = '8px';
  toast.style.background = '#0f172a';
  toast.style.color = '#38bdf8';
  toast.style.fontFamily = '-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif';
  toast.style.fontSize = '13px';
  toast.style.fontWeight = '500';
  toast.style.boxShadow = '0 4px 12px rgba(0, 0, 0, 0.35)';
  toast.style.border = '1px solid #1e293b';
  toast.style.transition = 'opacity 0.3s ease, transform 0.3s ease';
  toast.style.opacity = '0';
  toast.style.transform = 'translateY(-8px)';

  document.body.appendChild(toast);

  requestAnimationFrame(() => {
    toast.style.opacity = '1';
    toast.style.transform = 'translateY(0)';
  });

  setTimeout(() => {
    toast.style.opacity = '0';
    toast.style.transform = 'translateY(-8px)';
    setTimeout(() => toast.remove(), 300);
  }, 2200);
}

// Runtime message listener
chrome.runtime.onMessage.addListener((request, _sender, sendResponse) => {
  if (request.action === 'EXTRACT_ARTICLE' || request.type === 'CAPTURE_ARTICLE') {
    try {
      const result = extractArticleMarkdown(document);
      showCaptureToast('⚡ PaperPilot: Clean Article Extracted');
      sendResponse({ success: true, data: result });
    } catch (err: any) {
      sendResponse({ success: false, error: err.message || String(err) });
    }
    return true;
  }

  if (request.action === 'CHECK_PAGE_TYPE') {
    sendResponse({
      isPdf: isPdfViewerPage(),
      url: window.location.href,
      title: document.title,
    });
    return true;
  }
});
