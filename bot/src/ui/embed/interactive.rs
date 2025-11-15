use crate::error::BotResult;
use crate::ui::color::UiColor;
use crate::ui::embed::CreateEmbedExt;
use crate::Context;
use poise::futures_util::StreamExt;
use poise::serenity_prelude::{
    ComponentInteraction, CreateActionRow, CreateEmbed, CreateEmbedFooter,
    CreateInteractionResponse, CreateInteractionResponseMessage,
};
use poise::CreateReply;
use std::time::Duration;

pub enum InteractionResult {
    Continue(CreateEmbed),
    Stop(CreateEmbed),
    Acknowledge,
}

#[async_trait::async_trait]
pub trait InteractiveRow: Send + Sync {
    fn render(&self) -> CreateActionRow;

    fn matches(&self, custom_id: &str) -> bool;

    async fn handle(
        &self,
        context: &Context,
        interaction: &ComponentInteraction,
    ) -> InteractionResult;
}

pub struct InteractiveEmbed<'a> {
    embed: CreateEmbed,
    rows: Vec<Box<dyn InteractiveRow>>,
    context: &'a Context<'a>,
    timeout: Duration,
}

impl<'a> InteractiveEmbed<'a> {
    pub fn new(context: &'a Context<'a>, embed: CreateEmbed) -> Self {
        Self {
            embed,
            rows: vec![],
            context,
            timeout: Duration::from_secs(300),
        }
    }

    pub fn row(mut self, row: impl InteractiveRow + 'static) -> Self {
        self.rows.push(Box::new(row));
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub async fn run(self) -> BotResult<()> {
        let action_rows = self.render_rows();

        let reply_handle = self
            .context
            .send(
                CreateReply::default()
                    .embed(self.embed.clone())
                    .components(action_rows.clone()),
            )
            .await?;

        let message = reply_handle.message().await?;
        let collector = message
            .await_component_interaction(self.context.serenity_context())
            .timeout(self.timeout);

        let mut collector_stream = collector.stream();
        while let Some(interaction) = collector_stream.next().await {
            let Some(component) = self.find_component(&interaction.data.custom_id) else {
                interaction
                    .create_response(
                        self.context.serenity_context(),
                        CreateInteractionResponse::Acknowledge,
                    )
                    .await?;
                continue;
            };

            let interaction_result = component.handle(self.context, &interaction).await;

            match interaction_result {
                InteractionResult::Continue(new_embed) => {
                    let action_rows = self.render_rows();
                    interaction
                        .create_response(
                            self.context.serenity_context(),
                            CreateInteractionResponse::UpdateMessage(
                                CreateInteractionResponseMessage::default()
                                    .embed(new_embed)
                                    .components(action_rows),
                            ),
                        )
                        .await?;
                }
                InteractionResult::Stop(new_embed) => {
                    interaction
                        .create_response(
                            self.context.serenity_context(),
                            CreateInteractionResponse::UpdateMessage(
                                CreateInteractionResponseMessage::default()
                                    .embed(new_embed)
                                    .components(vec![]),
                            ),
                        )
                        .await?;
                    return Ok(());
                }
                InteractionResult::Acknowledge => {
                    interaction
                        .create_response(
                            self.context.serenity_context(),
                            CreateInteractionResponse::Acknowledge,
                        )
                        .await?;
                }
            }
        }

        let timeout_embed = self
            .embed
            .footer(CreateEmbedFooter::new("This interaction has timed out."))
            .ui_color(UiColor::Gray);

        reply_handle
            .edit(
                *self.context,
                CreateReply::default()
                    .embed(timeout_embed)
                    .components(vec![]),
            )
            .await?;

        Ok(())
    }

    fn render_rows(&self) -> Vec<CreateActionRow> {
        self.rows.iter().map(|c| c.render()).collect()
    }

    fn find_component(&self, custom_id: &str) -> Option<&Box<dyn InteractiveRow>> {
        self.rows.iter().find(|c| c.matches(custom_id))
    }
}
