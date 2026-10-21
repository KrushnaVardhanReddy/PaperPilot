use crate::cli::WorkflowSubcommand;
use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::workflow::{SavedWorkflow, get_default_presets, get_preset_by_id};
use paperpilot_pdf::operations::workflow::run_workflow_bytes;
use std::fs;
use std::path::{Path, PathBuf};

pub fn handle_workflow_command(
    subcommand: Option<&WorkflowSubcommand>,
    input: Option<&PathBuf>,
    output: Option<&PathBuf>,
    preset: Option<&String>,
    recipe: Option<&PathBuf>,
) -> OperationResult<()> {
    match subcommand {
        Some(WorkflowSubcommand::List) => {
            let presets = get_default_presets();
            println!("Available Workflow Presets:\n");
            for p in presets {
                println!("ID: {}", p.id);
                println!("Name: {}", p.name);
                if let Some(desc) = &p.description {
                    println!("Description: {}", desc);
                }
                println!("Steps Count: {}\n", p.steps.len());
            }
            Ok(())
        }
        Some(WorkflowSubcommand::Inspect { preset, recipe }) => {
            let wf = resolve_workflow(preset.as_deref(), recipe.as_deref())?;
            println!("Workflow ID: {}", wf.id);
            println!("Workflow Name: {}", wf.name);
            if let Some(desc) = &wf.description {
                println!("Description: {}", desc);
            }
            println!("Steps ({}):", wf.steps.len());
            for (idx, step) in wf.steps.iter().enumerate() {
                println!(
                    "  {}. [{}] - label: {:?}",
                    idx + 1,
                    step.operation_id,
                    step.label
                );
                if !step.params.is_empty() {
                    println!(
                        "     Params: {}",
                        serde_json::to_string(&step.params).unwrap_or_default()
                    );
                }
            }
            Ok(())
        }
        Some(WorkflowSubcommand::Run {
            input,
            output,
            preset,
            recipe,
        }) => run_workflow(input, output, preset.as_deref(), recipe.as_deref()),
        None => {
            // Direct top-level usage
            if let (Some(in_path), Some(out_path)) = (input, output) {
                run_workflow(
                    in_path,
                    out_path,
                    preset.map(|s| s.as_str()),
                    recipe.map(|p| p.as_ref()),
                )
            } else {
                Err(PdfError::InvalidInput(
                    "Missing required arguments. Usage: paperpilot workflow run --input <IN> --output <OUT> [--preset <PRESET> | --recipe <RECIPE>]".to_string(),
                ))
            }
        }
    }
}

fn resolve_workflow(preset: Option<&str>, recipe: Option<&Path>) -> OperationResult<SavedWorkflow> {
    if let Some(preset_name) = preset {
        get_preset_by_id(preset_name)
            .ok_or_else(|| PdfError::InvalidInput(format!("Preset not found: {}", preset_name)))
    } else if let Some(recipe_path) = recipe {
        let content = fs::read_to_string(recipe_path)?;
        let wf: SavedWorkflow = serde_json::from_str(&content)
            .map_err(|e| PdfError::ParseError(format!("Invalid workflow recipe JSON: {}", e)))?;
        Ok(wf)
    } else {
        Err(PdfError::InvalidInput(
            "Must specify either --preset or --recipe".to_string(),
        ))
    }
}

fn run_workflow(
    input: &Path,
    output: &Path,
    preset: Option<&str>,
    recipe: Option<&Path>,
) -> OperationResult<()> {
    let wf = resolve_workflow(preset, recipe)?;
    let input_bytes = fs::read(input)?;
    let (out_bytes, report) = run_workflow_bytes(&input_bytes, &wf)?;
    fs::write(output, out_bytes)?;
    println!(
        "Workflow '{}' completed successfully in {}ms ({} steps, output: {} bytes)",
        report.workflow_name, report.duration_ms, report.steps_completed, report.output_bytes_len
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_preset() {
        let wf = resolve_workflow(Some("preset-clean-scan"), None).unwrap();
        assert_eq!(wf.id, "preset-clean-scan");
    }

    #[test]
    fn test_resolve_unknown_preset_fails() {
        let err = resolve_workflow(Some("does-not-exist"), None).unwrap_err();
        assert!(err.to_string().contains("Preset not found"));
    }
}
