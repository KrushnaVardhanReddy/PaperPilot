use crate::cli::{Commands, WorkflowSubcommand};
use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::workflow::{
    SavedWorkflow, get_default_presets, get_preset_by_id, parse_workflow,
};
use paperpilot_pdf::operations::workflow::run_workflow_bytes_detailed;
use serde_json::Value;
use std::fs;
use std::io::Write;
use std::path::Path;

pub fn execute(command: &Commands, json: bool) -> (OperationResult<()>, Option<Value>) {
    let (subcommand, input, output, preset, recipe) = match command {
        Commands::Run {
            recipe,
            input,
            output,
        } => return run_workflow(input, output, None, Some(recipe), json),
        Commands::Workflow {
            subcommand,
            input,
            output,
            preset,
            recipe,
        } => (subcommand, input, output, preset, recipe),
        _ => {
            return (
                Err(PdfError::InvalidInput("Not a workflow command".into())),
                None,
            );
        }
    };
    match subcommand {
        Some(WorkflowSubcommand::List) => {
            let presets = get_default_presets();
            if !json {
                println!("Available Workflow Presets:");
                for p in &presets { println!("{}: {} ({} steps)", p.id, p.name, p.steps.len()); }
            }
            (Ok(()), Some(serde_json::to_value(presets).unwrap()))
        }
        Some(WorkflowSubcommand::Inspect { preset, recipe }) => {
            match resolve_workflow(preset.as_deref(), recipe.as_deref()) {
                Ok(wf) => {
                    if !json {
                        println!("Workflow: {} ({})", wf.name, wf.id);
                        for step in &wf.steps {
                            // Avoid printing parameter values, which may include passwords.
                            println!("{}: {} condition={:?}, on_no_match={:?}, depends_on={:?}", step.id, step.operation_id, step.condition, step.on_no_match, step.depends_on);
                        }
                    }
                    // Structured inspection exposes the plan but never parameter values.
                    let stages: Vec<_> = wf.steps.iter().map(|s| serde_json::json!({
                        "id": s.id, "operationId": s.operation_id, "condition": s.condition,
                        "on_no_match": s.on_no_match, "depends_on": s.depends_on,
                        "parameter_names": s.params.keys().collect::<Vec<_>>()
                    })).collect();
                    (Ok(()), Some(serde_json::json!({"id":wf.id,"name":wf.name,"version":wf.version,"steps":stages})))
                }
                Err(e) => (Err(e), None),
            }
        }
        Some(WorkflowSubcommand::Run { input, output, preset, recipe }) => run_workflow(input, output, preset.as_deref(), recipe.as_deref(), json),
        None => match (input, output) {
            (Some(input), Some(output)) => run_workflow(input, output, preset.as_deref(), recipe.as_deref(), json),
            _ => (Err(PdfError::InvalidInput("Use workflow run --input IN --output OUT --recipe RECIPE, or run RECIPE IN --output OUT".into())), None),
        },
    }
}

fn resolve_workflow(preset: Option<&str>, recipe: Option<&Path>) -> OperationResult<SavedWorkflow> {
    match (preset, recipe) {
        (Some(name), None) => get_preset_by_id(name)
            .ok_or_else(|| PdfError::InvalidInput(format!("Preset not found: {name}"))),
        (None, Some(path)) => parse_workflow(&fs::read_to_string(path)?),
        _ => Err(PdfError::InvalidInput(
            "Specify exactly one of --preset or --recipe".into(),
        )),
    }
}

fn run_workflow(
    input: &Path,
    output: &Path,
    preset: Option<&str>,
    recipe: Option<&Path>,
    json: bool,
) -> (OperationResult<()>, Option<Value>) {
    let prepared = (|| {
        let wf = resolve_workflow(preset, recipe)?;
        if output.exists() && fs::canonicalize(input)? == fs::canonicalize(output)? {
            return Err(PdfError::InvalidInput(
                "Input and output must be different files".into(),
            ));
        }
        Ok((wf, fs::read(input)?))
    })();
    let (wf, bytes) = match prepared {
        Ok(value) => value,
        Err(error) => return (Err(error), None),
    };
    match run_workflow_bytes_detailed(&bytes, &wf) {
        Ok((out, mut report)) => {
            let result = (|| -> OperationResult<()> {
                let parent = output
                    .parent()
                    .filter(|p| !p.as_os_str().is_empty())
                    .unwrap_or(Path::new("."));
                let mut temporary = tempfile::NamedTempFile::new_in(parent)?;
                temporary.write_all(&out)?;
                temporary
                    .persist(output)
                    .map_err(|e| PdfError::IoError(e.error))?;
                Ok(())
            })();
            if let Err(error) = &result {
                report.error = Some(error.to_string());
                report.output_bytes_len = 0;
            }
            if !json {
                eprintln!(
                    "Workflow '{}': {} completed, {} skipped, archive_original={}",
                    report.workflow_name,
                    report.steps_completed,
                    report.steps_skipped,
                    report.archived_original
                );
            }
            (result, Some(serde_json::to_value(report).unwrap()))
        }
        Err(failure) => (
            Err(failure.cause),
            Some(serde_json::to_value(failure.report).unwrap()),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn resolve_requires_one_source() {
        assert!(resolve_workflow(Some("clean-scan"), None).is_ok());
        assert!(resolve_workflow(Some("missing"), None).is_err());
        assert!(resolve_workflow(None, None).is_err());
        assert!(resolve_workflow(Some("clean-scan"), Some(Path::new("unused"))).is_err());
    }
}
