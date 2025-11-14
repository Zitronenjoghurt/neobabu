use crate::types::feature::Feature;

pub type CoreResult<T> = Result<T, CoreError>;

#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[error("Database error: {0}")]
    Database(#[from] sea_orm::DbErr),
    #[error("Feature not enabled on server: {0:?}")]
    FeatureNotEnabled(Feature),
    #[error("Invalid birthday: {0}")]
    InvalidBirthday(String),
}

impl CoreError {
    pub fn invalid_birthday(reason: impl Into<String>) -> Self {
        Self::InvalidBirthday(reason.into())
    }
}
