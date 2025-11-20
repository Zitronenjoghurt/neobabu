use crate::types::feature::Feature;

pub type CoreResult<T> = Result<T, CoreError>;

#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[error("Cron scheduler error: {0}")]
    CronScheduler(#[from] tokio_cron_scheduler::JobSchedulerError),
    #[error("Database error: {0}")]
    Database(#[from] sea_orm::DbErr),
    #[error("Birthday updated too recently")]
    BirthdayTimeout,
    #[error("Failed to parse date: {0}")]
    ChronoParse(#[from] chrono::ParseError),
    #[error("Feature not enabled on server: {0:?}")]
    FeatureNotEnabled(Feature),
    #[error("Invalid birthday: {0}")]
    InvalidBirthday(String),
    #[error("Invalid month: {0}")]
    InvalidMonth(u32),
    #[error("Invalid header value: {0}")]
    InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue),
    #[error("Missing NASA API key")]
    MissingNasaApiKey,
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Reqwest middleware error: {0}")]
    ReqwestMiddleware(#[from] reqwest_middleware::Error),
    #[error("Failed to parse URL: {0}")]
    UrlParse(#[from] url::ParseError),
}

impl CoreError {
    pub fn is_user_error(&self) -> bool {
        match self {
            Self::BirthdayTimeout | Self::FeatureNotEnabled(_) | Self::InvalidBirthday(_) => true,
            Self::CronScheduler(_)
            | Self::ChronoParse(_)
            | Self::Database(_)
            | Self::InvalidHeaderValue(_)
            | Self::InvalidMonth(_)
            | Self::MissingNasaApiKey
            | Self::Reqwest(_)
            | Self::ReqwestMiddleware(_)
            | Self::UrlParse(_) => false,
        }
    }

    pub fn invalid_birthday(reason: impl Into<String>) -> Self {
        Self::InvalidBirthday(reason.into())
    }
}
