<script lang="ts">
  import { onMount } from 'svelte';

  interface StrokePoint {
    x: number;
    y: number;
  }

  interface Stroke {
    points: StrokePoint[];
    color: string;
    width: number;
  }

  let {
    color = '#000000',
    strokeWidth = 3,
    onStrokeChange = (isEmpty: boolean) => {}
  } = $props<{
    color?: string;
    strokeWidth?: number;
    onStrokeChange?: (isEmpty: boolean) => void;
  }>();

  let canvasRef: HTMLCanvasElement;
  let ctx: CanvasRenderingContext2D | null = null;
  let isDrawing = $state(false);
  let strokes = $state<Stroke[]>([]);
  let currentStrokePoints: StrokePoint[] = [];

  export function isEmpty(): boolean {
    return strokes.length === 0;
  }

  export function clear() {
    strokes = [];
    currentStrokePoints = [];
    redraw();
    onStrokeChange(true);
  }

  export function undo() {
    if (strokes.length > 0) {
      strokes = strokes.slice(0, -1);
      redraw();
      onStrokeChange(strokes.length === 0);
    }
  }

  function redraw() {
    if (!ctx || !canvasRef) return;
    ctx.clearRect(0, 0, canvasRef.width, canvasRef.height);

    for (const stroke of strokes) {
      if (stroke.points.length === 0) continue;
      ctx.strokeStyle = stroke.color;
      ctx.fillStyle = stroke.color;
      ctx.lineWidth = stroke.width;
      ctx.lineCap = 'round';
      ctx.lineJoin = 'round';

      if (stroke.points.length === 1) {
        ctx.beginPath();
        ctx.arc(stroke.points[0].x, stroke.points[0].y, stroke.width / 2, 0, Math.PI * 2);
        ctx.fill();
        continue;
      }

      ctx.beginPath();
      ctx.moveTo(stroke.points[0].x, stroke.points[0].y);

      for (let i = 1; i < stroke.points.length; i++) {
        const pPrev = stroke.points[i - 1];
        const pCurr = stroke.points[i];
        const midX = (pPrev.x + pCurr.x) / 2;
        const midY = (pPrev.y + pCurr.y) / 2;

        if (i === 1) {
          ctx.lineTo(midX, midY);
        } else {
          ctx.quadraticCurveTo(pPrev.x, pPrev.y, midX, midY);
        }
      }

      const lastPoint = stroke.points[stroke.points.length - 1];
      ctx.lineTo(lastPoint.x, lastPoint.y);
      ctx.stroke();
    }
  }

  function getCanvasCoords(e: PointerEvent): StrokePoint {
    const rect = canvasRef.getBoundingClientRect();
    const scaleX = canvasRef.width / rect.width;
    const scaleY = canvasRef.height / rect.height;
    return {
      x: (e.clientX - rect.left) * scaleX,
      y: (e.clientY - rect.top) * scaleY
    };
  }

  function handlePointerDown(e: PointerEvent) {
    if (!canvasRef) return;
    canvasRef.setPointerCapture(e.pointerId);
    isDrawing = true;
    const pt = getCanvasCoords(e);
    currentStrokePoints = [pt];

    if (ctx) {
      ctx.strokeStyle = color;
      ctx.fillStyle = color;
      ctx.lineWidth = strokeWidth;
      ctx.lineCap = 'round';
      ctx.lineJoin = 'round';
      ctx.beginPath();
      ctx.arc(pt.x, pt.y, strokeWidth / 2, 0, Math.PI * 2);
      ctx.fill();
    }
  }

  function handlePointerMove(e: PointerEvent) {
    if (!isDrawing || !ctx) return;
    const pt = getCanvasCoords(e);
    currentStrokePoints.push(pt);

    const pts = currentStrokePoints;
    if (pts.length >= 3) {
      const p0 = pts[pts.length - 3];
      const p1 = pts[pts.length - 2];
      const p2 = pts[pts.length - 1];
      const mid1 = { x: (p0.x + p1.x) / 2, y: (p0.y + p1.y) / 2 };
      const mid2 = { x: (p1.x + p2.x) / 2, y: (p1.y + p2.y) / 2 };

      ctx.strokeStyle = color;
      ctx.lineWidth = strokeWidth;
      ctx.lineCap = 'round';
      ctx.lineJoin = 'round';

      ctx.beginPath();
      ctx.moveTo(mid1.x, mid1.y);
      ctx.quadraticCurveTo(p1.x, p1.y, mid2.x, mid2.y);
      ctx.stroke();
    }
  }

  function handlePointerUp(e: PointerEvent) {
    if (!isDrawing) return;
    isDrawing = false;
    if (canvasRef && e.pointerId) {
      try {
        canvasRef.releasePointerCapture(e.pointerId);
      } catch {}
    }

    if (currentStrokePoints.length > 0) {
      strokes = [...strokes, {
        points: [...currentStrokePoints],
        color,
        width: strokeWidth
      }];
      currentStrokePoints = [];
      redraw();
      onStrokeChange(false);
    }
  }

  export function getSignatureData(): { dataUrl: string; width: number; height: number; isEmpty: boolean } {
    if (!canvasRef || strokes.length === 0) {
      return { dataUrl: '', width: 0, height: 0, isEmpty: true };
    }

    // Trim bounding box
    const w = canvasRef.width;
    const h = canvasRef.height;
    const imgData = ctx?.getImageData(0, 0, w, h);
    if (!imgData) {
      return { dataUrl: canvasRef.toDataURL('image/png'), width: w, height: h, isEmpty: false };
    }

    const data = imgData.data;
    let minX = w, minY = h, maxX = 0, maxY = 0;
    let hasPixel = false;

    for (let y = 0; y < h; y++) {
      for (let x = 0; x < w; x++) {
        const alpha = data[(y * w + x) * 4 + 3];
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
      return { dataUrl: '', width: 0, height: 0, isEmpty: true };
    }

    // Add padding
    const padding = 16;
    const cropX = Math.max(0, minX - padding);
    const cropY = Math.max(0, minY - padding);
    const cropW = Math.min(w - cropX, (maxX - minX) + padding * 2);
    const cropH = Math.min(h - cropY, (maxY - minY) + padding * 2);

    const trimmed = document.createElement('canvas');
    trimmed.width = cropW;
    trimmed.height = cropH;
    const tCtx = trimmed.getContext('2d');
    if (tCtx) {
      tCtx.drawImage(canvasRef, cropX, cropY, cropW, cropH, 0, 0, cropW, cropH);
      return {
        dataUrl: trimmed.toDataURL('image/png'),
        width: cropW,
        height: cropH,
        isEmpty: false
      };
    }

    return {
      dataUrl: canvasRef.toDataURL('image/png'),
      width: w,
      height: h,
      isEmpty: false
    };
  }

  onMount(() => {
    if (canvasRef) {
      ctx = canvasRef.getContext('2d');
      redraw();
    }
  });
</script>

<div class="pad-container">
  <div class="canvas-wrapper">
    <canvas
      bind:this={canvasRef}
      width={700}
      height={260}
      class="signature-pad-canvas"
      onpointerdown={handlePointerDown}
      onpointermove={handlePointerMove}
      onpointerup={handlePointerUp}
      onpointercancel={handlePointerUp}
      aria-label="Signature drawing pad"
    ></canvas>
    <div class="signature-baseline-guide"></div>
    <div class="signature-x-mark">✕</div>
    {#if strokes.length === 0}
      <div class="pad-placeholder">Sign here using mouse or stylus</div>
    {/if}
  </div>

  <div class="pad-toolbar">
    <div class="pad-status">
      {#if strokes.length > 0}
        <span class="stroke-count">{strokes.length} stroke{strokes.length > 1 ? 's' : ''}</span>
      {/if}
    </div>
    <div class="pad-actions">
      <button
        type="button"
        class="pad-btn"
        onclick={undo}
        disabled={strokes.length === 0}
        title="Undo last stroke"
      >
        ↩ Undo
      </button>
      <button
        type="button"
        class="pad-btn pad-btn-clear"
        onclick={clear}
        disabled={strokes.length === 0}
        title="Clear signature pad"
      >
        🗑️ Clear
      </button>
    </div>
  </div>
</div>

<style>
  .pad-container {
    display: flex;
    flex-direction: column;
    width: 100%;
    gap: 0.75rem;
  }

  .canvas-wrapper {
    position: relative;
    width: 100%;
    height: 220px;
    background: #ffffff;
    border: 2px dashed #cbd5e1;
    border-radius: 8px;
    overflow: hidden;
    touch-action: none;
    box-shadow: inset 0 2px 4px rgba(0, 0, 0, 0.04);
  }

  .canvas-wrapper:hover {
    border-color: #94a3b8;
  }

  .signature-pad-canvas {
    width: 100%;
    height: 100%;
    display: block;
    cursor: crosshair;
  }

  .signature-baseline-guide {
    position: absolute;
    bottom: 50px;
    left: 40px;
    right: 40px;
    height: 1px;
    background-color: #e2e8f0;
    pointer-events: none;
  }

  .signature-x-mark {
    position: absolute;
    bottom: 54px;
    left: 44px;
    font-size: 14px;
    color: #94a3b8;
    user-select: none;
    pointer-events: none;
  }

  .pad-placeholder {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    color: #94a3b8;
    font-size: 14px;
    pointer-events: none;
    user-select: none;
  }

  .pad-toolbar {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .pad-status {
    font-size: 12px;
    color: #64748b;
  }

  .pad-actions {
    display: flex;
    gap: 0.5rem;
  }

  .pad-btn {
    padding: 0.35rem 0.75rem;
    font-size: 13px;
    background: #f1f5f9;
    border: 1px solid #cbd5e1;
    border-radius: 6px;
    color: #334155;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .pad-btn:hover:not(:disabled) {
    background: #e2e8f0;
    color: #0f172a;
  }

  .pad-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .pad-btn-clear:hover:not(:disabled) {
    background: #fee2e2;
    border-color: #fca5a5;
    color: #b91c1c;
  }
</style>
