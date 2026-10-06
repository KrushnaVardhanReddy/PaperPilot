<script lang="ts">
  let {
    recipientName = "Recipient Name",
    title = "Certificate of Achievement",
    description = "For successfully completing the intensive curriculum...",
    date = "October 2026",
    issuerName = "PaperPilot Academy",
    issuerTitle = "Lead Instructor",
    certId = "CERT-000000",
    verificationUrl = "https://usepaperpilot.com/verify",
    sha256Hash = ""
  } = $props();
</script>

<div class="certificate-container">
  <div class="grid-overlay"></div>
  <div class="glow top-left"></div>
  <div class="glow bottom-right"></div>

  <div class="border-frame">
    <div class="corner top-left-corner"></div>
    <div class="corner top-right-corner"></div>
    <div class="corner bottom-left-corner"></div>
    <div class="corner bottom-right-corner"></div>

    <div class="header">
      <div class="badge">
        <svg width="40" height="40" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
          <path d="M12 2L2 7L12 12L22 7L12 2Z" stroke="#00FF41" stroke-width="2" stroke-linejoin="round"/>
          <path d="M2 17L12 22L22 17" stroke="#00FF41" stroke-width="2" stroke-linejoin="round"/>
          <path d="M2 12L12 17L22 12" stroke="#00FF41" stroke-width="2" stroke-linejoin="round"/>
        </svg>
      </div>
      <div class="title-container">
        <div class="sys-text">SYS.ACHIEVEMENT.UNLOCKED</div>
        <h1 class="title">{title}</h1>
      </div>
    </div>

    <div class="body">
      <div class="terminal-prompt">&gt; identify --user</div>
      <h2 class="recipient">{recipientName}</h2>
      <div class="terminal-prompt">&gt; cat description.txt</div>
      <p class="description">{description}</p>
    </div>

    <div class="footer">
      <div class="data-block">
        <div class="label">TIMESTAMP</div>
        <div class="value">{date}</div>
      </div>

      <div class="data-block auth">
        <div class="label">AUTH_SIGNATURE // {issuerTitle}</div>
        <div class="value signature">{issuerName}</div>
      </div>
    </div>

    <div class="verification-badge">
      <div class="cert-info">
        <span class="id">UID: {certId}</span>
        {#if sha256Hash}
          <span class="hash">SHA256: {sha256Hash.substring(0, 32)}...</span>
        {/if}
      </div>
      <div class="verify-branding">
        VERIFIED_WITH_PAPERPILOT <span class="url">[{verificationUrl}]</span>
      </div>
    </div>
  </div>
</div>

<style>
  .certificate-container {
    width: 100%;
    aspect-ratio: 1.414 / 1;
    background: #0D1117;
    box-sizing: border-box;
    font-family: 'Fira Code', 'Courier New', Courier, monospace;
    color: #C9D1D9;
    position: relative;
    overflow: hidden;
    padding: 24px;
    box-shadow: 0 10px 30px rgba(0,255,65,0.1);
  }

  .grid-overlay {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-image:
      linear-gradient(rgba(0, 255, 65, 0.05) 1px, transparent 1px),
      linear-gradient(90deg, rgba(0, 255, 65, 0.05) 1px, transparent 1px);
    background-size: 20px 20px;
    z-index: 1;
  }

  .glow {
    position: absolute;
    width: 300px;
    height: 300px;
    background: rgba(0, 255, 65, 0.15);
    filter: blur(100px);
    border-radius: 50%;
    z-index: 2;
  }

  .top-left {
    top: -100px;
    left: -100px;
  }

  .bottom-right {
    bottom: -100px;
    right: -100px;
    background: rgba(88, 166, 255, 0.15);
  }

  .border-frame {
    position: relative;
    z-index: 3;
    width: 100%;
    height: 100%;
    border: 1px solid rgba(0, 255, 65, 0.3);
    box-sizing: border-box;
    display: flex;
    flex-direction: column;
    padding: 40px;
    background: rgba(13, 17, 23, 0.7);
    backdrop-filter: blur(4px);
  }

  .corner {
    position: absolute;
    width: 15px;
    height: 15px;
    border: 2px solid #00FF41;
  }

  .top-left-corner { top: -1px; left: -1px; border-right: none; border-bottom: none; }
  .top-right-corner { top: -1px; right: -1px; border-left: none; border-bottom: none; }
  .bottom-left-corner { bottom: -1px; left: -1px; border-right: none; border-top: none; }
  .bottom-right-corner { bottom: -1px; right: -1px; border-left: none; border-top: none; }

  .header {
    display: flex;
    align-items: center;
    gap: 24px;
    margin-bottom: 40px;
    border-bottom: 1px dashed rgba(201, 209, 217, 0.2);
    padding-bottom: 24px;
  }

  .title-container {
    display: flex;
    flex-direction: column;
  }

  .sys-text {
    font-size: 0.8rem;
    color: #58A6FF;
    margin-bottom: 4px;
  }

  .title {
    font-size: 2rem;
    font-weight: 700;
    color: #00FF41;
    margin: 0;
    text-transform: uppercase;
    letter-spacing: 2px;
  }

  .body {
    flex: 1;
    display: flex;
    flex-direction: column;
    justify-content: center;
  }

  .terminal-prompt {
    color: #58A6FF;
    font-size: 0.9rem;
    margin-bottom: 8px;
  }

  .recipient {
    font-size: 3.5rem;
    font-weight: 700;
    color: #FFFFFF;
    margin: 0 0 32px 0;
    text-shadow: 0 0 10px rgba(0, 255, 65, 0.3);
  }

  .description {
    font-size: 1.1rem;
    line-height: 1.6;
    color: #8B949E;
    max-width: 90%;
    margin: 0;
    border-left: 2px solid rgba(0, 255, 65, 0.3);
    padding-left: 16px;
  }

  .footer {
    display: flex;
    justify-content: flex-start;
    gap: 80px;
    margin-top: 40px;
  }

  .data-block {
    display: flex;
    flex-direction: column;
  }

  .label {
    font-size: 0.75rem;
    color: #8B949E;
    margin-bottom: 4px;
  }

  .value {
    font-size: 1.2rem;
    color: #C9D1D9;
  }

  .signature {
    font-family: 'Brush Script MT', cursive, sans-serif;
    font-size: 1.8rem;
    color: #00FF41;
  }

  .verification-badge {
    position: absolute;
    bottom: 20px;
    left: 40px;
    right: 40px;
    display: flex;
    justify-content: space-between;
    font-size: 0.7rem;
    color: #484F58;
    border-top: 1px solid rgba(72, 79, 88, 0.3);
    padding-top: 12px;
  }

  .verify-branding {
    color: #8B949E;
  }

  .verify-branding .url {
    color: #58A6FF;
  }
</style>
