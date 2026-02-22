use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

pub struct AppError(anyhow::Error);

// Explicitly implement From for anyhow::Error to avoid trait inference issues
impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        Self(err)
    }
}

// Implement IntoResponse for Axum 0.8
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!("Application error: {:#}", self.0);

        let body = Json(json!({
            "error": "Internal Server Error",
            "message": format!("{:#}", self.0)
        }));

        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}
