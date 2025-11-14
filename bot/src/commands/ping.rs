use crate::error::BotResult;
use crate::ui::embed::CreateEmbedExt;
use crate::Context;
use poise::serenity_prelude::CreateEmbed;

#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> BotResult<()> {
    let latency = ctx.ping().await;

    let embed = CreateEmbed::default()
        .success_user(ctx.author())
        .title("Pong!")
        .field(
            "Shard Heartbeat Latency",
            format!("{:.2}ms", latency.as_micros() as f64 / 1000.0),
            false,
        )
        .footer_text("If the shard just started, heartbeat latency will be 0ms.");

    let reply = embed.create_reply();
    ctx.send(reply).await?;

    Ok(())
}
