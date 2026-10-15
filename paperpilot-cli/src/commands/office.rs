use clap::Subcommand;
use paperpilot_core::error::OperationResult;
use paperpilot_pdf::operations::office::{
    ExcelToCsvOperation, ExcelToJsonOperation, OfficeExtractTextOperation,
    OfficeModernizeOperation, OfficeToMarkdownOperation,
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
    /// Ingest Excel spreadsheet and output structured JSON
    ExcelToJson {
        #[arg(long, required = true)]
        input: PathBuf,
        #[arg(long)]
        output: Option<PathBuf>,
        #[arg(long)]
        sheet: Option<String>,
        #[arg(long)]
        no_headers: bool,
    },
    /// Export Excel spreadsheet sheet to CSV
    ExcelToCsv {
        #[arg(long, required = true)]
        input: PathBuf,
        #[arg(long, required = true)]
        output: PathBuf,
        #[arg(long)]
        sheet: Option<String>,
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
        OfficeCommands::ExcelToJson {
            input,
            output,
            sheet,
            no_headers,
        } => {
            let json_str = ExcelToJsonOperation::execute(input, sheet.as_deref(), !no_headers)?;
            match output {
                Some(path) => std::fs::write(path, json_str)
                    .map_err(paperpilot_core::error::PdfError::IoError)?,
                None => println!("{}", json_str),
            }
            Ok(())
        }
        OfficeCommands::ExcelToCsv {
            input,
            output,
            sheet,
        } => ExcelToCsvOperation::execute(input, output, sheet.as_deref()),
    }
}
