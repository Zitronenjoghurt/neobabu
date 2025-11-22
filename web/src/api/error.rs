use crate::error::ServerError;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use tracing::error;

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Internal server error")]
    Core(#[from] neobabu_core::error::CoreError),
    #[error("Invalid CSRF token")]
    InvalidCsrfToken,
    #[error("Missing CSRF token")]
    MissingCsrfToken,
    #[error("Missing PKCE verifier")]
    MissingPkceVerifier,
    #[error("Failed to exchange OAuth token")]
    OauthTokenExchangeFailed,
    #[error("Failed to make HTTP request")]
    Reqwest(#[from] reqwest::Error),
    #[error("Internal server error")]
    Server(#[from] ServerError),
    #[error("Session error")]
    Session(#[from] tower_sessions::session::Error),
    #[error("Unauthorized")]
    Unauthorized,
}

impl ApiError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::Core(_)
            | Self::Reqwest(_)
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
            Self::Core(_) | Self::Server(_) | Self::Session(_) => true,
            Self::InvalidCsrfToken
            | Self::MissingCsrfToken
            | Self::MissingPkceVerifier
            | Self::Reqwest(_)
            | Self::OauthTokenExchangeFailed
            | Self::Unauthorized => false,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        if self.should_log() {
            error!("{self}");
        };
        (self.status_code(), self.to_string()).into_response()
    }
}
