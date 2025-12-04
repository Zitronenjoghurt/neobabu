use crate::types::feature::Feature;

pub type CoreResult<T> = Result<T, CoreError>;

#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[error("Aed error: {0}")]
    Aed(#[from] chacha20poly1305::aead::Error),
    #[error("Base64 decode error: {0}")]
    Base64Decode(#[from] base64::DecodeError),
    #[error("Birthday updated too recently")]
    BirthdayTimeout,
    #[error("Failed to parse date: {0}")]
    ChronoParse(#[from] chrono::ParseError),
    #[error("Cron scheduler error: {0}")]
    CronScheduler(#[from] tokio_cron_scheduler::JobSchedulerError),
    #[error("Database error: {0}")]
    Database(#[from] sea_orm::DbErr),
    #[error("Decrypt data too short")]
    DecryptDataTooShort,
    #[error("Feature not enabled on server: {0:?}")]
    FeatureNotEnabled(Feature),
    #[error("Youtube channel already subscribed on this server")]
    GuildYoutubeChannelAlreadySubscribed,
    #[error("Server youtube channel limit reached")]
    GuildYoutubeChannelLimitReached,
    #[error("Hex error: {0}")]
    Hex(#[from] hex::FromHexError),
    #[error("Image error: {0}")]
    Image(#[from] image::ImageError),
    #[error("Invalid birthday: {0}")]
    InvalidBirthday(String),
    #[error("Invalid month: {0}")]
    InvalidMonth(u32),
    #[error("Invalid header value: {0}")]
    InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue),
    #[error("Json ser/de error: {0}")]
    JsonSerde(#[from] serde_json::Error),
    #[error("Missing NASA API key")]
    MissingNasaApiKey,
    #[error("Missing Youtube API key")]
    MissingYoutubeApiKey,
    #[error("OS error: {0}")]
    Os(#[from] rand::rand_core::OsError),
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Reqwest middleware error: {0}")]
    ReqwestMiddleware(#[from] reqwest_middleware::Error),
    #[error("Serenity error: {0}")]
    Serenity(#[from] serenity::Error),
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Unknown item kind: {0}")]
    UnknownItemKind(i32),
    #[error("Failed to parse URL: {0}")]
    UrlParse(#[from] url::ParseError),
    #[error("User youtube channel limit reached")]
    UserYoutubeChannelLimitReached,
    #[error("UTF-8 error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error(
        "Youtube channel not found, make sure you used the correct handle. Like @veritasium (usually found in the channels URL)."
    )]
    YoutubeChannelNotFound,
    #[error("Youtube hub callback URL missing")]
    YoutubeHubCallbackUrlMissing,
    #[error("Youtube hub secret missing")]
    YoutubeHubSecretMissing,
}

impl CoreError {
    pub fn is_user_error(&self) -> bool {
        match self {
            Self::BirthdayTimeout
            | Self::FeatureNotEnabled(_)
            | Self::GuildYoutubeChannelAlreadySubscribed
            | Self::GuildYoutubeChannelLimitReached
            | Self::InvalidBirthday(_)
            | Self::Unauthorized
            | Self::UserYoutubeChannelLimitReached
            | Self::YoutubeChannelNotFound => true,
            Self::Aed(_)
            | Self::Base64Decode(_)
            | Self::CronScheduler(_)
            | Self::ChronoParse(_)
            | Self::Database(_)
            | Self::DecryptDataTooShort
            | Self::Hex(_)
            | Self::Image(_)
            | Self::InvalidHeaderValue(_)
            | Self::InvalidMonth(_)
            | Self::JsonSerde(_)
            | Self::MissingNasaApiKey
            | Self::MissingYoutubeApiKey
            | Self::Os(_)
            | Self::Reqwest(_)
            | Self::ReqwestMiddleware(_)
            | Self::Serenity(_)
            | Self::UnknownItemKind(_)
            | Self::UrlParse(_)
            | Self::Utf8(_)
            | Self::YoutubeHubCallbackUrlMissing
            | Self::YoutubeHubSecretMissing => false,
        }
    }

    pub fn invalid_birthday(reason: impl Into<String>) -> Self {
        Self::InvalidBirthday(reason.into())
    }
}
