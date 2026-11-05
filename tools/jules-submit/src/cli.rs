use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// List all pending task prompts under `prompts/tasks/`
    #[arg(long)]
    pub list: bool,

    /// Submit a specific prompt file
    #[arg(long)]
    pub file: Option<String>,

    /// Fuzzy-find by task ID (e.g. `P1-T1` or `P5_7_1`)
    #[arg(long)]
    pub task: Option<String>,

    /// Batch-submit all pending prompts in `prompts/tasks/phase{N}/`
    #[arg(long)]
    pub phase: Option<u32>,

    /// Target Git starting branch
    #[arg(long, default_value = "main")]
    pub branch: String,

    /// Format payload and display target without making network call
    #[arg(long)]
    pub dry_run: bool,

    /// Target agent engine ('jules' for remote cloud session, 'gemini' / 'agy' for local pair programmer)
    #[arg(long, default_value = "jules")]
    pub agent: String,

    /// Interactive mode for local agent (launches agy in interactive pair programming session)
    #[arg(long)]
    pub interactive: bool,
}

