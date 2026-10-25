from typing import Dict, List, Any

# Define linguistic tiers for generating prompts

class TemplateCategory:
    DIRECT_IMPERATIVE = "direct_imperative"
    CONVERSATIONAL = "conversational"
    DOMAIN_LEGAL_FINANCE = "domain_legal_finance"
    TYPO_NOISE = "typo_noise"
    COMPOUND_WORKFLOW = "compound_workflow"

# Templates for single tools
# Each list contains templates with placeholders that match the tool's required/optional arguments.

SINGLE_TOOL_TEMPLATES: Dict[str, Dict[str, List[str]]] = {
    "pdf_merge": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Merge {inputs}.",
            "Combine the following files: {inputs}.",
        ],
        TemplateCategory.CONVERSATIONAL: [
            "hey, can you put {inputs} together into one file?",
            "i need to merge {inputs} please.",
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Compile exhibits {inputs} into a single consolidated production volume.",
            "Merge all appendix files {inputs} into the main briefing document.",
        ],
        TemplateCategory.TYPO_NOISE: [
            "mrege {inputs} plz",
            "combnine {inputs} into one",
        ],
    },
    "pdf_split": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Split {input} and save the pages to {output_dir}.",
            "Extract all pages from {input} into {output_dir}.",
        ],
        TemplateCategory.CONVERSATIONAL: [
            "can you break apart {input} and put the files in {output_dir}?",
            "split {input} into individual pages in {output_dir}",
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Decompile the evidence binder {input} into separate files in {output_dir}.",
            "Split the contract {input} into single pages for review, saving to {output_dir}.",
        ],
        TemplateCategory.TYPO_NOISE: [
            "splt {input} to {output_dir}",
            "extarct pages from {input} into {output_dir}",
        ],
    },
    "pdf_compress": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Compress {input} and save to {output} with {quality} quality.",
            "Reduce the file size of {input} (quality: {quality}) to {output}.",
        ],
        TemplateCategory.CONVERSATIONAL: [
            "this pdf {input} is too big for email, make it smaller at {quality} quality and save as {output}",
            "shrink {input} to {output} using {quality} compression",
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Optimize {input} for e-filing. Apply {quality} compression and save to {output}.",
            "Downsample the scanned ledger {input} to {quality} quality and export to {output}.",
        ],
        TemplateCategory.TYPO_NOISE: [
            "compres {input} to {output} q: {quality}",
            "make {input} smaler ({quality}) to {output}",
        ],
    },
    "pdf_rotate": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Rotate pages {pages} of {input} by {angle} degrees and save to {output}.",
            "Turn {input} pages {pages} by {angle} degrees, output to {output}.",
        ],
        TemplateCategory.CONVERSATIONAL: [
            "can you rotate page {pages} in {input} {angle} degrees? save it as {output}",
            "turn pages {pages} of {input} {angle} deg and write to {output}",
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Correct the orientation of appendix pages {pages} in {input} by {angle} degrees and generate {output}.",
            "Rotate the landscape schematic pages {pages} in {input} by {angle} degrees for standardized review in {output}.",
        ],
        TemplateCategory.TYPO_NOISE: [
            "rotte pages {pages} of {input} by {angle} and save {output}",
            "turn {input} pgs {pages} {angle} dgrs to {output}",
        ],
    },
    "pdf_watermark": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Apply watermark '{text}' to {input} on pages {pages} and save to {output}. Use color {color}, font size {font_size}, opacity {opacity}, rotation {rotation}, and angle {angle}.",
            "Watermark {input} with '{text}' (color: {color}, opacity: {opacity}, size: {font_size}, angle: {angle}, rotation: {rotation}) on pages {pages}. Save to {output}.",
        ],
        TemplateCategory.CONVERSATIONAL: [
            "stamp '{text}' on {input} pages {pages}. make it {color}, {font_size}pt, {opacity} opacity, angle {angle} and rot {rotation}. save as {output}",
            "can you put a watermark saying '{text}' on {input} (pages {pages}, {color}, {font_size} size, {angle} angle, {rotation} rot, {opacity} transparent)? output to {output}",
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Apply the '{text}' confidentiality stamp to {input} on pages {pages}. Specifications: {color}, {font_size}pt, opacity {opacity}, angle {angle}, rotation {rotation}. Output to {output}.",
            "Mark production document {input} pages {pages} with '{text}'. Formatting: {color}, size {font_size}, angle {angle}, rot {rotation}, {opacity} opacity. Save to {output}.",
        ],
        TemplateCategory.TYPO_NOISE: [
            "watermrak {input} with '{text}' on pgs {pages} (colr {color}, size {font_size}, ang {angle}, rot {rotation}, op {opacity}) to {output}",
            "add '{text}' to {input} pages {pages} (color {color}, {font_size} sz, {angle} ang, {rotation} rot, opac {opacity}) -> {output}",
        ],
    },
    "pdf_encrypt": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Encrypt {input} with password '{password}' and save to {output}.",
            "Protect {input} using '{password}' as the password. Save to {output}.",
        ],
        TemplateCategory.CONVERSATIONAL: [
            "lock {input} with the password '{password}' and put it in {output}",
            "can you password protect {input} with '{password}'? save as {output}",
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Secure the financial statement {input} with password '{password}' for external distribution as {output}.",
            "Encrypt sensitive HR records in {input} using passphrase '{password}' and output to {output}.",
        ],
        TemplateCategory.TYPO_NOISE: [
            "encrpyt {input} with pass '{password}' to {output}",
            "lock {input} pwd '{password}' -> {output}",
        ],
    },
    "pdf_decrypt": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Decrypt {input} using password '{password}' and save to {output}.",
            "Remove password protection from {input} with password '{password}' to {output}.",
        ],
        TemplateCategory.CONVERSATIONAL: [
            "unlock {input} (the password is '{password}') and save as {output}",
            "take the password off {input} using '{password}' -> {output}",
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Decrypt the received deposition {input} using credential '{password}' and save clear copy to {output}.",
            "Remove encryption from client intake form {input} with '{password}' and store as {output}.",
        ],
        TemplateCategory.TYPO_NOISE: [
            "decrpyt {input} pass '{password}' -> {output}",
            "unlck {input} pwd '{password}' to {output}",
        ],
    },
    "pdf_ocr": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Run OCR on {input} and save to {output}.",
            "Extract text from image PDF {input} via OCR to {output}.",
        ],
        TemplateCategory.CONVERSATIONAL: [
            "make {input} searchable and save it as {output}",
            "run ocr on {input} so i can select the text, save to {output}",
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Apply optical character recognition to the scanned contract {input} and output searchable PDF to {output}.",
            "OCR the discovery document {input} for full-text indexing, saving to {output}.",
        ],
        TemplateCategory.TYPO_NOISE: [
            "run orc on {input} to {output}",
            "make {input} searcahble -> {output}",
        ],
    },
    "pdf_bates": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Apply Bates numbering to {input} starting at {start_number} with prefix '{prefix}' and padding {padding}. Save to {output}.",
            "Bates stamp {input}: start={start_number}, prefix='{prefix}', padding={padding}, output={output}.",
        ],
        TemplateCategory.CONVERSATIONAL: [
            "can you bates stamp {input} for me? use prefix '{prefix}', start at {start_number}, with {padding} padding, and save as {output}",
            "add bates numbers to {input} ('{prefix}', start {start_number}, pad {padding}) -> {output}",
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Stamp exhibit {input} with Bates numbers (Prefix: '{prefix}', Start: {start_number}, Padding: {padding}) for court production. Save to {output}.",
            "Apply legal Bates prefix '{prefix}' starting at {start_number} (pad {padding}) to discovery file {input} and export to {output}.",
        ],
        TemplateCategory.TYPO_NOISE: [
            "bates stmp {input} (pref '{prefix}', strt {start_number}, pad {padding}) to {output}",
            "add bates {input} pref '{prefix}' start {start_number} pad {padding} -> {output}",
        ],
    },
    "pdf_extract_text": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Extract text from {input} (pages: {pages}) and save to {output}.",
            "Get raw text from {input} on pages {pages} into {output}.",
        ],
        TemplateCategory.CONVERSATIONAL: [
            "pull the text out of {input} pages {pages} and save it in {output}",
            "extract text from {input} pgs {pages} -> {output}",
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Extract the full text payload from the regulatory filing {input} (pages {pages}) for NLP analysis, saving to {output}.",
            "Scrape text content from contract {input} pages {pages} into {output}.",
        ],
        TemplateCategory.TYPO_NOISE: [
            "extarct text {input} pgs {pages} to {output}",
            "get txt from {input} (pages {pages}) -> {output}",
        ],
    },
    "pdf_extract_tables": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Extract tables from {input} to {output} in {format} format.",
            "Parse tabular data in {input}, outputting to {output} as {format}.",
        ],
        TemplateCategory.CONVERSATIONAL: [
            "grab the tables in {input} and save them as {format} in {output}",
            "extract tables from {input} into a {format} file called {output}",
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Extract financial ledgers from {input} and export to {output} formatted as {format}.",
            "Parse the cap table from {input} into a structured {format} file at {output}.",
        ],
        TemplateCategory.TYPO_NOISE: [
            "extarct tables {input} to {output} ({format})",
            "get tables {input} -> {output} in {format}",
        ],
    },
    "pdf_redact": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Redact {input} on page {page} at x:{x}, y:{y}, w:{width}, h:{height}. Save to {output}.",
            "Apply redaction to {input} (page {page}, rect: {x},{y},{width},{height}) -> {output}.",
        ],
        TemplateCategory.CONVERSATIONAL: [
            "black out the area on page {page} of {input} (x:{x}, y:{y}, w:{width}, h:{height}) and save as {output}",
            "redact {input} pg {page} at {x},{y} (size {width}x{height}) -> {output}",
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Redact sensitive client PII in {input} on page {page} at coordinates ({x}, {y}, {width}, {height}) before public disclosure. Output to {output}.",
            "Apply black box redaction to {input} page {page} (x:{x}, y:{y}, w:{width}, h:{height}) and save to {output}.",
        ],
        TemplateCategory.TYPO_NOISE: [
            "redct {input} pg {page} x{x} y{y} w{width} h{height} -> {output}",
            "blackout {input} pge {page} rect {x},{y},{width},{height} to {output}",
        ],
    },
    "pdf_sanitize": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Sanitize {input} (remove metadata: {remove_metadata}, attachments: {remove_attachments}, scripts: {remove_scripts}) and save to {output}.",
            "Clean {input} -> {output} (meta: {remove_metadata}, attach: {remove_attachments}, scripts: {remove_scripts}).",
        ],
        TemplateCategory.CONVERSATIONAL: [
            "strip {input} of private stuff (metadata {remove_metadata}, attachments {remove_attachments}, scripts {remove_scripts}) -> {output}",
            "sanitize {input} and save to {output}. (remove meta: {remove_metadata}, attach: {remove_attachments}, scripts: {remove_scripts})",
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Deep sanitize {input} prior to opposing counsel production (metadata: {remove_metadata}, attachments: {remove_attachments}, scripts: {remove_scripts}). Save as {output}.",
            "Scrub {input} for compliance (meta={remove_metadata}, attach={remove_attachments}, scripts={remove_scripts}) and output to {output}.",
        ],
        TemplateCategory.TYPO_NOISE: [
            "sanitze {input} to {output} (meta {remove_metadata}, att {remove_attachments}, scrpt {remove_scripts})",
            "clean {input} (meta {remove_metadata}, att {remove_attachments}, scrpt {remove_scripts}) -> {output}",
        ],
    },
    "pdf_auto_rotate": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Auto-rotate {input} and save to {output}.",
            "Automatically upright pages in {input} -> {output}.",
        ],
        TemplateCategory.CONVERSATIONAL: [
            "fix the sideways pages in {input} automatically and save as {output}",
            "auto rotate {input} to {output}",
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Run automatic orientation correction on the scanned exhibit {input} and output to {output}.",
            "Ensure all pages in {input} are upright via auto-rotation, save to {output}.",
        ],
        TemplateCategory.TYPO_NOISE: [
            "auto rotte {input} -> {output}",
            "fix orientation {input} to {output}",
        ],
    },
    "document_pii_scan": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Scan {input} for PII (SSN: {scan_ssn}, CC: {scan_credit_cards}, Email: {scan_emails}, Phone: {scan_phones}, IBAN: {scan_iban}) and save report to {output}.",
            "Run PII scan on {input} (ssn={scan_ssn}, cc={scan_credit_cards}, emails={scan_emails}, phones={scan_phones}, iban={scan_iban}) -> {output}.",
        ],
        TemplateCategory.CONVERSATIONAL: [
            "check {input} for personal info (ssn {scan_ssn}, credit cards {scan_credit_cards}, emails {scan_emails}, phones {scan_phones}, iban {scan_iban}) and save to {output}",
            "scan {input} for pii (ssn:{scan_ssn}, cc:{scan_credit_cards}, email:{scan_emails}, phone:{scan_phones}, iban:{scan_iban}) -> {output}",
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Audit {input} for protected PII (SSN: {scan_ssn}, PAN: {scan_credit_cards}, Email: {scan_emails}, Phone: {scan_phones}, IBAN: {scan_iban}) prior to data room upload. Output {output}.",
            "Perform compliance scan on {input} (ssn={scan_ssn}, cc={scan_credit_cards}, emails={scan_emails}, phones={scan_phones}, iban={scan_iban}) -> {output}.",
        ],
        TemplateCategory.TYPO_NOISE: [
            "scan pii {input} (ssn {scan_ssn}, cc {scan_credit_cards}, eml {scan_emails}, phn {scan_phones}, ibn {scan_iban}) -> {output}",
            "check {input} for pii (ssn {scan_ssn}, cc {scan_credit_cards}, emails {scan_emails}, phones {scan_phones}, iban {scan_iban}) -> {output}",
        ],
    },
}

# Compound workflows define a sequence of tools to run.
# The template string should just broadly describe the intent of the combined steps.
# The generator will supply the required intermediate/final args when generating the JSON array.

COMPOUND_WORKFLOW_TEMPLATES = [
    {
        "tools": ["pdf_auto_rotate", "pdf_sanitize", "pdf_ocr"],
        "templates": [
            "Clean this scan {input}, upright sideways pages, run OCR, and save to {output}.",
            "Prep the scanned document {input} by fixing orientation, stripping metadata, and making it searchable. Final output {output}.",
        ]
    },
    {
        "tools": ["pdf_bates", "pdf_watermark", "document_pii_scan"],
        "templates": [
            "Prepare {input} for legal production: Bates stamp it, add a confidential watermark, scan for PII, and save to {output}.",
            "Run legal processing on {input}: apply Bates numbers, watermark 'CONFIDENTIAL', check for PII, output to {output}."
        ]
    },
    {
        "tools": ["pdf_merge", "pdf_compress"],
        "templates": [
            "Merge {inputs} and compress the final file to {output}.",
            "Combine {inputs} into a single small file at {output}."
        ]
    }
]
