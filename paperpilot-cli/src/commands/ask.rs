use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_nlp::resolver::OfflineNlpResolver;
use paperpilot_nlp::traits::{NlpResolver, ResolverContext};
use std::io::Write;
use std::path::Path;

pub fn handle_ask(
    prompt: &str,
    input: Option<&Path>,
    yes: bool,
    _offline: bool,
    output_override: Option<&Path>,
    dry_run: bool,
    json: bool,
) -> OperationResult<()> {
    let mut context = ResolverContext::default();
    if let Some(input_path) = input {
        context.active_document = Some(input_path.to_string_lossy().to_string());
    }

    let resolver = OfflineNlpResolver::new();
    let mut plan = resolver
        .resolve_with_context(prompt, &context)
        .map_err(|e| {
            PdfError::InvalidInput(format!("Failed to resolve natural language query: {}", e))
        })?;

    if let Some(out) = output_override {
        plan.output_file = Some(out.to_string_lossy().to_string());
    }

    if dry_run {
        if json {
            let out = serde_json::to_string_pretty(&plan).unwrap();
            println!("{}", out);
        } else {
            println!("📋 Interpreted Plan:");
            println!("  • Intent: {:?}", plan.intent);
            println!("  • Inputs: {:?}", plan.input_files);
            if let Some(out) = &plan.output_file {
                println!("  • Output: {}", out);
            }
            if !plan.page_ranges.is_empty() {
                println!("  • Pages: {:?}", plan.page_ranges);
            }
            if !plan.angles.is_empty() {
                println!("  • Angles: {:?}", plan.angles);
            }
        }
        return Ok(());
    }

    if !yes {
        println!("📋 Interpreted Plan:");
        println!("  • Intent: {:?}", plan.intent);
        println!("  • Inputs: {:?}", plan.input_files);
        if let Some(out) = &plan.output_file {
            println!("  • Output: {}", out);
        }
        print!("Execute this plan? [Y/n]: ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let ans = input.trim().to_lowercase();
        if ans == "n" || ans == "no" {
            return Err(PdfError::InvalidInput(
                "Execution aborted by user".to_string(),
            ));
        }
    }

    let inputs_paths: Vec<std::path::PathBuf> = plan
        .input_files
        .iter()
        .map(std::path::PathBuf::from)
        .collect();
    let output_path = plan.output_file.as_ref().map(std::path::PathBuf::from);

    use paperpilot_nlp::intent::Intent;
    #[allow(unreachable_patterns)]
    match plan.intent {
        Intent::Merge => {
            if inputs_paths.is_empty() {
                return Err(PdfError::InvalidInput("Merge requires input files".into()));
            }
            let out = output_path.unwrap_or_else(|| std::path::PathBuf::from("merged.pdf"));
            crate::commands::group_a::handle_merge(&inputs_paths, &out)
        }
        Intent::Split => {
            if inputs_paths.is_empty() {
                return Err(PdfError::InvalidInput(
                    "Split requires an input file".into(),
                ));
            }
            let out = output_path.unwrap_or_else(|| std::path::PathBuf::from("split_out"));
            let pages = plan.page_ranges.first().cloned();
            crate::commands::group_a::handle_split(
                &inputs_paths[0],
                pages.as_deref().unwrap_or("1"),
                None,
                &out,
            )
        }
        Intent::Extract => {
            if inputs_paths.is_empty() {
                return Err(PdfError::InvalidInput(
                    "Extract requires an input file".into(),
                ));
            }
            let out = output_path.unwrap_or_else(|| std::path::PathBuf::from("extract_out.pdf"));
            let pages = plan
                .page_ranges
                .first()
                .cloned()
                .unwrap_or_else(|| "1".into());
            crate::commands::group_a::handle_extract(&inputs_paths[0], &pages, &out)
        }
        Intent::Delete => {
            if inputs_paths.is_empty() {
                return Err(PdfError::InvalidInput(
                    "Delete requires an input file".into(),
                ));
            }
            let out = output_path.unwrap_or_else(|| std::path::PathBuf::from("delete_out.pdf"));
            let pages = plan
                .page_ranges
                .first()
                .cloned()
                .unwrap_or_else(|| "1".into());
            crate::commands::group_a::handle_delete(&inputs_paths[0], &pages, &out)
        }
        Intent::Reorder => {
            if inputs_paths.is_empty() {
                return Err(PdfError::InvalidInput(
                    "Reorder requires an input file".into(),
                ));
            }
            let out = output_path.unwrap_or_else(|| std::path::PathBuf::from("reorder_out.pdf"));
            let pages = plan
                .page_ranges
                .first()
                .cloned()
                .unwrap_or_else(|| "1".into());
            crate::commands::group_a::handle_reorder(&inputs_paths[0], &pages, &out)
        }
        Intent::Rotate => {
            if inputs_paths.is_empty() {
                return Err(PdfError::InvalidInput(
                    "Rotate requires an input file".into(),
                ));
            }
            let out = output_path.unwrap_or_else(|| std::path::PathBuf::from("rotate_out.pdf"));
            let angle = plan.angles.first().copied().unwrap_or(90);
            let pages = plan.page_ranges.first().cloned();
            crate::commands::group_a::handle_rotate(
                &inputs_paths[0],
                pages.as_deref().unwrap_or("1-end"),
                angle,
                &out,
            )
        }
        Intent::Crop => {
            if inputs_paths.is_empty() {
                return Err(PdfError::InvalidInput("Crop requires an input file".into()));
            }
            let out = output_path.unwrap_or_else(|| std::path::PathBuf::from("crop_out.pdf"));
            let pages = plan.page_ranges.first().cloned();
            crate::commands::group_a::handle_crop(
                &inputs_paths[0],
                pages.as_deref().unwrap_or("1-end"),
                "0,0,10,10",
                &out,
            )
        }
        Intent::Burst => {
            if inputs_paths.is_empty() {
                return Err(PdfError::InvalidInput(
                    "Burst requires an input file".into(),
                ));
            }
            let out = output_path.unwrap_or_else(|| std::path::PathBuf::from("burst_out"));
            crate::commands::group_a::handle_burst(&inputs_paths[0], &out)
        }
        Intent::Compress => {
            if inputs_paths.is_empty() {
                return Err(PdfError::InvalidInput(
                    "Compress requires an input file".into(),
                ));
            }
            let out = output_path.unwrap_or_else(|| std::path::PathBuf::from("compress_out.pdf"));
            crate::commands::group_b::handle_compress(&inputs_paths[0], "low", &out)
        }
        Intent::Repair => {
            if inputs_paths.is_empty() {
                return Err(PdfError::InvalidInput(
                    "Repair requires an input file".into(),
                ));
            }
            let out = output_path.unwrap_or_else(|| std::path::PathBuf::from("repair_out.pdf"));
            crate::commands::group_b::handle_repair(&inputs_paths[0], &out)
        }
        Intent::Linearize => {
            if inputs_paths.is_empty() {
                return Err(PdfError::InvalidInput(
                    "Linearize requires an input file".into(),
                ));
            }
            let out = output_path.unwrap_or_else(|| std::path::PathBuf::from("linearize_out.pdf"));
            crate::commands::group_b::handle_linearize(&inputs_paths[0], &out)
        }
        Intent::Encrypt => {
            if inputs_paths.is_empty() {
                return Err(PdfError::InvalidInput(
                    "Encrypt requires an input file".into(),
                ));
            }
            let out = output_path.unwrap_or_else(|| std::path::PathBuf::from("encrypt_out.pdf"));
            let pw = plan
                .passwords
                .first()
                .cloned()
                .unwrap_or_else(|| "password".into());
            crate::commands::group_b::handle_encrypt(
                &inputs_paths[0],
                &Some(pw.clone()),
                &Some(pw),
                &out,
            )
        }
        Intent::Decrypt => {
            if inputs_paths.is_empty() {
                return Err(PdfError::InvalidInput(
                    "Decrypt requires an input file".into(),
                ));
            }
            let out = output_path.unwrap_or_else(|| std::path::PathBuf::from("decrypt_out.pdf"));
            let pw = plan.passwords.first().cloned().unwrap_or_else(|| "".into());
            crate::commands::group_b::handle_decrypt(&inputs_paths[0], &Some(pw), &out)
        }
        Intent::Watermark => {
            if inputs_paths.is_empty() {
                return Err(PdfError::InvalidInput(
                    "Watermark requires an input file".into(),
                ));
            }
            let out = output_path.unwrap_or_else(|| std::path::PathBuf::from("watermark_out.pdf"));
            crate::commands::group_b::handle_watermark(
                &inputs_paths[0],
                "Watermark",
                0.0,
                1.0,
                None,
                None,
                &out,
            )
        }
        Intent::Redact => {
            if inputs_paths.is_empty() {
                return Err(PdfError::InvalidInput(
                    "Redact requires an input file".into(),
                ));
            }
            let out = output_path.unwrap_or_else(|| std::path::PathBuf::from("redact_out.pdf"));
            crate::commands::group_b::handle_redact(&inputs_paths[0], "redact", "0,0,10,10", &out)
        }
        Intent::Metadata => {
            if inputs_paths.is_empty() {
                return Err(PdfError::InvalidInput(
                    "Metadata requires an input file".into(),
                ));
            }
            crate::commands::group_b::handle_metadata(
                &inputs_paths[0],
                &None,
                &None,
                &None,
                &None,
                &inputs_paths[0],
            )
        }
        Intent::Sign => {
            if inputs_paths.is_empty() {
                return Err(PdfError::InvalidInput("Sign requires an input file".into()));
            }
            let _out = output_path.unwrap_or_else(|| std::path::PathBuf::from("sign_out.pdf"));
            return Err(PdfError::InvalidInput("Sign intent via natural language currently lacks certificate path inference. Please use the CLI directly.".into()));
        }
        Intent::Flatten => {
            if inputs_paths.is_empty() {
                return Err(PdfError::InvalidInput(
                    "Flatten requires an input file".into(),
                ));
            }
            let out = output_path.unwrap_or_else(|| std::path::PathBuf::from("flatten_out.pdf"));
            crate::commands::group_b::handle_flatten(&inputs_paths[0], &out)
        }
        Intent::PdfA => {
            if inputs_paths.is_empty() {
                return Err(PdfError::InvalidInput("PdfA requires an input file".into()));
            }
            let out = output_path.unwrap_or_else(|| std::path::PathBuf::from("pdfa_out.pdf"));
            crate::commands::group_b::handle_pdf_a(&inputs_paths[0], &out)
        }
        Intent::HeaderFooter => {
            if inputs_paths.is_empty() {
                return Err(PdfError::InvalidInput(
                    "HeaderFooter requires an input file".into(),
                ));
            }
            let out = output_path.unwrap_or_else(|| std::path::PathBuf::from("hf_out.pdf"));
            crate::commands::group_b::handle_header_footer(&inputs_paths[0], "Header", &out)
        }
        Intent::Bates => {
            if inputs_paths.is_empty() {
                return Err(PdfError::InvalidInput(
                    "Bates requires an input file".into(),
                ));
            }
            let out = output_path.unwrap_or_else(|| std::path::PathBuf::from("bates_out.pdf"));
            crate::commands::group_b::handle_bates(&inputs_paths[0], "PREFIX-", 1, &out)
        }
        Intent::ExtractText => {
            if inputs_paths.is_empty() {
                return Err(PdfError::InvalidInput(
                    "ExtractText requires an input file".into(),
                ));
            }
            let out = output_path.unwrap_or_else(|| std::path::PathBuf::from("text_out.txt"));
            crate::commands::group_c::handle_extract_text(&inputs_paths[0], &out, "txt")
        }
        Intent::ExtractImages => {
            if inputs_paths.is_empty() {
                return Err(PdfError::InvalidInput(
                    "ExtractImages requires an input file".into(),
                ));
            }
            let out = output_path.unwrap_or_else(|| std::path::PathBuf::from("images_out"));
            crate::commands::group_c::handle_extract_images(&inputs_paths[0], &out)
        }
        Intent::Search => {
            if inputs_paths.is_empty() {
                return Err(PdfError::InvalidInput(
                    "Search requires an input file".into(),
                ));
            }
            crate::commands::group_c::handle_search(&inputs_paths[0], "query")
        }
        Intent::Render => {
            if inputs_paths.is_empty() {
                return Err(PdfError::InvalidInput(
                    "Render requires an input file".into(),
                ));
            }
            let out = output_path.unwrap_or_else(|| std::path::PathBuf::from("render_out"));
            crate::commands::group_c::handle_render(&inputs_paths[0], &out)
        }
        Intent::Compare => {
            if inputs_paths.len() < 2 {
                return Err(PdfError::InvalidInput(
                    "Compare requires two input files".into(),
                ));
            }
            crate::commands::group_c::handle_compare(&inputs_paths[0], &inputs_paths[1])
        }
        Intent::Ocr => {
            if inputs_paths.is_empty() {
                return Err(PdfError::InvalidInput("Ocr requires an input file".into()));
            }
            let out = output_path.unwrap_or_else(|| std::path::PathBuf::from("ocr_out.pdf"));
            crate::commands::group_c::handle_ocr(&inputs_paths[0], Some(&out), "eng", false, None)
        }
        Intent::Bookmarks => {
            if inputs_paths.is_empty() {
                return Err(PdfError::InvalidInput(
                    "Bookmarks requires an input file".into(),
                ));
            }
            crate::commands::group_c::handle_bookmarks(&inputs_paths[0])
        }
        Intent::ImagesToPdf => {
            if inputs_paths.is_empty() {
                return Err(PdfError::InvalidInput(
                    "ImagesToPdf requires input files".into(),
                ));
            }
            let out = output_path.unwrap_or_else(|| std::path::PathBuf::from("images_out.pdf"));
            crate::commands::group_c::handle_images_to_pdf(&inputs_paths, &out)
        }
        Intent::Validate => {
            if inputs_paths.is_empty() {
                return Err(PdfError::InvalidInput(
                    "Validate requires an input file".into(),
                ));
            }
            crate::commands::validate::handle_validate(&inputs_paths[0])
        }
        Intent::Hash => {
            if inputs_paths.is_empty() {
                return Err(PdfError::InvalidInput("Hash requires an input file".into()));
            }
            crate::commands::validate::handle_hash(&inputs_paths[0])
        }
        Intent::FormRead => {
            if inputs_paths.is_empty() {
                return Err(PdfError::InvalidInput(
                    "FormRead requires an input file".into(),
                ));
            }
            crate::commands::group_e::handle_form_read(&inputs_paths[0])
        }
        Intent::FormFill => {
            if inputs_paths.is_empty() {
                return Err(PdfError::InvalidInput(
                    "FormFill requires an input file".into(),
                ));
            }
            let _out = output_path.unwrap_or_else(|| std::path::PathBuf::from("form_fill_out.pdf"));
            return Err(PdfError::InvalidInput(
                "FormFill intent via natural language currently lacks data JSON path inference."
                    .into(),
            ));
        }
        Intent::FormCreate => {
            if inputs_paths.is_empty() {
                return Err(PdfError::InvalidInput(
                    "FormCreate requires an input file".into(),
                ));
            }
            let out =
                output_path.unwrap_or_else(|| std::path::PathBuf::from("form_create_out.pdf"));
            crate::commands::group_e::handle_form_add_field(
                &inputs_paths[0],
                "Field",
                "text",
                1,
                "0,0,100,20",
                &out,
            )
        }
        Intent::ToMarkdown => {
            if inputs_paths.is_empty() {
                return Err(PdfError::InvalidInput(
                    "ToMarkdown requires an input file".into(),
                ));
            }
            let out = output_path.unwrap_or_else(|| std::path::PathBuf::from("out.md"));
            crate::commands::group_d::handle_convert(
                "markdown",
                &inputs_paths[0],
                Some(&out),
                None,
                None,
                None,
            )
        }
        Intent::ToJson => {
            if inputs_paths.is_empty() {
                return Err(PdfError::InvalidInput(
                    "ToJson requires an input file".into(),
                ));
            }
            let out = output_path.unwrap_or_else(|| std::path::PathBuf::from("out.json"));
            crate::commands::group_d::handle_convert(
                "json",
                &inputs_paths[0],
                Some(&out),
                None,
                None,
                None,
            )
        }
        Intent::ToDocx => {
            if inputs_paths.is_empty() {
                return Err(PdfError::InvalidInput(
                    "ToDocx requires an input file".into(),
                ));
            }
            let out = output_path.unwrap_or_else(|| std::path::PathBuf::from("out.docx"));
            crate::commands::group_d::handle_convert(
                "docx",
                &inputs_paths[0],
                Some(&out),
                None,
                None,
                None,
            )
        }
        Intent::ToPptx => {
            if inputs_paths.is_empty() {
                return Err(PdfError::InvalidInput(
                    "ToPptx requires an input file".into(),
                ));
            }
            let out = output_path.unwrap_or_else(|| std::path::PathBuf::from("out.pptx"));
            crate::commands::group_d::handle_convert(
                "pptx",
                &inputs_paths[0],
                Some(&out),
                None,
                None,
                None,
            )
        }
        Intent::ToHtml => {
            if inputs_paths.is_empty() {
                return Err(PdfError::InvalidInput(
                    "ToHtml requires an input file".into(),
                ));
            }
            let out = output_path.unwrap_or_else(|| std::path::PathBuf::from("out.html"));
            crate::commands::group_d::handle_convert(
                "html",
                &inputs_paths[0],
                Some(&out),
                None,
                None,
                None,
            )
        }
        Intent::Classify => {
            if inputs_paths.is_empty() {
                return Err(PdfError::InvalidInput(
                    "Classify requires an input file".into(),
                ));
            }
            crate::commands::group_d::handle_classify(&inputs_paths[0])
        }
        _ => Err(PdfError::UnsupportedOperation(
            "Unsupported NLP intent".into(),
        )),
    }
}
