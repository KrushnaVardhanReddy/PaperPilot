<script lang="ts">
    import { appState } from '$lib/state/app.svelte';

    // The raw plan from NLP
    export let plan: any;
    // Callback when user clicks execute
    export let onExecute: (args: Record<string, any>) => void;

    let activeDoc = appState.selectedDocumentIndex !== null ? appState.documents[appState.selectedDocumentIndex] : null;
    let totalPages = 1; // Default fallback
    // We would ideally fetch the true page count if needed, but for the badge we can just show activeDoc exists
    let maxPagesDisplay = "Active Document";

    // Extracted state for editable UI
    let pagesMode = 'all'; // 'all', 'current', 'custom'
    let customPages = '';

    let angle = 90;
    let textInput = '';
    let watermarkOrientation = 'diagonal';
    let password = '';
    let regions = '';
    let cropBox = '';
    let batesPrefix = '';
    let batesStart = 1;
    let batesPadding = 6;

    let outputPath = '';
    let isExecuting = false;

    // Initialize state from plan
    if (plan) {
        if (plan.page_ranges && plan.page_ranges.length > 0) {
            pagesMode = 'custom';
            customPages = plan.page_ranges.join(', ');
        }
        if (plan.angles && plan.angles.length > 0) {
            angle = plan.angles[0];
        }
        if (plan.passwords && plan.passwords.length > 0) {
            password = plan.passwords[0];
        }
        if (plan.output_file) {
            outputPath = plan.output_file;
        }
    }

    function execute() {
        // Collect args based on intent and edited state
        isExecuting = true;
        const args: Record<string, any> = { intent: plan.intent };

        // Resolve input files to real local filesystem paths
        const resolveToLocalPath = (fileNameOrPath: string): string => {
            const idx = appState.documents.findIndex(d => d.name === fileNameOrPath || (d as any)._localPath === fileNameOrPath);
            if (idx !== -1 && appState.documentPaths[idx]) {
                return appState.documentPaths[idx];
            }
            if (idx !== -1 && (appState.documents[idx] as any)._localPath) {
                return (appState.documents[idx] as any)._localPath;
            }
            return fileNameOrPath;
        };

        if (plan.input_files && plan.input_files.length > 0) {
            args.input = resolveToLocalPath(plan.input_files[0]);
            args.inputs = plan.input_files.map(resolveToLocalPath); // For merge
        } else if (activeDoc) {
            const activeIdx = appState.selectedDocumentIndex ?? 0;
            const path = appState.documentPaths[activeIdx] || (activeDoc as any)._localPath || activeDoc.name;
            args.input = path;
            args.inputs = [path];
        }

        // Output file extension mapping based on intent
        let ext = '.pdf';
        if (['ExtractText'].includes(plan.intent)) ext = '.txt';
        else if (['ToJson'].includes(plan.intent)) ext = '.json';
        else if (['ToMarkdown'].includes(plan.intent)) ext = '.md';
        else if (['ToHtml'].includes(plan.intent)) ext = '.html';
        else if (['ToDocx'].includes(plan.intent)) ext = '.docx';
        else if (['ToXlsx'].includes(plan.intent)) ext = '.xlsx';
        else if (['ToPptx'].includes(plan.intent)) ext = '.pptx';

        const defaultOutput = args.input ? args.input.replace(/\.pdf$/i, `_${plan.intent.toLowerCase()}${ext}`) : `output${ext}`;
        const defaultDir = args.input ? args.input.replace(/\.pdf$/i, `_${plan.intent.toLowerCase()}`) : 'output_dir';

        if (outputPath) {
            args.output = outputPath;
            args.output_dir = outputPath;
            if (plan.intent === 'Split') {
                args.output_pattern = outputPath;
            }
        } else {
            args.output = defaultOutput;
            if (['Burst', 'ExtractImages', 'Split'].includes(plan.intent)) {
                args.output_dir = defaultDir;
            }
            if (plan.intent === 'Split') {
                args.output_pattern = args.input ? args.input.replace(/\.pdf$/i, '_p%d.pdf') : 'output_%d.pdf';
            }
        }

        // Pages
        if (pagesMode === 'all') {
            args.pages = 'all';
        } else if (pagesMode === 'current') {
            args.pages = '1';
        } else {
            args.pages = customPages || 'all';
        }

        // Operation specific args
        if (['Rotate'].includes(plan.intent)) args.angle = angle;
        if (['Split', 'Extract'].includes(plan.intent)) args.ranges = customPages || '1-2';
        if (['Watermark'].includes(plan.intent)) {
            args.text = textInput;
            args.angle = watermarkOrientation === 'diagonal' ? 45 : 0;
            args.opacity = 0.2;
        }
        if (['HeaderFooter'].includes(plan.intent)) {
            args.header_left = textInput; // Just map it to header_left for simplicity
            args.footer_center = "";
        }
        if (['Encrypt', 'Decrypt'].includes(plan.intent)) {
            args.password = password || 'secure123';
            args.user_password = password || 'secure123';
        }
        if (plan.intent === 'Redact') args.regions = regions;
        if (plan.intent === 'Crop') args.box = cropBox;
        if (plan.intent === 'Bates') {
            args.prefix = batesPrefix;
            args.start_number = batesStart;
            args.padding = batesPadding;
        }

        onExecute(args);
        isExecuting = false;
    }
</script>

<div class="plan-card">
    <h4>{plan.intent} Operation</h4>
    <div class="plan-details form">

        <!-- Pages target for operations that typically require it -->
        {#if ['Rotate', 'Delete', 'Extract', 'Split'].includes(plan.intent)}
            <div class="field">
                <label>
                    Pages Target:
                    <span class="badge">{maxPagesDisplay}</span>
                </label>
                <div class="segmented">
                    <button class={pagesMode === 'all' ? 'active' : ''} onclick={() => pagesMode = 'all'}>All Pages</button>
                    <button class={pagesMode === 'current' ? 'active' : ''} onclick={() => pagesMode = 'current'}>Current Page</button>
                    <button class={pagesMode === 'custom' ? 'active' : ''} onclick={() => pagesMode = 'custom'}>Custom Pages</button>
                </div>
                {#if pagesMode === 'custom'}
                    <input type="text" id="custom-pages-input" bind:value={customPages} placeholder="e.g. '1, 4' or '1-5'" />
                    <small class="hint">e.g. '1, 4' or range '1-5' (or 'all' for entire document)</small>
                {/if}
            </div>
        {/if}

        {#if plan.intent === 'Rotate'}
            <div class="field">
                <label>Angle:</label>
                <div class="segmented">
                    <button class={angle === 90 ? 'active' : ''} onclick={() => angle = 90}>90°</button>
                    <button class={angle === 180 ? 'active' : ''} onclick={() => angle = 180}>180°</button>
                    <button class={angle === 270 ? 'active' : ''} onclick={() => angle = 270}>270°</button>
                </div>
                <small class="hint">Clockwise rotation angle in degrees</small>
            </div>
        {/if}

        {#if ['Watermark', 'HeaderFooter'].includes(plan.intent)}
            <div class="field">
                <label>Text / Content:</label>
                <input type="text" id="text-input" bind:value={textInput} placeholder="Enter text..." />
                <small class="hint">e.g. 'CONFIDENTIAL', 'DRAFT', or company name</small>
            </div>
        {/if}
        {#if plan.intent === 'Watermark'}
            <div class="field">
                <label>Orientation:</label>
                <div class="segmented">
                    <button class={watermarkOrientation === 'diagonal' ? 'active' : ''} onclick={() => watermarkOrientation = 'diagonal'}>Diagonal (45°)</button>
                    <button class={watermarkOrientation === 'horizontal' ? 'active' : ''} onclick={() => watermarkOrientation = 'horizontal'}>Horizontal (0°)</button>
                </div>
            </div>
        {/if}

        {#if ['Encrypt', 'Decrypt'].includes(plan.intent)}
            <div class="field">
                <label>Password:</label>
                <input type="password" id="password-input" bind:value={password} placeholder="Enter password..." />
                <small class="hint">e.g. Minimum 6 characters (AES-256 protected)</small>
            </div>
        {/if}

        {#if plan.intent === 'Crop'}
            <div class="field">
                <label>Crop Box:</label>
                <input type="text" id="cropbox-input" bind:value={cropBox} placeholder="x,y,width,height" />
                <small class="hint">Format: x,y,width,height e.g. '50,50,400,600'</small>
            </div>
        {/if}

        {#if plan.intent === 'Bates'}
            <div class="field">
                <label>Bates Prefix:</label>
                <input type="text" id="bates-prefix-input" bind:value={batesPrefix} placeholder="CASE-2024-" />
                <label>Start & Padding:</label>
                <div class="flex-row">
                    <input type="number" id="bates-start-input" bind:value={batesStart} placeholder="1" />
                    <input type="number" id="bates-padding-input" bind:value={batesPadding} placeholder="6" />
                </div>
                <small class="hint">Prefix e.g. 'CASE-2024-' | Start number e.g. '001' | Padding: 6</small>
            </div>
        {/if}

        {#if ['ToDocx', 'ToXlsx', 'ToPptx', 'ToMarkdown', 'ToHtml', 'PdfA'].includes(plan.intent)}
            <div class="field conversion-indicator">
                <label>Conversion Target:</label>
                <div class="badge format-badge">
                    {#if plan.intent === 'ToDocx'}DOCX{/if}
                    {#if plan.intent === 'ToXlsx'}XLSX{/if}
                    {#if plan.intent === 'ToPptx'}PPTX{/if}
                    {#if plan.intent === 'ToMarkdown'}MARKDOWN{/if}
                    {#if plan.intent === 'ToHtml'}HTML{/if}
                    {#if plan.intent === 'PdfA'}PDF/A{/if}
                </div>
            </div>
        {/if}

        <div class="field">
            <label>Output Path (optional):</label>
            <input type="text" id="output-path-input" bind:value={outputPath} placeholder="/path/to/output.pdf" />
            <small class="hint">Destination file path (e.g. /path/to/doc_rotated.pdf)</small>
        </div>

    </div>
    <button class="execute-btn" onclick={execute} disabled={isExecuting}>
        {isExecuting ? 'Executing...' : '⚡ Execute Action'}
    </button>
</div>

<style>
    .plan-card {
        margin-top: 0.5rem;
        background: var(--bg-surface);
        border: 1px solid var(--border-color);
        border-radius: 8px;
        padding: 1rem;
        font-size: 0.85rem;
    }
    h4 {
        margin: 0 0 0.5rem 0;
        color: var(--accent-primary);
    }
    .field {
        margin-bottom: 0.75rem;
        display: flex;
        flex-direction: column;
        gap: 0.25rem;
    }
    .field label {
        font-weight: bold;
        color: var(--text-primary);
        display: flex;
        justify-content: space-between;
        align-items: center;
    }
    .badge {
        background: var(--bg-secondary);
        color: var(--text-muted);
        padding: 0.15rem 0.4rem;
        border-radius: 4px;
        font-size: 0.7rem;
        font-weight: normal;
    }
    .format-badge {
        background: var(--accent-primary);
        color: white;
        display: inline-block;
        width: fit-content;
        font-weight: bold;
    }
    .segmented {
        display: flex;
        gap: 0.25rem;
        margin-bottom: 0.25rem;
    }
    .segmented button {
        background: var(--bg-primary);
        border: 1px solid var(--border-color);
        padding: 0.25rem 0.5rem;
        border-radius: 4px;
        color: var(--text-secondary);
        cursor: pointer;
    }
    .segmented button.active {
        background: var(--accent-primary);
        color: white;
        border-color: var(--accent-primary);
    }
    input {
        padding: 0.5rem;
        border: 1px solid var(--border-color);
        border-radius: 4px;
        background: var(--bg-primary);
        color: var(--text-primary);
    }
    .flex-row {
        display: flex;
        gap: 0.5rem;
    }
    .flex-row input {
        flex: 1;
    }
    .hint {
        color: var(--text-muted);
        font-size: 0.75rem;
    }
    .execute-btn {
        width: 100%;
        background: var(--accent-primary);
        color: white;
        border: none;
        border-radius: 4px;
        padding: 0.5rem;
        cursor: pointer;
        font-weight: bold;
        transition: opacity 0.2s;
    }
    .execute-btn:hover:not(:disabled) {
        opacity: 0.9;
    }
    .execute-btn:disabled {
        opacity: 0.5;
        cursor: not-allowed;
    }
</style>
