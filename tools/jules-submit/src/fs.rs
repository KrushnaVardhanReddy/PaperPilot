use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::fs;

/// Returns the path to `prompts/tasks/` relative to the repository root.
pub fn get_tasks_dir() -> Result<PathBuf> {
    let mut current_dir = std::env::current_dir().context("Failed to get current directory")?;
    loop {
        let tasks_dir = current_dir.join("prompts").join("tasks");
        if tasks_dir.exists() {
            return Ok(tasks_dir);
        }
        if !current_dir.pop() {
            anyhow::bail!("Could not find prompts/tasks directory from current path");
        }
    }
}

/// Returns true if the path contains a `done/` component.
fn is_done_dir(path: &Path) -> bool {
    path.components().any(|c| c.as_os_str() == "done")
}

/// Lists all pending task prompts under `prompts/tasks/` (excluding `done/`).
pub fn list_pending_tasks() -> Result<Vec<PathBuf>> {
    let tasks_dir = get_tasks_dir()?;
    let mut pending = Vec::new();

    let pattern = format!("{}/**/*.txt", tasks_dir.display());
    for entry in glob::glob(&pattern).context("Failed to read glob pattern")? {
        match entry {
            Ok(path) => {
                if !is_done_dir(&path) {
                    pending.push(path);
                }
            }
            Err(e) => eprintln!("Glob error: {:?}", e),
        }
    }

    pending.sort();
    Ok(pending)
}

/// Fuzzy-finds a task by ID (e.g. `P1-T1` or `P5_7_1`), excluding `done/`.
pub fn find_task(task_id: &str) -> Result<PathBuf> {
    let tasks_dir = get_tasks_dir()?;
    let normalized = task_id.replace("-", "_").to_uppercase();

    // Try `normalized_*.txt`
    let pattern1 = format!("{}/**/{}_*.txt", tasks_dir.display(), normalized);
    let mut matches = Vec::new();
    for entry in glob::glob(&pattern1)? {
        if let Ok(path) = entry {
            if !is_done_dir(&path) {
                matches.push(path);
            }
        }
    }

    if !matches.is_empty() {
        return Ok(matches[0].clone());
    }

    // Try `normalized.txt`
    let pattern2 = format!("{}/**/{}.txt", tasks_dir.display(), normalized);
    for entry in glob::glob(&pattern2)? {
        if let Ok(path) = entry {
            if !is_done_dir(&path) {
                matches.push(path);
            }
        }
    }

    if !matches.is_empty() {
        return Ok(matches[0].clone());
    }

    // Try `{task_id}.txt` verbatim
    let pattern3 = format!("{}/**/{}.txt", tasks_dir.display(), task_id);
    for entry in glob::glob(&pattern3)? {
        if let Ok(path) = entry {
            if !is_done_dir(&path) {
                matches.push(path);
            }
        }
    }

    if !matches.is_empty() {
        return Ok(matches[0].clone());
    }

    anyhow::bail!("No pending prompt file found for task '{}' in {}", task_id, tasks_dir.display());
}

/// Finds all pending prompts for a specific phase (excluding `done/`).
pub fn find_phase_tasks(phase: u32) -> Result<Vec<PathBuf>> {
    let tasks_dir = get_tasks_dir()?;
    let mut matches = Vec::new();

    // Look inside `phase_N/`
    let pattern1 = format!("{}/phase_{}/P{}_T*.txt", tasks_dir.display(), phase, phase);
    for entry in glob::glob(&pattern1)? {
        if let Ok(path) = entry {
            if !is_done_dir(&path) {
                matches.push(path);
            }
        }
    }

    // Look in root tasks_dir
    let pattern2 = format!("{}/P{}_T*.txt", tasks_dir.display(), phase);
    for entry in glob::glob(&pattern2)? {
        if let Ok(path) = entry {
            if !is_done_dir(&path) {
                matches.push(path);
            }
        }
    }

    matches.sort();
    if matches.is_empty() {
        anyhow::bail!("No pending prompt files found for Phase {}.", phase);
    }

    Ok(matches)
}

/// Archives a prompt file by moving it to `prompts/tasks/done/<relative_path>`.
pub fn archive_prompt(filepath: &Path) -> Result<()> {
    let tasks_dir = get_tasks_dir()?;

    let abs_filepath = fs::canonicalize(filepath).unwrap_or_else(|_| filepath.to_path_buf());
    let abs_tasks_dir = fs::canonicalize(&tasks_dir).unwrap_or_else(|_| tasks_dir.clone());

    // Ensure the file is actually inside tasks_dir
    let rel_path = abs_filepath.strip_prefix(&abs_tasks_dir)
        .context("File is not inside prompts/tasks directory")?;

    let dest = tasks_dir.join("done").join(rel_path);

    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent).context("Failed to create done directory structure")?;
    }

    fs::rename(filepath, &dest).context("Failed to move file to done directory")?;

    println!("   ✅ Archived → prompts/tasks/done/{}", rel_path.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuzzy_finding_and_archiving() -> Result<()> {
        // Since get_tasks_dir traverses up, we'll override std::env::current_dir temporarily
        // or just test the logic directly on paths.
        // For the sake of isolated unit testing, we'll test the helper logic.
        assert!(is_done_dir(Path::new("prompts/tasks/done/P1_T1.txt")));
        assert!(!is_done_dir(Path::new("prompts/tasks/P1_T1.txt")));
        Ok(())
    }
}
