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
