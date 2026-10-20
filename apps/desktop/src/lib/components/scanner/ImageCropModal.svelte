<script lang="ts">
  import { onMount } from 'svelte';

  interface Point {
    x: number; // percentage 0 - 100
    y: number; // percentage 0 - 100
  }

  let {
    imageSrc,
    onConfirm,
    onCancel
  }: {
    imageSrc: string;
    onConfirm: (croppedDataUrl: string) => void;
    onCancel: () => void;
  } = $props();

  let containerEl = $state<HTMLDivElement | null>(null);
  let imgEl = $state<HTMLImageElement | null>(null);
  let canvasEl = $state<HTMLCanvasElement | null>(null);

  // 4 corners of the quad (Kaagaz style arbitrary quadrilateral)
  let corners = $state<Point[]>([
    { x: 8, y: 8 },    // 0: top-left
    { x: 92, y: 8 },   // 1: top-right
    { x: 92, y: 92 },  // 2: bottom-right
    { x: 8, y: 92 }    // 3: bottom-left
  ]);

  let rotation = $state(0);
  let activeIndex = $state<number | null>(null);
  let isMovingAll = $state(false);
  let startClient = { x: 0, y: 0 };
  let startCorners: Point[] = [];

  function handleTouchStartCorner(index: number, clientX: number, clientY: number) {
    activeIndex = index;
    isMovingAll = false;
    startClient = { x: clientX, y: clientY };
    startCorners = corners.map(c => ({ ...c }));
  }

  function handleTouchStartCenter(clientX: number, clientY: number) {
    activeIndex = null;
    isMovingAll = true;
    startClient = { x: clientX, y: clientY };
    startCorners = corners.map(c => ({ ...c }));
  }

  function handlePointerMove(clientX: number, clientY: number) {
    if (!containerEl) return;
    const rect = containerEl.getBoundingClientRect();
    if (rect.width === 0 || rect.height === 0) return;

    const deltaX = ((clientX - startClient.x) / rect.width) * 100;
    const deltaY = ((clientY - startClient.y) / rect.height) * 100;

    if (activeIndex !== null) {
      // Move individual corner freely
      const orig = startCorners[activeIndex];
      const newX = Math.min(Math.max(0, orig.x + deltaX), 100);
      const newY = Math.min(Math.max(0, orig.y + deltaY), 100);
      corners[activeIndex] = { x: newX, y: newY };
    } else if (isMovingAll) {
      // Move whole polygon
      let canMove = true;
      for (const c of startCorners) {
        if (c.x + deltaX < 0 || c.x + deltaX > 100 || c.y + deltaY < 0 || c.y + deltaY > 100) {
          canMove = false;
          break;
        }
      }
      if (canMove) {
        corners = startCorners.map(c => ({
          x: c.x + deltaX,
          y: c.y + deltaY
        }));
      }
    }
  }

  function handlePointerUp() {
    activeIndex = null;
    isMovingAll = false;
  }

  function rotateClockwise() {
    rotation = (rotation + 90) % 360;
  }

  function resetFull() {
    corners = [
      { x: 0, y: 0 },
      { x: 100, y: 0 },
      { x: 100, y: 100 },
      { x: 0, y: 100 }
    ];
  }

  // Perspective warp / quadrilateral transformation
  function getPerspectiveTransform(src: Point[], dst: Point[]) {
    // Computes homography 3x3 matrix from 4 points
    let a: number[][] = [];
    let b: number[] = [];

    for (let i = 0; i < 4; i++) {
      a.push([src[i].x, src[i].y, 1, 0, 0, 0, -src[i].x * dst[i].x, -src[i].y * dst[i].x]);
      b.push(dst[i].x);
      a.push([0, 0, 0, src[i].x, src[i].y, 1, -src[i].x * dst[i].y, -src[i].y * dst[i].y]);
      b.push(dst[i].y);
    }

    // Solve Ax = B with Gaussian elimination
    const n = 8;
    for (let i = 0; i < n; i++) {
      let maxEl = Math.abs(a[i][i]);
      let maxRow = i;
      for (let k = i + 1; k < n; k++) {
        if (Math.abs(a[k][i]) > maxEl) {
          maxEl = Math.abs(a[k][i]);
          maxRow = k;
        }
      }
      for (let k = i; k < n; k++) {
        const tmp = a[maxRow][k];
        a[maxRow][k] = a[i][k];
        a[i][k] = tmp;
      }
      const tmp = b[maxRow];
      b[maxRow] = b[i];
      b[i] = tmp;

      for (let k = i + 1; k < n; k++) {
        const c = -a[k][i] / a[i][i];
        for (let j = i; j < n; j++) {
          if (i === j) a[k][j] = 0;
          else a[k][j] += c * a[i][j];
        }
        b[k] += c * b[i];
      }
    }

    let x = new Array(8);
    for (let i = n - 1; i >= 0; i--) {
      x[i] = b[i] / a[i][i];
      for (let k = i - 1; k >= 0; k--) {
        b[k] -= a[k][i] * x[i];
      }
    }

    return [
      x[0], x[1], x[2],
      x[3], x[4], x[5],
      x[6], x[7], 1
    ];
  }

  function applyCrop() {
    if (!imgEl || !canvasEl) return;

    const naturalWidth = imgEl.naturalWidth;
    const naturalHeight = imgEl.naturalHeight;

    // First render rotated image to an offscreen canvas
    let sourceCanvas = document.createElement('canvas');
    if (rotation === 90 || rotation === 270) {
      sourceCanvas.width = naturalHeight;
      sourceCanvas.height = naturalWidth;
    } else {
      sourceCanvas.width = naturalWidth;
      sourceCanvas.height = naturalHeight;
    }

    const sCtx = sourceCanvas.getContext('2d');
    if (!sCtx) return;

    sCtx.translate(sourceCanvas.width / 2, sourceCanvas.height / 2);
    sCtx.rotate((rotation * Math.PI) / 180);
    sCtx.drawImage(imgEl, -naturalWidth / 2, -naturalHeight / 2);

    // Convert percentage corners to pixels in sourceCanvas space
    const pts = corners.map(c => ({
      x: (c.x / 100) * sourceCanvas.width,
      y: (c.y / 100) * sourceCanvas.height
    }));

    // Target rectangle dimensions based on average edge lengths
    const topWidth = Math.hypot(pts[1].x - pts[0].x, pts[1].y - pts[0].y);
    const bottomWidth = Math.hypot(pts[2].x - pts[3].x, pts[2].y - pts[3].y);
    const leftHeight = Math.hypot(pts[3].x - pts[0].x, pts[3].y - pts[0].y);
    const rightHeight = Math.hypot(pts[2].x - pts[1].x, pts[2].y - pts[1].y);

    const outWidth = Math.max(50, Math.round(Math.max(topWidth, bottomWidth)));
    const outHeight = Math.max(50, Math.round(Math.max(leftHeight, rightHeight)));

    canvasEl.width = outWidth;
    canvasEl.height = outHeight;

    const ctx = canvasEl.getContext('2d');
    if (!ctx) return;

    // Destination target rectangle
    const dstPts = [
      { x: 0, y: 0 },
      { x: outWidth, y: 0 },
      { x: outWidth, y: outHeight },
      { x: 0, y: outHeight }
    ];

    // Compute inverse homography (dst -> src) for inverse mapping
    const H_inv = getPerspectiveTransform(dstPts, pts);

    const srcImgData = sCtx.getImageData(0, 0, sourceCanvas.width, sourceCanvas.height);
    const dstImgData = ctx.createImageData(outWidth, outHeight);
    const srcBuf = srcImgData.data;
    const dstBuf = dstImgData.data;
    const sW = sourceCanvas.width;
    const sH = sourceCanvas.height;

    // Fast bilinear or nearest sample
    for (let dy = 0; dy < outHeight; dy++) {
      for (let dx = 0; dx < outWidth; dx++) {
        const denom = H_inv[6] * dx + H_inv[7] * dy + H_inv[8];
        const sx = (H_inv[0] * dx + H_inv[1] * dy + H_inv[2]) / denom;
        const sy = (H_inv[3] * dx + H_inv[4] * dy + H_inv[5]) / denom;

        const isx = Math.round(sx);
        const isy = Math.round(sy);

        if (isx >= 0 && isx < sW && isy >= 0 && isy < sH) {
          const sIdx = (isy * sW + isx) * 4;
          const dIdx = (dy * outWidth + dx) * 4;

          dstBuf[dIdx] = srcBuf[sIdx];
          dstBuf[dIdx + 1] = srcBuf[sIdx + 1];
          dstBuf[dIdx + 2] = srcBuf[sIdx + 2];
          dstBuf[dIdx + 3] = 255;
        }
      }
    }

    ctx.putImageData(dstImgData, 0, 0);

    const croppedUrl = canvasEl.toDataURL('image/jpeg', 0.92);
    onConfirm(croppedUrl);
  }

  // Derived polygon points string for SVG
  let polygonPoints = $derived(
    corners.map(c => `${c.x},${c.y}`).join(' ')
  );
</script>

<svelte:window
  onmousemove={(e) => handlePointerMove(e.clientX, e.clientY)}
  onmouseup={handlePointerUp}
  ontouchmove={(e) => {
    if (e.touches.length > 0) {
      handlePointerMove(e.touches[0].clientX, e.touches[0].clientY);
    }
  }}
  ontouchend={handlePointerUp}
/>

<div class="crop-modal">
  <div class="crop-header">
    <button class="header-btn" onclick={onCancel}>Cancel</button>
    <h3>Kaagaz Edge Scanner</h3>
    <button class="header-btn rotate-btn" onclick={rotateClockwise} title="Rotate 90°">🔄 Rotate</button>
  </div>

  <div class="crop-area">
    <div class="crop-container" bind:this={containerEl}>
      <img
        bind:this={imgEl}
        src={imageSrc}
        alt="To crop"
        style="transform: rotate({rotation}deg);"
      />

      <!-- SVG Overlay with non-rectangular polygon and draggable handles -->
      <svg class="crop-svg" viewBox="0 0 100 100" preserveAspectRatio="none">
        <defs>
          <mask id="polygon-hole">
            <rect width="100" height="100" fill="white" />
            <polygon points={polygonPoints} fill="black" />
          </mask>
        </defs>

        <!-- Dark outer mask with polygon cut-out -->
        <rect width="100" height="100" fill="rgba(0, 0, 0, 0.65)" mask="url(#polygon-hole)" />

        <!-- Interior draggable polygon body -->
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <polygon
          points={polygonPoints}
          class="poly-interior"
          onmousedown={(e) => {
            e.stopPropagation();
            handleTouchStartCenter(e.clientX, e.clientY);
          }}
          ontouchstart={(e) => {
            e.stopPropagation();
            if (e.touches.length > 0) {
              handleTouchStartCenter(e.touches[0].clientX, e.touches[0].clientY);
            }
          }}
        />

        <!-- Edge lines -->
        {#each [
          [corners[0], corners[1]],
          [corners[1], corners[2]],
          [corners[2], corners[3]],
          [corners[3], corners[0]]
        ] as [p1, p2]}
          <line
            x1={p1.x}
            y1={p1.y}
            x2={p2.x}
            y2={p2.y}
            stroke="#6366f1"
            stroke-width="0.8"
            stroke-dasharray="2,1"
          />
        {/each}
      </svg>

      <!-- 4 Corner Touch Handles (Free Zig-Zag positioning) -->
      {#each corners as corner, idx}
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div
          class="corner-handle"
          style="left: {corner.x}%; top: {corner.y}%;"
          onmousedown={(e) => {
            e.stopPropagation();
            handleTouchStartCorner(idx, e.clientX, e.clientY);
          }}
          ontouchstart={(e) => {
            e.stopPropagation();
            if (e.touches.length > 0) {
              handleTouchStartCorner(idx, e.touches[0].clientX, e.touches[0].clientY);
            }
          }}
        >
          <div class="handle-inner">
            <span class="corner-dot"></span>
          </div>
        </div>
      {/each}
    </div>
  </div>

  <div class="crop-footer">
    <button class="footer-btn secondary" onclick={resetFull}>
      Fit to Full
    </button>
    <button class="footer-btn primary" onclick={applyCrop}>
      ✓ Straighten & Use
    </button>
  </div>

  <canvas bind:this={canvasEl} style="display: none;"></canvas>
</div>

<style>
  .crop-modal {
    position: fixed;
    top: 0; left: 0; right: 0; bottom: 0;
    background: #0b0b0e;
    z-index: 10001;
    display: flex;
    flex-direction: column;
    color: white;
    user-select: none;
    touch-action: none;
  }

  .crop-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 16px;
    background: rgba(18, 18, 22, 0.95);
    border-bottom: 1px solid rgba(255, 255, 255, 0.1);
  }

  .crop-header h3 {
    margin: 0;
    font-size: 1rem;
    font-weight: 600;
  }

  .header-btn {
    background: transparent;
    border: none;
    color: #cbd5e1;
    font-size: 0.95rem;
    cursor: pointer;
    padding: 6px 12px;
    border-radius: 6px;
  }

  .rotate-btn {
    color: #818cf8;
    font-weight: 500;
  }

  .crop-area {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    position: relative;
    padding: 12px;
    overflow: hidden;
  }

  .crop-container {
    position: relative;
    max-width: 100%;
    max-height: 100%;
    display: inline-block;
  }

  .crop-container img {
    display: block;
    max-width: 100vw;
    max-height: 65vh;
    object-fit: contain;
    pointer-events: none;
    transition: transform 0.2s ease;
  }

  .crop-svg {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    pointer-events: none;
  }

  .poly-interior {
    fill: rgba(99, 102, 241, 0.08);
    cursor: move;
    pointer-events: auto;
  }

  .corner-handle {
    position: absolute;
    width: 44px;
    height: 44px;
    margin-left: -22px;
    margin-top: -22px;
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 20;
    cursor: pointer;
    touch-action: none;
  }

  .handle-inner {
    width: 24px;
    height: 24px;
    border-radius: 50%;
    background: #6366f1;
    border: 3px solid #ffffff;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.6);
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .corner-dot {
    width: 4px;
    height: 4px;
    background: white;
    border-radius: 50%;
  }

  .crop-footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 14px 18px;
    background: rgba(18, 18, 22, 0.95);
    border-top: 1px solid rgba(255, 255, 255, 0.1);
    padding-bottom: env(safe-area-inset-bottom, 16px);
    gap: 12px;
  }

  .footer-btn {
    flex: 1;
    padding: 14px 18px;
    border-radius: 8px;
    font-size: 0.95rem;
    font-weight: 600;
    cursor: pointer;
    border: none;
    text-align: center;
  }

  .footer-btn.secondary {
    background: #27272a;
    color: #e4e4e7;
  }

  .footer-btn.primary {
    background: #4f46e5;
    color: #ffffff;
  }

  .footer-btn.primary:active {
    background: #4338ca;
  }
</style>
