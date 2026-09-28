pub mod group_b;
pub mod group_c;

use crate::cli::Commands;
use paperpilot_core::error::OperationResult;

pub fn handle_command(command: Commands) -> OperationResult<()> {
    match command {
        // Group B
        Commands::Compress { input, output } => group_b::handle_compress(input, output),
        Commands::Repair { input, output } => group_b::handle_repair(input, output),
        Commands::Linearize { input, output } => group_b::handle_linearize(input, output),
        Commands::Encrypt {
            input,
            user_password,
            owner_password,
            output,
        } => group_b::handle_encrypt(input, user_password, owner_password, output),
        Commands::Decrypt {
            input,
            password,
            output,
        } => group_b::handle_decrypt(input, password, output),
        Commands::Watermark {
            input,
            text,
            output,
        } => group_b::handle_watermark(input, text, output),
        Commands::HeaderFooter {
            input,
            text,
            output,
        } => group_b::handle_header_footer(input, text, output),
        Commands::Bates {
            input,
            prefix,
            start,
            output,
        } => group_b::handle_bates(input, prefix, start, output),
        Commands::Metadata {
            input,
            title,
            author,
            subject,
            keywords,
            output,
        } => group_b::handle_metadata(input, title, author, subject, keywords, output),
        Commands::Signature {
            input,
            cert,
            output,
        } => group_b::handle_signature(input, cert, output),
        Commands::Redact {
            input,
            pages,
            rect,
            output,
        } => group_b::handle_redact(input, pages, rect, output),
        Commands::Flatten { input, output } => group_b::handle_flatten(input, output),
        Commands::PdfA { input, output } => group_b::handle_pdf_a(input, output),

        // Group C
        Commands::ExtractText { input, output } => group_c::handle_extract_text(input, output),
        Commands::ExtractImages { input, output } => group_c::handle_extract_images(input, output),
        Commands::ImagesToPdf { images, output } => group_c::handle_images_to_pdf(images, output),
        Commands::Search { input, query } => group_c::handle_search(input, query),
        Commands::Compare { input1, input2 } => group_c::handle_compare(input1, input2),
        Commands::Render { input, output } => group_c::handle_render(input, output),
        Commands::Ocr { input, output } => group_c::handle_ocr(input, output),
        Commands::Bookmarks { input } => group_c::handle_bookmarks(input),
    }
}
