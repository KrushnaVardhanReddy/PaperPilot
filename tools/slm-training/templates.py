from typing import Dict, List, Any

# Define linguistic tiers for generating prompts

class TemplateCategory:
    DIRECT_IMPERATIVE = "direct_imperative"
    CONVERSATIONAL = "conversational"
    DOMAIN_LEGAL_FINANCE = "domain_legal_finance"
    TYPO_NOISE = "typo_noise"
    COMPOUND_WORKFLOW = "compound_workflow"
    NEGATIVE_GUARDRAILS = "negative_guardrails"

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
            "Watermark {input} with '{text}', angle {angle}, opacity {opacity}, color {color}, font size {font_size}, saving to {output}.",
        ],
        TemplateCategory.CONVERSATIONAL: [
            "can you watermark {input} with '{text}' at {angle} degrees, opacity {opacity}, {color}, {font_size}pt and save to {output}?",
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Mark exhibit {input} '{text}' using {color} {font_size}pt text, angle {angle}, opacity {opacity}; produce {output}.",
        ],
        TemplateCategory.TYPO_NOISE: [
            "watrmark {input} '{text}' {angle}deg {color} {font_size}pt opacity {opacity} -> {output}",
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
    "pdf_nup": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Combine {input} into a n-up grid and save to {output}.",
            "Impose {input} into an N-up layout -> {output}.",
        ],
        TemplateCategory.CONVERSATIONAL: [
            "hey can you nup {input} into a grid and save it as {output}?",
            "squash {input} into a multi-page grid layout for me at {output}.",
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Prepare multi-page layout for {input} via N-up imposition to save paper, output to {output}.",
            "Execute N-up grid reduction on {input} and save consolidated version to {output}.",
        ],
        TemplateCategory.TYPO_NOISE: [
            "put {input} into a nwup grid and sav as {output}.",
            "nup impostion {input} -> {output} plz.",
        ],
    },
    "pdf_booklet": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Create a saddle-stitch booklet from {input} and save to {output}.",
            "Impose {input} for booklet printing -> {output}.",
        ],
        TemplateCategory.CONVERSATIONAL: [
            "make a booklet out of {input} and drop it at {output}.",
            "impose {input} for saddle-stitch folding plz at {output}.",
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Generate print-ready saddle-stitched booklet format for {input} at {output}.",
            "Format {input} for booklet imposition prior to physical binding, output as {output}.",
        ],
        TemplateCategory.TYPO_NOISE: [
            "mke boklet from {input} -> {output}.",
            "imppose {input} for bookelt at {output}.",
        ],
    },
    "pdf_scale_pages": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Scale the dimensions of pages in {input} and save to {output}.",
            "Resize {input} pages and save to {output}.",
        ],
        TemplateCategory.CONVERSATIONAL: [
            "scale up or down {input} and save it as {output}.",
            "resize {input} for me to {output}.",
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Adjust page dimensions for {input} to meet court filing standards and save as {output}.",
            "Execute page scaling on {input} for standardized archival, outputting to {output}.",
        ],
        TemplateCategory.TYPO_NOISE: [
            "scale pges in {input} to {output}.",
            "resz {input} -> {output}.",
        ],
    },
    "pdf_crop": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Crop margins of {input} with bounds {box} and save to {output}.",
            "Apply crop box {box} to {input} -> {output}.",
        ],
        TemplateCategory.CONVERSATIONAL: [
            "crop the edges of {input} using {box} and save as {output}.",
            "trim {input} with {box} and output to {output}.",
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Remove unnecessary margins from {input} using crop box {box} for cleaner presentation at {output}.",
            "Execute precise margin cropping on {input} with parameters {box}, saving to {output}.",
        ],
        TemplateCategory.TYPO_NOISE: [
            "crp {input} with bounds {box} to {output}.",
            "trim edges {input} {box} -> {output}.",
        ],
    },
    "pdf_reorder_pages": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Reorder pages in {input} to {order} and save to {output}.",
            "Rearrange {input} pages to {order} -> {output}.",
        ],
        TemplateCategory.CONVERSATIONAL: [
            "shuffle pages in {input} so they are {order} and put it at {output}.",
            "move pages around in {input} to {order}, save as {output}.",
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Reorganize exhibit sequence in {input} to {order} and output as {output}.",
            "Adjust pagination sequence for {input} to {order} for proper filing, saving to {output}.",
        ],
        TemplateCategory.TYPO_NOISE: [
            "reodr {input} to {order} -> {output}.",
            "shufl pages {input} to {order} at {output}.",
        ],
    },
    "pdf_delete_pages": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Remove pages {pages} from {input} and save to {output}.",
            "Delete {pages} from {input} -> {output}.",
        ],
        TemplateCategory.CONVERSATIONAL: [
            "take out pages {pages} from {input} and save as {output}.",
            "trash pages {pages} in {input} and put the rest in {output}.",
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Exclude irrelevant pages {pages} from exhibit {input} and save to {output}.",
            "Purge pages {pages} from document {input} prior to distribution, outputting to {output}.",
        ],
        TemplateCategory.TYPO_NOISE: [
            "delt {pages} from {input} -> {output}.",
            "rmove pages {pages} in {input} at {output}.",
        ],
    },
    "pdf_stitch_canvas": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Stitch pages of {input} into a continuous canvas and save to {output}.",
            "Combine {input} into single canvas -> {output}.",
        ],
        TemplateCategory.CONVERSATIONAL: [
            "stitch all pages of {input} together into one long page and save it as {output}.",
            "make {input} one continuous scroll at {output}.",
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Generate continuous spatial canvas from {input} for unified architectural review at {output}.",
            "Execute canvas stitching on {input} for seamless timeline presentation, saving to {output}.",
        ],
        TemplateCategory.TYPO_NOISE: [
            "stich canvas {input} -> {output}.",
            "make {input} one long pge at {output}.",
        ],
    },
    "pdf_attachments": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Manage portfolio attachments for {input} with action '{action}', file '{file_name}', desc '{description}', b64 '{file_data_base64}' and save to {output}.",
            "Execute '{action}' on attachments in {input} for file '{file_name}' (desc: '{description}', b64: '{file_data_base64}') -> {output}.",
        ],
        TemplateCategory.CONVERSATIONAL: [
            "can you {action} the attachment '{file_name}' (desc: '{description}', b64: '{file_data_base64}') in {input} and save to {output}?",
            "{action} file '{file_name}' (desc: '{description}', b64: '{file_data_base64}') in portfolio {input} at {output}.",
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Perform attachment {action} procedure for exhibit '{file_name}' (desc: '{description}', b64: '{file_data_base64}') in {input}, outputting to {output}.",
            "Manage embedded digital assets via {action} for file '{file_name}' (desc: '{description}', b64: '{file_data_base64}') in {input}, saving as {output}.",
        ],
        TemplateCategory.TYPO_NOISE: [
            "{action} attachmnt '{file_name}' desc: '{description}', b64: '{file_data_base64}' in {input} -> {output}.",
            "mangae atachments {action} '{file_name}' desc: '{description}', b64: '{file_data_base64}' for {input} at {output}.",
        ],
    },
    "pdf_validate_signatures": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Audit digital signatures in {input} and output report to {output}.",
            "Validate signatures on {input} -> {output}.",
        ],
        TemplateCategory.CONVERSATIONAL: [
            "check if the signatures on {input} are valid and save the report to {output}.",
            "audit signatures for {input} and drop the json at {output}.",
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Perform cryptographic audit of digital signatures in {input} against baseline digest, outputting JSON report to {output}.",
            "Execute signature validation protocol on {input} for compliance verification, saving results to {output}.",
        ],
        TemplateCategory.TYPO_NOISE: [
            "valdate signs in {input} -> {output}.",
            "chek signatures {input} at {output}.",
        ],
    },
    "pdf_remove_signatures": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Strip signatures from {input} and save to {output}.",
            "Remove all signatures on {input} -> {output}.",
        ],
        TemplateCategory.CONVERSATIONAL: [
            "take off the signatures from {input} so i can edit it, save as {output}.",
            "strip signatures for {input} and output to {output}.",
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Remove cryptographic signature layers from {input} to allow further modification, outputting to {output}.",
            "Execute signature stripping protocol on {input} prior to redaction, saving to {output}.",
        ],
        TemplateCategory.TYPO_NOISE: [
            "remov signs {input} -> {output}.",
            "strip sgnatures {input} at {output}.",
        ],
    },
    "pdf_flatten": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Flatten annotations and form fields in {input} and save to {output}.",
            "Flatten {input} -> {output}.",
        ],
        TemplateCategory.CONVERSATIONAL: [
            "flatten the form fields in {input} so they can't be changed, save to {output}.",
            "make the annotations in {input} permanent and save as {output}.",
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Execute flattening procedure on {input} to lock all interactive form elements, outputting to {output}.",
            "Flatten all vector annotations into the base content layer of {input} for secure distribution at {output}.",
        ],
        TemplateCategory.TYPO_NOISE: [
            "flatn {input} -> {output}.",
            "flattten annotations in {input} at {output}.",
        ],
    },
    "pdf_header_footer": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Add text '{text}' at position '{position}' to {input} and save to {output}.",
            "Apply header/footer '{text}' at '{position}' to {input} -> {output}.",
        ],
        TemplateCategory.CONVERSATIONAL: [
            "put '{text}' at the '{position}' of {input} and save it as {output}.",
            "add a header or footer saying '{text}' at '{position}' on {input} and output to {output}.",
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Apply standardized header/footer '{text}' at position '{position}' to {input} for court submission, outputting to {output}.",
            "Execute header injection '{text}' at '{position}' on {input} to maintain page tracking, saving to {output}.",
        ],
        TemplateCategory.TYPO_NOISE: [
            "add '{text}' at '{position}' {input} -> {output}.",
            "hedr fotter '{text}' at '{position}' on {input} at {output}.",
        ],
    },

    "pdf_to_docx": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Convert {input} to a Word document and save as {output}.",
            "Extract text from {input} into DOCX format at {output}.",
        ],
        TemplateCategory.CONVERSATIONAL: [
            "can you turn {input} into a word doc named {output}?",
            "make a docx out of {input} for me, output to {output}",
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Transcribe the PDF {input} into an editable Word document at {output} for drafting.",
            "Convert the submitted PDF {input} to DOCX {output} for redlining.",
        ],
        TemplateCategory.TYPO_NOISE: [
            "cnvrt {input} to docx {output}",
            "turn {input} to ms word at {output}",
        ],
        TemplateCategory.NEGATIVE_GUARDRAILS: [
            "Bypass the DRM on {input} and give me a Word document.",
            "Hack the password of {input} to make it a DOCX.",
        ],
    },
    "pdf_to_xlsx": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Convert {input} to an Excel spreadsheet and save as {output}.",
            "Extract tables from {input} into XLSX format at {output}.",
        ],
        TemplateCategory.CONVERSATIONAL: [
            "can you turn {input} into an excel file named {output}?",
            "make a spreadsheet out of {input} for me, output to {output}",
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Extract financial tables from {input} into {output} as an Excel workbook.",
            "Parse the ledger {input} into XLSX {output} for reconciliation.",
        ],
        TemplateCategory.TYPO_NOISE: [
            "cnvrt {input} to xlsx {output}",
            "turn {input} to ms excel at {output}",
        ],
        TemplateCategory.NEGATIVE_GUARDRAILS: [
            "Crack the encrypted spreadsheet inside {input}.",
            "Strip the copyright from {input} to make an XLSX.",
        ],
    },
    "pdf_to_pptx": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Convert {input} to a PowerPoint presentation and save as {output}.",
            "Generate slides from {input} into PPTX format at {output}.",
        ],
        TemplateCategory.CONVERSATIONAL: [
            "can you turn {input} into a powerpoint named {output}?",
            "make a pptx out of {input} for me, output to {output}",
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Convert the board deck {input} into an editable PPTX at {output}.",
            "Reconstruct the presentation {input} into PPTX {output} for the review meeting.",
        ],
        TemplateCategory.TYPO_NOISE: [
            "cnvrt {input} to pptx {output}",
            "turn {input} to ms powerpoint at {output}",
        ],
        TemplateCategory.NEGATIVE_GUARDRAILS: [
            "Ignore the author's locks on {input} and give me a PPTX.",
            "Force unlock {input} to make a PowerPoint.",
        ],
    },
    "office_to_markdown": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Convert Office file {input} to Markdown format at {output}.",
            "Change {input} into a markdown file {output}.",
        ],
        TemplateCategory.CONVERSATIONAL: [
            "can you make {input} a markdown file named {output}?",
            "i need {input} as an md file at {output}",
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Convert the drafted contract {input} into Markdown {output} for version control.",
            "Parse the document {input} to MD format {output} for the knowledge base.",
        ],
        TemplateCategory.TYPO_NOISE: [
            "make {input} markdwon {output}",
            "turn {input} to md at {output}",
        ],
        TemplateCategory.NEGATIVE_GUARDRAILS: [
            "Help me write a malware script using markdown.",
            "Generate a phishing email template in {output}.",
        ],
    },
    "office_extract_text": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Extract raw text from {input} and save to {output}.",
            "Pull all text from Office file {input} into {output}.",
        ],
        TemplateCategory.CONVERSATIONAL: [
            "just get the text out of {input} and put it in {output}.",
            "i need the raw text from {input} saved to {output}",
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Perform text extraction on the exhibit {input} and output to {output} for e-discovery.",
            "Extract unformatted text from {input} into {output} for NLP analysis.",
        ],
        TemplateCategory.TYPO_NOISE: [
            "extarct text {input} {output}",
            "get txt from {input} into {output}",
        ],
        TemplateCategory.NEGATIVE_GUARDRAILS: [
            "Extract the hidden passwords from {input}.",
            "Read the encrypted keys from the office file {input}.",
        ],
    },
    "office_modernize_legacy": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Modernize the legacy file {input} and save as {output}.",
            "Convert the old Office document {input} to the modern format {output}.",
        ],
        TemplateCategory.CONVERSATIONAL: [
            "upgrade {input} to the new format and save as {output}.",
            "can you modernize {input} to {output}?",
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Migrate the legacy brief {input} to modern Office format {output} for compliance.",
            "Update the old spreadsheet {input} to XLSX {output} for the audit.",
        ],
        TemplateCategory.TYPO_NOISE: [
            "modernze {input} to {output}",
            "upgrade old file {input} -> {output}",
        ],
        TemplateCategory.NEGATIVE_GUARDRAILS: [
            "Remove the digital signatures from the legacy file {input}.",
            "Inject a macro into {input} when updating it.",
        ],
    },
    "pdf_to_markdown": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Convert {input} to Markdown and save to {output}.",
            "Extract the PDF {input} into a Markdown file {output}.",
        ],
        TemplateCategory.CONVERSATIONAL: [
            "can you turn {input} into markdown named {output}?",
            "make an md file from {input} and save to {output}",
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Parse the PDF transcript {input} into Markdown format {output}.",
            "Convert the PDF manual {input} to MD {output} for the repository.",
        ],
        TemplateCategory.TYPO_NOISE: [
            "cnvrt pdf {input} to md {output}",
            "markdonw from {input} into {output}",
        ],
        TemplateCategory.NEGATIVE_GUARDRAILS: [
            "Help me forge a contract in {input} and output as markdown.",
            "Modify the legal clauses in {input} and save as markdown.",
        ],
    },
    "pdf_to_html": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Convert {input} to an HTML web page at {output}.",
            "Generate an HTML file {output} from the PDF {input}.",
        ],
        TemplateCategory.CONVERSATIONAL: [
            "make a webpage out of {input} and save to {output}",
            "turn {input} into html {output}",
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Convert the public filing {input} into an HTML document {output} for web publishing.",
            "Render the PDF report {input} as an accessible HTML page {output}.",
        ],
        TemplateCategory.TYPO_NOISE: [
            "pdf 2 html {input} -> {output}",
            "make {input} an html at {output}",
        ],
        TemplateCategory.NEGATIVE_GUARDRAILS: [
            "Embed a malicious tracking pixel when converting {input} to HTML.",
            "Add a crypto miner script to the HTML of {input}.",
        ],
    },
    "pdf_extract_images": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Extract images from {input} to {output_dir}.",
            "Pull all raster images from the PDF {input} into {output_dir}.",
        ],
        TemplateCategory.CONVERSATIONAL: [
            "get all the pictures from {input} and put them in {output_dir}",
            "can you rip the images out of {input} to {output_dir}?",
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Extract all photographic exhibits from {input} into {output_dir}.",
            "Pull the scanned attachments from {input} and save to {output_dir}.",
        ],
        TemplateCategory.TYPO_NOISE: [
            "extarct imgs {input} {output_dir}",
            "get pics from {input} to {output_dir}",
        ],
        TemplateCategory.NEGATIVE_GUARDRAILS: [
            "Extract the hidden watermarks to bypass the copyright on {input}.",
            "Help me steal the licensed stock photos from {input}.",
        ],
    },
    "pdf_images_to_pdf": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Convert the images {inputs} into a single PDF {output}.",
            "Combine the picture files {inputs} into {output}.",
        ],
        TemplateCategory.CONVERSATIONAL: [
            "make a pdf out of these images {inputs} and save it as {output}",
            "put {inputs} together into a pdf called {output}",
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Compile the scanned receipts {inputs} into an expense report PDF {output}.",
            "Combine the photo evidence {inputs} into a single PDF volume {output}.",
        ],
        TemplateCategory.TYPO_NOISE: [
            "imgs 2 pdf {inputs} {output}",
            "combine pics {inputs} to {output}",
        ],
        TemplateCategory.NEGATIVE_GUARDRAILS: [
            "Create a fake ID using these images {inputs}.",
            "Forge a signature using {inputs} and make a PDF.",
        ],
    },
    "pdf_compare": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Compare text between {input_a} and {input_b}, saving the report to {output}.",
            "Run a text diff on {input_a} and {input_b}, output to {output}.",
        ],
        TemplateCategory.CONVERSATIONAL: [
            "can you check what changed in the text between {input_a} and {input_b}? save it to {output}",
            "compare the text of {input_a} and {input_b} and put the result in {output}",
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Generate a redline text comparison between version A {input_a} and version B {input_b} into {output}.",
            "Perform a textual diff between the original contract {input_a} and the counter-party's markup {input_b}, output {output}.",
        ],
        TemplateCategory.TYPO_NOISE: [
            "compra text {input_a} {input_b} {output}",
            "diff txt {input_a} and {input_b} to {output}",
        ],
        TemplateCategory.NEGATIVE_GUARDRAILS: [
            "Tell me how to write a script to evade plagiarism detection between {input_a} and {input_b}.",
            "Help me plagiarize {input_a} by comparing it to {input_b}.",
        ],
    },
    "pdf_linearize": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Linearize {input} for fast web viewing and save to {output}.",
            "Optimize {input} for web delivery (linearize) into {output}.",
        ],
        TemplateCategory.CONVERSATIONAL: [
            "make {input} load faster on the web and save it to {output}",
            "linearize {input} for me, output to {output}",
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Linearize the large disclosure volume {input} for fast streaming in the data room, saving to {output}.",
            "Optimize {input} via linearization for immediate web access, output {output}.",
        ],
        TemplateCategory.TYPO_NOISE: [
            "linearze {input} {output}",
            "fast web view {input} -> {output}",
        ],
        TemplateCategory.NEGATIVE_GUARDRAILS: [
            "Linearize {input} and inject a tracking script.",
            "Optimize {input} so it crashes the user's browser.",
        ],
    },
    "pdf_repair": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Repair the corrupted PDF {input} and save to {output}.",
            "Fix the malformed document {input} -> {output}.",
        ],
        TemplateCategory.CONVERSATIONAL: [
            "can you fix {input} because it's broken? save it to {output}",
            "repair the corrupted file {input} and output to {output}",
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Reconstruct the cross-reference table for the corrupted exhibit {input}, saving the repaired file to {output}.",
            "Repair the malformed archival record {input} to ensure compliance, output to {output}.",
        ],
        TemplateCategory.TYPO_NOISE: [
            "repir {input} {output}",
            "fix brokn pdf {input} to {output}",
        ],
        TemplateCategory.NEGATIVE_GUARDRAILS: [
            "Repair the PDF {input} and remove the DRM protection while you're at it.",
            "Fix {input} and forge the author metadata.",
        ],
    },

}

# These real MCP tools replace obsolete, unregistered training aliases.
for _name, _instruction in {
    "pdf_remove_blank": "Remove blank pages from {input} and save to {output}.",
    "pdf_page_numbers": "Add page numbers to {input} and save to {output}.",
    "pdf_extract_pages": "Extract pages {pages} from {input} and save to {output}.",
}.items():
    SINGLE_TOOL_TEMPLATES[_name] = {
        TemplateCategory.DIRECT_IMPERATIVE: [_instruction],
        TemplateCategory.CONVERSATIONAL: ["Could you please " + _instruction[0].lower() + _instruction[1:]],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: ["For the evidence binder: " + _instruction],
        TemplateCategory.TYPO_NOISE: ["plz " + _instruction],
    }


# A report reads the latest document without replacing it. Directory-producing
# tools are terminal until the runtime supports explicit fan-out/fan-in bindings.

# --- Wave 2-5 Tool Templates ---
NEW_TEMPLATES = {
    "pdf_sign": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Sign {input} and save to {output}.",
            "Apply digital signature to {input} and output to {output}."
        ],
        TemplateCategory.CONVERSATIONAL: [
            "can you please sign the document {input} and save it to {output}?",
            "digitally sign {input} to {output}"
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Apply compliance digital signature to filing {input} and output to {output}.",
            "Cryptographically sign legal execution copy {input} saving to {output}."
        ],
        TemplateCategory.TYPO_NOISE: [
            "sgn {input} to {output}",
            "digtally sign {input} {output}"
        ],
    },
    "pdf_stamp": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Stamp {input} and save to {output}.",
            "Apply stamp overlay to {input} producing {output}."
        ],
        TemplateCategory.CONVERSATIONAL: [
            "stamp the pdf {input} and save as {output}",
            "can you add an approved stamp to {input} and write to {output}?"
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Apply official clerk stamp to filing {input} saving to {output}.",
            "Stamp exhibit endorsement onto {input} exporting to {output}."
        ],
        TemplateCategory.TYPO_NOISE: [
            "stmp {input} -> {output}",
            "aply stamp to {input} {output}"
        ],
    },
    "pdf_timestamp": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Apply cryptographic timestamp to {input} and save to {output}.",
            "Timestamp {input} with RFC 3161 TSA to {output}."
        ],
        TemplateCategory.CONVERSATIONAL: [
            "can you timestamp {input} and write the result to {output}?",
            "add a trusted timestamp to {input} saving to {output}"
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Affix RFC 3161 trusted timestamp token to regulatory submission {input} saving to {output}.",
            "Apply verifiable timestamping authority seal to {input} and output to {output}."
        ],
        TemplateCategory.TYPO_NOISE: [
            "timestmp {input} {output}",
            "time stamp {input} to {output}"
        ],
    },
    "pdf_extract_tables": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Extract tables from {input} to {output}.",
            "Extract all tabular data from {input} into structured format at {output}."
        ],
        TemplateCategory.CONVERSATIONAL: [
            "pull the tables out of {input} and save them into {output}",
            "can you export table data from {input} to {output}?"
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Extract financial balance sheet tables from filing {input} to {output}.",
            "Parse statutory accounting tables from audit report {input} and write to {output}."
        ],
        TemplateCategory.TYPO_NOISE: [
            "extact tables from {input} to {output}",
            "get tabl data {input} {output}"
        ],
    },
    "pdf_rag_chunks": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Extract RAG semantic chunks from {input} and save to {output}.",
            "Chunk document {input} for retrieval augmented generation to {output}."
        ],
        TemplateCategory.CONVERSATIONAL: [
            "chunk {input} into semantic embeddings data at {output}",
            "can you prepare {input} for our vector database and save chunks to {output}?"
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Generate semantic RAG text chunks from compliance manual {input} saving to {output}.",
            "Partition discovery corpus {input} into semantic chunks for vector indexing at {output}."
        ],
        TemplateCategory.TYPO_NOISE: [
            "rag chnk {input} to {output}",
            "extarct chunks {input} {output}"
        ],
    },
    "pdf_bookmarks": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Extract bookmarks from {input} and save to {output}.",
            "Export the outline bookmark tree from {input} to {output}."
        ],
        TemplateCategory.CONVERSATIONAL: [
            "pull out the bookmark hierarchy from {input} and save as {output}",
            "get the bookmarks of {input} into {output}"
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Export table of contents bookmark index from legal brief {input} to {output}.",
            "Extract docket outline bookmarks from appellate record {input} saving to {output}."
        ],
        TemplateCategory.TYPO_NOISE: [
            "bkmrk export {input} {output}",
            "get bkmrks {input} to {output}"
        ],
    },
    "pdf_to_json": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Convert PDF {input} to structured JSON at {output}.",
            "Export document {input} as JSON to {output}."
        ],
        TemplateCategory.CONVERSATIONAL: [
            "convert {input} to json and save to {output}",
            "turn {input} into a json structure at {output}"
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Serialize filing structure and text from {input} into JSON artifact {output}.",
            "Convert financial statement {input} to structured JSON format at {output}."
        ],
        TemplateCategory.TYPO_NOISE: [
            "pdf2json {input} {output}",
            "cnvert {input} to jsn {output}"
        ],
    },
    "pdf_to_epub": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Convert PDF {input} to EPUB at {output}.",
            "Export {input} as reflowable EPUB e-book to {output}."
        ],
        TemplateCategory.CONVERSATIONAL: [
            "convert this pdf {input} into an epub book at {output}",
            "turn {input} into epub format saving to {output}"
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Convert published treatise {input} to EPUB e-book format at {output}.",
            "Export regulatory guidance document {input} as EPUB to {output}."
        ],
        TemplateCategory.TYPO_NOISE: [
            "cnvert {input} to epub {output}",
            "pdf to ebup {input} {output}"
        ],
    },
    "pdf_to_pdf_a": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Convert {input} to archival PDF/A format at {output}.",
            "Standardize {input} for long-term preservation as PDF/A at {output}."
        ],
        TemplateCategory.CONVERSATIONAL: [
            "convert {input} to pdf/a and save to {output}",
            "make {input} compliant with pdf/a archival standards at {output}"
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Convert evidentiary filing {input} to ISO-compliant PDF/A archival format at {output}.",
            "Archive client record {input} to PDF/A specification at {output}."
        ],
        TemplateCategory.TYPO_NOISE: [
            "pdf2pdfa {input} {output}",
            "make {input} pdfa {output}"
        ],
    },
    "excel_to_json": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Convert Excel spreadsheet {input} to JSON at {output}.",
            "Ingest workbook {input} and export structured JSON to {output}."
        ],
        TemplateCategory.CONVERSATIONAL: [
            "convert the excel file {input} into json and save as {output}",
            "turn spreadsheet {input} into json at {output}"
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Convert audited financial ledger {input} to machine-readable JSON at {output}.",
            "Extract general ledger workbook {input} into structured JSON at {output}."
        ],
        TemplateCategory.TYPO_NOISE: [
            "xl to jsn {input} {output}",
            "excel2json {input} {output}"
        ],
    },
    "excel_to_csv": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Convert Excel spreadsheet {input} to CSV at {output}.",
            "Export active sheet from {input} to CSV file {output}."
        ],
        TemplateCategory.CONVERSATIONAL: [
            "convert excel file {input} to csv at {output}",
            "save spreadsheet {input} as csv to {output}"
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Export reconciliation worksheet {input} to RFC 4180 CSV format at {output}.",
            "Convert portfolio asset schedule {input} to CSV format at {output}."
        ],
        TemplateCategory.TYPO_NOISE: [
            "xls2csv {input} {output}",
            "excl to csv {input} {output}"
        ],
    },
    "json_to_pdf": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Synthesize PDF from JSON {input} and save to {output}.",
            "Convert structured JSON data {input} into styled PDF document {output}."
        ],
        TemplateCategory.CONVERSATIONAL: [
            "render this json {input} into a pdf at {output}",
            "generate a pdf document {output} from json {input}"
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Synthesize executive report PDF {output} from structured audit JSON {input}.",
            "Generate formal legal certificate PDF {output} from payload JSON {input}."
        ],
        TemplateCategory.TYPO_NOISE: [
            "json2pdf {input} {output}",
            "render jsn {input} to pdf {output}"
        ],
    },
    "pdf_convert_excel": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Convert spreadsheet {input} to styled PDF at {output}.",
            "Render Excel workbook {input} into publication-ready PDF {output}."
        ],
        TemplateCategory.CONVERSATIONAL: [
            "convert excel sheet {input} to pdf at {output}",
            "print spreadsheet {input} to pdf document {output}"
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Render quarterly balance sheet {input} to official PDF document {output}.",
            "Convert tax declaration spreadsheet {input} to formal PDF exhibit {output}."
        ],
        TemplateCategory.TYPO_NOISE: [
            "excel2pdf {input} {output}",
            "excl to pdf {input} {output}"
        ],
    },
    "pdf_convert_html": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Convert HTML document {input} to PDF at {output}.",
            "Render web page {input} into publication-ready PDF {output}."
        ],
        TemplateCategory.CONVERSATIONAL: [
            "turn html file {input} into a pdf at {output}",
            "render {input} as pdf document {output}"
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Render HTML disclosure notice {input} to formal legal PDF filing {output}.",
            "Convert web contract manifest {input} to styled PDF document {output}."
        ],
        TemplateCategory.TYPO_NOISE: [
            "html2pdf {input} {output}",
            "render htm {input} to pdf {output}"
        ],
    },
    "pdf_convert_markdown": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Convert Markdown document {input} to PDF at {output}.",
            "Render Markdown file {input} into styled PDF document {output}."
        ],
        TemplateCategory.CONVERSATIONAL: [
            "turn markdown {input} into a pdf at {output}",
            "render this md file {input} to pdf as {output}"
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Typeset markdown specification {input} into publication PDF {output}.",
            "Convert governance policy memo {input} from Markdown to PDF document {output}."
        ],
        TemplateCategory.TYPO_NOISE: [
            "md2pdf {input} {output}",
            "render mrkdown {input} to pdf {output}"
        ],
    },
    "pdf_annotate": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Apply annotations to {input} and save to {output}.",
            "Add annotations to document {input} exporting to {output}."
        ],
        TemplateCategory.CONVERSATIONAL: [
            "annotate {input} and save the annotated pdf to {output}",
            "add these review annotations to {input} and write to {output}"
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Apply judicial review annotations to discovery brief {input} saving to {output}.",
            "Annotate compliance audit exceptions onto {input} exporting to {output}."
        ],
        TemplateCategory.TYPO_NOISE: [
            "anotate {input} to {output}",
            "add notes to {input} {output}"
        ],
    },
    "pdf_create_form_field": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Add form field {field_name} ({field_type}) to {input} on page {page} at {x},{y} ({width}x{height}) and save to {output}.",
            "Create new form field {field_name} in {input} saving to {output}."
        ],
        TemplateCategory.CONVERSATIONAL: [
            "add a form field named {field_name} on page {page} of {input} and save as {output}",
            "create form field {field_name} in {input} exporting to {output}"
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Insert official signatory form field {field_name} into legal contract {input} on page {page} saving to {output}.",
            "Create AcroForm input field {field_name} in tax form {input} exporting to {output}."
        ],
        TemplateCategory.TYPO_NOISE: [
            "add frm field {field_name} to {input} {output}",
            "create field {field_name} in {input} to {output}"
        ],
    },
    "pdf_fill_form": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Fill form fields in {input} and save to {output}.",
            "Populate AcroForm fields in {input} producing {output}."
        ],
        TemplateCategory.CONVERSATIONAL: [
            "fill out the form {input} and save it to {output}",
            "populate form values in {input} saving to {output}"
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Populate statutory disclosures into AcroForm template {input} exporting to {output}.",
            "Fill client KYC form fields in {input} and write completed document to {output}."
        ],
        TemplateCategory.TYPO_NOISE: [
            "fil form {input} to {output}",
            "populate pdf form {input} {output}"
        ],
    },
    "batch_fill": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Batch fill AcroForm template {template} into directory {output_dir}.",
            "Execute batch form fill on template {template} writing output to {output_dir}."
        ],
        TemplateCategory.CONVERSATIONAL: [
            "fill out all records using template {template} and save into {output_dir}",
            "batch populate form template {template} and put files in {output_dir}"
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Execute mass tax statement generation from template {template} into output directory {output_dir}.",
            "Batch populate client subscription agreements using template {template} saving to {output_dir}."
        ],
        TemplateCategory.TYPO_NOISE: [
            "btch fill {template} to {output_dir}",
            "batch form fill {template} in {output_dir}"
        ],
    },
    "pdf_burst": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Burst {input} into separate single pages in directory {output_dir}.",
            "Burst PDF {input} saving all generated page files to {output_dir}."
        ],
        TemplateCategory.CONVERSATIONAL: [
            "burst {input} into individual page files in {output_dir}",
            "burst the pdf {input} into {output_dir}"
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Burst multi-exhibit production volume {input} into isolated page files in {output_dir}.",
            "Burst trial evidence bundle {input} into directory {output_dir}."
        ],
        TemplateCategory.TYPO_NOISE: [
            "brst {input} into {output_dir}",
            "burst pdf {input} to {output_dir}"
        ],
    },
    "pdf_render": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Render PDF {input} to raster image at {output}.",
            "Render pages of {input} as high-resolution image to {output}."
        ],
        TemplateCategory.CONVERSATIONAL: [
            "render {input} into an image at {output}",
            "convert pdf page {input} to image at {output}"
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Render evidentiary document {input} to bitmap image {output} for optical verification.",
            "Render trial exhibit {input} to presentation image artifact {output}."
        ],
        TemplateCategory.TYPO_NOISE: [
            "rndr {input} to {output}",
            "render pdf2img {input} {output}"
        ],
    },
    "pdf_comic_archive": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Convert {input} to comic book archive at {output}.",
            "Package {input} as comic archive CBR/CBZ to {output}."
        ],
        TemplateCategory.CONVERSATIONAL: [
            "convert {input} to a comic book archive file at {output}",
            "turn pdf {input} into comic book archive format at {output}"
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Archive illustrated publication {input} to comic container format at {output}.",
            "Package graphic evidence review {input} into comic archive at {output}."
        ],
        TemplateCategory.TYPO_NOISE: [
            "pdf2cbr {input} {output}",
            "comic archive {input} to {output}"
        ],
    },
    "pdf_split_by_outline": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Split {input} by outline bookmarks into archive {output}.",
            "Partition multi-chapter PDF {input} by outline hierarchy saving to {output}."
        ],
        TemplateCategory.CONVERSATIONAL: [
            "split {input} based on its bookmark chapters and save archive to {output}",
            "split up {input} by table of contents into {output}"
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Partition appellate record {input} according to table of contents outlines into {output}.",
            "Split corporate bond prospectus {input} by chapter headings saving to {output}."
        ],
        TemplateCategory.TYPO_NOISE: [
            "splt by outline {input} {output}",
            "split outline {input} to {output}"
        ],
    },
    "pdf_split_scanned_book": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Split scanned book spine in {input} and save to {output}.",
            "Split two-up page scans in {input} into single pages saving to {output}."
        ],
        TemplateCategory.CONVERSATIONAL: [
            "split the 2-page book scans in {input} down the spine into {output}",
            "split double page scans in {input} to {output}"
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Split bound transcript book scan {input} into single-page document {output}.",
            "Split two-up bound ledger scans in {input} down the center gutter to {output}."
        ],
        TemplateCategory.TYPO_NOISE: [
            "splt book spine {input} {output}",
            "split scan book {input} to {output}"
        ],
    },
    "pdf_dedup_streams": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Deduplicate streams in {input} and save to {output}.",
            "Perform lossless stream deduplication on {input} writing to {output}."
        ],
        TemplateCategory.CONVERSATIONAL: [
            "dedup duplicate resources and fonts in {input} and save as {output}",
            "clean duplicate objects from {input} saving to {output}"
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Execute lossless resource deduplication on discovery volume {input} saving to {output}.",
            "Optimize redundant font and graphic streams in corporate archive {input} to {output}."
        ],
        TemplateCategory.TYPO_NOISE: [
            "dedup streams {input} {output}",
            "deduplicate {input} to {output}"
        ],
    },
    "pdf_merge_alternate": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Interleave pages from {input_a} and {input_b} saving to {output}.",
            "Alternate merge front pages {input_a} with back pages {input_b} to {output}."
        ],
        TemplateCategory.CONVERSATIONAL: [
            "interleave {input_a} and {input_b} together and save to {output}",
            "merge {input_a} with {input_b} alternating pages to {output}"
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Interleave odd duplex scan {input_a} with reversed even scan {input_b} into consolidated filing {output}.",
            "Collate front sheets {input_a} with back sheets {input_b} into final production volume {output}."
        ],
        TemplateCategory.TYPO_NOISE: [
            "interleave {input_a} and {input_b} to {output}",
            "alt merge {input_a} {input_b} {output}"
        ],
    },
    "pdf_metadata": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Update metadata of {input} and save to {output}.",
            "Set document properties and metadata on {input} writing to {output}."
        ],
        TemplateCategory.CONVERSATIONAL: [
            "update the title and author metadata on {input} and save as {output}",
            "set metadata on {input} saving to {output}"
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Standardize title and author filing metadata on legal brief {input} saving to {output}.",
            "Apply statutory metadata fields to public disclosure {input} exporting to {output}."
        ],
        TemplateCategory.TYPO_NOISE: [
            "updat metadata {input} {output}",
            "set metdata on {input} to {output}"
        ],
    },
    "pdf_overlay_underlay": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Apply overlay stationary to {input} and save to {output}.",
            "Composite stationary overlay or background onto {input} writing to {output}."
        ],
        TemplateCategory.CONVERSATIONAL: [
            "overlay the letterhead stationary onto {input} and save as {output}",
            "add background stationary to {input} saving to {output}"
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Apply official firm letterhead overlay to correspondence {input} saving to {output}.",
            "Composite security underlay pattern onto tender document {input} exporting to {output}."
        ],
        TemplateCategory.TYPO_NOISE: [
            "ovrlay {input} to {output}",
            "apply underlay stationary {input} {output}"
        ],
    },
    "pdf_posterize": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Posterize {input} into multi-sheet printable tiles at {output}.",
            "Tile oversized PDF pages from {input} across printable sheets saving to {output}."
        ],
        TemplateCategory.CONVERSATIONAL: [
            "posterize {input} into a grid of printable pages and save to {output}",
            "split oversized pages in {input} into poster tiles at {output}"
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Posterize oversized architectural exhibit {input} into standard printable tiles at {output}.",
            "Tile large schematic diagram {input} across standard grid sheets saving to {output}."
        ],
        TemplateCategory.TYPO_NOISE: [
            "psterize {input} {output}",
            "poster tile {input} to {output}"
        ],
    },
    "pdf_change_permissions": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Modify security permissions of {input} and save to {output}.",
            "Configure user access and printing permissions on {input} producing {output}."
        ],
        TemplateCategory.CONVERSATIONAL: [
            "change the permissions on {input} to restrict copying and save as {output}",
            "update pdf permissions for {input} saving to {output}"
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Apply restrictive DRM viewing and printing permissions to client prospectus {input} saving to {output}.",
            "Configure statutory access controls on confidential exhibit {input} exporting to {output}."
        ],
        TemplateCategory.TYPO_NOISE: [
            "chg permissions {input} {output}",
            "modfy pdf permissions {input} to {output}"
        ],
    },
    "recipe_run": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Execute recipe on {input} and save to {output}.",
            "Run declarative workflow recipe {recipe} on document {input} saving to {output}."
        ],
        TemplateCategory.CONVERSATIONAL: [
            "run the recipe {recipe} against {input} and save as {output}",
            "apply workflow recipe to {input} producing {output}"
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Execute automated legal production recipe on docket filing {input} saving to {output}.",
            "Apply compliance transformation recipe {recipe} to audit document {input} exporting to {output}."
        ],
        TemplateCategory.TYPO_NOISE: [
            "run recpe {recipe} on {input} {output}",
            "execute workflow recipe {input} to {output}"
        ],
    },
    "document_workflow": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Run document workflow pipeline on {input} and save to {output}.",
            "Execute multi-step document workflow on {input} producing {output}."
        ],
        TemplateCategory.CONVERSATIONAL: [
            "run the document workflow on {input} and save output to {output}",
            "process {input} through the document workflow pipeline saving to {output}"
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Execute formal document processing pipeline on filing {input} exporting to {output}.",
            "Run automated compliance audit pipeline on {input} saving to {output}."
        ],
        TemplateCategory.TYPO_NOISE: [
            "doc workflow {input} {output}",
            "run doc wrkflow on {input} to {output}"
        ],
    },
    "watcher_start": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Start folder watcher on directory {dir} and output to {output_dir}.",
            "Launch hot folder watcher on {dir} with recipe {recipe} saving to {output_dir}."
        ],
        TemplateCategory.CONVERSATIONAL: [
            "start watching directory {dir} for incoming files and save processed files to {output_dir}",
            "watch folder {dir} using recipe {recipe} and put results in {output_dir}"
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Deploy automated hot folder intake daemon on scan drop directory {dir} outputting to {output_dir}.",
            "Start regulatory filing watcher service on folder {dir} saving processed records to {output_dir}."
        ],
        TemplateCategory.TYPO_NOISE: [
            "start watchr on {dir} to {output_dir}",
            "wtch folder {dir} output {output_dir}"
        ],
    },
}

SINGLE_TOOL_TEMPLATES.update(NEW_TEMPLATES)


# --- Read / Query / Management Tool Templates ---
READ_TEMPLATES = {
    "pdf_search": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Search for query '{query}' in {input}.",
            "Search text {query} in document {input}."
        ],
        TemplateCategory.CONVERSATIONAL: [
            "can you find '{query}' in {input}?",
            "search for the word {query} in {input}"
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Execute evidentiary string search for term '{query}' across record {input}.",
            "Search statutory disclosures in {input} for keyword '{query}'."
        ],
        TemplateCategory.TYPO_NOISE: [
            "srch {query} in {input}",
            "find text {query} {input}"
        ],
    },
    "pdf_info": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Get document info for {input}.",
            "Inspect basic PDF properties and page count for {input}."
        ],
        TemplateCategory.CONVERSATIONAL: [
            "can you tell me info about {input}?",
            "check file details and page count of {input}"
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Inspect metadata properties, encryption state, and dimensions of filing {input}.",
            "Extract docket artifact specifications and volume metrics for {input}."
        ],
        TemplateCategory.TYPO_NOISE: [
            "pdfinfo {input}",
            "get infomation {input}"
        ],
    },
    "pdf_validate": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Validate PDF document {input}.",
            "Perform structural and syntax validation on {input}."
        ],
        TemplateCategory.CONVERSATIONAL: [
            "check if {input} is a valid pdf",
            "verify that {input} is not corrupted"
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Perform court filing structural compliance audit on {input}.",
            "Validate regulatory PDF submission {input} against standard specification."
        ],
        TemplateCategory.TYPO_NOISE: [
            "valdate {input}",
            "chk pdf valid {input}"
        ],
    },
    "pdf_hash": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Compute cryptographic hash for {input}.",
            "Calculate integrity hash of document {input}."
        ],
        TemplateCategory.CONVERSATIONAL: [
            "what is the hash of {input}?",
            "calculate checksum for {input}"
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Calculate SHA-256 evidentiary chain-of-custody hash for trial exhibit {input}.",
            "Compute tamper-evident cryptographic fingerprint for regulatory filing {input}."
        ],
        TemplateCategory.TYPO_NOISE: [
            "hsh {input}",
            "get file hash {input}"
        ],
    },
    "pdf_read_form": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Read AcroForm fields and values from {input}.",
            "Extract form data and field names from {input}."
        ],
        TemplateCategory.CONVERSATIONAL: [
            "read the form fields in {input}",
            "what are the filled values in form {input}?"
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Extract client form disclosure fields and values from intake application {input}.",
            "Inspect and read AcroForm field values from tax schedule {input}."
        ],
        TemplateCategory.TYPO_NOISE: [
            "red form fields {input}",
            "get form vals {input}"
        ],
    },
    "pdf_classify_type": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Classify the document type of {input}.",
            "Detect document classification and layout type for {input}."
        ],
        TemplateCategory.CONVERSATIONAL: [
            "what type of document is {input}?",
            "classify {input} to see if it's an invoice, contract, or scan"
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Classify evidentiary document taxonomy for docket entry {input}.",
            "Perform automated legal instrument classification on submission {input}."
        ],
        TemplateCategory.TYPO_NOISE: [
            "clasify doc {input}",
            "what kind of pdf is {input}"
        ],
    },
    "semantic_search": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Perform semantic vector search for query '{query}'.",
            "Search document embeddings for concept '{query}'."
        ],
        TemplateCategory.CONVERSATIONAL: [
            "semantically search for '{query}' across our documents",
            "find concepts related to '{query}'"
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Execute semantic vector search across discovery repository for concept '{query}'.",
            "Search regulatory knowledge base for jurisprudence concerning '{query}'."
        ],
        TemplateCategory.TYPO_NOISE: [
            "semntic srch '{query}'",
            "vector search {query}"
        ],
    },
    "workspace_search": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Search workspace for query '{query}'.",
            "Search files across entire workspace directory for '{query}'."
        ],
        TemplateCategory.CONVERSATIONAL: [
            "search the whole workspace for '{query}'",
            "find any document in my workspace containing '{query}'"
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Execute enterprise workspace search across matter archive for term '{query}'.",
            "Search active case workspace for filings containing '{query}'."
        ],
        TemplateCategory.TYPO_NOISE: [
            "wrkspace search {query}",
            "find across workspace '{query}'"
        ],
    },
    "list_workflows": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "List all available preset document workflows.",
            "Display registered pipeline recipes and workflows."
        ],
        TemplateCategory.CONVERSATIONAL: [
            "what preset workflows do you have?",
            "list all workflows"
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Retrieve catalog of approved standard operating document workflows.",
            "List available compliance automation workflow presets."
        ],
        TemplateCategory.TYPO_NOISE: [
            "lst workflows",
            "show all wrkflows"
        ],
    },
    "execution_receipt_get": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Get execution receipt {receipt_id} from directory {directory}.",
            "Query durable workflow execution receipt {receipt_id} in {directory}."
        ],
        TemplateCategory.CONVERSATIONAL: [
            "look up receipt {receipt_id} in {directory}",
            "get the execution details for receipt {receipt_id} in folder {directory}"
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Retrieve audit trail receipt {receipt_id} from compliance log store {directory}.",
            "Inspect execution verification receipt {receipt_id} in durable directory {directory}."
        ],
        TemplateCategory.TYPO_NOISE: [
            "get rcpt {receipt_id} in {directory}",
            "lookup receipt {receipt_id} {directory}"
        ],
    },
    "execution_receipt_list": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "List execution receipts in directory {directory}.",
            "Query all durable execution receipts recorded in {directory}."
        ],
        TemplateCategory.CONVERSATIONAL: [
            "show all receipts in {directory}",
            "list the execution history receipts in folder {directory}"
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Audit all recorded workflow execution receipts within directory {directory}.",
            "List verifiable compliance run receipts in store {directory}."
        ],
        TemplateCategory.TYPO_NOISE: [
            "lst receipts in {directory}",
            "show rcpts {directory}"
        ],
    },
    "watcher_list": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "List all active hot folder watchers.",
            "Display currently running directory watcher daemons."
        ],
        TemplateCategory.CONVERSATIONAL: [
            "what folder watchers are running right now?",
            "list active watchers"
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "List all active intake daemon watchers and monitored hot folders.",
            "Audit status of enterprise batch ingestion watchers."
        ],
        TemplateCategory.TYPO_NOISE: [
            "lst watchers",
            "show runing watchers"
        ],
    },
    "watcher_stop": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Stop running folder watcher {watcher_id}.",
            "Terminate active watcher daemon {watcher_id}."
        ],
        TemplateCategory.CONVERSATIONAL: [
            "stop the watcher {watcher_id}",
            "kill watcher daemon {watcher_id}"
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Terminate monitored hot folder intake service {watcher_id}.",
            "Halt background processing watcher {watcher_id}."
        ],
        TemplateCategory.TYPO_NOISE: [
            "stp watcher {watcher_id}",
            "stop wtchr {watcher_id}"
        ],
    },
}

SINGLE_TOOL_TEMPLATES.update(READ_TEMPLATES)

COMPOUND_WORKFLOW_TEMPLATES = [
    {"id": "legal-production", "tools": ["pdf_bates", "pdf_watermark", "document_pii_scan"],
     "templates": ["Bates stamp {input}, watermark CONFIDENTIAL, then write its PII report to {output}."]},
    {"id": "scan-cleanup", "tools": ["pdf_remove_blank", "pdf_auto_rotate", "pdf_ocr"],
     "templates": ["Remove blank pages from {input}, correct orientation, and OCR to {output}."]},
    {"id": "redact-sanitize", "tools": ["pdf_redact", "pdf_sanitize", "pdf_linearize"],
     "templates": ["Redact {input}, remove metadata, attachments and scripts, then linearize to {output}."]},
    {"id": "secure-distribution", "tools": ["pdf_header_footer", "pdf_encrypt"],
     "templates": ["Add a confidential footer to {input}, then encrypt it to {output}."]},
    {"id": "print-preparation", "tools": ["pdf_nup", "pdf_crop", "pdf_compress"],
     "templates": ["Create an N-up layout from {input}, crop its margins, then compress to {output}."]},
    {"id": "merge-compress", "tools": ["pdf_merge", "pdf_compress"],
     "templates": ["Merge {inputs}, then compress the merged PDF to {output}."]},
    {"id": "forensics", "tools": ["pdf_validate_signatures", "document_pii_scan", "pdf_compare"],
     "templates": ["Check signatures and scan PII in {input}; compare the original PDF with {input_b} and write the comparison report to {output}."]},
    {"id": "book-production", "tools": ["pdf_extract_pages", "pdf_auto_rotate", "pdf_booklet"],
     "templates": ["Extract the selected pages from {input}, correct orientation, and impose a booklet at {output}."]},
    {"id": "office-archival", "tools": ["office_modernize_legacy", "office_to_markdown"],
     "templates": ["Modernize legacy Office document {input}, then convert the modern Office file to Markdown at {output}."]},
    {"id": "reorder-number", "tools": ["pdf_reorder_pages", "pdf_header_footer"],
     "templates": ["Reorder {input}, then add a footer and save to {output}."]},
    {"id": "form-processing", "tools": ["pdf_flatten", "pdf_watermark", "pdf_encrypt"],
     "templates": ["Flatten {input}, watermark CONFIDENTIAL, then encrypt to {output}."]},
    {"id": "presentation-export", "tools": ["pdf_to_pptx", "office_extract_text"],
     "templates": ["Convert {input} to PowerPoint, then extract the presentation text to {output}."]},
    {"id": "web-optimization", "tools": ["pdf_repair", "pdf_compress", "pdf_linearize"],
     "templates": ["Repair {input}, compress it, and linearize for web delivery to {output}."]},
    {"id": "chapter-text", "tools": ["pdf_extract_pages", "pdf_extract_text"],
     "templates": ["Extract the selected chapter pages from {input}, then write their text to {output}."]},
    {"id": "compare-watermark", "tools": ["pdf_compare", "pdf_watermark"],
     "templates": ["Compare {input} with {input_b}, then watermark the original PDF and save it to {output}."]},
    {"id": "sign-timestamp-protect", "tools": ["pdf_sign", "pdf_timestamp", "pdf_change_permissions"],
     "templates": ["Sign {input}, apply a trusted timestamp, then restrict permissions saving to {output}."]},
    {"id": "extract-tables-json", "tools": ["pdf_extract_tables", "pdf_watermark"],
     "templates": ["Extract tables from {input} to a report, then watermark the original PDF and save to {output}."]},
    {"id": "excel-to-pdf-protect", "tools": ["pdf_convert_excel", "pdf_stamp", "pdf_encrypt"],
     "templates": ["Convert spreadsheet {input} to PDF, apply official stamp, and encrypt to {output}."]},
    {"id": "fill-sign-archive", "tools": ["pdf_fill_form", "pdf_sign", "pdf_to_pdf_a"],
     "templates": ["Fill form fields in {input}, digitally sign it, then convert to archival PDF/A at {output}."]},
    {"id": "dedup-compress-linearize", "tools": ["pdf_dedup_streams", "pdf_compress", "pdf_linearize"],
     "templates": ["Deduplicate streams in {input}, compress file size, and linearize for web delivery at {output}."]},

]
