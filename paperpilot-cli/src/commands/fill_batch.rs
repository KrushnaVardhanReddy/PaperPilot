//! Thin CLI adapter for the shared local batch runner.
use crate::cli::FillBatchArgs;
use paperpilot_core::error::OperationResult;
pub use paperpilot_pdf::operations::batch_fill::BatchReceipt;
use paperpilot_pdf::operations::batch_fill::{self, BatchFillConfig};
pub fn run(args: &FillBatchArgs) -> OperationResult<BatchReceipt> {
    batch_fill::run(&BatchFillConfig {
        template: args.template.clone(),
        data: args.data.clone(),
        output_dir: args.output_dir.clone(),
        name_template: args.name_template.clone(),
        field_map: args.field_map.clone(),
        keep_editable: args.keep_editable,
        date: args.date.clone(),
    })
}
