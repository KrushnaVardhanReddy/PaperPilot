use clap::Parser;
use std::process::ExitCode;

mod generator;

#[derive(Parser, Debug)]
#[command(
    name = "penta-scaffold",
    about = "Pure-Rust Penta-Interface Tool Scaffolding Generator"
)]
pub struct ScaffoldArgs {
    #[arg(long, help = "Tool name in snake_case (e.g. pdf_validate_signatures)")]
    name: String,

    #[arg(long, help = "Title of the tool (e.g. Validate Digital Signatures)")]
    title: String,

    #[arg(long, help = "Category (e.g. security, organize, conversion)")]
    category: String,

    #[arg(long, help = "Icon (e.g. 🔏)")]
    icon: String,

    #[arg(long, help = "Input format (e.g. pdf)")]
    input_format: String,

    #[arg(long, help = "Output format (e.g. json, pdf, csv)")]
    output_format: String,

    #[arg(long, help = "Description of the tool")]
    description: String,

    #[arg(long, help = "Print changes without editing files")]
    dry_run: bool,
}

fn main() -> ExitCode {
    let args = ScaffoldArgs::parse();

    match generator::run(args) {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Error generating scaffold: {:?}", e);
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_cli_args_parsing() {
        // Just verify that clap parser parses correctly
        use clap::CommandFactory;
        ScaffoldArgs::command().debug_assert();
    }
}
