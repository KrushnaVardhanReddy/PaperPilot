pub mod annotate;
pub mod group_a;
pub mod group_b;
pub mod group_c;
pub mod group_d;
pub mod group_e;
#[cfg(feature = "office")]
#[cfg(feature = "office")]
pub mod office;
pub mod pdf_attachments;
pub mod pdf_auto_rotate;
pub mod pdf_booklet;
pub mod pdf_change_permissions;
pub mod pdf_comic_archive;
pub mod pdf_nup;
pub mod pdf_overlay_underlay;
pub mod pdf_remove_signatures;
pub mod pdf_scale_pages;
pub mod pdf_split_scanned_book;
pub mod pdf_stamp;
pub mod pdf_stitch_canvas;
pub mod pdf_timestamp;
pub mod pdf_validate_signatures;
pub mod pii;
pub mod sanitize;
pub mod table;
pub mod validate;
pub mod watch;
pub mod workflow;

use crate::cli::Commands;
use paperpilot_core::error::OperationResult;

fn resolve_input_path(path: &std::path::Path) -> OperationResult<std::path::PathBuf> {
    if let Some(s) = path.to_str()
        && (s.starts_with("http://") || s.starts_with("https://"))
    {
        return paperpilot_core::url_fetch::resolve_url_to_temp_file(s);
    }
    Ok(path.to_path_buf())
}

pub fn execute_command(command: &Commands) -> OperationResult<()> {
    match command {
        Commands::Completions { shell } => {
            use clap::CommandFactory;
            let mut app = crate::cli::Cli::command();
            let bin_name = app.get_name().to_string();
            clap_complete::generate(*shell, &mut app, bin_name, &mut std::io::stdout());
            Ok(())
        }
        // Group A
        Commands::Merge { input, output } => {
            let resolved: Result<Vec<_>, _> = input.iter().map(|p| resolve_input_path(p)).collect();
            group_a::handle_merge(&resolved?, output)
        }
        Commands::Split {
            input,
            pages,
            every,
            output,
        } => group_a::handle_split(&resolve_input_path(input)?, pages, *every, output),
        Commands::Extract {
            input,
            pages,
            output,
        } => group_a::handle_extract(&resolve_input_path(input)?, pages, output),
        Commands::Delete {
            input,
            pages,
            output,
        } => group_a::handle_delete(&resolve_input_path(input)?, pages, output),
        Commands::Reorder {
            input,
            order,
            output,
        } => group_a::handle_reorder(&resolve_input_path(input)?, order, output),
        Commands::Rotate {
            input,
            pages,
            degrees,
            output,
        } => group_a::handle_rotate(
            &resolve_input_path(input)?,
            &pages.clone().unwrap_or_else(|| "all".to_string()),
            *degrees,
            output,
        ),
        Commands::Crop {
            input,
            pages,
            rect,
            x,
            y,
            width,
            height,
            output,
        } => {
            let resolved_rect = rect.clone().unwrap_or_else(|| {
                format!(
                    "{},{},{},{}",
                    x.unwrap_or(0.0),
                    y.unwrap_or(0.0),
                    width.unwrap_or(0.0),
                    height.unwrap_or(0.0)
                )
            });
            let resolved_pages = pages.clone().unwrap_or_else(|| "all".to_string());
            group_a::handle_crop(input, &resolved_pages, &resolved_rect, output)
        }
        Commands::Burst { input, output } => {
            group_a::handle_burst(&resolve_input_path(input)?, output)
        }
        Commands::RemoveBlank { input, output } => {
            group_a::handle_remove_blank(&resolve_input_path(input)?, output)
        }

        // Group B
        Commands::Compress {
            input,
            quality,
            output,
        } => group_b::handle_compress(&resolve_input_path(input)?, quality, output),
        Commands::Repair { input, output } => {
            group_b::handle_repair(&resolve_input_path(input)?, output)
        }
        Commands::Linearize { input, output } => {
            group_b::handle_linearize(&resolve_input_path(input)?, output)
        }
        Commands::Encrypt {
            input,
            user_password,
            owner_password,
            output,
        } => group_b::handle_encrypt(
            &resolve_input_path(input)?,
            user_password,
            owner_password,
            output,
        ),
        Commands::Decrypt {
            input,
            password,
            output,
        } => group_b::handle_decrypt(&resolve_input_path(input)?, password, output),
        Commands::Watermark {
            input,
            text,
            angle,
            opacity,
            color,
            font_size,
            output,
        } => group_b::handle_watermark(
            &resolve_input_path(input)?,
            text,
            *angle,
            *opacity,
            color.clone(),
            *font_size,
            output,
        ),
        Commands::Redact {
            input,
            pages,
            rect,
            output,
        } => group_b::handle_redact(&resolve_input_path(input)?, pages, rect, output),
        Commands::PiiScan {
            input,
            output,
            scan_ssn,
            scan_credit_cards,
            scan_emails,
            scan_phones,
            scan_iban,
        } => pii::handle_pii_scan(
            &resolve_input_path(input)?,
            output,
            *scan_ssn,
            *scan_credit_cards,
            *scan_emails,
            *scan_phones,
            *scan_iban,
        ),
        Commands::Sanitize {
            input,
            output,
            remove_metadata,
            remove_attachments,
            remove_scripts,
        } => sanitize::handle_sanitize_command(
            &resolve_input_path(input)?,
            output,
            *remove_metadata,
            *remove_attachments,
            *remove_scripts,
        ),
        Commands::Metadata {
            input,
            title,
            author,
            subject,
            keywords,
            output,
        } => group_b::handle_metadata(
            &resolve_input_path(input)?,
            title,
            author,
            subject,
            keywords,
            output,
        ),
        Commands::Signature {
            input,
            cert,
            output,
        } => group_b::handle_signature(&resolve_input_path(input)?, cert, output),
        Commands::Flatten { input, output } => {
            group_b::handle_flatten(&resolve_input_path(input)?, output)
        }
        Commands::PdfA { input, output } => {
            group_b::handle_pdf_a(&resolve_input_path(input)?, output)
        }
        Commands::HeaderFooter {
            input,
            text,
            output,
        } => group_b::handle_header_footer(&resolve_input_path(input)?, text, output),
        Commands::Bates {
            input,
            prefix,
            start,
            output,
        } => group_b::handle_bates(&resolve_input_path(input)?, prefix, *start, output),
        Commands::PageNumbers {
            input,
            output,
            position,
        } => group_b::handle_page_numbers(&resolve_input_path(input)?, output, position),

        // Group C
        Commands::ExtractText {
            input,
            output,
            format,
        } => group_c::handle_extract_text(&resolve_input_path(input)?, output, format),
        Commands::ExtractImages { input, output } => {
            group_c::handle_extract_images(&resolve_input_path(input)?, output)
        }
        Commands::ExtractTables {
            input,
            output,
            format,
            pages,
        } => table::execute(
            resolve_input_path(input)?,
            output.clone(),
            format.clone(),
            pages.clone(),
        ),
        Commands::Search { input, query } => {
            group_c::handle_search(&resolve_input_path(input)?, query)
        }
        Commands::Render {
            input,
            page: _,
            output,
        } => group_c::handle_render(&resolve_input_path(input)?, output),
        Commands::Compare { input, input_b } => {
            group_c::handle_compare(&resolve_input_path(input)?, &resolve_input_path(input_b)?)
        }
        Commands::Ocr {
            input,
            output,
            format,
            clipboard,
            tier,
        } => group_c::handle_ocr(
            &resolve_input_path(input)?,
            output.as_deref(),
            format,
            *clipboard,
            tier.as_deref(),
        ),
        Commands::Bookmarks { input } => group_c::handle_bookmarks(&resolve_input_path(input)?),
        Commands::ImagesToPdf { images, output } => {
            let resolved: Result<Vec<_>, _> =
                images.iter().map(|p| resolve_input_path(p)).collect();
            group_c::handle_images_to_pdf(&resolved?, output)
        }

        // Validate
        Commands::Annotate {
            input,
            data,
            output,
        } => annotate::handle_annotate(&resolve_input_path(input)?, data, output),
        Commands::Validate { input } => validate::handle_validate(&resolve_input_path(input)?),
        Commands::Hash { input } => validate::handle_hash(&resolve_input_path(input)?),
        Commands::Verify {
            input,
            expected_hash,
        } => validate::handle_verify(&resolve_input_path(input)?, expected_hash),
        // Group D

        // Group E
        Commands::Form { command: form_cmd } => match form_cmd {
            crate::cli::FormCommands::Read { input } => {
                group_e::handle_form_read(&resolve_input_path(input)?)
            }
            crate::cli::FormCommands::Fill {
                input,
                data,
                output,
            } => group_e::handle_form_fill(&resolve_input_path(input)?, data, output),
            crate::cli::FormCommands::AddField {
                input,
                name,
                r#type,
                page,
                rect,
                output,
            } => group_e::handle_form_add_field(
                &resolve_input_path(input)?,
                name,
                r#type,
                *page,
                rect,
                output,
            ),
        },
        Commands::Convert {
            format,
            input,
            output,
            css,
            css_preset,
            inline_css,
        } => group_d::handle_convert(
            format,
            &resolve_input_path(input)?,
            output.as_deref(),
            css.as_deref(),
            css_preset.as_deref(),
            inline_css.as_deref(),
        ),
        Commands::Classify { input } => group_d::handle_classify(&resolve_input_path(input)?),
        Commands::Workflow {
            subcommand,
            input,
            output,
            preset,
            recipe,
        } => workflow::handle_workflow_command(
            subcommand.as_ref(),
            input.as_ref(),
            output.as_ref(),
            preset.as_ref(),
            recipe.as_ref(),
        ),
        Commands::PdfValidateSignatures {
            input,
            output,
            baseline,
        } => {
            pdf_validate_signatures::handle_pdf_validate_signatures_command(input, output, baseline)
        }
        Commands::PdfBooklet { input, output } => {
            pdf_booklet::handle_pdf_booklet_command(input, output)
        }
        Commands::PdfNup { input, output } => pdf_nup::handle_pdf_nup_command(input, output),
        Commands::PdfScalePages { input, output } => {
            pdf_scale_pages::handle_pdf_scale_pages_command(input, output)
        }
        Commands::PdfRemoveSignatures { input, output } => {
            pdf_remove_signatures::handle_pdf_remove_signatures_command(input, output)
        }
        Commands::PdfStamp { input, output } => pdf_stamp::handle_pdf_stamp_command(input, output),
        Commands::PdfOverlayUnderlay { input, output } => {
            pdf_overlay_underlay::handle_pdf_overlay_underlay_command(input, output)
        }
        Commands::PdfAutoRotate { input, output } => {
            pdf_auto_rotate::handle_pdf_auto_rotate_command(input, output)
        }
        Commands::PdfStitchCanvas { input, output } => {
            pdf_stitch_canvas::handle_pdf_stitch_canvas_command(input, output)
        }
        Commands::PdfAttachments { input, output } => {
            pdf_attachments::handle_pdf_attachments_command(input, output)
        }
        Commands::PdfTimestamp { input, output } => {
            pdf_timestamp::handle_pdf_timestamp_command(input, output)
        }
        Commands::PdfChangePermissions { input, output } => {
            pdf_change_permissions::handle_pdf_change_permissions_command(input, output)
        }
        Commands::PdfSplitScannedBook { input, output } => {
            pdf_split_scanned_book::handle_pdf_split_scanned_book_command(input, output)
        }
        Commands::PdfComicArchive { input, output } => {
            pdf_comic_archive::handle_pdf_comic_archive_command(input, output)
        }
        Commands::ListTools => {
            println!("PaperPilot Available Tools (52 Core & Conversion Engines):\n");
            println!("{:<4} {:<24} {:<12} DESCRIPTION", "#", "TOOL ID", "STATUS");
            println!("{}", "-".repeat(80));

            let tools = [
                (
                    "pdf_merge",
                    "Stable (v0.5)",
                    "Merge multiple PDF files into one",
                ),
                (
                    "pdf_split",
                    "Stable (v0.5)",
                    "Split PDF into separate files or ranges",
                ),
                (
                    "pdf_rotate",
                    "Stable (v0.5)",
                    "Rotate pages by 90, 180, or 270 degrees",
                ),
                (
                    "pdf_compress",
                    "Stable (v0.5)",
                    "Optimize and compress PDF file size",
                ),
                (
                    "pdf_extract_pages",
                    "Stable (v0.5)",
                    "Extract specific pages into a new document",
                ),
                (
                    "pdf_delete_pages",
                    "Stable (v0.5)",
                    "Delete selected pages from document",
                ),
                (
                    "pdf_reorder_pages",
                    "Stable (v0.5)",
                    "Reorder document pages by index sequence",
                ),
                (
                    "pdf_crop",
                    "Stable (v0.5)",
                    "Crop pages to specified rectangular margins",
                ),
                (
                    "pdf_flatten",
                    "Stable (v0.5)",
                    "Flatten form fields and annotations into static content",
                ),
                (
                    "pdf_metadata",
                    "Stable (v0.5)",
                    "Read and update PDF metadata fields",
                ),
                (
                    "pdf_watermark",
                    "Stable (v0.5)",
                    "Add text watermarks with custom opacity and angle",
                ),
                (
                    "pdf_info",
                    "Stable (v0.5)",
                    "Display document metadata, page count, and dimensions",
                ),
                (
                    "pdf_extract_text",
                    "Stable (v0.5)",
                    "Extract raw text from PDF documents",
                ),
                (
                    "pdf_extract_images",
                    "Stable (v0.5)",
                    "Extract embedded raster images from document",
                ),
                (
                    "pdf_encrypt",
                    "Stable (v0.5)",
                    "Password protect and encrypt PDF files",
                ),
                (
                    "pdf_decrypt",
                    "Stable (v0.5)",
                    "Remove encryption password from document",
                ),
                (
                    "pdf_burst",
                    "Stable (v0.5)",
                    "Burst document into individual single-page PDFs",
                ),
                (
                    "pdf_remove_blank",
                    "Stable (v0.5)",
                    "Detect and remove empty or blank pages",
                ),
                (
                    "pdf_page_numbers",
                    "Stable (v0.5)",
                    "Apply dynamic page numbers (e.g. Page X of Y)",
                ),
                (
                    "pdf_header_footer",
                    "Stable (v0.5)",
                    "Add formatted headers and footers across pages",
                ),
                (
                    "pdf_bates",
                    "Stable (v0.5)",
                    "Apply legal Bates stamp numbering sequences",
                ),
                (
                    "pdf_linearize",
                    "Stable (v0.5)",
                    "Linearize document for fast web viewing",
                ),
                (
                    "pdf_repair",
                    "Stable (v0.5)",
                    "Repair damaged or corrupted PDF xref tables",
                ),
                (
                    "pdf_search",
                    "Stable (v0.5)",
                    "Search text with regex matching across document",
                ),
                (
                    "pdf_compare",
                    "Stable (v0.5)",
                    "Visual diff between two PDF documents",
                ),
                (
                    "pdf_hash",
                    "Stable (v0.5)",
                    "Compute SHA256 integrity hash of document",
                ),
                (
                    "pdf_validate",
                    "Stable (v0.5)",
                    "Validate conformance with PDF ISO standards",
                ),
                (
                    "pdf_bookmarks",
                    "Stable (v0.5)",
                    "Extract and inspect document bookmark outline",
                ),
                (
                    "pdf_render",
                    "Stable (v0.5)",
                    "Rasterize PDF page to crisp PNG image at custom DPI",
                ),
                (
                    "pdf_images_to_pdf",
                    "Stable (v0.5)",
                    "Assemble JPEG/PNG image sequence into single PDF",
                ),
                (
                    "pdf_annotate",
                    "Stable (v0.5)",
                    "Add annotations, highlights, and notes",
                ),
                (
                    "pdf_sign",
                    "Stable (v0.5)",
                    "Digitally sign PDF with X.509 certificate",
                ),
                (
                    "pdf_to_pdf_a",
                    "Stable (v0.5)",
                    "Convert document to archival PDF/A standard",
                ),
                (
                    "pdf_redact",
                    "Stable (v0.5)",
                    "Permanently sanitize and redact sensitive text",
                ),
                (
                    "pdf_read_form",
                    "Stable (v0.5)",
                    "Read AcroForm interactive form field values",
                ),
                (
                    "pdf_fill_form",
                    "Stable (v0.5)",
                    "Fill AcroForm interactive form fields",
                ),
                (
                    "pdf_create_form_field",
                    "Stable (v0.5)",
                    "Add new interactive form fields to PDF",
                ),
                (
                    "pdf_classify_type",
                    "Stable (v0.5)",
                    "Classify document type (invoice, contract, receipt)",
                ),
                (
                    "pdf_rag_chunks",
                    "Stable (v0.5)",
                    "Chunk document text for AI RAG embedding pipelines",
                ),
                (
                    "pdf_ocr",
                    "Preview",
                    "Client-side neural OCR via ocrs and rten engine",
                ),
                (
                    "pdf_to_markdown",
                    "Preview",
                    "High-fidelity PDF to Markdown with embedded media",
                ),
                (
                    "pdf_to_html",
                    "Preview",
                    "Convert PDF to responsive semantic HTML",
                ),
                (
                    "pdf_to_docx",
                    "Preview",
                    "Decompile PDF to editable Word OpenXML (.docx)",
                ),
                (
                    "pdf_to_xlsx",
                    "Preview",
                    "Extract tabular data to Excel spreadsheet (.xlsx)",
                ),
                (
                    "pdf_to_pptx",
                    "Preview",
                    "Convert PDF slides to PowerPoint presentation (.pptx)",
                ),
                (
                    "pdf_to_json",
                    "Preview",
                    "Export complete PDF layout AST to JSON",
                ),
                (
                    "json_to_pdf",
                    "Preview",
                    "Synthesize styled PDF directly from JSON AST",
                ),
                (
                    "office_to_markdown",
                    "Stable (v0.5)",
                    "Convert DOCX/XLSX/PPTX directly to Markdown",
                ),
                (
                    "office_extract_text",
                    "Stable (v0.5)",
                    "Extract text from Word, Excel, and PowerPoint",
                ),
                (
                    "office_modernize_legacy",
                    "Stable (v0.5)",
                    "Modernize legacy binary DOC/XLS/PPT to modern formats",
                ),
                (
                    "excel_to_json",
                    "Stable (v0.5)",
                    "Ingest Excel spreadsheet to structured JSON",
                ),
                (
                    "excel_to_csv",
                    "Stable (v0.5)",
                    "Export Excel worksheets to standard CSV files",
                ),
                (
                    "pdf_comic_archive",
                    "Preview",
                    "Convert between PDF documents and comic book archive formats (CBZ/CBR).",
                ),
                (
                    "pdf_split_scanned_book",
                    "Preview",
                    "Detect center binding gutter spine and split two-up page scans into single pages.",
                ),
                (
                    "pdf_change_permissions",
                    "Preview",
                    "Configure user and owner permissions for printing, copying, and modification.",
                ),
                (
                    "pdf_timestamp",
                    "Preview",
                    "Apply RFC 3161 cryptographically verified TSA timestamp tokens.",
                ),
                (
                    "pdf_attachments",
                    "Preview",
                    "Inspect, extract, or embed file attachments into PDF portfolios.",
                ),
                (
                    "pdf_stitch_canvas",
                    "Preview",
                    "Combine multiple pages vertically or horizontally into a continuous single-page canvas.",
                ),
                (
                    "pdf_auto_rotate",
                    "Preview",
                    "Heuristically detect text baseline and rotation angle to automatically upright pages.",
                ),
                (
                    "pdf_overlay_underlay",
                    "Preview",
                    "Apply letterheads, watermarked stationary, and templates as background or foreground.",
                ),
                (
                    "pdf_stamp",
                    "Preview",
                    "Place coordinate-exact images, logos, and badges onto PDF pages.",
                ),
                (
                    "pdf_remove_signatures",
                    "Preview",
                    "Strip digital signature fields, cryptographic locks, and widget annotations.",
                ),
                (
                    "pdf_scale_pages",
                    "Preview",
                    "Scale page dimensions between standard formats or percentages.",
                ),
                (
                    "pdf_nup",
                    "Preview",
                    "Combine multiple pages per sheet in a 2, 4, 9, or 16-up grid layout.",
                ),
                (
                    "pdf_booklet",
                    "Preview",
                    "Reorder and impose pages for 2-sided folding booklet printing.",
                ),
                (
                    "pdf_validate_signatures",
                    "Preview",
                    "Cryptographically verify digital signatures, certificate chains, and document integrity.",
                ),
            ];

            for (idx, (id, status, desc)) in tools.iter().enumerate() {
                println!("{:<4} {:<24} {:<12} {}", idx + 1, id, status, desc);
            }
            println!("\nTotal: {} tools registered.", tools.len());
            Ok(())
        }
        #[cfg(feature = "office")]
        #[cfg(feature = "office")]
        Commands::Office { command } => office::handle_office_command(command),

        #[cfg(feature = "daemon")]
        #[cfg(feature = "daemon")]
        Commands::Serve { port, bind } => {
            let rt = tokio::runtime::Runtime::new()
                .map_err(paperpilot_core::error::PdfError::IoError)?;
            rt.block_on(async {
                paperpilot_gateway::server::start(*port, bind, std::future::pending()).await
            })
            .map_err(paperpilot_core::error::PdfError::IoError)?;
            Ok(())
        }
    }
}
