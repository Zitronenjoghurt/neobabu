pub type BotResult<T> = Result<T, BotError>;

#[derive(Debug, thiserror::Error)]
pub enum BotError {
    #[error("Serenity error: {0}")]
    Serenity(#[from] poise::serenity_prelude::Error),
}
