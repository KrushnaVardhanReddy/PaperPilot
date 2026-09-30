import pandas as pd
import random
import os

intents = {
    "Bates": ["add bates numbering to {doc}", "bates stamp this {doc}", "apply bates numbers", "bates number these {doc}s"],
    "Bookmarks": ["add bookmarks to {doc}", "create bookmarks in {doc}", "bookmark this {doc}", "generate bookmarks for {doc}"],
    "Burst": ["burst this {doc}", "burst {doc} into single pages", "break {doc} into separate pages", "burst"],
    "Compare": ["compare these {doc}s", "find differences between {doc}s", "diff these two {doc}s", "compare {doc} a and b"],
    "Compress": ["compress this {doc}", "make {doc} smaller", "reduce file size of {doc}", "shrink {doc}"],
    "ToMarkdown": ["convert {doc} to markdown", "make this a markdown file", "{doc} to md", "export to markdown"],
    "ToJson": ["convert {doc} to json", "make this a json file", "{doc} to json", "export to json"],
    "ToDocx": ["convert {doc} to docx", "make this a word document", "{doc} to word", "export to docx", "make this a docx"],
    "ToPptx": ["convert {doc} to pptx", "make this a powerpoint", "{doc} to ppt", "export to pptx"],
    "ToHtml": ["convert {doc} to html", "make this a webpage", "{doc} to html", "export to html"],
    "Crop": ["crop this {doc}", "trim the edges of {doc}", "crop pages in {doc}", "resize pages in {doc}"],
    "Decrypt": ["decrypt {doc}", "remove password from {doc}", "unlock {doc}", "unprotect {doc}"],
    "Delete": ["delete pages from {doc}", "remove page 2 from {doc}", "delete the last page of {doc}", "drop pages from {doc}"],
    "Encrypt": ["encrypt {doc}", "add password to {doc}", "put a password on this {doc}", "protect {doc}", "lock {doc}"],
    "Extract": ["extract pages from {doc}", "pull out page 5 from {doc}", "get pages 1-3 of {doc}", "extract page 5"],
    "ExtractImages": ["extract images from {doc}", "pull pictures from {doc}", "get all images in {doc}", "rip photos from {doc}"],
    "ExtractText": ["extract text from {doc}", "get text out of {doc}", "pull text from {doc}", "read text from {doc}"],
    "FormFill": ["fill out this form", "fill form in {doc}", "populate form fields in {doc}", "form fill {doc}"],
    "FormRead": ["read form data from {doc}", "get form fields from {doc}", "extract data from form", "read form in {doc}"],
    "FormCreate": ["create a form in {doc}", "make a form from {doc}", "add form fields to {doc}", "generate form"],
    "Flatten": ["flatten {doc}", "flatten annotations in {doc}", "flatten form in {doc}", "make {doc} flat"],
    "Hash": ["get hash of {doc}", "calculate checksum for {doc}", "hash this {doc}", "verify {doc} hash"],
    "HeaderFooter": ["add header to {doc}", "add footer to {doc}", "put page numbers on {doc}", "add headers and footers"],
    "ImagesToPdf": ["convert images to pdf", "jpg to pdf", "combine pictures into {doc}", "make pdf from images"],
    "Linearize": ["linearize {doc}", "optimize {doc} for web", "fast web view for {doc}", "make {doc} load faster"],
    "Merge": ["merge these {doc}s", "combine these {doc}s", "join {doc}s together", "put these {doc}s together", "combine these documents"],
    "Metadata": ["get metadata of {doc}", "view properties of {doc}", "read document info", "show author of {doc}"],
    "Ocr": ["ocr this {doc}", "run optical character recognition on {doc}", "make {doc} searchable", "recognize text in {doc}"],
    "PdfA": ["convert {doc} to pdf/a", "make {doc} archivable", "pdfa conversion for {doc}", "save as pdf/a"],
    "Redact": ["redact {doc}", "black out text in {doc}", "censor {doc}", "hide info in {doc}"],
    "Render": ["render {doc}", "convert {doc} to images", "rasterize {doc}", "save {doc} as image"],
    "Reorder": ["reorder pages in {doc}", "move page 5 to the front of {doc}", "rearrange {doc}", "shuffle pages in {doc}"],
    "Repair": ["repair {doc}", "fix corrupted {doc}", "recover {doc}", "salvage {doc}"],
    "Rotate": ["rotate {doc}", "spin pages in {doc}", "turn {doc} upside down", "rotate page 1 of {doc}"],
    "Search": ["search for text in {doc}", "find words in {doc}", "look for 'invoice' in {doc}", "search {doc}"],
    "Sign": ["sign {doc}", "add digital signature to {doc}", "sign this {doc}", "put signature on {doc}"],
    "Split": ["split {doc}", "divide {doc} in half", "break {doc} into pieces", "split this {doc}"],
    "Validate": ["validate {doc}", "check if {doc} is valid", "verify {doc} standard", "validate pdf/a"],
    "Watermark": ["add watermark to {doc}", "watermark this {doc}", "stamp {doc}", "put a watermark on {doc}"],
    "Classify": ["classify {doc}", "what kind of document is {doc}", "categorize {doc}", "detect document type"],
}

docs_synonyms = ["file", "document", "pdf", "it", "this"]

dataset = []
for intent, templates in intents.items():
    for template in templates:
        for doc_syn in docs_synonyms:
            query = template.replace("{doc}", doc_syn)
            dataset.append({"query": query, "label": intent})

            # create some slight variations to increase dataset size
            dataset.append({"query": "please " + query, "label": intent})
            dataset.append({"query": "can you " + query, "label": intent})
            dataset.append({"query": query + " please", "label": intent})

# Add generic queries
generic_queries = []
for intent in intents.keys():
    for i in range(5):
       dataset.append({"query": f"I want to {intent.lower()} a file", "label": intent})
       dataset.append({"query": f"execute {intent} on document", "label": intent})

df = pd.DataFrame(dataset)

# Shuffle dataset
df = df.sample(frac=1, random_state=42).reset_index(drop=True)

df.to_csv("tools/train-nlp/dataset.csv", index=False)
print(f"Generated {len(df)} samples in tools/train-nlp/dataset.csv")
