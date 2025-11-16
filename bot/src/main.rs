use crate::error::BotError;
use crate::events::event_handler;
use crate::state::BotState;
use tracing::info;
use tracing_subscriber::EnvFilter;

mod commands;
mod config;
mod context;
mod error;
mod events;
mod state;
mod ui;
mod utils;

pub type Context<'a> = poise::Context<'a, BotState, BotError>;

#[tokio::main]
async fn main() {
    init_tracing();
    info!("Starting bot...");

    let state = BotState::initialize().await.unwrap();
    let config = state.config.clone();

    let commands = commands::get_commands();
    let options = poise::FrameworkOptions {
        commands,
        on_error: |error| Box::pin(error::handler(error)),
        event_handler: |ctx, event, framework, state| {
            Box::pin(event_handler(ctx, event, framework, state))
        },
        ..Default::default()
    };

    let framework = poise::Framework::<BotState, BotError>::builder()
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                //info!("Registering commands...");
                //let guild_id = poise::serenity_prelude::GuildId::new();
                //poise::builtins::register_in_guild(ctx, &framework.options().commands, guild_id).await?;
                //poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                //info!("Commands registered!");
                Ok(state)
            })
        })
        .options(options)
        .build();

    let intents = poise::serenity_prelude::GatewayIntents::privileged();
    let client = poise::serenity_prelude::ClientBuilder::new(&config.credentials.token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();
}

fn init_tracing() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();
    info!("Tracing initialized");
}
