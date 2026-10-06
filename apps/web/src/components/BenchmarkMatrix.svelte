<script lang="ts">
</script>

<section class="benchmark-section">
  <div class="benchmark-container">
    <div class="benchmark-header">
      <h2>Why PaperPilot?</h2>
      <p>Here is what makes our purely client-side approach so powerful.</p>
    </div>

    <div class="table-wrapper">
      <table class="benchmark-table">
        <thead>
          <tr>
            <th>Feature</th>
            <th class="highlight-col">PaperPilot</th>
          </tr>
        </thead>
        <tbody>
          <tr>
            <td class="feature-name">Server Uploads Required</td>
            <td class="highlight-col text-success">None (100% Client-Side)</td>
          </tr>
          <tr>
            <td class="feature-name">Execution Speed (avg)</td>
            <td class="highlight-col text-success">~50-100ms</td>
          </tr>
          <tr>
            <td class="feature-name">C++ Security Vulnerabilities</td>
            <td class="highlight-col text-success">Zero (Memory-Safe Rust)</td>
          </tr>
          <tr>
            <td class="feature-name">Licensing</td>
            <td class="highlight-col text-success">Free & Open Source</td>
          </tr>
          <tr>
            <td class="feature-name">Bundle Size</td>
            <td class="highlight-col text-success">~4MB (WASM)</td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</section>

<style>
  .benchmark-section {
    padding: 80px 32px;
    background-color: var(--bg-primary);
  }

  .benchmark-container {
    max-width: 1000px;
    margin: 0 auto;
  }

  .benchmark-header {
    text-align: center;
    margin-bottom: 48px;
  }

  .benchmark-header h2 {
    font-size: 2.5rem;
    margin-bottom: 16px;
  }

  .benchmark-header p {
    font-size: 1.1rem;
    color: var(--text-secondary);
  }

  .table-wrapper {
    overflow-x: auto;
    background-color: var(--surface-1);
    border: 1px solid var(--border-color);
    border-radius: var(--border-radius-xl);
    box-shadow: var(--shadow-md);
  }

  .benchmark-table {
    width: 100%;
    border-collapse: collapse;
    text-align: left;
  }

  .benchmark-table th, .benchmark-table td {
    padding: 20px 24px;
    border-bottom: 1px solid var(--border-color);
  }

  .benchmark-table th {
    background-color: var(--surface-0);
    font-weight: 600;
    color: var(--text-primary);
    font-size: 1.05rem;
    white-space: nowrap;
  }

  .benchmark-table tr:last-child td {
    border-bottom: none;
  }

  .feature-name {
    font-weight: 500;
    color: var(--text-primary);
  }

  .highlight-col {
    background-color: rgba(94, 106, 210, 0.05);
    font-weight: 600;
  }

  .benchmark-table th.highlight-col {
    background-color: rgba(94, 106, 210, 0.1);
    color: var(--accent-primary);
    border-top: 3px solid var(--accent-primary);
  }

  .text-success { color: #10B981; }
  .text-warning { color: #F59E0B; }
  .text-danger { color: #EF4444; }

  @media (max-width: 768px) {
    .benchmark-table th, .benchmark-table td {
      padding: 16px;
      font-size: 0.9rem;
    }
  }
</style>
