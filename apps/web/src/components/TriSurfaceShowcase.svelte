<script lang="ts">
  import { DOWNLOAD_URL } from "../constants";
</script>

<section class="showcase-section">
  <div class="showcase-header">
    <h2>One Engine. Three Surfaces.</h2>
    <p>
      Deploy PaperPilot exactly where you need it, powered by the same
      blazing-fast Rust core.
    </p>
  </div>

  <div class="showcase-grid">
    <div class="surface-card">
      <div class="card-icon">🖥️</div>
      <h3>Desktop App</h3>
      <p class="card-desc">
        44 native tools, batch folder processing, and offline AI chat. 100%
        local privacy for sensitive documents.
      </p>
      <ul class="feature-list">
        <li>✓ Mac, Windows & Linux</li>
        <li>✓ Batch processing</li>
        <li>✓ Zero data collection</li>
      </ul>
      <a
        href={DOWNLOAD_URL}
        target="_blank"
        rel="noopener noreferrer"
        class="card-link">Download Free &rarr;</a
      >
    </div>

    <div class="surface-card highlight">
      <div class="card-icon">🧩</div>
      <h3>Drop-In Embed Widget</h3>
      <p class="card-desc">
        Add client-side PDF powers to any website with just 2 lines of HTML.
        Integrates with WordPress, Webflow, and Shopify.
      </p>
      <ul class="feature-list">
        <li>✓ Zero server costs</li>
        <li>✓ Customizable branding</li>
        <li>✓ ~50–100ms WASM execution</li>
      </ul>
      <button
        class="card-link"
        onclick={() =>
          document
            .getElementById("embed-generator")
            ?.scrollIntoView({ behavior: "smooth" })}
        >Configure Widget &rarr;</button
      >
    </div>

    <div class="surface-card">
      <div class="card-icon">☁️</div>
      <h3>Cloud API & MCP Gateway</h3>
      <p class="card-desc">
        Headless REST API running on Edge WASM, plus direct MCP integration for
        Claude and Cursor AI assistants.
      </p>
      <ul class="feature-list">
        <li>✓ Swagger UI included</li>
        <li>✓ Standard JSON-RPC</li>
        <li>✓ Serverless ready</li>
      </ul>
      <button
        class="card-link"
        onclick={() =>
          document
            .getElementById("api-explorer")
            ?.scrollIntoView({ behavior: "smooth" })}>Explore API &rarr;</button
      >
    </div>
  </div>
</section>

<style>
  .showcase-section {
    padding: 80px 32px;
    max-width: 1200px;
    margin: 0 auto;
    position: relative;
    z-index: 5;
  }

  .showcase-header {
    text-align: center;
    margin-bottom: 64px;
  }

  .showcase-header h2 {
    font-size: 2.5rem;
    margin-bottom: 16px;
  }

  .showcase-header p {
    font-size: 1.1rem;
    color: var(--text-secondary);
    max-width: 600px;
    margin: 0 auto;
  }

  .showcase-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 32px;
  }

  .surface-card {
    background-color: var(--surface-1);
    border: 1px solid var(--border-color);
    border-radius: var(--border-radius-xl);
    padding: 32px;
    display: flex;
    flex-direction: column;
    transition:
      transform var(--transition-normal),
      box-shadow var(--transition-normal);
  }

  .surface-card:hover {
    transform: translateY(-8px);
    box-shadow: var(--shadow-lg);
  }

  .surface-card.highlight {
    border-color: var(--accent-primary);
    background: linear-gradient(
      to bottom,
      var(--surface-1),
      rgba(94, 106, 210, 0.05)
    );
    position: relative;
  }

  .surface-card.highlight::before {
    content: "Most Popular";
    position: absolute;
    top: -12px;
    left: 50%;
    transform: translateX(-50%);
    background-color: var(--accent-primary);
    color: white;
    font-size: 0.75rem;
    font-weight: 700;
    text-transform: uppercase;
    padding: 4px 12px;
    border-radius: 12px;
  }

  .card-icon {
    font-size: 3rem;
    margin-bottom: 24px;
  }

  .surface-card h3 {
    font-size: 1.5rem;
    margin-bottom: 16px;
  }

  .card-desc {
    color: var(--text-secondary);
    margin-bottom: 24px;
    flex-grow: 1;
  }

  .feature-list {
    list-style: none;
    padding: 0;
    margin: 0 0 32px 0;
    color: var(--text-primary);
    font-size: 0.95rem;
  }

  .feature-list li {
    margin-bottom: 8px;
    display: flex;
    align-items: center;
  }

  .card-link {
    color: var(--accent-primary);
    font-weight: 600;
    text-decoration: none;
    background: transparent;
    border: none;
    padding: 0;
    cursor: pointer;
    text-align: left;
    font-size: 1rem;
    display: inline-flex;
    align-items: center;
    gap: 8px;
    transition: color var(--transition-fast);
  }

  .card-link:hover {
    color: var(--accent-hover);
  }

  @media (max-width: 900px) {
    .showcase-grid {
      grid-template-columns: 1fr;
    }
  }
</style>
