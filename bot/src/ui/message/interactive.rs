use crate::error::{handle_command_error, BotResult};
use crate::ui::color::UiColor;
use crate::ui::message::interactive::state::{InteractiveState, InteractiveStateResponse};
use crate::ui::message::CreateEmbedExt;
use crate::Context;
use poise::futures_util::StreamExt;
use poise::serenity_prelude::{
    ComponentInteraction, CreateEmbed, CreateEmbedFooter, CreateInteractionResponse,
    CreateInteractionResponseMessage,
};
use poise::{CreateReply, ReplyHandle};
use std::time::Duration;

pub mod state;

pub struct InteractiveMessage<'a> {
    state: Box<dyn InteractiveState>,
    ctx: &'a Context<'a>,
    timeout: Duration,
    tick_interval: Option<Duration>,
    on_timeout: Option<CreateEmbed>,
    allow_anyone_to_interact: bool,
}

impl<'a> InteractiveMessage<'a> {
    pub fn new(ctx: &'a Context<'a>, state: impl InteractiveState + 'static) -> Self {
        Self {
            state: Box::new(state),
            ctx,
            timeout: Duration::from_secs(300),
            tick_interval: None,
            on_timeout: None,
            allow_anyone_to_interact: false,
        }
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn on_timeout(mut self, embed: CreateEmbed) -> Self {
        self.on_timeout = Some(embed);
        self
    }

    pub fn allow_anyone_to_interact(mut self, allow: bool) -> Self {
        self.allow_anyone_to_interact = allow;
        self
    }

    pub fn tick_interval(mut self, interval: Duration) -> Self {
        self.tick_interval = Some(interval);
        self
    }

    pub async fn run(mut self) -> BotResult<()> {
        let mut reply = CreateReply::default()
            .embed(self.state.render_embed(self.ctx).await?)
            .components(self.state.render_rows(self.ctx).await?);
        if let Some(message) = &self.state.render_content(self.ctx).await? {
            reply = reply.content(message);
        }

        let reply_handle = self.ctx.send(reply).await?;
        let result = self.do_run(&reply_handle).await;
        if let Err(error) = result {
            let error_embed = handle_command_error(error, self.ctx).await;
            reply_handle
                .edit(
                    *self.ctx,
                    CreateReply::default().embed(error_embed).components(vec![]),
                )
                .await?;
        }

        Ok(())
    }

    async fn do_run(&mut self, reply_handle: &ReplyHandle<'_>) -> BotResult<()> {
        let message = reply_handle.message().await?;
        let collector = message.await_component_interaction(self.ctx.serenity_context());

        let mut collector_stream = collector.stream();
        let mut tick_interval = self.tick_interval.map(|d| tokio::time::interval(d));

        let sleep = tokio::time::sleep(self.timeout);
        tokio::pin!(sleep);

        loop {
            tokio::select! {
                Some(interaction) = collector_stream.next() => {
                    if !self.allow_anyone_to_interact && interaction.user.id != self.ctx.author().id {
                        interaction
                            .create_response(
                                self.ctx.serenity_context(),
                                CreateInteractionResponse::Acknowledge,
                            )
                            .await?;
                        continue;
                    }

                    let response = self.state.handle_interaction(self.ctx, &interaction).await?;

                    if response.do_update {
                        self.update_interaction(&interaction, &response).await?;
                    } else {
                        interaction
                            .create_response(
                                self.ctx.serenity_context(),
                                CreateInteractionResponse::Acknowledge,
                            )
                            .await?;
                    }

                    if response.do_stop {
                        return Ok(());
                    }
                }

                Some(_) = async {
                    match &mut tick_interval {
                        Some(interval) => Some(interval.tick().await),
                        None => std::future::pending().await
                    }
                } => {
                    let response = self.state.on_tick(self.ctx).await?;

                    if response.do_update {
                        self.update_reply(reply_handle, &response).await?;
                    }

                    if response.do_stop {
                        return Ok(());
                    }
                }

                _ = &mut sleep => {
                    let timeout_embed = if let Some(embed) = self.on_timeout.clone() {
                        embed
                    } else {
                        self.state.render_embed(self.ctx).await?
                            .footer(CreateEmbedFooter::new("This interaction has timed out."))
                            .ui_color(UiColor::Gray)
                    };

                    reply_handle
                        .edit(
                            *self.ctx,
                            CreateReply::default()
                                .embed(timeout_embed)
                                .components(vec![]),
                        )
                        .await?;

                    break;
                }
            }
        }

        Ok(())
    }

    async fn update_interaction(
        &mut self,
        interaction: &ComponentInteraction,
        response: &InteractiveStateResponse,
    ) -> BotResult<()> {
        let components = if response.do_stop {
            vec![]
        } else {
            self.state.render_rows(self.ctx).await?
        };

        interaction
            .create_response(
                self.ctx.serenity_context(),
                CreateInteractionResponse::UpdateMessage(
                    CreateInteractionResponseMessage::default()
                        .content(
                            self.state
                                .render_content(self.ctx)
                                .await?
                                .unwrap_or_default(),
                        )
                        .embed(self.state.render_embed(self.ctx).await?)
                        .components(components),
                ),
            )
            .await?;
        Ok(())
    }

    async fn update_reply(
        &mut self,
        reply_handle: &ReplyHandle<'_>,
        response: &InteractiveStateResponse,
    ) -> BotResult<()> {
        let components = if response.do_stop {
            vec![]
        } else {
            self.state.render_rows(self.ctx).await?
        };

        reply_handle
            .edit(
                *self.ctx,
                CreateReply::default()
                    .content(
                        self.state
                            .render_content(self.ctx)
                            .await?
                            .unwrap_or_default(),
                    )
                    .embed(self.state.render_embed(self.ctx).await?)
                    .components(components),
            )
            .await?;
        Ok(())
    }
}
