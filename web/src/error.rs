use oauth2::url;
use std::env;

pub type ServerResult<T> = Result<T, ServerError>;

#[derive(Debug, thiserror::Error)]
pub enum ServerError {
    #[error("Core error: {0}")]
    Core(#[from] neobabu_core::error::CoreError),
    #[error("Env error: {0}")]
    Env(#[from] env::VarError),
    #[error("Url parse error: {0}")]
    UrlParse(#[from] url::ParseError),
}
