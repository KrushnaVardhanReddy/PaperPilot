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
  <div class="geometric-accent top-right"></div>
  <div class="geometric-accent bottom-left"></div>

  <div class="content">
    <div class="header">
      <div class="logo">
        <svg width="40" height="40" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect x="3" y="3" width="18" height="18" rx="2" stroke="#2C3E50" stroke-width="2"/>
          <path d="M8 12L11 15L16 9" stroke="#3498DB" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
      </div>
      <h1 class="title">{title}</h1>
    </div>

    <div class="body">
      <p class="subtitle">PROUDLY PRESENTED TO</p>
      <h2 class="recipient">{recipientName}</h2>
      <div class="divider"></div>
      <p class="description">{description}</p>
    </div>

    <div class="footer">
      <div class="info-block">
        <span class="label">DATE</span>
        <span class="value">{date}</span>
      </div>

      <div class="signature-block">
        <span class="label">{issuerTitle.toUpperCase()}</span>
        <span class="value">{issuerName}</span>
      </div>
    </div>

    <div class="verification-badge">
      <div class="cert-info">
        <span class="id">ID: {certId}</span>
        {#if sha256Hash}
          <span class="hash">Hash: {sha256Hash.substring(0, 16)}...</span>
        {/if}
      </div>
      <div class="verify-branding">
        Verified with PaperPilot <span class="url">{verificationUrl}</span>
      </div>
    </div>
  </div>
</div>

<style>
  .certificate-container {
    width: 100%;
    aspect-ratio: 1.414 / 1;
    background: #FFFFFF;
    box-sizing: border-box;
    font-family: 'Inter', 'Segoe UI', Roboto, Helvetica, Arial, sans-serif;
    color: #2C3E50;
    position: relative;
    overflow: hidden;
    box-shadow: 0 10px 30px rgba(0,0,0,0.1);
  }

  .geometric-accent {
    position: absolute;
    background: #F8F9FA;
    z-index: 1;
  }

  .top-right {
    top: -10%;
    right: -10%;
    width: 40%;
    height: 60%;
    background: linear-gradient(135deg, #F8F9FA 0%, #E9ECEF 100%);
    transform: rotate(45deg);
  }

  .bottom-left {
    bottom: -15%;
    left: -15%;
    width: 50%;
    height: 50%;
    background: linear-gradient(135deg, #E9ECEF 0%, #F8F9FA 100%);
    transform: rotate(45deg);
  }

  .content {
    position: relative;
    z-index: 2;
    padding: 60px 80px;
    height: 100%;
    display: flex;
    flex-direction: column;
    box-sizing: border-box;
  }

  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    border-bottom: 2px solid #E9ECEF;
    padding-bottom: 24px;
  }

  .title {
    font-size: 1.5rem;
    font-weight: 700;
    letter-spacing: 4px;
    text-transform: uppercase;
    color: #34495E;
    margin: 0;
  }

  .body {
    flex: 1;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: flex-start;
  }

  .subtitle {
    font-size: 1rem;
    font-weight: 600;
    letter-spacing: 2px;
    color: #7F8C8D;
    margin-bottom: 8px;
  }

  .recipient {
    font-size: 4.5rem;
    font-weight: 800;
    color: #2C3E50;
    margin: 0 0 24px 0;
    line-height: 1.1;
    letter-spacing: -1px;
  }

  .divider {
    width: 60px;
    height: 4px;
    background: #3498DB;
    margin-bottom: 24px;
  }

  .description {
    font-size: 1.25rem;
    line-height: 1.6;
    color: #555;
    max-width: 80%;
    margin: 0;
  }

  .footer {
    display: flex;
    justify-content: flex-start;
    gap: 120px;
    margin-top: 40px;
    padding-bottom: 20px;
  }

  .info-block, .signature-block {
    display: flex;
    flex-direction: column;
  }

  .label {
    font-size: 0.75rem;
    font-weight: 700;
    letter-spacing: 1px;
    color: #95A5A6;
    margin-bottom: 8px;
  }

  .value {
    font-size: 1.25rem;
    font-weight: 600;
    color: #2C3E50;
  }

  .verification-badge {
    position: absolute;
    bottom: 24px;
    left: 80px;
    right: 80px;
    display: flex;
    justify-content: space-between;
    font-size: 0.75rem;
    color: #95A5A6;
  }

  .verify-branding {
    font-weight: 500;
  }

  .verify-branding .url {
    color: #3498DB;
    margin-left: 4px;
  }
</style>
