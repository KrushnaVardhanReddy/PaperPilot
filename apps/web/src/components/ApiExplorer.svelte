<script lang="ts">
  let activeTab = $state('cli');

  const codeSnippets = {
    cli: `# Fast, zero-config CLI — 100% offline & private
paperpilot merge report1.pdf report2.pdf -o merged_report.pdf --json`,
    mcp: `{
  "mcpServers": {
    "paperpilot": {
      "command": "paperpilot",
      "args": ["mcp"]
    }
  }
}`,
    curl: `# Local self-hosted REST Gateway — zero external telemetry
curl -X POST http://localhost:8080/api/v1/pdf/merge \\
  -F "files=@report1.pdf" \\
  -F "files=@report2.pdf" \\
  -o merged_report.pdf`,
    wasm: `// 100% In-Browser WASM Engine — files never leave client
import { WasmPdfEngine } from '@paperpilot/wasm';

const mergedBytes = await WasmPdfEngine.merge([pdfBytes1, pdfBytes2]);`
  };
</script>

<section id="api-explorer" class="api-section">
  <div class="api-container">
    <div class="api-header">
      <h2>Built for Developers. Ready for AI.</h2>
      <p>Integrate PaperPilot's tools into your backend, equip Claude and Cursor with native MCP, or run client-side in the browser via WASM.</p>
    </div>

    <div class="api-content">
      <div class="code-window">
        <div class="tabs">
          <button class="tab {activeTab === 'cli' ? 'active' : ''}" onclick={() => activeTab = 'cli'}>CLI</button>
          <button class="tab {activeTab === 'mcp' ? 'active' : ''}" onclick={() => activeTab = 'mcp'}>MCP (Claude/Cursor)</button>
          <button class="tab {activeTab === 'curl' ? 'active' : ''}" onclick={() => activeTab = 'curl'}>cURL (Local API)</button>
          <button class="tab {activeTab === 'wasm' ? 'active' : ''}" onclick={() => activeTab = 'wasm'}>WASM (Browser)</button>
        </div>
        <div class="code-content">
          <pre><code>{codeSnippets[activeTab as keyof typeof codeSnippets]}</code></pre>
        </div>
      </div>

      <div class="api-info">
        <h3>100% Local & Private by Design</h3>
        <p>Explore all 44 tools across CLI, MCP, Local REST API, and Browser WASM. No accounts, no API keys, and zero external telemetry.</p>

        <ul class="api-features">
          <li>✓ Standard Model Context Protocol (MCP) for AI IDEs</li>
          <li>✓ Blazing sub-10ms native execution with zero cloud lock-in</li>
          <li>✓ Fully typed OpenAPI 3.0 spec for local REST gateway</li>
        </ul>

        <div class="swagger-action-box">
          <a href="/swagger-ui" target="_blank" rel="noopener noreferrer" class="swagger-btn">
            Launch Swagger Explorer &rarr;
          </a>
          <span class="swagger-hint">Interactive OpenAPI 3.1 & Schema Viewer</span>
        </div>
      </div>
    </div>
  </div>
</section>

<style>
  .api-section {
    padding: 80px 32px;
    max-width: 1200px;
    margin: 0 auto;
  }

  .api-header {
    text-align: center;
    margin-bottom: 64px;
  }

  .api-header h2 {
    font-size: 2.5rem;
    margin-bottom: 16px;
  }

  .api-header p {
    font-size: 1.1rem;
    color: var(--text-secondary);
    max-width: 700px;
    margin: 0 auto;
  }

  .api-content {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 48px;
    align-items: center;
  }

  .code-window {
    background-color: #1e1e1e;
    border-radius: var(--border-radius-xl);
    border: 1px solid #333;
    overflow: hidden;
    box-shadow: var(--shadow-lg);
  }

  .tabs {
    display: flex;
    background-color: #2d2d2d;
    border-bottom: 1px solid #1e1e1e;
  }

  .tab {
    padding: 12px 24px;
    background: transparent;
    border: none;
    color: #888;
    cursor: pointer;
    font-family: var(--font-sans);
    font-size: 0.9rem;
    border-bottom: 2px solid transparent;
    transition: all 0.2s;
  }

  .tab:hover {
    color: #ccc;
  }

  .tab.active {
    color: #fff;
    border-bottom-color: var(--accent-primary);
    background-color: #1e1e1e;
  }

  .code-content {
    padding: 24px;
    min-height: 250px;
  }

  .code-content pre {
    margin: 0;
    color: #d4d4d4;
    font-family: 'Fira Code', Consolas, Monaco, monospace;
    font-size: 0.95rem;
    line-height: 1.6;
    overflow-x: auto;
  }

  .api-info h3 {
    font-size: 1.8rem;
    margin-bottom: 16px;
  }

  .api-info p {
    color: var(--text-secondary);
    margin-bottom: 24px;
    font-size: 1.05rem;
  }

  .api-features {
    list-style: none;
    padding: 0;
    margin: 0 0 32px 0;
  }

  .api-features li {
    margin-bottom: 12px;
    color: var(--text-primary);
    font-weight: 500;
  }

  .swagger-btn {
    display: inline-block;
    background-color: var(--surface-1);
    color: var(--text-primary);
    border: 1px solid var(--border-color);
    padding: 12px 24px;
    border-radius: var(--border-radius-md);
    text-decoration: none;
    font-weight: 600;
    transition: all var(--transition-fast);
  }

  .swagger-action-box {
    display: flex;
    align-items: center;
    gap: 16px;
    flex-wrap: wrap;
  }

  .swagger-hint {
    font-size: 0.85rem;
    color: var(--text-muted);
  }

  .swagger-btn:hover {
    background-color: var(--surface-2);
    border-color: var(--text-secondary);
  }

  @media (max-width: 900px) {
    .api-content {
      grid-template-columns: 1fr;
    }

    .code-window {
      order: 2;
    }

    .api-info {
      order: 1;
      text-align: center;
    }

    .api-features li {
      justify-content: center;
    }
  }
</style>
