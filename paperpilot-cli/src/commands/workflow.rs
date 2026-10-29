use crate::cli::{Commands, WorkflowSubcommand};
use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::workflow::{
    SavedWorkflow, get_default_presets, get_preset_by_id, parse_workflow,
};
use paperpilot_pdf::operations::receipts;
use serde_json::Value;
use std::fs;
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
        Some(WorkflowSubcommand::History { directory, id, limit }) => {
            let result = match id {
                Some(id) => receipts::get(directory, id).map(|v| serde_json::json!(v)),
                None => receipts::list(directory, *limit).map(|v| serde_json::json!(v)),
            };
            match result {
                Ok(value) => { if !json { println!("{}", serde_json::to_string_pretty(&value).unwrap()); } (Ok(()), Some(value)) },
                Err(error) => (Err(error), None),
            }
        }
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
    let wf = match resolve_workflow(preset, recipe) {
        Ok(wf) => wf,
        Err(e) => return (Err(e), None),
    };
    match receipts::run_to_path(input, output, &wf) {
        Ok(run) => {
            if !json {
                eprintln!("Execution receipt: {}", run.receipt_path.display());
            }
            (run.outcome(), Some(run.report_json()))
        }
        Err(error) => (Err(error), None),
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
