use clap::Subcommand;
use paperpilot_core::error::OperationResult;
use paperpilot_pdf::operations::office::{
    OfficeExtractTextOperation, OfficeModernizeOperation, OfficeToMarkdownOperation,
};
use std::path::PathBuf;

#[derive(Subcommand, Debug)]
pub enum OfficeCommands {
    /// Convert an Office document (DOCX, XLSX, PPTX) to Markdown
    ToMarkdown {
        #[arg(long, required = true)]
        input: PathBuf,
        #[arg(long, required = true)]
        output: PathBuf,
    },
    /// Extract text from an Office document
    Extract {
        #[arg(long, required = true)]
        input: PathBuf,
        #[arg(long, required = true)]
        output: PathBuf,
    },
    /// Modernize legacy binary Office documents (DOC/XLS/PPT) to modern formats
    Modernize {
        #[arg(long, required = true)]
        input: PathBuf,
        #[arg(long, required = true)]
        output: PathBuf,
    },
}

pub fn handle_office_command(command: &OfficeCommands) -> OperationResult<()> {
    match command {
        OfficeCommands::ToMarkdown { input, output } => {
            OfficeToMarkdownOperation::execute(input, output)
        }
        OfficeCommands::Extract { input, output } => {
            OfficeExtractTextOperation::execute(input, output)
        }
        OfficeCommands::Modernize { input, output } => {
            OfficeModernizeOperation::execute(input, output)
        }
    }
}
