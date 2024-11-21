use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use tracing::error;

/// Custom error type for the API.
/// The `#[from]` attribute allows for easy conversion from other error types.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Converts from an Axum built-in extractor error.
    #[error("Invalid payload.")]
    InvalidJsonBody(#[from] JsonRejection),

    /// For errors that occur during manual validation.
    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    /// Return `401 Unauthorized`
    #[error("authentication required")]
    Unauthorized,

    /// A SQLx call returned an error.
    ///
    /// The exact error contents are not reported to the user in order to avoid leaking
    /// information about databse internals.
    #[error("an internal database error occurred")]
    Sqlx(#[from] sqlx::Error),

    /// Similarly, we don't want to report random `anyhow` errors to the user.
    #[error("an internal server error occurred")]
    Anyhow(#[from] anyhow::Error),
}
impl Error {
    // Determine the appropriate status code.
    pub fn status_code(&self) -> StatusCode {
        match self {
            Error::InvalidJsonBody(_) | Error::InvalidRequest(_) => StatusCode::BAD_REQUEST,
            Error::Sqlx(_) | Error::Anyhow(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Unauthorized => StatusCode::UNAUTHORIZED,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResp {
    pub message: String,
}

// The IntoResponse implementation for Error logs the error message.
//
// To avoid exposing implementation details to API consumers, we separate
// the message that we log from the API response message.
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        // Log detailed error for telemetry.
        let error_to_log = match &self {
            Error::InvalidJsonBody(ref err) => match err {
                JsonRejection::JsonDataError(e) => e.body_text(),
                JsonRejection::JsonSyntaxError(e) => e.body_text(),
                JsonRejection::MissingJsonContentType(_) => {
                    "Missing `Content-Type: application/json` header".to_string()
                }
                JsonRejection::BytesRejection(_) => "Failed to buffer request body".to_string(),
                _ => "Unknown error".to_string(),
            },
            Error::Unauthorized => "Unauthorized".to_string(),
            Error::InvalidRequest(_) => format!("{}", self),
            Error::Sqlx(ref err) => format!("{}", err),
            Error::Anyhow(ref err) => format!("{}", err),
        };
        error!("{}", error_to_log);

        // Create a generic response to hide specific implementation details.
        let resp = ErrorResp {
            message: self.to_string(),
        };

        (self.status_code(), Json(resp)).into_response()
    }
}
