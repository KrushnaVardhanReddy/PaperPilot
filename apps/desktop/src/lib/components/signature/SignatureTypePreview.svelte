<script lang="ts">
  let {
    name = $bindable('John Doe'),
    initials = $bindable('JD'),
    selectedColor = $bindable('#000000'),
    selectedFontId = $bindable('caveat'),
    isInitialsMode = $bindable(false)
  } = $props<{
    name?: string;
    initials?: string;
    selectedColor?: string;
    selectedFontId?: string;
    isInitialsMode?: boolean;
  }>();

  export const FONT_OPTIONS = [
    {
      id: 'caveat',
      name: 'Caveat Style',
      family: '"Caveat", "Brush Script MT", cursive',
      cssClass: 'font-caveat',
      scale: 1.15
    },
    {
      id: 'dancing',
      name: 'Dancing Script',
      family: '"Dancing Script", "Apple Chancery", cursive',
      cssClass: 'font-dancing',
      scale: 1.05
    },
    {
      id: 'sacramento',
      name: 'Sacramento Calligraphy',
      family: '"Sacramento", "Snell Roundhand", cursive',
      cssClass: 'font-sacramento',
      scale: 1.25
    },
    {
      id: 'cedarville',
      name: 'Cedarville Cursive',
      family: '"Cedarville Cursive", "Segoe Script", cursive',
      cssClass: 'font-cedarville',
      scale: 1.0
    }
  ];

  export const COLOR_OPTIONS = [
    { label: 'Black', value: '#000000' },
    { label: 'Blue', value: '#1e40af' },
    { label: 'Dark Slate', value: '#334155' }
  ];

  $effect(() => {
    // Automatically update initials if user didn't customize it or it matches previous initials
    if (name && !isInitialsMode) {
      const parts = name.trim().split(/\s+/).filter(Boolean);
      if (parts.length > 0) {
        initials = parts.map((p: string) => p[0]?.toUpperCase() || '').join('');
      }
    }
  });

  export function getActiveText(): string {
    return isInitialsMode ? (initials.trim() || 'JD') : (name.trim() || 'John Doe');
  }

  export function getSignatureData(): { dataUrl: string; width: number; height: number; text: string } {
    const text = getActiveText();
    const font = FONT_OPTIONS.find(f => f.id === selectedFontId) || FONT_OPTIONS[0];

    const canvas = document.createElement('canvas');
    const baseW = isInitialsMode ? 400 : 700;
    const baseH = 220;
    canvas.width = baseW;
    canvas.height = baseH;

    const ctx = canvas.getContext('2d');
    if (!ctx) {
      let fallbackUrl = 'data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+M9QDwADhgGAWjR9awAAAABJRU5ErkJggg==';
      try {
        if (typeof canvas.toDataURL === 'function') {
          const url = canvas.toDataURL('image/png');
          if (url && url !== 'data:,') fallbackUrl = url;
        }
      } catch {}
      return { dataUrl: fallbackUrl, width: baseW, height: baseH, text };
    }

    ctx.clearRect(0, 0, baseW, baseH);
    ctx.fillStyle = selectedColor;

    const fontSize = Math.round((isInitialsMode ? 72 : 54) * font.scale);
    ctx.font = `${fontSize}px ${font.family}`;
    ctx.textAlign = 'center';
    ctx.textBaseline = 'middle';
    ctx.fillText(text, baseW / 2, baseH / 2);

    // Measure bounding box to trim
    const imgData = ctx.getImageData(0, 0, baseW, baseH);
    const data = imgData.data;
    let minX = baseW, minY = baseH, maxX = 0, maxY = 0;
    let hasPixel = false;

    for (let y = 0; y < baseH; y++) {
      for (let x = 0; x < baseW; x++) {
        const alpha = data[(y * baseW + x) * 4 + 3];
        if (alpha > 10) {
          hasPixel = true;
          if (x < minX) minX = x;
          if (x > maxX) maxX = x;
          if (y < minY) minY = y;
          if (y > maxY) maxY = y;
        }
      }
    }

    if (!hasPixel) {
      return { dataUrl: canvas.toDataURL('image/png'), width: 200, height: 80, text };
    }

    const padding = 16;
    const cropX = Math.max(0, minX - padding);
    const cropY = Math.max(0, minY - padding);
    const cropW = Math.min(baseW - cropX, (maxX - minX) + padding * 2);
    const cropH = Math.min(baseH - cropY, (maxY - minY) + padding * 2);

    const trimmed = document.createElement('canvas');
    trimmed.width = cropW;
    trimmed.height = cropH;
    const tCtx = trimmed.getContext('2d');
    if (tCtx) {
      tCtx.drawImage(canvas, cropX, cropY, cropW, cropH, 0, 0, cropW, cropH);
      return {
        dataUrl: trimmed.toDataURL('image/png'),
        width: cropW,
        height: cropH,
        text
      };
    }

    return {
      dataUrl: canvas.toDataURL('image/png'),
      width: baseW,
      height: baseH,
      text
    };
  }
</script>

<div class="type-preview-container">
  <!-- Mode & Input Fields -->
  <div class="type-header-row">
    <div class="mode-toggle">
      <button
        type="button"
        class="toggle-chip"
        class:active={!isInitialsMode}
        onclick={() => isInitialsMode = false}
      >
        ✍ Full Signature
      </button>
      <button
        type="button"
        class="toggle-chip"
        class:active={isInitialsMode}
        onclick={() => isInitialsMode = true}
      >
        📝 Initials
      </button>
    </div>

    <div class="color-picker-row">
      <span class="color-picker-label">Ink Color:</span>
      <div class="color-swatches">
        {#each COLOR_OPTIONS as c}
          <button
            type="button"
            class="color-swatch-btn"
            class:selected={selectedColor === c.value}
            style="background-color: {c.value};"
            title={c.label}
            aria-label={c.label}
            onclick={() => selectedColor = c.value}
          >
            {#if selectedColor === c.value}
              <span class="swatch-check">✓</span>
            {/if}
          </button>
        {/each}
      </div>
    </div>
  </div>

  <div class="input-fields-row">
    {#if !isInitialsMode}
      <div class="input-group">
        <label for="sig-full-name-input">Full Name</label>
        <input
          id="sig-full-name-input"
          type="text"
          class="text-input"
          bind:value={name}
          placeholder="e.g. John Doe"
        />
      </div>
    {:else}
      <div class="input-group">
        <label for="sig-initials-input">Initials</label>
        <input
          id="sig-initials-input"
          type="text"
          class="text-input"
          bind:value={initials}
          placeholder="e.g. JD"
          maxlength="6"
        />
      </div>
    {/if}
  </div>

  <!-- 4 Script Font Preview Cards -->
  <div class="font-cards-grid">
    {#each FONT_OPTIONS as font}
      <button
        type="button"
        class="font-card"
        class:selected={selectedFontId === font.id}
        onclick={() => selectedFontId = font.id}
      >
        <div class="font-card-header">
          <span class="font-name">{font.name}</span>
          {#if selectedFontId === font.id}
            <span class="selected-badge">Selected</span>
          {/if}
        </div>
        <div
          class="font-preview-text {font.cssClass}"
          style="color: {selectedColor}; font-family: {font.family};"
        >
          {getActiveText()}
        </div>
      </button>
    {/each}
  </div>
</div>

<style>
  @import url('https://fonts.googleapis.com/css2?family=Caveat:wght@600&family=Cedarville+Cursive&family=Dancing+Script:wght@600&family=Sacramento&display=swap');

  .type-preview-container {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    width: 100%;
  }

  .type-header-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    flex-wrap: wrap;
    gap: 0.75rem;
  }

  .mode-toggle {
    display: flex;
    background: #f1f5f9;
    padding: 3px;
    border-radius: 8px;
    gap: 2px;
  }

  .toggle-chip {
    padding: 0.35rem 0.85rem;
    font-size: 13px;
    font-weight: 500;
    border-radius: 6px;
    border: none;
    background: transparent;
    color: #64748b;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .toggle-chip.active {
    background: #ffffff;
    color: #0f172a;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  }

  .color-picker-row {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .color-picker-label {
    font-size: 13px;
    color: #64748b;
    font-weight: 500;
  }

  .color-swatches {
    display: flex;
    gap: 0.4rem;
  }

  .color-swatch-btn {
    width: 26px;
    height: 26px;
    border-radius: 50%;
    border: 2px solid #ffffff;
    box-shadow: 0 0 0 1px #cbd5e1;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: transform 0.15s ease;
    padding: 0;
  }

  .color-swatch-btn:hover {
    transform: scale(1.1);
  }

  .color-swatch-btn.selected {
    box-shadow: 0 0 0 2px #2563eb;
    transform: scale(1.05);
  }

  .swatch-check {
    color: #ffffff;
    font-size: 12px;
    font-weight: bold;
  }

  .input-fields-row {
    display: flex;
    width: 100%;
  }

  .input-group {
    display: flex;
    flex-direction: column;
    gap: 0.35rem;
    width: 100%;
  }

  .input-group label {
    font-size: 12px;
    font-weight: 600;
    color: #475569;
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }

  .text-input {
    width: 100%;
    padding: 0.6rem 0.85rem;
    font-size: 15px;
    border: 1px solid #cbd5e1;
    border-radius: 8px;
    background: #ffffff;
    color: #0f172a;
    outline: none;
    transition: border-color 0.15s ease, box-shadow 0.15s ease;
  }

  .text-input:focus {
    border-color: #2563eb;
    box-shadow: 0 0 0 3px rgba(37, 99, 235, 0.15);
  }

  .font-cards-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 0.75rem;
  }

  .font-card {
    background: #ffffff;
    border: 1.5px solid #e2e8f0;
    border-radius: 8px;
    padding: 0.75rem 1rem;
    cursor: pointer;
    text-align: left;
    transition: all 0.15s ease;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    height: 100px;
  }

  .font-card:hover {
    border-color: #94a3b8;
    background: #f8fafc;
  }

  .font-card.selected {
    border-color: #2563eb;
    background: #eff6ff;
    box-shadow: 0 0 0 1px #2563eb;
  }

  .font-card-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    width: 100%;
  }

  .font-name {
    font-size: 11px;
    font-weight: 500;
    color: #64748b;
  }

  .selected-badge {
    font-size: 10px;
    font-weight: 600;
    color: #2563eb;
    background: #dbeafe;
    padding: 1px 6px;
    border-radius: 4px;
  }

  .font-preview-text {
    font-size: 28px;
    line-height: 1.2;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    padding-top: 0.25rem;
  }

  .font-caveat {
    font-family: 'Caveat', cursive, sans-serif;
  }

  .font-dancing {
    font-family: 'Dancing Script', cursive, sans-serif;
  }

  .font-sacramento {
    font-family: 'Sacramento', cursive, sans-serif;
    font-size: 32px;
  }

  .font-cedarville {
    font-family: 'Cedarville Cursive', cursive, sans-serif;
  }
</style>
