use std::path::Path;
use std::io::{Error, ErrorKind};

pub fn workspace_search(path_str: &str, _query: &str) -> Result<Vec<String>, Error> {
    let path = Path::new(path_str);
    let mut results = Vec::new();

    if path.is_dir() {
        // Walk directory recursively for documents (.pdf, .docx, .xlsx, .md, .txt)
        let walker = walkdir::WalkDir::new(path).into_iter();
        for entry in walker.filter_map(|e: Result<walkdir::DirEntry, walkdir::Error>| e.ok()) {
            let p = entry.path();
            if p.is_file() {
                if let Some(ext) = p.extension() {
                    let ext_str = ext.to_string_lossy().to_lowercase();
                    if ["pdf", "docx", "xlsx", "md", "txt"].contains(&ext_str.as_str()) {
                        results.push(p.to_string_lossy().into_owned());
                    }
                }
            }
        }
    } else if path.is_file() {
        // Search single file or index
        results.push(path.to_string_lossy().into_owned());
    } else {
        return Err(Error::new(ErrorKind::NotFound, "Path not found"));
    }

    Ok(results)
}
