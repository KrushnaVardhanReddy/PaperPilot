#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Intent {
    Bates,
    Bookmarks,
    Burst,
    Compare,
    Compress,
    ToMarkdown,
    ToJson,
    ToDocx,
    ToPptx,
    ToHtml,
    Crop,
    Decrypt,
    Delete,
    Encrypt,
    Extract,
    ExtractImages,
    ExtractText,
    FormFill,
    FormRead,
    FormCreate,
    Flatten,
    Hash,
    HeaderFooter,
    ImagesToPdf,
    Linearize,
    Merge,
    Metadata,
    Ocr,
    PdfA,
    Redact,
    Render,
    Reorder,
    Repair,
    Rotate,
    Search,
    Sign,
    Split,
    Validate,
    Watermark,
    Classify,
}

pub struct IntentDefinition {
    pub intent: Intent,
    pub canonical_name: &'static str,
    pub description: &'static str,
    pub aliases: &'static [&'static str],
}

impl Intent {
    pub fn all() -> Vec<Intent> {
        vec![
            Intent::Bates,
            Intent::Bookmarks,
            Intent::Burst,
            Intent::Compare,
            Intent::Compress,
            Intent::ToMarkdown,
            Intent::ToJson,
            Intent::ToDocx,
            Intent::ToPptx,
            Intent::ToHtml,
            Intent::Crop,
            Intent::Decrypt,
            Intent::Delete,
            Intent::Encrypt,
            Intent::Extract,
            Intent::ExtractImages,
            Intent::ExtractText,
            Intent::FormFill,
            Intent::FormRead,
            Intent::FormCreate,
            Intent::Flatten,
            Intent::Hash,
            Intent::HeaderFooter,
            Intent::ImagesToPdf,
            Intent::Linearize,
            Intent::Merge,
            Intent::Metadata,
            Intent::Ocr,
            Intent::PdfA,
            Intent::Redact,
            Intent::Render,
            Intent::Reorder,
            Intent::Repair,
            Intent::Rotate,
            Intent::Search,
            Intent::Sign,
            Intent::Split,
            Intent::Validate,
            Intent::Watermark,
            Intent::Classify,
        ]
    }

    pub fn definition(&self) -> IntentDefinition {
        match self {
            Intent::Bates => IntentDefinition {
                intent: *self,
                canonical_name: "Bates",
                description: "Apply Bates numbering to a document.",
                aliases: &["bates numbering", "bates stamp", "legal numbering", "bates number", "apply bates"],
            },
            Intent::Bookmarks => IntentDefinition {
                intent: *self,
                canonical_name: "Bookmarks",
                description: "Manage bookmarks and outlines.",
                aliases: &["bookmarks", "outline", "table of contents", "toc", "navigation links"],
            },
            Intent::Burst => IntentDefinition {
                intent: *self,
                canonical_name: "Burst",
                description: "Burst a document into single-page PDFs.",
                aliases: &["burst", "explode", "break apart", "separate all pages", "shatter"],
            },
            Intent::Compare => IntentDefinition {
                intent: *self,
                canonical_name: "Compare",
                description: "Compare two documents for differences.",
                aliases: &["compare", "diff", "differences", "find changes", "compare files"],
            },
            Intent::Compress => IntentDefinition {
                intent: *self,
                canonical_name: "Compress",
                description: "Compress a document to reduce its file size.",
                aliases: &["compress", "shrink", "reduce size", "make smaller", "optimize"],
            },
            Intent::ToMarkdown => IntentDefinition {
                intent: *self,
                canonical_name: "ToMarkdown",
                description: "Convert a document to Markdown format.",
                aliases: &["to markdown", "convert to md", "make markdown", "save as md", "export to markdown"],
            },
            Intent::ToJson => IntentDefinition {
                intent: *self,
                canonical_name: "ToJson",
                description: "Convert a document to JSON format.",
                aliases: &["to json", "convert to json", "make json", "save as json", "export to json"],
            },
            Intent::ToDocx => IntentDefinition {
                intent: *self,
                canonical_name: "ToDocx",
                description: "Convert a document to DOCX format.",
                aliases: &["to docx", "convert to word", "make docx", "save as word", "export to docx", "convert to docx"],
            },
            Intent::ToPptx => IntentDefinition {
                intent: *self,
                canonical_name: "ToPptx",
                description: "Convert a document to PPTX format.",
                aliases: &["to pptx", "convert to powerpoint", "make pptx", "save as pptx", "export to powerpoint", "convert to pptx"],
            },
            Intent::ToHtml => IntentDefinition {
                intent: *self,
                canonical_name: "ToHtml",
                description: "Convert a document to HTML format.",
                aliases: &["to html", "convert to web page", "make html", "save as html", "export to html", "convert to html"],
            },
            Intent::Crop => IntentDefinition {
                intent: *self,
                canonical_name: "Crop",
                description: "Crop pages in a document.",
                aliases: &["crop", "trim", "cut edges", "resize page", "remove margins"],
            },
            Intent::Decrypt => IntentDefinition {
                intent: *self,
                canonical_name: "Decrypt",
                description: "Decrypt a document to remove its password.",
                aliases: &["decrypt", "unlock", "remove password", "unprotect", "clear password"],
            },
            Intent::Delete => IntentDefinition {
                intent: *self,
                canonical_name: "Delete",
                description: "Delete specific pages from a document.",
                aliases: &["delete", "remove pages", "delete pages", "drop pages", "erase pages"],
            },
            Intent::Encrypt => IntentDefinition {
                intent: *self,
                canonical_name: "Encrypt",
                description: "Encrypt a document with a password.",
                aliases: &["encrypt", "lock", "add password", "protect", "secure"],
            },
            Intent::Extract => IntentDefinition {
                intent: *self,
                canonical_name: "Extract",
                description: "Extract specific pages from a document.",
                aliases: &["extract", "extract pages", "pull pages", "get pages", "isolate pages"],
            },
            Intent::ExtractImages => IntentDefinition {
                intent: *self,
                canonical_name: "ExtractImages",
                description: "Extract images embedded in a document.",
                aliases: &["extract images", "rip pictures", "get photos", "pull out graphics", "save images"],
            },
            Intent::ExtractText => IntentDefinition {
                intent: *self,
                canonical_name: "ExtractText",
                description: "Extract text content from a document.",
                aliases: &["extract text", "get text", "read text", "pull text", "grab text"],
            },
            Intent::FormFill => IntentDefinition {
                intent: *self,
                canonical_name: "FormFill",
                description: "Fill a PDF form with data.",
                aliases: &["form fill", "fill form", "complete form", "fill out form", "populate form"],
            },
            Intent::FormRead => IntentDefinition {
                intent: *self,
                canonical_name: "FormRead",
                description: "Read data from a PDF form.",
                aliases: &["form read", "read form", "get form data", "extract form fields", "read fields"],
            },
            Intent::FormCreate => IntentDefinition {
                intent: *self,
                canonical_name: "FormCreate",
                description: "Create a new PDF form.",
                aliases: &["form create", "create form", "make form", "add form fields", "generate form"],
            },
            Intent::Flatten => IntentDefinition {
                intent: *self,
                canonical_name: "Flatten",
                description: "Flatten forms and annotations in a document.",
                aliases: &["flatten", "flatten form", "remove interactives", "bake in annotations", "flatten annotations"],
            },
            Intent::Hash => IntentDefinition {
                intent: *self,
                canonical_name: "Hash",
                description: "Calculate the hash of a document.",
                aliases: &["hash", "checksum", "calculate hash", "verify integrity", "file hash"],
            },
            Intent::HeaderFooter => IntentDefinition {
                intent: *self,
                canonical_name: "HeaderFooter",
                description: "Add headers and footers to a document.",
                aliases: &["header footer", "add header", "add footer", "page numbers", "headers and footers"],
            },
            Intent::ImagesToPdf => IntentDefinition {
                intent: *self,
                canonical_name: "ImagesToPdf",
                description: "Convert a set of images into a PDF document.",
                aliases: &["images to pdf", "jpg to pdf", "convert images", "pictures to pdf", "png to pdf"],
            },
            Intent::Linearize => IntentDefinition {
                intent: *self,
                canonical_name: "Linearize",
                description: "Linearize a document for fast web viewing.",
                aliases: &["linearize", "fast web view", "optimize for web", "web optimize", "fast loading"],
            },
            Intent::Merge => IntentDefinition {
                intent: *self,
                canonical_name: "Merge",
                description: "Merge multiple documents into one.",
                aliases: &["merge", "combine", "join", "append", "concat", "put together"],
            },
            Intent::Metadata => IntentDefinition {
                intent: *self,
                canonical_name: "Metadata",
                description: "View or modify document metadata.",
                aliases: &["metadata", "properties", "document info", "author info", "title info"],
            },
            Intent::Ocr => IntentDefinition {
                intent: *self,
                canonical_name: "Ocr",
                description: "Perform optical character recognition on a document.",
                aliases: &["ocr", "optical character recognition", "recognize text", "scanned text", "make searchable"],
            },
            Intent::PdfA => IntentDefinition {
                intent: *self,
                canonical_name: "PdfA",
                description: "Convert a document to PDF/A for archiving.",
                aliases: &["pdfa", "pdf/a", "archive format", "convert to pdf/a", "make archivable"],
            },
            Intent::Redact => IntentDefinition {
                intent: *self,
                canonical_name: "Redact",
                description: "Redact sensitive information from a document.",
                aliases: &["redact", "black out", "censor", "hide text", "cover up"],
            },
            Intent::Render => IntentDefinition {
                intent: *self,
                canonical_name: "Render",
                description: "Render document pages as images.",
                aliases: &["render", "convert to image", "rasterize", "save as image", "pdf to image"],
            },
            Intent::Reorder => IntentDefinition {
                intent: *self,
                canonical_name: "Reorder",
                description: "Reorder the pages within a document.",
                aliases: &["reorder", "rearrange", "move pages", "shuffle pages", "change page order"],
            },
            Intent::Repair => IntentDefinition {
                intent: *self,
                canonical_name: "Repair",
                description: "Repair a corrupted document.",
                aliases: &["repair", "fix", "recover", "salvage", "restore"],
            },
            Intent::Rotate => IntentDefinition {
                intent: *self,
                canonical_name: "Rotate",
                description: "Rotate pages in a document.",
                aliases: &["rotate", "turn", "spin", "flip", "change orientation"],
            },
            Intent::Search => IntentDefinition {
                intent: *self,
                canonical_name: "Search",
                description: "Search for text within a document.",
                aliases: &["search", "find", "look for", "search text", "query"],
            },
            Intent::Sign => IntentDefinition {
                intent: *self,
                canonical_name: "Sign",
                description: "Apply a digital signature to a document.",
                aliases: &["sign", "signature", "digital signature", "sign document", "add signature"],
            },
            Intent::Split => IntentDefinition {
                intent: *self,
                canonical_name: "Split",
                description: "Split a document into multiple smaller documents.",
                aliases: &["split", "divide", "cut in half", "split pdf", "break into pieces"],
            },
            Intent::Validate => IntentDefinition {
                intent: *self,
                canonical_name: "Validate",
                description: "Validate a document's conformance to standards.",
                aliases: &["validate", "check validity", "verify pdf", "is valid", "validate file"],
            },
            Intent::Watermark => IntentDefinition {
                intent: *self,
                canonical_name: "Watermark",
                description: "Add a watermark to a document.",
                aliases: &["watermark", "add watermark", "stamp", "background text", "overlay text"],
            },
            Intent::Classify => IntentDefinition {
                intent: *self,
                canonical_name: "Classify",
                description: "Classify the type of document.",
                aliases: &["classify", "categorize", "identify type", "what kind of document", "detect type"],
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_all_intents_have_aliases() {
        for intent in Intent::all() {
            let def = intent.definition();
            assert!(!def.aliases.is_empty(), "Intent {:?} has no aliases", intent);
            assert!(def.aliases.len() >= 3, "Intent {:?} needs at least 3 aliases", intent);
        }
    }

    #[test]
    fn test_no_duplicate_aliases() {
        let mut all_aliases = HashSet::new();
        for intent in Intent::all() {
            for &alias in intent.definition().aliases {
                assert!(all_aliases.insert(alias), "Duplicate alias found: {}", alias);
            }
        }
    }
}
