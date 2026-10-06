<script lang="ts">
  let activeTab = $state('curl');

  const codeSnippets = {
    curl: `curl -X POST https://api.usepaperpilot.com/v1/tools/merge \\
  -H "Authorization: Bearer YOUR_API_KEY" \\
  -F "files=@report1.pdf" \\
  -F "files=@report2.pdf" \\
  -o merged_report.pdf`,
    ts: `import { PaperPilot } from '@paperpilot/client';

const client = new PaperPilot(process.env.PAPERPILOT_API_KEY);

const result = await client.tools.merge({
  files: ['report1.pdf', 'report2.pdf']
});

await result.saveAs('merged_report.pdf');`,
    python: `from paperpilot import PaperPilot

client = PaperPilot(api_key="YOUR_API_KEY")

result = client.tools.merge(
    files=["report1.pdf", "report2.pdf"]
)

result.save_as("merged_report.pdf")`,
    mcp: `{
  "mcpServers": {
    "paperpilot": {
      "command": "npx",
      "args": ["-y", "@paperpilot/mcp-server"],
      "env": {
        "PAPERPILOT_API_KEY": "YOUR_API_KEY"
      }
    }
  }
}`
  };
</script>

<section id="api-explorer" class="api-section">
  <div class="api-container">
    <div class="api-header">
      <h2>Built for Developers. Ready for AI.</h2>
      <p>Integrate PaperPilot's tools into your backend, or equip Claude and Cursor with PDF superpowers via MCP.</p>
    </div>

    <div class="api-content">
      <div class="code-window">
        <div class="tabs">
          <button class="tab {activeTab === 'curl' ? 'active' : ''}" onclick={() => activeTab = 'curl'}>cURL</button>
          <button class="tab {activeTab === 'ts' ? 'active' : ''}" onclick={() => activeTab = 'ts'}>TypeScript</button>
          <button class="tab {activeTab === 'python' ? 'active' : ''}" onclick={() => activeTab = 'python'}>Python</button>
          <button class="tab {activeTab === 'mcp' ? 'active' : ''}" onclick={() => activeTab = 'mcp'}>MCP (Claude/Cursor)</button>
        </div>
        <div class="code-content">
          <pre><code>{codeSnippets[activeTab as keyof typeof codeSnippets]}</code></pre>
        </div>
      </div>

      <div class="api-info">
        <h3>Comprehensive OpenAPI Spec</h3>
        <p>Explore all 44 tools interactively. Our API is strictly typed and built on Edge WASM for sub-10ms latency (excluding file transfer).</p>

        <ul class="api-features">
          <li>✓ Fully documented REST endpoints</li>
          <li>✓ Official SDKs for Node.js and Python</li>
          <li>✓ Standard Model Context Protocol (MCP) support</li>
        </ul>

        <a href="/swagger-ui" class="swagger-btn">
          Launch Swagger Explorer &rarr;
        </a>
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
