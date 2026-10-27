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
    "pdf_reorder": {
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
    "pdf_reverse": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Reverse page sequence of {input} and save to {output}.",
            "Invert page order for {input} -> {output}.",
        ],
        TemplateCategory.CONVERSATIONAL: [
            "flip the pages backwards in {input} and save as {output}.",
            "reverse the order of {input} and put it at {output}.",
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Invert document pagination sequence for {input} to match scan order, output to {output}.",
            "Execute page reversal on {input} for proper chronological alignment, saving to {output}.",
        ],
        TemplateCategory.TYPO_NOISE: [
            "revers {input} -> {output}.",
            "flp pages backwrds {input} at {output}.",
        ],
    },
    "pdf_stamp": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Stamp {input} and save to {output}.",
            "Apply stamp to {input} -> {output}.",
        ],
        TemplateCategory.CONVERSATIONAL: [
            "put a stamp on {input} and save it as {output}.",
            "stamp {input} for me at {output}.",
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Apply official seal or stamp to document {input} and output to {output}.",
            "Execute stamping procedure on {input} for authentication, saving to {output}.",
        ],
        TemplateCategory.TYPO_NOISE: [
            "stmp {input} -> {output}.",
            "aply stamp to {input} at {output}.",
        ],
    },
    "pdf_overlay_underlay": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Overlay or underlay onto {input} and save to {output}.",
            "Apply overlay/underlay to {input} -> {output}.",
        ],
        TemplateCategory.CONVERSATIONAL: [
            "put a layer over or under {input} and save as {output}.",
            "overlay {input} and put it at {output}.",
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Apply standardized letterhead underlay to {input} and output as {output}.",
            "Execute document overlay procedure on {input} for composite presentation, saving to {output}.",
        ],
        TemplateCategory.TYPO_NOISE: [
            "ovrly {input} -> {output}.",
            "apply layer to {input} at {output}.",
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
            "Manage portfolio attachments for {input} with action '{action}', file '{file_name}', desc '{description}', b64 '{b64}' and save to {output}.",
            "Execute '{action}' on attachments in {input} for file '{file_name}' (desc: '{description}', b64: '{b64}') -> {output}.",
        ],
        TemplateCategory.CONVERSATIONAL: [
            "can you {action} the attachment '{file_name}' (desc: '{description}', b64: '{b64}') in {input} and save to {output}?",
            "{action} file '{file_name}' (desc: '{description}', b64: '{b64}') in portfolio {input} at {output}.",
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Perform attachment {action} procedure for exhibit '{file_name}' (desc: '{description}', b64: '{b64}') in {input}, outputting to {output}.",
            "Manage embedded digital assets via {action} for file '{file_name}' (desc: '{description}', b64: '{b64}') in {input}, saving as {output}.",
        ],
        TemplateCategory.TYPO_NOISE: [
            "{action} attachmnt '{file_name}' desc: '{description}', b64: '{b64}' in {input} -> {output}.",
            "mangae atachments {action} '{file_name}' desc: '{description}', b64: '{b64}' for {input} at {output}.",
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
    "pdf_flatten_annotations": {
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
    "pdf_to_images": {
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
    "images_to_pdf": {
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
    "pdf_compare_text": {
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
    "pdf_compare_visual": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Visually compare {input_a} and {input_b}, saving the diff to {output}.",
            "Run a visual diff between {input_a} and {input_b} -> {output}.",
        ],
        TemplateCategory.CONVERSATIONAL: [
            "look for visual differences between {input_a} and {input_b} and save the diff to {output}",
            "visually compare {input_a} with {input_b}, output to {output}",
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Perform a pixel-perfect visual comparison of the scanned blueprints {input_a} and {input_b}, output to {output}.",
            "Run a visual differential analysis on the signed document {input_a} versus {input_b}, save to {output}.",
        ],
        TemplateCategory.TYPO_NOISE: [
            "vis diff {input_a} {input_b} {output}",
            "compare vis {input_a} and {input_b} to {output}",
        ],
        TemplateCategory.NEGATIVE_GUARDRAILS: [
            "Use visual diff to find the hidden security markers on {input_a} to forge them.",
            "Compare {input_a} and {input_b} to extract the digital watermark.",
        ],
    },
    "pdf_invert_colors": {
        TemplateCategory.DIRECT_IMPERATIVE: [
            "Invert the colors of {input} using {mode} mode on pages {pages} and save to {output}.",
            "Apply {mode} color inversion to {input} (pages {pages}) and output to {output}.",
        ],
        TemplateCategory.CONVERSATIONAL: [
            "can you invert the colors on {input} with {mode} for pages {pages}? save it as {output}",
            "make {input} dark mode ({mode}) on pages {pages} and save to {output}",
        ],
        TemplateCategory.DOMAIN_LEGAL_FINANCE: [
            "Invert colors of the scanned microfilm {input} using {mode} on pages {pages} for enhanced readability, save to {output}.",
            "Apply contrast inversion ({mode}) to the evidentiary scan {input} on pages {pages}, outputting {output}.",
        ],
        TemplateCategory.TYPO_NOISE: [
            "invrt colors {input} {mode} {pages} {output}",
            "make dark {input} {mode} on {pages} to {output}",
        ],
        TemplateCategory.NEGATIVE_GUARDRAILS: [
            "Invert the colors to hide the redactions on {input}.",
            "Use color inversion to bypass the copy protection of {input}.",
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

# Compound workflows define a sequence of tools to run.
# The template string should just broadly describe the intent of the combined steps.
# The generator will supply the required intermediate/final args when generating the JSON array.

COMPOUND_WORKFLOW_TEMPLATES = [
    {
        "tools": ["pdf_bates", "pdf_watermark", "document_pii_scan"],
        "templates": [
            "Prepare {input} for legal production: Bates stamp it, add a confidential watermark, scan for PII, and save to {output}.",
            "Run legal processing on {input}: apply Bates numbers, watermark 'CONFIDENTIAL', check for PII, output to {output}."
        ]
    },
    {
        "tools": ["pdf_delete_pages", "pdf_auto_rotate", "pdf_ocr"],
        "templates": [
            "Clean this scan {input}, upright sideways pages, run OCR, and save to {output}.",
            "Prep the scanned document {input} by deleting blank pages, fixing orientation, and making it searchable. Final output {output}."
        ]
    },
    {
        "tools": ["pdf_redact", "pdf_sanitize", "pdf_linearize"],
        "templates": [
            "Sanitize {input} by redacting it, stripping metadata, and linearizing for web fast view, output to {output}.",
            "Redact, sanitize, and linearize {input}, saving as {output}."
        ]
    },
    {
        "tools": ["pdf_header_footer", "pdf_encrypt"],
        "templates": [
            "Add page numbers to {input} and then encrypt it, saving to {output}.",
            "Secure {input} for distribution: add headers/footers, encrypt with a password, output to {output}."
        ]
    },
    {
        "tools": ["pdf_nup", "pdf_crop", "pdf_compress"],
        "templates": [
            "Prepare {input} for print: create N-up layout, crop margins, compress, save to {output}.",
            "Print prep for {input}: n-up, crop, and compress to {output}."
        ]
    },
    {
        "tools": ["pdf_merge", "pdf_compress"],
        "templates": [
            "Merge {inputs} and compress the final file to {output}.",
            "Combine {inputs} into a single small file at {output}."
        ]
    },
    {
        "tools": ["pdf_validate_signatures", "pdf_extract_tables", "pdf_compare_visual"],
        "templates": [
            "Run forensic analysis on {input}: validate signatures, extract tables, and run a visual comparison. Save output to {output}.",
            "Check signatures, extract tables, and visually compare {input}. Final output {output}."
        ]
    },
    {
        "tools": ["pdf_split", "pdf_auto_rotate", "pdf_booklet"],
        "templates": [
            "Split {input}, auto-rotate pages, and format as a booklet, saving to {output}.",
            "Create a booklet from {input} by splitting and rotating first, then output to {output}."
        ]
    },
    {
        "tools": ["office_modernize_legacy", "pdf_to_markdown", "pdf_compress"],
        "templates": [
            "Modernize legacy document {input}, convert to markdown, and compress, saving to {output}.",
            "Archive prep: modernize {input}, extract as markdown, and compress to {output}."
        ]
    },
    {
        "tools": ["pdf_reorder", "pdf_header_footer"],
        "templates": [
            "Reorder alternating pages in {input} and add page numbers, saving to {output}.",
            "Fix alternating pages in {input}, add a header/footer, and save to {output}."
        ]
    },
    {
        "tools": ["pdf_flatten_annotations", "pdf_stamp", "pdf_encrypt"],
        "templates": [
            "Flatten forms in {input}, stamp it, and encrypt before saving to {output}.",
            "Process form {input}: flatten annotations, add a stamp, encrypt, output to {output}."
        ]
    },
    {
        "tools": ["pdf_to_pptx", "office_extract_text"],
        "templates": [
            "Convert {input} to PowerPoint and extract the text for notes, saving to {output}.",
            "Create slides from {input} and extract the text. Output to {output}."
        ]
    },
    {
        "tools": ["pdf_repair", "pdf_compress", "pdf_linearize"],
        "templates": [
            "Repair the broken web PDF {input}, compress it, and linearize for fast view, saving to {output}.",
            "Fix, compress, and linearize {input} for web usage, output to {output}."
        ]
    },
    {
        "tools": ["pdf_split", "pdf_extract_text"],
        "templates": [
            "Split {input} into chapters and extract the text to {output}.",
            "Extract text from each chapter of {input} by splitting it first, saving to {output}."
        ]
    },
    {
        "tools": ["pdf_compare_visual", "pdf_invert_colors"],
        "templates": [
            "Visually compare {input} and invert colors for dark mode, output to {output}.",
            "Run visual diff on {input} and invert the colors, saving to {output}."
        ]
    }
]
