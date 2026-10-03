<script lang="ts">
  import openapiSpec from '../../../../../../docs/api/openapi.json';
  import { onMount } from 'svelte';

  let baseUrl = $state('http://127.0.0.1:7823');
  let searchQuery = $state('');
  let selectedCategory = $state('All');

  let status = $state('Unknown');
  let latency = $state<number | null>(null);
  let isChecking = $state(false);

  // Derive categories from tags + 'All'
  let categories = $derived(['All', ...(openapiSpec.tags ? openapiSpec.tags.map(t => t.name) : [])]);

  type Endpoint = {
    path: string;
    method: string;
    id: string;
    summary: string;
    description: string;
    tags: string[];
    parameters: any[];
    requestBody: any;
    responses: any;
  };

  let endpoints: Endpoint[] = [];
  for (const [path, methods] of Object.entries(openapiSpec.paths)) {
    for (const [method, operation] of Object.entries(methods as Record<string, any>)) {
      endpoints.push({
        path,
        method: method.toUpperCase(),
        id: `${method}-${path}`.replace(/[^a-zA-Z0-9]/g, '-'),
        summary: operation.summary || '',
        description: operation.description || '',
        tags: operation.tags || [],
        parameters: operation.parameters || [],
        requestBody: operation.requestBody || null,
        responses: operation.responses || {}
      });
    }
  }

  let filteredEndpoints = $derived(
    endpoints.filter(ep => {
      const matchSearch = ep.path.toLowerCase().includes(searchQuery.toLowerCase()) ||
                          ep.summary.toLowerCase().includes(searchQuery.toLowerCase()) ||
                          ep.description.toLowerCase().includes(searchQuery.toLowerCase());
      const matchCategory = selectedCategory === 'All' || ep.tags.includes(selectedCategory);
      return matchSearch && matchCategory;
    })
  );

  let expandedEndpoints = $state<Record<string, boolean>>({});

  function toggleEndpoint(id: string) {
    expandedEndpoints[id] = !expandedEndpoints[id];
  }

  async function checkHealth() {
    isChecking = true;
    const start = performance.now();
    try {
      const res = await fetch(`${baseUrl}/health`);
      if (res.ok) {
        status = 'Online';
      } else {
        status = 'Error';
      }
    } catch (e) {
      status = 'Offline';
    } finally {
      latency = Math.round(performance.now() - start);
      isChecking = false;
    }
  }

  function getMethodColor(method: string) {
    switch (method) {
      case 'GET': return '#3b82f6'; // blue
      case 'POST': return '#10b981'; // emerald
      case 'DELETE': return '#f43f5e'; // rose
      case 'PUT': return '#f59e0b'; // amber
      default: return '#6b7280'; // gray
    }
  }

  function generateCurl(ep: Endpoint, baseUrl: string) {
    let curl = `curl -X ${ep.method} "${baseUrl}${ep.path}"`;
    if (ep.requestBody) {
      curl += ` \\\n  -H "Content-Type: application/json"`;

      // Attempt to generate a basic example payload from schema
      let examplePayload: Record<string, any> = {};
      const content = ep.requestBody.content;
      if (content && content['application/json']) {
         const schema = content['application/json'].schema;
         if (schema && schema.$ref) {
            // Find in components
            const refName = schema.$ref.split('/').pop();
            const componentSchema = (openapiSpec.components?.schemas as any)?.[refName];
            if (componentSchema && componentSchema.properties) {
               for (const [key, prop] of Object.entries(componentSchema.properties)) {
                  examplePayload[key] = (prop as any).example ?? ((prop as any).type === 'string' ? "string" : null);
               }
            }
         }
      }

      curl += ` \\\n  -d '${JSON.stringify(examplePayload)}'`;
    }
    return curl;
  }

  function downloadSpec() {
    const blob = new Blob([JSON.stringify(openapiSpec, null, 2)], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = 'openapi.json';
    a.click();
    URL.revokeObjectURL(url);
  }

  let testResponses = $state<Record<string, { status: number, body: string }>>({});

  async function sendTestRequest(ep: Endpoint) {
    try {
       const url = `${baseUrl}${ep.path.replace(/\{.*?\}/g, 'test')}`; // Replace path params with 'test'
       const options: RequestInit = {
         method: ep.method,
         headers: {
           'Accept': 'application/json'
         }
       };

       if (ep.requestBody && ep.method !== 'GET') {
          options.headers = { ...options.headers, 'Content-Type': 'application/json' };
          // Simple payload extraction
          let examplePayload: Record<string, any> = {};
          const content = ep.requestBody.content;
          if (content && content['application/json']) {
            const schema = content['application/json'].schema;
            if (schema && schema.$ref) {
                const refName = schema.$ref.split('/').pop();
                const componentSchema = (openapiSpec.components?.schemas as any)?.[refName];
                if (componentSchema && componentSchema.properties) {
                  for (const [key, prop] of Object.entries(componentSchema.properties)) {
                      examplePayload[key] = (prop as any).example ?? ((prop as any).type === 'string' ? "string" : null);
                  }
                }
            }
          }
          options.body = JSON.stringify(examplePayload);
       }

       const res = await fetch(url, options);
       const text = await res.text();
       let bodyStr = text;
       try {
         bodyStr = JSON.stringify(JSON.parse(text), null, 2);
       } catch (e) {}

       testResponses[ep.id] = { status: res.status, body: bodyStr };
    } catch (e) {
       testResponses[ep.id] = { status: 0, body: String(e) };
    }
  }

  async function copyToClipboard(text: string) {
    try {
      await navigator.clipboard.writeText(text);
    } catch (e) {
      console.error("Failed to copy", e);
    }
  }
</script>

<div class="api-docs">
  <div class="header">
    <div class="title-row">
      <h1>PaperPilot API & Gateway</h1>
      <button id="btn-view-openapi-json" class="btn btn-outline" onclick={downloadSpec}>Download openapi.json</button>
    </div>

    <div class="gateway-config">
      <div class="input-group">
        <label for="api-gateway-url">Gateway URL</label>
        <input type="text" id="api-gateway-url" bind:value={baseUrl} />
      </div>
      <button class="btn btn-primary" onclick={checkHealth} disabled={isChecking}>
        {isChecking ? 'Checking...' : 'Check Status'}
      </button>
      <div class="status-badge" class:online={status === 'Online'} class:offline={status === 'Offline'} class:error={status === 'Error'}>
        <span class="dot"></span>
        {status} {latency !== null ? `(${latency}ms)` : ''}
      </div>
    </div>

    <div class="search-filter">
      <input type="text" id="api-search-input" placeholder="Search endpoints by path or description..." bind:value={searchQuery} />

      <div class="categories">
        {#each categories as category}
          <button
            class="category-pill"
            class:active={selectedCategory === category}
            onclick={() => selectedCategory = category}
          >
            {category}
          </button>
        {/each}
      </div>
    </div>
  </div>

  <div class="endpoints-list">
    {#each filteredEndpoints as ep (ep.id)}
      <div class="endpoint-card">
        <div class="endpoint-header" onclick={() => toggleEndpoint(ep.id)} role="button" tabindex="0" onkeydown={e => e.key === 'Enter' && toggleEndpoint(ep.id)}>
          <span class="method-badge" style="background-color: {getMethodColor(ep.method)}; color: white;">{ep.method}</span>
          <span class="path">{ep.path}</span>
          <span class="summary">{ep.summary}</span>
          <span class="expand-icon">{expandedEndpoints[ep.id] ? '▼' : '▶'}</span>
        </div>

        {#if expandedEndpoints[ep.id]}
          <div class="endpoint-details">
            <p class="description">{ep.description}</p>

            {#if ep.parameters.length > 0}
              <div class="section">
                <h4>Parameters</h4>
                <table class="params-table">
                  <thead>
                    <tr>
                      <th>Name</th>
                      <th>In</th>
                      <th>Required</th>
                      <th>Description</th>
                    </tr>
                  </thead>
                  <tbody>
                    {#each ep.parameters as param}
                      <tr>
                        <td><code>{param.name}</code></td>
                        <td>{param.in}</td>
                        <td>{param.required ? 'Yes' : 'No'}</td>
                        <td>{param.description || ''}</td>
                      </tr>
                    {/each}
                  </tbody>
                </table>
              </div>
            {/if}

            <div class="action-row">
              <div class="curl-box">
                <pre>{generateCurl(ep, baseUrl)}</pre>
                <button id="copy-curl-{ep.id}" class="btn btn-sm btn-outline" onclick={() => copyToClipboard(generateCurl(ep, baseUrl))}>Copy cURL</button>
              </div>
            </div>

            <div class="test-section">
              <button id="send-request-{ep.id}" class="btn btn-primary" onclick={() => sendTestRequest(ep)}>Send Request</button>
              {#if testResponses[ep.id]}
                <div class="response-box" class:error={testResponses[ep.id].status >= 400 || testResponses[ep.id].status === 0}>
                  <div class="response-header">
                    <span class="status-code">Status: {testResponses[ep.id].status}</span>
                  </div>
                  <pre>{testResponses[ep.id].body}</pre>
                </div>
              {/if}
            </div>
          </div>
        {/if}
      </div>
    {/each}
  </div>
</div>

<style>
  .api-docs {
    padding: 32px;
    color: var(--text-primary);
    max-width: 1200px;
    margin: 0 auto;
    font-family: var(--font-family, system-ui, sans-serif);
  }

  .header {
    margin-bottom: 32px;
  }

  .title-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 24px;
  }

  h1 {
    font-size: 2rem;
    margin: 0;
  }

  .gateway-config {
    display: flex;
    align-items: center;
    gap: 16px;
    background: var(--bg-surface);
    padding: 16px;
    border-radius: var(--border-radius-md, 8px);
    border: 1px solid var(--border-color);
    margin-bottom: 24px;
  }

  .input-group {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .input-group label {
    font-weight: 500;
  }

  input[type="text"] {
    padding: 8px 12px;
    border-radius: 4px;
    border: 1px solid var(--border-color);
    background: var(--bg-secondary);
    color: var(--text-primary);
    width: 300px;
  }

  .btn {
    padding: 8px 16px;
    border-radius: 4px;
    cursor: pointer;
    font-weight: 500;
    border: none;
    transition: background 0.2s;
  }

  .btn-primary {
    background: var(--accent-color, #38bdf8);
    color: #fff;
  }

  .btn-primary:hover {
    filter: brightness(1.1);
  }

  .btn-primary:disabled {
    opacity: 0.7;
    cursor: not-allowed;
  }

  .btn-outline {
    background: transparent;
    border: 1px solid var(--border-color);
    color: var(--text-primary);
  }

  .btn-outline:hover {
    background: var(--bg-surface-hover);
  }

  .btn-sm {
    padding: 4px 8px;
    font-size: 0.85rem;
  }

  .status-badge {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 4px 12px;
    border-radius: 999px;
    font-size: 0.9rem;
    font-weight: 500;
    background: var(--bg-secondary);
  }

  .dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: gray;
  }

  .status-badge.online .dot { background: #10b981; }
  .status-badge.offline .dot { background: #f43f5e; }
  .status-badge.error .dot { background: #f59e0b; }

  .search-filter {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  #api-search-input {
    width: 100%;
    max-width: 600px;
    padding: 10px 16px;
    border-radius: 6px;
    border: 1px solid var(--border-color);
    background: var(--bg-surface);
    color: var(--text-primary);
  }

  .categories {
    display: flex;
    gap: 8px;
    flex-wrap: wrap;
  }

  .category-pill {
    padding: 6px 12px;
    border-radius: 999px;
    border: 1px solid var(--border-color);
    background: transparent;
    color: var(--text-secondary);
    cursor: pointer;
    font-size: 0.9rem;
    transition: all 0.2s;
  }

  .category-pill.active {
    background: var(--accent-color, #38bdf8);
    color: #fff;
    border-color: var(--accent-color, #38bdf8);
  }

  .endpoints-list {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .endpoint-card {
    border: 1px solid var(--border-color);
    border-radius: 8px;
    background: var(--bg-surface);
    overflow: hidden;
  }

  .endpoint-header {
    display: flex;
    align-items: center;
    padding: 12px 16px;
    cursor: pointer;
    gap: 16px;
  }

  .endpoint-header:hover {
    background: var(--bg-surface-hover);
  }

  .method-badge {
    padding: 4px 10px;
    border-radius: 4px;
    font-weight: 700;
    font-size: 0.85rem;
    min-width: 60px;
    text-align: center;
  }

  .path {
    font-family: monospace;
    font-weight: 600;
    font-size: 1.05rem;
  }

  .summary {
    color: var(--text-secondary);
    flex: 1;
  }

  .expand-icon {
    color: var(--text-muted);
  }

  .endpoint-details {
    padding: 16px;
    border-top: 1px solid var(--border-color);
    background: var(--bg-secondary);
  }

  .description {
    margin-bottom: 16px;
    color: var(--text-secondary);
  }

  .section {
    margin-bottom: 24px;
  }

  h4 {
    margin-top: 0;
    margin-bottom: 12px;
  }

  .params-table {
    width: 100%;
    border-collapse: collapse;
    margin-bottom: 16px;
  }

  .params-table th, .params-table td {
    text-align: left;
    padding: 8px 12px;
    border-bottom: 1px solid var(--border-color);
  }

  .params-table th {
    background: var(--bg-surface);
  }

  code {
    background: var(--bg-surface);
    padding: 2px 6px;
    border-radius: 4px;
    font-family: monospace;
  }

  .action-row {
    margin-top: 24px;
  }

  .curl-box {
    background: var(--bg-surface);
    border: 1px solid var(--border-color);
    border-radius: 6px;
    padding: 12px;
    position: relative;
    margin-bottom: 16px;
  }

  .curl-box pre {
    margin: 0;
    white-space: pre-wrap;
    word-break: break-all;
    color: var(--text-primary);
  }

  .curl-box button {
    position: absolute;
    top: 12px;
    right: 12px;
  }

  .test-section {
    margin-top: 16px;
    padding-top: 16px;
    border-top: 1px dashed var(--border-color);
  }

  .response-box {
    margin-top: 12px;
    background: #1e1e1e;
    color: #d4d4d4;
    border-radius: 6px;
    overflow: hidden;
  }

  .response-header {
    background: #2d2d2d;
    padding: 8px 12px;
    font-size: 0.9rem;
    font-weight: 500;
  }

  .response-box.error .response-header {
    background: #5c1616;
  }

  .response-box pre {
    margin: 0;
    padding: 12px;
    overflow-x: auto;
    max-height: 400px;
  }
</style>
