use crate::context::ContextExt;
use crate::error::BotResult;
use crate::ui::color::UiColor;
use crate::ui::message::interactive::state::simple_accept::SimpleAcceptStateTrait;
use crate::ui::message::interactive::InteractiveMessage;
use crate::ui::message::CreateEmbedExt;
use crate::Context;
use neobabu_core::database::entity::{guild, user, youtube_channel};
use neobabu_core::error::CoreError;
use poise::serenity_prelude::{ComponentInteraction, CreateEmbed, CreateEmbedAuthor};

/// Subscribe to a youtube channel to receive new video notifications on this server.
#[poise::command(
    slash_command,
    guild_only,
    required_permissions = "MANAGE_GUILD",
    user_cooldown = "30"
)]
pub async fn subscribe(
    ctx: Context<'_>,
    #[description = "The handle of the youtube channel, like: @veritasium."] handle: String,
) -> BotResult<()> {
    ctx.defer_ephemeral().await?;

    let guild = ctx.fetch_guild_model().await?;
    let user = ctx.fetch_author_model().await?;
    ctx.services()
        .youtube
        .verify_can_subscribe(&guild, &user)
        .await?;

    let Some(channel) = ctx
        .services()
        .youtube
        .find_channel_by_handle(&handle)
        .await?
    else {
        return Err(CoreError::YoutubeChannelNotFound.into());
    };

    ctx.services()
        .youtube
        .verify_can_subscribe_to_channel(&guild, &channel)
        .await?;

    let state = YoutubeSubscribeState {
        channel,
        guild,
        user,
    };

    InteractiveMessage::new(&ctx, state.build())
        .timeout(std::time::Duration::from_secs(120))
        .run()
        .await?;

    Ok(())
}

struct YoutubeSubscribeState {
    channel: youtube_channel::Model,
    guild: guild::Model,
    user: user::Model,
}

impl YoutubeSubscribeState {
    pub fn basic_embed(&self) -> CreateEmbed {
        let mut author = CreateEmbedAuthor::new(&self.channel.name).url(self.channel.url());
        if let Some(icon_url) = &self.channel.icon_url {
            author = author.icon_url(icon_url);
        }
        CreateEmbed::default().author(author)
    }
}

#[async_trait::async_trait]
impl SimpleAcceptStateTrait for YoutubeSubscribeState {
    async fn embed_question(&self, _ctx: &Context) -> BotResult<CreateEmbed> {
        Ok(self.basic_embed()
            .ui_color(UiColor::Warning)
            .title("New channel subscription")
            .description(format!("Do you want to subscribe to this channel?\n\nDouble check this is the correct channel before proceeding:\n{}", self.channel.url())))
    }

    async fn embed_accepted(&self, _ctx: &Context) -> BotResult<CreateEmbed> {
        Ok(self.basic_embed()
            .ui_color(UiColor::Success)
            .title("Channel successfully subscribed")
            .description("If you enabled the youtube notification settings on this server, you will begin to receive notifications for every new video this channel uploads."))
    }

    async fn embed_denied(&self, _ctx: &Context) -> BotResult<CreateEmbed> {
        Ok(self
            .basic_embed()
            .ui_color(UiColor::Gray)
            .title("Subscription request cancelled")
            .description("You will not receive any notifications for this channel."))
    }

    async fn on_accept(
        &mut self,
        ctx: &Context<'_>,
        _interaction: &ComponentInteraction,
    ) -> BotResult<()> {
        ctx.services()
            .youtube
            .guild_subscribe(&self.guild, &self.user, &self.channel)
            .await?;
        Ok(())
    }

    fn accept_text(&self) -> &'static str {
        "Yes"
    }

    fn deny_text(&self) -> &'static str {
        "No"
    }
}
