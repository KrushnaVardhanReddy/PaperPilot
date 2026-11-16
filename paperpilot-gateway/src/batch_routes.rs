//! Thin REST adapter to the shared native batch runner.
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use paperpilot_pdf::operations::batch_fill::BatchFillRequest;
use serde_json::json;
pub async fn fill(Json(request): Json<BatchFillRequest>) -> Response {
    match tokio::task::spawn_blocking(move || request.execute()).await {
        Ok(Ok(receipt)) => (
            if receipt.success {
                StatusCode::OK
            } else {
                StatusCode::MULTI_STATUS
            },
            Json(receipt),
        )
            .into_response(),
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
