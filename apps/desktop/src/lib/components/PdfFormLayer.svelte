<script lang="ts">
  import * as pdfjsLib from 'pdfjs-dist';

  let { pdfDoc, pageNum, scale, formValues = $bindable({}), hasFormFields = $bindable(false) } = $props();
  let fields: any[] = $state([]);

  $effect(() => {
    async function fetchFields() {
      if (!pdfDoc) return;
      try {
        const page = await pdfDoc.getPage(pageNum);
        const annotations = await page.getAnnotations();
        const viewport = page.getViewport({ scale: 1.0 });

        // Filter for AcroForm Widgets
        fields = annotations.filter((a: any) => a.subtype === 'Widget');
        hasFormFields = fields.length > 0;
        fields = fields.map((a: any) => {
            if (formValues[a.fieldName] === undefined && a.fieldValue !== undefined) {
               formValues[a.fieldName] = a.fieldValue;
            }

            const [x1, y1, x2, y2] = a.rect;
            const rect = pdfjsLib.Util.normalizeRect([x1, y1, x2, y2]);
            const p1 = viewport.convertToViewportPoint(rect[0], rect[1]);
            const p2 = viewport.convertToViewportPoint(rect[2], rect[3]);

            const left = Math.min(p1[0], p2[0]);
            const top = Math.min(p1[1], p2[1]);
            const width = Math.abs(p2[0] - p1[0]);
            const height = Math.abs(p2[1] - p1[1]);

            return { ...a, style: `left: ${left}px; top: ${top}px; width: ${width}px; height: ${height}px;` };
        });
      } catch (e) {
        console.error('Failed to load form fields', e);
      }
    }
    fetchFields();
  });
</script>

<div class="form-layer" style="transform: scale({scale}); transform-origin: top left;">
  {#each fields as field}
    {#if field.fieldType === 'Tx'}
      <input
        type="text"
        class="pdf-input"
        style={field.style}
        bind:value={formValues[field.fieldName]}
        placeholder={field.alternativeText || ''}
      />
    {:else if field.fieldType === 'Btn'}
      {#if field.checkBox || field.radioButton === undefined}
          <input
            type="checkbox"
            class="pdf-checkbox"
            style={field.style}
            bind:checked={formValues[field.fieldName]}
          />
      {:else}
          <input
            type="radio"
            class="pdf-checkbox"
            style={field.style}
            value={field.buttonValue || 'On'}
            bind:group={formValues[field.fieldName]}
          />
      {/if}
    {:else if field.fieldType === 'Ch'}
      <select class="pdf-select" style={field.style} bind:value={formValues[field.fieldName]}>
        {#each field.options || [] as option}
          <option value={option.exportValue}>{option.displayValue}</option>
        {/each}
      </select>
    {/if}
  {/each}
</div>

<style>
  .form-layer {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    pointer-events: none;
  }
  .pdf-input, .pdf-select, .pdf-checkbox {
    position: absolute;
    background: rgba(238, 242, 255, 0.4);
    border: 1px solid #c7d2fe;
    font-family: inherit;
    padding: 2px;
    pointer-events: auto;
  }
  .pdf-input:focus, .pdf-select:focus, .pdf-checkbox:focus {
    background: rgba(255, 255, 255, 0.9);
    border-color: #4f46e5;
    outline: none;
  }
</style>
