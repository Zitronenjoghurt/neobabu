use crate::context::ContextExt;
use crate::error::BotResult;
use crate::ui::color::UiColor;
use crate::ui::message::CreateEmbedExt;
use crate::Context;
use neobabu_core::types::currency::Currency;
use poise::serenity_prelude::CreateEmbed;
use strum::IntoEnumIterator;

/// View your wallet.
#[poise::command(slash_command, ephemeral)]
pub async fn wallet(ctx: Context<'_>) -> BotResult<()> {
    let user = ctx.fetch_author_model().await?;
    let wallet = ctx.services().economy.wallet(&user).await?;

    let mut result = String::new();
    for currency in Currency::iter() {
        let Some(balance) = wallet.currencies.get(&currency) else {
            continue;
        };

        let emoji = ctx.emoji_text(currency.into());
        let amount = if balance.available == balance.total {
            format!("**`{}`** {emoji}", balance.total)
        } else {
            format!(
                "**`{}`** {emoji} (*{} reserved*)",
                balance.available,
                balance.total - balance.available
            )
        };

        let text = format!("**{currency}**: {amount}");
        result.push_str(&text);
        result.push('\n');
    }

    let embed = CreateEmbed::default()
        .title("WALLET")
        .ui_color(UiColor::Yellow)
        .description(result);
    ctx.send(embed.create_reply()).await?;

    Ok(())
}
