use crate::ui::color::UiColor;
use crate::Context;
use poise::serenity_prelude::{Colour, CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter, User};
use poise::{async_trait, CreateReply};

pub mod interactive;

#[async_trait]
pub trait CreateEmbedExt: Sized {
    fn create_reply(self) -> CreateReply;
    fn ui_color(self, color: UiColor) -> Self;
    fn user(self, user: &User) -> Self;
    async fn member_color(self, ctx: &Context<'_>) -> Self;
    fn footer_text(self, text: impl Into<String>) -> Self;

    fn success(self) -> Self {
        self.ui_color(UiColor::Success)
    }

    fn success_user(self, user: &User) -> Self {
        self.success().user(user)
    }

    fn error(self) -> Self {
        self.ui_color(UiColor::Error)
    }
}

#[async_trait]
impl CreateEmbedExt for CreateEmbed {
    fn create_reply(self) -> CreateReply {
        CreateReply::default().embed(self)
    }

    fn ui_color(self, color: UiColor) -> Self {
        self.color(color.as_serenity())
    }

    fn user(self, user: &User) -> Self {
        let avatar_url = user.avatar_url().unwrap_or(user.default_avatar_url());
        let embed_author = CreateEmbedAuthor::new(&user.name).icon_url(avatar_url);
        self.author(embed_author)
    }

    async fn member_color(self, ctx: &Context<'_>) -> Self {
        if let Some(member) = ctx.author_member().await {
            self.color(
                member
                    .colour(&ctx.serenity_context().cache)
                    .unwrap_or(Colour::LIGHT_GREY),
            )
        } else {
            self
        }
    }

    fn footer_text(self, text: impl Into<String>) -> Self {
        self.footer(CreateEmbedFooter::new(text))
    }
}
