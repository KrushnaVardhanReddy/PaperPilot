mod api;
mod cli;
mod env;
mod fs;
mod local_agent;

use anyhow::{Context, Result};
use clap::Parser;
use std::path::Path;

fn prepare_prompt(raw_prompt: &str, is_local_agent: bool, force_include: bool, force_exclude: bool) -> String {
    // Determine whether to prepend SYSTEM_PROMPT.md
    // - If force_exclude is true: do not prepend
    // - If force_include is true: prepend
    // - Otherwise: default to true for Jules, default to true for local agent when system_prompt requested or sensible
    let should_include = if force_exclude {
        false
    } else if force_include {
        true
    } else {
        // Default behavior: include by default for cloud Jules
        !is_local_agent
    };

    if should_include {
        format!("{}\n\n---\n\n{}", api::get_system_prompt(), raw_prompt)
    } else {
        raw_prompt.to_string()
    }
}

fn submit_file(
    filepath: &Path,
    label: Option<&str>,
    branch: &str,
    agent: &str,
    interactive: bool,
    dry_run: bool,
    api_key: Option<&str>,
    system_prompt: bool,
    no_system_prompt: bool,
) -> Result<()> {
    if !filepath.exists() {
        anyhow::bail!("❌ File not found: {}", filepath.display());
    }

    let raw_prompt = std::fs::read_to_string(filepath)
        .with_context(|| format!("Failed to read file {}", filepath.display()))?;

    let name = label.unwrap_or_else(|| filepath.file_name().unwrap().to_str().unwrap());
    let is_local = agent == "gemini" || agent == "agy" || agent == "local";
    let prompt_content = prepare_prompt(&raw_prompt, is_local, system_prompt, no_system_prompt);

    // Check target agent engine
    if is_local {
        local_agent::run_local_agent(&prompt_content, name, interactive, dry_run)?;
        if !dry_run {
            fs::archive_prompt(filepath)?;
        }
        return Ok(());
    }

    if dry_run {
        println!("[⏸️  DRY RUN] Would submit to Jules: {}", name);
        let payload = api::build_payload(&prompt_content, branch);
        println!("{}", serde_json::to_string_pretty(&payload).unwrap());
        return Ok(());
    }

    let key = api_key.context("API key required for Jules cloud submission")?;

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

    let is_local_agent = args.agent == "gemini" || args.agent == "agy" || args.agent == "local";

    let api_key = if !args.dry_run && !is_local_agent {
        Some(env::get_jules_api_key()?)
    } else {
        None
    };

    if let Some(filepath) = args.file {
        submit_file(
            Path::new(&filepath),
            None,
            &args.branch,
            &args.agent,
            args.interactive,
            args.dry_run,
            api_key.as_deref(),
            args.system_prompt,
            args.no_system_prompt,
        )?;
        return Ok(());
    }

    if let Some(task_id) = args.task {
        let filepath = fs::find_task(&task_id)?;
        submit_file(
            &filepath,
            Some(&task_id),
            &args.branch,
            &args.agent,
            args.interactive,
            args.dry_run,
            api_key.as_deref(),
            args.system_prompt,
            args.no_system_prompt,
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
                &args.agent,
                args.interactive,
                args.dry_run,
                api_key.as_deref(),
                args.system_prompt,
                args.no_system_prompt,
            )?;
        }
        return Ok(());
    }

    // Default help behavior if no valid flags provided (or clap handles this)
    println!("Please provide a valid command. Run with --help for options.");

    Ok(())
}

