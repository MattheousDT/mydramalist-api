use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

pub enum AppError {
    NotFound(String),
    Internal(anyhow::Error),
    BadRequest(String),
}

// Explicitly implement From for anyhow::Error to avoid trait inference issues
impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        Self::Internal(err)
    }
}

// Implement IntoResponse for Axum 0.8
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Internal(err) => {
                tracing::error!("Application error: {:#}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, format!("{:#}", err))
            }
        };

        let body = Json(json!({
            "error": status.canonical_reason().unwrap_or("Unknown Error"),
            "message": message
        }));

        (status, body).into_response()
    }
}
