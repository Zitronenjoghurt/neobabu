use crate::error::{handle_command_error, BotResult};
use crate::ui::color::UiColor;
use crate::ui::embed::interactive::response::{
    InteractiveEmbedResponse, InteractiveEmbedRowUpdate,
};
use crate::ui::embed::CreateEmbedExt;
use crate::Context;
use poise::futures_util::StreamExt;
use poise::serenity_prelude::{
    ComponentInteraction, CreateActionRow, CreateEmbed, CreateEmbedFooter,
    CreateInteractionResponse, CreateInteractionResponseMessage,
};
use poise::{CreateReply, ReplyHandle};
use rows::InteractiveRow;
use std::time::Duration;

pub mod response;
pub mod rows;

pub struct InteractiveEmbed<'a> {
    embed: CreateEmbed,
    rows: Vec<Box<dyn InteractiveRow>>,
    context: &'a Context<'a>,
    timeout: Duration,
    allow_anyone_to_interact: bool,
}

impl<'a> InteractiveEmbed<'a> {
    pub fn new(context: &'a Context<'a>, embed: CreateEmbed) -> Self {
        Self {
            embed,
            rows: vec![],
            context,
            timeout: Duration::from_secs(300),
            allow_anyone_to_interact: false,
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

    pub fn allow_anyone_to_interact(mut self, allow: bool) -> Self {
        self.allow_anyone_to_interact = allow;
        self
    }

    pub async fn run(mut self) -> BotResult<()> {
        let reply_handle = self
            .context
            .send(
                CreateReply::default()
                    .embed(self.embed.clone())
                    .components(self.render_rows()),
            )
            .await?;

        let result = self.do_run(&reply_handle).await;
        if let Err(error) = result {
            let error_embed = handle_command_error(error, self.context).await;
            reply_handle
                .edit(
                    *self.context,
                    CreateReply::default().embed(error_embed).components(vec![]),
                )
                .await?;
        }

        Ok(())
    }

    async fn do_run(&mut self, reply_handle: &ReplyHandle<'_>) -> BotResult<()> {
        let message = reply_handle.message().await?;
        let collector = message
            .await_component_interaction(self.context.serenity_context())
            .timeout(self.timeout);

        let mut collector_stream = collector.stream();
        while let Some(interaction) = collector_stream.next().await {
            if !self.allow_anyone_to_interact && interaction.user.id != self.context.author().id {
                interaction
                    .create_response(
                        self.context.serenity_context(),
                        CreateInteractionResponse::Acknowledge,
                    )
                    .await?;
                continue;
            }

            let Some(component_index) = self.find_component_index(&interaction.data.custom_id)
            else {
                interaction
                    .create_response(
                        self.context.serenity_context(),
                        CreateInteractionResponse::Acknowledge,
                    )
                    .await?;
                continue;
            };

            let component = self.rows.get_mut(component_index).unwrap();
            let response = component.handle(self.context, &interaction).await?;
            let do_stop = response.do_stop;
            self.handle_row_response(component_index, &interaction, response)
                .await?;

            if do_stop {
                return Ok(());
            }
        }

        let timeout_embed = self
            .embed
            .clone()
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
        self.rows
            .iter()
            .filter_map(|c| c.render(self.context))
            .collect()
    }

    fn find_component_index(&self, custom_id: &str) -> Option<usize> {
        self.rows
            .iter()
            .enumerate()
            .find(|(_, c)| c.matches(custom_id))
            .map(|(i, _)| i)
    }

    async fn handle_row_response(
        &mut self,
        component_index: usize,
        interaction: &ComponentInteraction,
        response: InteractiveEmbedResponse,
    ) -> BotResult<()> {
        if response.has_no_change() {
            interaction
                .create_response(
                    self.context.serenity_context(),
                    CreateInteractionResponse::Acknowledge,
                )
                .await?;
            return Ok(());
        }

        match response.row_update {
            InteractiveEmbedRowUpdate::Keep => {}
            InteractiveEmbedRowUpdate::Remove => {
                self.rows.remove(component_index);
            }
            InteractiveEmbedRowUpdate::RemoveAll => self.rows = vec![],
            InteractiveEmbedRowUpdate::Replace(row) => self.rows[component_index] = row,
            InteractiveEmbedRowUpdate::ReplaceAll(rows) => self.rows = rows,
        };

        let action_rows = self.render_rows();
        if let Some(embed) = response.embed {
            self.embed = embed;
        }

        interaction
            .create_response(
                self.context.serenity_context(),
                CreateInteractionResponse::UpdateMessage(
                    CreateInteractionResponseMessage::default()
                        .embed(self.embed.clone())
                        .components(action_rows),
                ),
            )
            .await?;

        Ok(())
    }
}
