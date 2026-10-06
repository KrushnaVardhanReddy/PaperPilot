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
  <div class="certificate-inner">
    <div class="border-outer">
      <div class="border-inner">
        <div class="watermark"></div>
        <div class="header">
          <div class="logo">
            <svg width="48" height="48" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M12 2L2 7L12 12L22 7L12 2Z" fill="#D4AF37"/>
              <path d="M2 17L12 22L22 17" stroke="#D4AF37" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
              <path d="M2 12L12 17L22 12" stroke="#D4AF37" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
          </div>
          <h1 class="title">{title}</h1>
        </div>

        <div class="body">
          <p class="presents">This is to certify that</p>
          <h2 class="recipient">{recipientName}</h2>
          <p class="description">{description}</p>
        </div>

        <div class="footer">
          <div class="signature-block">
            <div class="line"></div>
            <p class="name">{issuerName}</p>
            <p class="role">{issuerTitle}</p>
          </div>

          <div class="seal">
            <svg width="80" height="80" viewBox="0 0 100 100">
              <circle cx="50" cy="50" r="45" fill="none" stroke="#D4AF37" stroke-width="2" stroke-dasharray="4 4" />
              <circle cx="50" cy="50" r="38" fill="none" stroke="#D4AF37" stroke-width="1" />
              <circle cx="50" cy="50" r="35" fill="#FFF9E6" />
              <text x="50" y="55" font-family="Georgia, serif" font-size="24" fill="#D4AF37" text-anchor="middle" font-weight="bold">SEAL</text>
            </svg>
          </div>

          <div class="signature-block date-block">
            <div class="line"></div>
            <p class="name">{date}</p>
            <p class="role">Date</p>
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
  </div>
</div>

<style>
  .certificate-container {
    width: 100%;
    aspect-ratio: 1.414 / 1; /* ISO standard paper ratio landscape */
    background: white;
    padding: 2%;
    box-sizing: border-box;
    font-family: 'Times New Roman', Times, serif;
    color: #333;
    position: relative;
    overflow: hidden;
    box-shadow: 0 10px 30px rgba(0,0,0,0.1);
  }

  .certificate-inner {
    width: 100%;
    height: 100%;
    border: 10px solid #D4AF37; /* Gold border */
    padding: 4px;
    box-sizing: border-box;
  }

  .border-outer {
    width: 100%;
    height: 100%;
    border: 2px solid #B8860B;
    padding: 6px;
    box-sizing: border-box;
  }

  .border-inner {
    width: 100%;
    height: 100%;
    border: 1px solid #D4AF37;
    position: relative;
    display: flex;
    flex-direction: column;
    padding: 40px;
    box-sizing: border-box;
    background: #FFFCF5;
  }

  .header {
    text-align: center;
    margin-bottom: 1rem;
  }

  .logo {
    margin-bottom: 0.5rem;
  }

  .title {
    font-size: 2.4rem;
    font-weight: bold;
    text-transform: uppercase;
    color: #2C3E50;
    margin: 0;
    letter-spacing: 2px;
  }

  .body {
    flex: 1;
    text-align: center;
    display: flex;
    flex-direction: column;
    justify-content: center;
  }

  .presents {
    font-style: italic;
    font-size: 1.25rem;
    color: #555;
    margin-bottom: 0.75rem;
  }

  .recipient {
    font-size: 3.2rem;
    font-family: 'Georgia', serif;
    color: #D4AF37;
    margin: 0 0 1rem 0;
    font-weight: normal;
    font-style: italic;
    border-bottom: 2px solid #D4AF37;
    display: inline-block;
    padding: 0 2rem 0.4rem 2rem;
    align-self: center;
  }

  .description {
    font-size: 1.1rem;
    line-height: 1.5;
    max-width: 82%;
    margin: 0 auto;
    color: #444;
  }

  .footer {
    display: flex;
    justify-content: space-between;
    align-items: flex-end;
    margin-top: 1.5rem;
    padding-bottom: 2rem;
  }

  .signature-block {
    text-align: center;
    width: 250px;
  }

  .line {
    border-bottom: 1px solid #333;
    margin-bottom: 0.5rem;
    height: 40px;
  }

  .name {
    font-size: 1.2rem;
    font-weight: bold;
    margin: 0;
  }

  .role {
    font-size: 1rem;
    color: #666;
    margin: 0;
    font-style: italic;
  }

  .seal {
    display: flex;
    justify-content: center;
    align-items: center;
  }

  .verification-badge {
    position: absolute;
    bottom: 10px;
    left: 40px;
    right: 40px;
    display: flex;
    justify-content: space-between;
    font-size: 0.75rem;
    font-family: sans-serif;
    color: #888;
    border-top: 1px solid #E5E5E5;
    padding-top: 8px;
  }

  .verify-branding {
    font-weight: 600;
  }

  .verify-branding .url {
    color: #D4AF37;
    margin-left: 4px;
  }
</style>
