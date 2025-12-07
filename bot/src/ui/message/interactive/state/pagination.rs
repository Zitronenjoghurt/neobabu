use crate::context::ContextExt;
use crate::error::BotResult;
use crate::ui::emoji::EmojiType;
use crate::ui::message::interactive::state::{InteractiveState, InteractiveStateResponse};
use crate::ui::message::CreateEmbedExt;
use crate::Context;
use poise::serenity_prelude::{
    ButtonStyle, ComponentInteraction, CreateActionRow, CreateButton, CreateEmbed,
};

pub struct PaginationState<T: PaginationStateTrait>(pub T);

#[async_trait::async_trait]
pub trait PaginationStateTrait: Sized + Send + Sync {
    fn get_page(&self) -> usize;
    fn set_page(&mut self, page: usize);
    fn max_pages(&self) -> usize;
    async fn render_page(&self, page: usize, ctx: &Context) -> BotResult<CreateEmbed>;

    async fn content(&self, _ctx: &Context) -> BotResult<Option<String>> {
        Ok(None)
    }

    async fn render_current_page(&self, ctx: &Context) -> BotResult<CreateEmbed> {
        Ok(self
            .render_page(self.get_page(), ctx)
            .await?
            .footer_text(format!("Page {}/{}", self.get_page() + 1, self.max_pages())))
    }

    fn go_back(&mut self, count: usize) {
        if self.max_pages() == 0 {
            return;
        }

        let current = self.get_page() as isize;
        let max = self.max_pages() as isize;
        let count_i = count as isize;

        let new_page = if count > 1 && current != 0 {
            (current - count_i).max(0) as usize
        } else {
            (current - count_i).rem_euclid(max) as usize
        };

        self.set_page(new_page);
    }

    fn go_forward(&mut self, count: usize) {
        if self.max_pages() == 0 {
            return;
        }

        let current = self.get_page();
        let max = self.max_pages();

        let new_page = if count > 1 && current + 1 != max {
            (current + count).min(max - 1)
        } else {
            (current + count) % max
        };

        self.set_page(new_page);
    }

    fn double_page_count(&self) -> usize {
        self.max_pages() / 5 + 1
    }

    fn build(self) -> PaginationState<Self> {
        PaginationState(self)
    }
}

#[async_trait::async_trait]
impl<T: PaginationStateTrait> InteractiveState for PaginationState<T> {
    async fn handle_interaction(
        &mut self,
        _context: &Context,
        interaction: &ComponentInteraction,
    ) -> BotResult<InteractiveStateResponse> {
        let mut do_update = true;
        match interaction.data.custom_id.as_str() {
            "pagination_row_double_left" => {
                let count = self.0.double_page_count();
                self.0.go_back(count);
            }
            "pagination_row_left" => {
                self.0.go_back(1);
            }
            "pagination_row_right" => {
                self.0.go_forward(1);
            }
            "pagination_row_double_right" => {
                let count = self.0.double_page_count();
                self.0.go_forward(count);
            }
            "pagination_row_back" => self.0.set_page(0),
            _ => {
                do_update = false;
            }
        };

        Ok(InteractiveStateResponse::new().update(do_update))
    }

    async fn render_content(&self, context: &Context) -> BotResult<Option<String>> {
        self.0.content(context).await
    }

    async fn render_embed(&self, context: &Context) -> BotResult<CreateEmbed> {
        self.0.render_current_page(context).await
    }

    async fn render_rows(&self, context: &Context) -> BotResult<Vec<CreateActionRow>> {
        if self.0.max_pages() <= 1 {
            return Ok(vec![]);
        };

        if self.0.max_pages() <= 5 {
            return Ok(vec![CreateActionRow::Buttons(vec![
                CreateButton::new("pagination_row_left")
                    .style(ButtonStyle::Secondary)
                    .emoji(context.emoji(EmojiType::ArrowLeft)),
                CreateButton::new("pagination_row_right")
                    .style(ButtonStyle::Secondary)
                    .emoji(context.emoji(EmojiType::ArrowRight)),
            ])]);
        };

        Ok(vec![CreateActionRow::Buttons(vec![
            CreateButton::new("pagination_row_double_left")
                .style(ButtonStyle::Secondary)
                .emoji(context.emoji(EmojiType::ArrowDoubleLeft)),
            CreateButton::new("pagination_row_left")
                .style(ButtonStyle::Secondary)
                .emoji(context.emoji(EmojiType::ArrowLeft)),
            CreateButton::new("pagination_row_right")
                .style(ButtonStyle::Secondary)
                .emoji(context.emoji(EmojiType::ArrowRight)),
            CreateButton::new("pagination_row_double_right")
                .style(ButtonStyle::Secondary)
                .emoji(context.emoji(EmojiType::ArrowDoubleRight)),
            CreateButton::new("pagination_row_back")
                .style(ButtonStyle::Secondary)
                .emoji(context.emoji(EmojiType::ArrowBack)),
        ])])
    }
}
