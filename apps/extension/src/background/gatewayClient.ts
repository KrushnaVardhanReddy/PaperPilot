/**
 * PaperPilot Gateway Client
 * Provides high-speed HTTP communication with local PaperPilot Desktop daemon (127.0.0.1:7823).
 */

export const DEFAULT_GATEWAY_URL = 'http://127.0.0.1:7823';
export const FALLBACK_GATEWAY_URL = 'http://localhost:7823';

export interface GatewayHealth {
  ok: boolean;
  status?: string;
  version?: string;
  toolsCount?: number;
  error?: string;
  url?: string;
}

export interface ChunkItem {
  id?: string;
  index: number;
  text: string;
  page?: number;
  tokenCount?: number;
}

export interface ChunkResponse {
  success: boolean;
  chunks: ChunkItem[];
  count: number;
  totalTokens?: number;
  error?: string;
}

/**
 * Check health of PaperPilot Desktop Gateway daemon
 */
export async function checkGatewayHealth(customUrl?: string): Promise<GatewayHealth> {
  const urlsToTry = customUrl ? [customUrl] : [DEFAULT_GATEWAY_URL, FALLBACK_GATEWAY_URL];

  for (const url of urlsToTry) {
    try {
      const controller = new AbortController();
      const timeoutId = setTimeout(() => controller.abort(), 1200);

      const res = await fetch(`${url}/health`, {
        method: 'GET',
        headers: { 'Accept': 'application/json' },
        signal: controller.signal,
      });

      clearTimeout(timeoutId);

      if (res.ok) {
        const data = await res.json();
        return {
          ok: true,
          status: data.status || 'ok',
          version: data.version,
          toolsCount: data.tools_count ?? data.toolsCount,
          url,
        };
      }
    } catch (err: any) {
      // Continue to next URL candidate if available
    }
  }

  return {
    ok: false,
    error: 'Gateway offline or unreachable at 127.0.0.1:7823',
  };
}

/**
 * Relay a tool execution request to Desktop Gateway API
 */
export async function executeGatewayTool(
  toolName: string,
  payload: Record<string, any>,
  baseUrl = DEFAULT_GATEWAY_URL
): Promise<any> {
  const url = `${baseUrl}/api/v1/pdf/tools/${encodeURIComponent(toolName)}`;
  
  const res = await fetch(url, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'Accept': 'application/json',
    },
    body: JSON.stringify(payload),
  });

  if (!res.ok) {
    const errorText = await res.text();
    throw new Error(`Gateway tool ${toolName} failed (${res.status}): ${errorText}`);
  }

  return await res.json();
}

/**
 * Execute semantic chunking via local daemon
 */
export async function chunkDocumentViaGateway(
  source: { text?: string; bytes_base64?: string; path?: string },
  options: { max_chunk_size?: number } = {},
  baseUrl = DEFAULT_GATEWAY_URL
): Promise<ChunkResponse> {
  // Try dedicated chunk endpoint first
  try {
    const res = await fetch(`${baseUrl}/api/v1/pdf/chunk`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Accept': 'application/json',
      },
      body: JSON.stringify({
        ...source,
        max_chunk_size: options.max_chunk_size ?? 1000,
      }),
    });

    if (res.ok) {
      const data = await res.json();
      const rawChunks = data.chunks || data.result?.chunks || [];
      const chunks: ChunkItem[] = rawChunks.map((c: any, i: number) => ({
        index: c.chunk_index ?? c.index ?? i,
        text: typeof c === 'string' ? c : c.text || c.content || '',
        page: c.page ?? c.page_number,
        tokenCount: c.token_count ?? Math.ceil((c.text || '').length / 4),
      }));

      return {
        success: true,
        chunks,
        count: chunks.length,
        totalTokens: chunks.reduce((acc, c) => acc + (c.tokenCount || 0), 0),
      };
    }
  } catch (_e) {
    // Fall back to rag-chunks or tool endpoint
  }

  // Fallback to rag-chunks
  try {
    const res = await fetch(`${baseUrl}/api/v1/pdf/rag-chunks`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        text: source.text,
        file_base64: source.bytes_base64,
        max_chunk_size: options.max_chunk_size ?? 1000,
      }),
    });

    if (res.ok) {
      const data = await res.json();
      const rawChunks = data.chunks || data.result?.chunks || [];
      const chunks: ChunkItem[] = rawChunks.map((c: any, i: number) => ({
        index: i,
        text: typeof c === 'string' ? c : c.text || '',
        tokenCount: Math.ceil((typeof c === 'string' ? c : c.text || '').length / 4),
      }));

      return {
        success: true,
        chunks,
        count: chunks.length,
      };
    }
  } catch (err: any) {
    return {
      success: false,
      chunks: [],
      count: 0,
      error: err.message || 'Failed to chunk document via Gateway',
    };
  }

  return {
    success: false,
    chunks: [],
    count: 0,
    error: 'Gateway chunking endpoints unreachable',
  };
}
