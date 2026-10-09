mod api;
mod cli;
mod env;
mod fs;

use anyhow::{Context, Result};
use clap::Parser;
use std::path::Path;

fn submit_file(
    filepath: &Path,
    label: Option<&str>,
    branch: &str,
    dry_run: bool,
    api_key: Option<&str>,
) -> Result<()> {
    if !filepath.exists() {
        anyhow::bail!("❌ File not found: {}", filepath.display());
    }

    let prompt_content = std::fs::read_to_string(filepath)
        .with_context(|| format!("Failed to read file {}", filepath.display()))?;

    let name = label.unwrap_or_else(|| filepath.file_name().unwrap().to_str().unwrap());

    if dry_run {
        println!("[⏸️  DRY RUN] Would submit: {}", name);
        let payload = api::build_payload(&prompt_content, branch);
        println!("{}", serde_json::to_string_pretty(&payload).unwrap());
        return Ok(());
    }

    let key = api_key.context("API key required for submission")?;

    match api::submit_prompt(key, &prompt_content, name, branch) {
        Ok(_) => {
            // Archive prompt
            fs::archive_prompt(filepath)?;
            Ok(())
        }
        Err(e) => {
            anyhow::bail!("Failed to submit prompt: {}", e);
        }
    }
}

fn main() -> Result<()> {
    let args = cli::Cli::parse();

    if args.list {
        let tasks = fs::list_pending_tasks()?;
        if tasks.is_empty() {
            println!("⚠️  No pending task prompt files found.");
            return Ok(());
        }

        println!("\n📋 Pending tasks ({} total):\n", tasks.len());
        let tasks_dir = fs::get_tasks_dir()?;
        for task in tasks {
            let rel = task.strip_prefix(&tasks_dir).unwrap_or(&task);
            println!("   {}", rel.display());
        }
        println!();
        return Ok(());
    }

    let api_key = if !args.dry_run {
        Some(env::get_jules_api_key()?)
    } else {
        None
    };

    if let Some(filepath) = args.file {
        submit_file(
            Path::new(&filepath),
            None,
            &args.branch,
            args.dry_run,
            api_key.as_deref(),
        )?;
        return Ok(());
    }

    if let Some(task_id) = args.task {
        let filepath = fs::find_task(&task_id)?;
        submit_file(
            &filepath,
            Some(&task_id),
            &args.branch,
            args.dry_run,
            api_key.as_deref(),
        )?;
        return Ok(());
    }

    if let Some(phase_num) = args.phase {
        let files = fs::find_phase_tasks(phase_num)?;
        let action = if args.dry_run {
            "Previewing"
        } else {
            "Submitting"
        };
        println!(
            "📅 {} {} pending task(s) for Phase {}...",
            action,
            files.len(),
            phase_num
        );

        for filepath in files {
            let label = filepath.file_name().unwrap().to_str().unwrap();
            submit_file(
                &filepath,
                Some(label),
                &args.branch,
                args.dry_run,
                api_key.as_deref(),
            )?;
        }
        return Ok(());
    }

    // Default help behavior if no valid flags provided (or clap handles this)
    println!("Please provide a valid command. Run with --help for options.");

    Ok(())
}
