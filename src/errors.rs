use axum::http::StatusCode;
use axum::response::IntoResponse;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Authentication failed")]
    Unauthorized,

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Internal server error")]
    Internal(#[from] anyhow::Error),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Channel error: {0}")]
    Channel(
        #[from] tokio::sync::mpsc::error::SendError<crate::perceiver::event::NormalizedEvent>,
    ),

    #[error("Hyper HTTP error: {0}")]
    Http(#[from] hyper::Error),

    #[error("Reqwest HTTP client error: {0}")]
    HttpClient(#[from] reqwest::Error), // ✅ NEW VARIANT

    #[error("Request error: {0}")]
    RequestError(String), // ✅ OPTIONAL fallback if you want plain string handling
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status = match self {
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::ConfigError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Io(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Channel(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Http(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::HttpClient(_) => StatusCode::BAD_GATEWAY,
            AppError::RequestError(_) => StatusCode::BAD_REQUEST,
        };

        (status, self.to_string()).into_response()
    }
}
