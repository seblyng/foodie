use axum::{
    response::{IntoResponse, Response},
    Json,
};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};

pub mod api;
pub mod app;
pub mod auth_backend;
pub mod entities;
pub mod redis_store;
pub mod storage;

pub enum ApiError {
    RecordNotFound,
    DatabaseError(sea_orm::DbErr),
    UnknownError(String),
    ConflictError(String),
    StatusCode(StatusCode, String),
}

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    error: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status_code, error_message) = match self {
            ApiError::DatabaseError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("An unexpected exception has occured: {}", err),
            ),
            ApiError::UnknownError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Unknown error: {}", err),
            ),
            ApiError::RecordNotFound => (StatusCode::NOT_FOUND, r#"Record not found"#.to_string()),
            ApiError::ConflictError(err) => (StatusCode::CONFLICT, err),
            ApiError::StatusCode(status_code, err) => (status_code, err),
        };

        (
            status_code,
            Json(ErrorResponse {
                error: error_message,
            }),
        )
            .into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl From<anyhow::Error> for ApiError {
    fn from(err: anyhow::Error) -> Self {
        let error = err.to_string();
        Self::UnknownError(error)
    }
}

impl From<sea_orm::DbErr> for ApiError {
    fn from(error: sea_orm::DbErr) -> Self {
        ApiError::DatabaseError(error)
    }
}
