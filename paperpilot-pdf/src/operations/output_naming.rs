//! Validated local PDF names. Tokens derive from the actual input buffer.
use chrono::{
    DateTime, Utc,
    format::{Item, StrftimeItems},
};
use paperpilot_core::error::{OperationResult, PdfError};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fmt::Write as _;
use std::path::{Component, Path};

fn invalid(message: impl Into<String>) -> PdfError {
    PdfError::InvalidInput(message.into())
}
fn forbidden(c: char) -> bool {
    c.is_control() || matches!(c, '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|')
}
fn clean(s: &str) -> String {
    s.chars()
        .map(|c| if forbidden(c) { '_' } else { c })
        .collect()
}
#[derive(Debug, Clone)]
enum Token {
    Literal(String),
    InputName,
    InputStem,
    InputExt,
    Date(String),
    Hash(usize),
    Index(usize),
    Metadata(String),
}
#[derive(Debug, Clone)]
pub struct OutputNameTemplate {
    tokens: Vec<Token>,
}
impl OutputNameTemplate {
    /// Reject unknown tokens and unsafe patterns before starting a watcher.
    pub fn parse(pattern: &str) -> OperationResult<Self> {
        if pattern.is_empty() || pattern.len() > 1024 {
            return Err(invalid("Name template must contain 1..1024 UTF-8 bytes"));
        }
        let mut tokens = Vec::new();
        let mut rest = pattern;
        while !rest.is_empty() {
            let Some(start) = rest.find('{') else {
                if rest.contains('}') {
                    return Err(invalid("Unmatched naming brace"));
                }
                tokens.push(Token::Literal(rest.into()));
                break;
            };
            let literal = &rest[..start];
            if literal.contains('}') {
                return Err(invalid("Unmatched naming brace"));
            }
            tokens.push(Token::Literal(literal.into()));
            let end = rest[start + 1..]
                .find('}')
                .ok_or_else(|| invalid("Unclosed naming token"))?
                + start
                + 1;
            let word = &rest[start + 1..end];
            let token = match word {
                "input_name" => Token::InputName,
                "input_stem" => Token::InputStem,
                "input_ext" => Token::InputExt,
                "date" => Token::Date("%Y-%m-%d".into()),
                "hash" => Token::Hash(64),
                "index" => Token::Index(0),
                _ if word.starts_with("metadata.") => {
                    let key = &word[9..];
                    if !matches!(
                        key,
                        "title" | "author" | "subject" | "keywords" | "creator" | "producer"
                    ) {
                        return Err(invalid("Unsupported PDF metadata naming field"));
                    }
                    Token::Metadata(key.into())
                }
                _ if word.starts_with("date:") => {
                    let fmt = &word[5..];
                    if fmt.is_empty()
                        || StrftimeItems::new(fmt).any(|item| matches!(item, Item::Error))
                    {
                        return Err(invalid("Invalid date format"));
                    }
                    Token::Date(fmt.into())
                }
                _ if word.starts_with("hash:") => {
                    let width = word[5..]
                        .parse::<usize>()
                        .map_err(|_| invalid("Invalid hash width"))?;
                    if !(4..=64).contains(&width) {
                        return Err(invalid("Hash width must be 4..64"));
                    }
                    Token::Hash(width)
                }
                _ if word.starts_with("index:") => {
                    let width = word[6..]
                        .parse::<usize>()
                        .map_err(|_| invalid("Invalid index width"))?;
                    if width > 12 {
                        return Err(invalid("Index padding must not exceed 12"));
                    }
                    Token::Index(width)
                }
                _ => return Err(invalid(format!("Unsupported naming token: {{{word}}}"))),
            };
            tokens.push(token);
            rest = &rest[end + 1..];
        }
        let template = Self { tokens };
        let probe = [
            "title", "author", "subject", "keywords", "creator", "producer",
        ]
        .into_iter()
        .map(|key| (key.to_string(), "value".into()))
        .collect();
        template.render_context(
            Path::new("input.pdf"),
            &"a".repeat(64),
            Utc::now(),
            1,
            &probe,
        )?;
        Ok(template)
    }
    pub fn render(
        &self,
        input: &Path,
        sha256: &str,
        date: DateTime<Utc>,
        index: u64,
    ) -> OperationResult<String> {
        self.render_context(input, sha256, date, index, &HashMap::new())
    }

    /// Hash and metadata are derived from exactly the buffer the runner executes.
    pub fn render_pdf(
        &self,
        input: &Path,
        bytes: &[u8],
        date: DateTime<Utc>,
        index: u64,
    ) -> OperationResult<String> {
        let sha256: String = Sha256::digest(bytes)
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect();
        let mut metadata = HashMap::new();
        if self
            .tokens
            .iter()
            .any(|token| matches!(token, Token::Metadata(_)))
        {
            let doc = lopdf::Document::load_mem(bytes)
                .map_err(|e| invalid(format!("Cannot read naming metadata: {e}")))?;
            if doc.is_encrypted() {
                return Err(invalid("Metadata naming requires a decrypted input PDF"));
            }
            if let Ok(info) = doc.trailer.get(b"Info") {
                let (_, info) = doc.dereference(info).map_err(|e| invalid(e.to_string()))?;
                let info = info.as_dict().map_err(|e| invalid(e.to_string()))?;
                for token in &self.tokens {
                    if let Token::Metadata(key) = token {
                        let pdf_key = match key.as_str() {
                            "title" => "Title",
                            "author" => "Author",
                            "subject" => "Subject",
                            "keywords" => "Keywords",
                            "creator" => "Creator",
                            _ => "Producer",
                        };
                        if let Ok(value) = info.get(pdf_key.as_bytes()) {
                            let (_, value) =
                                doc.dereference(value).map_err(|e| invalid(e.to_string()))?;
                            let value = lopdf::decode_text_string(value)
                                .map_err(|e| invalid(e.to_string()))?;
                            metadata.insert(key.clone(), value);
                        }
                    }
                }
            }
        }
        self.render_context(input, &sha256, date, index, &metadata)
    }

    fn render_context(
        &self,
        input: &Path,
        sha256: &str,
        date: DateTime<Utc>,
        index: u64,
        metadata: &HashMap<String, String>,
    ) -> OperationResult<String> {
        if sha256.len() != 64 || !sha256.bytes().all(|c| c.is_ascii_hexdigit()) {
            return Err(invalid("Naming requires a SHA-256 digest"));
        }
        let part = |p: Option<&std::ffi::OsStr>| clean(&p.unwrap_or_default().to_string_lossy());
        let mut name = String::new();
        for token in &self.tokens {
            name += &match token {
                Token::Literal(s) => s.clone(),
                Token::InputName => part(input.file_name()),
                Token::InputStem => part(input.file_stem()),
                Token::InputExt => part(input.extension()),
                Token::Date(fmt) => {
                    let mut value = String::new();
                    write!(&mut value, "{}", date.format(fmt))
                        .map_err(|_| invalid("Unsupported output date format"))?;
                    value
                }
                Token::Hash(n) => sha256[..*n].to_ascii_lowercase(),
                Token::Index(width) => format!("{index:0width$}"),
                Token::Metadata(key) => clean(
                    metadata
                        .get(key)
                        .filter(|value| !value.trim().is_empty())
                        .ok_or_else(|| invalid(format!("Missing PDF metadata field: {key}")))?,
                ),
            };
        }
        validate_name(&name)?;
        Ok(name)
    }
}
fn validate_name(name: &str) -> OperationResult<()> {
    let stem = name
        .split('.')
        .next()
        .unwrap_or("")
        .trim_end_matches([' ', '.'])
        .to_ascii_uppercase();
    let reserved = matches!(stem.as_str(), "CON" | "PRN" | "AUX" | "NUL")
        || (stem.len() == 4
            && (stem.starts_with("COM") || stem.starts_with("LPT"))
            && matches!(stem.as_bytes()[3], b'1'..=b'9'));
    let mut parts = Path::new(name).components();
    if name.len() > 240
        || name.chars().any(forbidden)
        || reserved
        || name.ends_with([' ', '.'])
        || !matches!(parts.next(), Some(Component::Normal(_)))
        || parts.next().is_some()
        || !name.to_ascii_lowercase().ends_with(".pdf")
    {
        return Err(invalid(
            "Output must be a portable single .pdf basename of at most 240 UTF-8 bytes",
        ));
    }
    Ok(())
}
