use crate::error::ServerError;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use tracing::error;

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Core error: {0}")]
    Core(#[from] neobabu_core::error::CoreError),
    #[error("Invalid CSRF token")]
    InvalidCsrfToken,
    #[error("Missing CSRF token")]
    MissingCsrfToken,
    #[error("Missing PKCE verifier")]
    MissingPkceVerifier,
    #[error("Failed to exchange OAuth token")]
    OauthTokenExchangeFailed,
    #[error("Failed to make HTTP request: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Internal server error: {0}")]
    Server(#[from] ServerError),
    #[error("Session error: {0}")]
    Session(#[from] tower_sessions::session::Error),
    #[error("Unauthorized")]
    Unauthorized,
}

impl ApiError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::Core(err) => {
                if err.is_user_error() {
                    StatusCode::BAD_REQUEST
                } else {
                    StatusCode::INTERNAL_SERVER_ERROR
                }
            }
            Self::Reqwest(_)
            | Self::Server(_)
            | Self::Session(_)
            | Self::OauthTokenExchangeFailed => StatusCode::INTERNAL_SERVER_ERROR,
            Self::InvalidCsrfToken | Self::MissingCsrfToken | Self::MissingPkceVerifier => {
                StatusCode::BAD_REQUEST
            }
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
        }
    }

    pub fn should_log(&self) -> bool {
        match self {
            Self::Core(err) => !err.is_user_error(),
            Self::Reqwest(_) | Self::Server(_) | Self::Session(_) => true,
            Self::InvalidCsrfToken
            | Self::MissingCsrfToken
            | Self::MissingPkceVerifier
            | Self::OauthTokenExchangeFailed
            | Self::Unauthorized => false,
        }
    }

    pub fn user_message(&self) -> String {
        match self {
            Self::Core(err) => {
                if err.is_user_error() {
                    err.to_string()
                } else {
                    "An unexpected error occurred.".to_string()
                }
            }
            Self::Server(_) => "Internal server error".to_string(),
            Self::InvalidCsrfToken | Self::MissingCsrfToken | Self::MissingPkceVerifier => {
                "Invalid CSRF token".to_string()
            }
            Self::Unauthorized => "Unauthorized".to_string(),
            Self::OauthTokenExchangeFailed => "Failed to exchange OAuth token".to_string(),
            Self::Reqwest(_) => "Failed to make HTTP request".to_string(),
            Self::Session(_) => "Session error".to_string(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        if self.should_log() {
            error!("{self}");
        };
        (self.status_code(), self.user_message()).into_response()
    }
}
