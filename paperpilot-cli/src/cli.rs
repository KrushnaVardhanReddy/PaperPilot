use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Enable JSON output
    #[arg(long, global = true)]
    pub json: bool,

    /// URL to POST JSON payload to
    #[arg(long, global = true)]
    pub webhook: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Validates a PDF file
    Validate {
        /// The path to the PDF file to validate
        path: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parsing_validate() {
        let args = vec!["paperpilot", "--json", "validate", "test.pdf"];
        let cli = Cli::parse_from(args);

        assert!(cli.json);
        assert_eq!(cli.webhook, None);

        match cli.command {
            Commands::Validate { path } => {
                assert_eq!(path, "test.pdf");
            }
        }
    }

    #[test]
    fn test_cli_parsing_webhook() {
        let args = vec![
            "paperpilot",
            "--webhook",
            "http://example.com",
            "validate",
            "test.pdf",
        ];
        let cli = Cli::parse_from(args);

        assert!(!cli.json);
        assert_eq!(cli.webhook, Some("http://example.com".to_string()));
    }
}
