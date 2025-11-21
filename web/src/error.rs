use std::env;

pub type ServerResult<T> = Result<T, ServerError>;

#[derive(Debug, thiserror::Error)]
pub enum ServerError {
    #[error("Core error: {0}")]
    Core(#[from] neobabu_core::error::CoreError),
    #[error("Env error: {0}")]
    Env(#[from] env::VarError),
}
