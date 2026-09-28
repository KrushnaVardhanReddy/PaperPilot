pub mod group_a;
pub mod group_b;
pub mod group_c;
pub mod validate;

use crate::cli::Commands;
use paperpilot_core::error::OperationResult;

pub fn execute_command(command: &Commands) -> OperationResult<()> {
    match command {
        // Group A
        Commands::Merge { input, output } => group_a::handle_merge(input, output),
        Commands::Split { input, pages, output } => group_a::handle_split(input, pages, output),
        Commands::Extract { input, pages, output } => group_a::handle_extract(input, pages, output),
        Commands::Delete { input, pages, output } => group_a::handle_delete(input, pages, output),
        Commands::Reorder { input, order, output } => group_a::handle_reorder(input, order, output),
        Commands::Rotate { input, pages, degrees, output } => group_a::handle_rotate(input, pages, *degrees, output),
        Commands::Crop { input, pages, rect, output } => group_a::handle_crop(input, pages, rect, output),
        Commands::Burst { input, output } => group_a::handle_burst(input, output),

        // Group B
        Commands::Compress { input, quality, output } => group_b::handle_compress(input, quality, output),
        Commands::Repair { input, output } => group_b::handle_repair(input, output),
        Commands::Linearize { input, output } => group_b::handle_linearize(input, output),
        Commands::Encrypt { input, user_password, owner_password, output } => group_b::handle_encrypt(input, user_password, owner_password, output),
        Commands::Decrypt { input, password, output } => group_b::handle_decrypt(input, password, output),
        Commands::Watermark { input, text, output } => group_b::handle_watermark(input, text, output),
        Commands::Redact { input, pages, rect, output } => group_b::handle_redact(input, pages, rect, output),
        Commands::Metadata { input, title, author, subject, keywords, output } => group_b::handle_metadata(input, title, author, subject, keywords, output),
        Commands::Signature { input, cert, output } => group_b::handle_signature(input, cert, output),
        Commands::Flatten { input, output } => group_b::handle_flatten(input, output),
        Commands::PdfA { input, output } => group_b::handle_pdf_a(input, output),
        Commands::HeaderFooter { input, text, output } => group_b::handle_header_footer(input, text, output),
        Commands::Bates { input, prefix, start, output } => group_b::handle_bates(input, prefix, *start, output),

        // Group C
        Commands::ExtractText { input, output } => group_c::handle_extract_text(input, output),
        Commands::ExtractImages { input, output } => group_c::handle_extract_images(input, output),
        Commands::Search { input, query } => group_c::handle_search(input, query),
        Commands::Render { input, output } => group_c::handle_render(input, output),
        Commands::Compare { input, input_b } => group_c::handle_compare(input, input_b),
        Commands::Ocr { input, output } => group_c::handle_ocr(input, output),
        Commands::Bookmarks { input } => group_c::handle_bookmarks(input),
        Commands::ImagesToPdf { images, output } => group_c::handle_images_to_pdf(images, output),

        // Validate
        Commands::Validate { input } => validate::handle_validate(input),
    }
}
