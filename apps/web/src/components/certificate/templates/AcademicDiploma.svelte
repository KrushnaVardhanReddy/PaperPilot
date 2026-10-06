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
  <div class="content-wrapper">
    <div class="header">
      <div class="university-seal">
        <svg width="80" height="80" viewBox="0 0 100 100">
          <circle cx="50" cy="50" r="48" fill="none" stroke="#1A365D" stroke-width="2"/>
          <circle cx="50" cy="50" r="42" fill="none" stroke="#1A365D" stroke-width="1"/>
          <path d="M50 20 L80 80 L20 80 Z" fill="none" stroke="#1A365D" stroke-width="2"/>
          <circle cx="50" cy="50" r="10" fill="#1A365D"/>
        </svg>
      </div>
      <div class="university-name">PaperPilot University</div>
    </div>

    <div class="body">
      <h1 class="title">{title}</h1>
      <p class="confer-text">Upon the recommendation of the Faculty, hereby confers upon</p>
      <h2 class="recipient">{recipientName}</h2>
      <p class="description">{description}</p>
    </div>

    <div class="footer">
      <div class="signature-line">
        <div class="signature-image">{issuerName}</div>
        <div class="line"></div>
        <div class="title-text">{issuerTitle}</div>
      </div>

      <div class="honor-ribbon">
        <div class="ribbon-tail left"></div>
        <div class="ribbon-tail right"></div>
        <div class="ribbon-center">HONORS</div>
      </div>

      <div class="signature-line">
        <div class="signature-image">{date}</div>
        <div class="line"></div>
        <div class="title-text">Date of Issuance</div>
      </div>
    </div>

    <div class="verification-badge">
      <div class="cert-info">
        <span class="id">Document No. {certId}</span>
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
    background: #FDFBF7;
    box-sizing: border-box;
    font-family: 'Times New Roman', Times, serif;
    color: #1A365D;
    padding: 30px;
    box-shadow: 0 10px 30px rgba(0,0,0,0.1);
    position: relative;
  }

  .content-wrapper {
    width: 100%;
    height: 100%;
    border: 3px double #1A365D;
    padding: 40px;
    box-sizing: border-box;
    display: flex;
    flex-direction: column;
    position: relative;
    background-image: radial-gradient(circle at center, transparent 0%, rgba(26, 54, 93, 0.03) 100%);
  }

  .header {
    display: flex;
    flex-direction: column;
    align-items: center;
    margin-bottom: 30px;
  }

  .university-name {
    font-size: 1.8rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 4px;
    margin-top: 16px;
    color: #1A365D;
  }

  .body {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
  }

  .title {
    font-size: 2.8rem;
    font-family: 'Old English Text MT', 'Engravers Old English', 'Georgia', serif;
    font-weight: normal;
    color: #1A365D;
    margin: 0 0 24px 0;
  }

  .confer-text {
    font-size: 1.2rem;
    font-style: italic;
    color: #4A5568;
    margin-bottom: 24px;
  }

  .recipient {
    font-size: 3.5rem;
    font-weight: 700;
    font-family: 'Georgia', serif;
    color: #000000;
    margin: 0 0 32px 0;
    border-bottom: 1px solid #CBD5E0;
    padding-bottom: 8px;
    min-width: 60%;
  }

  .description {
    font-size: 1.2rem;
    line-height: 1.8;
    color: #2D3748;
    max-width: 75%;
    margin: 0;
  }

  .footer {
    display: flex;
    justify-content: space-between;
    align-items: flex-end;
    margin-top: auto;
    padding-bottom: 30px;
  }

  .signature-line {
    display: flex;
    flex-direction: column;
    align-items: center;
    width: 200px;
  }

  .signature-image {
    font-family: 'Brush Script MT', cursive, serif;
    font-size: 1.5rem;
    color: #1A365D;
    height: 40px;
    display: flex;
    align-items: flex-end;
    justify-content: center;
    padding-bottom: 4px;
  }

  .line {
    width: 100%;
    border-bottom: 1px solid #1A365D;
    margin-bottom: 8px;
  }

  .title-text {
    font-size: 0.9rem;
    color: #4A5568;
    text-transform: uppercase;
    letter-spacing: 1px;
  }

  .honor-ribbon {
    position: relative;
    width: 120px;
    height: 40px;
    display: flex;
    justify-content: center;
    align-items: center;
  }

  .ribbon-center {
    background: #9B2C2C;
    color: white;
    padding: 8px 24px;
    font-size: 1rem;
    font-weight: bold;
    letter-spacing: 2px;
    z-index: 2;
    box-shadow: 0 2px 4px rgba(0,0,0,0.2);
  }

  .ribbon-tail {
    position: absolute;
    width: 30px;
    height: 30px;
    background: #742A2A;
    bottom: -15px;
    z-index: 1;
  }

  .ribbon-tail.left {
    left: 10px;
    transform: skewY(30deg);
  }

  .ribbon-tail.right {
    right: 10px;
    transform: skewY(-30deg);
  }

  .verification-badge {
    position: absolute;
    bottom: 16px;
    left: 40px;
    right: 40px;
    display: flex;
    justify-content: space-between;
    font-size: 0.75rem;
    font-family: sans-serif;
    color: #718096;
  }

  .verify-branding {
    font-weight: 600;
  }

  .verify-branding .url {
    color: #1A365D;
  }
</style>
