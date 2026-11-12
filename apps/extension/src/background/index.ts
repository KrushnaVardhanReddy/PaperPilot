/**
 * PaperPilot Browser Companion - Service Worker (Background Script)
 * Manages context menus, tab communication, storage cache, and Gateway bridge.
 */

import {
  checkGatewayHealth,
  executeGatewayTool,
  chunkDocumentViaGateway,
  DEFAULT_GATEWAY_URL,
} from './gatewayClient';

export const MENU_EXTRACT_TEXT = 'paperpilot-extract-text';
export const MENU_SEMANTIC_CHUNK = 'paperpilot-semantic-chunk';
export const MENU_CLEAN_ARTICLE = 'paperpilot-clean-article';
export const MENU_OPEN_DESKTOP = 'paperpilot-open-desktop';

// Initialize context menus on install / update
chrome.runtime.onInstalled.addListener(() => {
  chrome.contextMenus.removeAll(() => {
    chrome.contextMenus.create({
      id: MENU_EXTRACT_TEXT,
      title: 'PaperPilot: Extract Clean Text',
      contexts: ['page', 'selection', 'link'],
    });

    chrome.contextMenus.create({
      id: MENU_SEMANTIC_CHUNK,
      title: 'PaperPilot: Semantic Chunk for AI',
      contexts: ['page', 'selection', 'link'],
    });

    chrome.contextMenus.create({
      id: MENU_CLEAN_ARTICLE,
      title: 'PaperPilot: Clean Article to Markdown',
      contexts: ['page'],
    });

    chrome.contextMenus.create({
      id: MENU_OPEN_DESKTOP,
      title: 'PaperPilot: Open in Desktop App',
      contexts: ['page', 'link'],
    });
  });

  // Initial gateway health check
  updateGatewayStatus();
});

// Update cached connection state
async function updateGatewayStatus() {
  const health = await checkGatewayHealth();
  await chrome.storage.local.set({
    gatewayConnected: health.ok,
    gatewayInfo: health,
    lastChecked: Date.now(),
  });

  if (health.ok) {
    chrome.action.setBadgeBackgroundColor({ color: '#10b981' }); // emerald green
    chrome.action.setBadgeText({ text: 'ON' });
  } else {
    chrome.action.setBadgeBackgroundColor({ color: '#3b82f6' }); // wasm blue
    chrome.action.setBadgeText({ text: 'WASM' });
  }
}

// Handle Context Menu Actions
chrome.contextMenus.onClicked.addListener(async (info, tab) => {
  const tabId = tab?.id;

  switch (info.menuItemId) {
    case MENU_CLEAN_ARTICLE: {
      if (!tabId) return;
      try {
        const response = await chrome.tabs.sendMessage(tabId, { action: 'EXTRACT_ARTICLE' });
        if (response && response.success) {
          await chrome.storage.local.set({
            lastExtractedArticle: response.data,
            lastActionTimestamp: Date.now(),
          });
          flashBadge('MD');
        }
      } catch (err) {
        console.warn('Could not contact content script on tab:', err);
      }
      break;
    }

    case MENU_EXTRACT_TEXT: {
      if (info.selectionText) {
        await chrome.storage.local.set({
          lastExtractedText: {
            text: info.selectionText,
            source: tab?.url || 'selection',
            timestamp: Date.now(),
          },
        });
        flashBadge('TXT');
      } else if (tabId) {
        try {
          const response = await chrome.tabs.sendMessage(tabId, { action: 'EXTRACT_ARTICLE' });
          if (response && response.success) {
            await chrome.storage.local.set({
              lastExtractedText: {
                text: response.data.markdown,
                title: response.data.title,
                source: tab?.url || 'page',
                timestamp: Date.now(),
              },
            });
            flashBadge('TXT');
          }
        } catch (err) {
          console.warn('Failed to extract text from page:', err);
        }
      }
      break;
    }

    case MENU_SEMANTIC_CHUNK: {
      const textToChunk = info.selectionText;
      if (textToChunk) {
        const health = await checkGatewayHealth();
        if (health.ok) {
          const chunkRes = await chunkDocumentViaGateway({ text: textToChunk });
          await chrome.storage.local.set({
            lastSemanticChunks: {
              chunks: chunkRes.chunks,
              count: chunkRes.count,
              source: tab?.url || 'selection',
              timestamp: Date.now(),
            },
          });
          flashBadge('AI');
        } else {
          // Client-side fallback splitting by paragraphs
          const paragraphs = textToChunk.split(/\n\n+/).filter(Boolean);
          const chunks = paragraphs.map((p, i) => ({
            index: i,
            text: p,
            tokenCount: Math.ceil(p.length / 4),
          }));
          await chrome.storage.local.set({
            lastSemanticChunks: {
              chunks,
              count: chunks.length,
              source: tab?.url || 'selection (local fallback)',
              timestamp: Date.now(),
            },
          });
          flashBadge('AI');
        }
      } else if (tabId) {
        try {
          const response = await chrome.tabs.sendMessage(tabId, { action: 'EXTRACT_ARTICLE' });
          if (response && response.success) {
            const docText = response.data.markdown;
            const health = await checkGatewayHealth();
            let chunks: any[] = [];
            if (health.ok) {
              const chunkRes = await chunkDocumentViaGateway({ text: docText });
              chunks = chunkRes.chunks;
            } else {
              chunks = docText.split(/\n\n+/).filter(Boolean).map((p: string, i: number) => ({
                index: i,
                text: p,
                tokenCount: Math.ceil(p.length / 4),
              }));
            }
            await chrome.storage.local.set({
              lastSemanticChunks: {
                chunks,
                count: chunks.length,
                title: response.data.title,
                source: tab?.url || 'page',
                timestamp: Date.now(),
              },
            });
            flashBadge('AI');
          }
        } catch (err) {
          console.warn('Failed to capture page for chunking:', err);
        }
      }
      break;
    }

    case MENU_OPEN_DESKTOP: {
      const health = await checkGatewayHealth();
      if (health.ok) {
        chrome.tabs.create({ url: `${health.url || DEFAULT_GATEWAY_URL}/swagger-ui` });
      } else {
        chrome.tabs.create({ url: 'https://github.com/krushndayshmookh/paperpilot' });
      }
      break;
    }
  }
});

function flashBadge(text: string) {
  chrome.action.setBadgeText({ text });
  setTimeout(() => {
    updateGatewayStatus();
  }, 2500);
}

// Runtime Message Handling (Gateway Bridge & Actions)
chrome.runtime.onMessage.addListener((request, _sender, sendResponse) => {
  if (request.type === 'CHECK_GATEWAY_HEALTH') {
    checkGatewayHealth(request.url).then((health) => {
      chrome.storage.local.set({ gatewayConnected: health.ok, gatewayInfo: health });
      sendResponse(health);
    });
    return true; // Keep channel open for async response
  }

  if (request.type === 'RELAY_GATEWAY_REQUEST') {
    executeGatewayTool(request.tool, request.payload, request.url)
      .then((data) => sendResponse({ success: true, data }))
      .catch((err) => sendResponse({ success: false, error: err.message }));
    return true;
  }

  if (request.type === 'GATEWAY_CHUNK') {
    chunkDocumentViaGateway(request.source, request.options, request.url)
      .then((data) => sendResponse({ success: true, data }))
      .catch((err) => sendResponse({ success: false, error: err.message }));
    return true;
  }

  if (request.type === 'GET_GATEWAY_STATUS') {
    chrome.storage.local.get(['gatewayConnected', 'gatewayInfo', 'lastChecked']).then(sendResponse);
    return true;
  }
});
