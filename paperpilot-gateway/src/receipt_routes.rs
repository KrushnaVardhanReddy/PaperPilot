use axum::{
    Json,
    extract::{Path, Query},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Deserialize;
use serde_json::{Value, json};
#[derive(Deserialize)]
pub struct ReceiptQuery {
    directory: String,
    limit: Option<usize>,
}
async fn run(
    operation: impl FnOnce() -> paperpilot_core::error::OperationResult<Value> + Send + 'static,
) -> Response {
    match tokio::task::spawn_blocking(operation).await {
        Ok(Ok(value)) => Json(value).into_response(),
        Ok(Err(e)) => (
            StatusCode::BAD_REQUEST,
            Json(json!({"error":e.to_string()})),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error":e.to_string()})),
        )
            .into_response(),
    }
}
pub async fn list(Query(query): Query<ReceiptQuery>) -> Response {
    run(move || {
        paperpilot_pdf::operations::receipts::list(
            std::path::Path::new(&query.directory),
            query.limit.unwrap_or(50),
        )
        .map(|v| json!(v))
    })
    .await
}
pub async fn get(Path(id): Path<String>, Query(query): Query<ReceiptQuery>) -> Response {
    run(move || {
        paperpilot_pdf::operations::receipts::get(std::path::Path::new(&query.directory), &id)
            .map(|v| json!(v))
    })
    .await
}
