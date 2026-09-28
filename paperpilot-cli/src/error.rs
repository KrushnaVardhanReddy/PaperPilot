use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("PDF processing error: {0}")]
    Pdf(String),
    #[error("Invalid arguments: {0}")]
    InvalidArgument(String),
}
