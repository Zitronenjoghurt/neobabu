use crate::context::ContextExt;
use crate::error::BotResult;
use crate::ui::embed::interactive::response::InteractiveEmbedResponse;
use crate::ui::embed::interactive::rows::InteractiveRow;
use crate::ui::embed::CreateEmbedExt;
use crate::ui::emoji::EmojiType;
use crate::Context;
use poise::serenity_prelude::{
    ButtonStyle, ComponentInteraction, CreateActionRow, CreateButton, CreateEmbed,
};
use std::ops::{Deref, DerefMut};

pub struct PaginationRow<T: PaginationRowTrait>(pub T);

impl<T> Deref for PaginationRow<T>
where
    T: PaginationRowTrait,
{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for PaginationRow<T>
where
    T: PaginationRowTrait,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[async_trait::async_trait]
pub trait PaginationRowTrait: Sized {
    fn get_page(&self) -> usize;
    fn set_page(&mut self, page: usize);
    fn max_pages(&self) -> usize;
    async fn render_page(&self, page: usize, ctx: &Context) -> BotResult<CreateEmbed>;

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

    fn build(self) -> PaginationRow<Self> {
        PaginationRow(self)
    }
}

#[async_trait::async_trait]
impl<T: PaginationRowTrait + Send + Sync> InteractiveRow for PaginationRow<T> {
    fn render(&self, context: &Context) -> Option<CreateActionRow> {
        if self.max_pages() <= 1 {
            return None;
        };

        if self.max_pages() <= 5 {
            return Some(CreateActionRow::Buttons(vec![
                CreateButton::new("pagination_row_left")
                    .style(ButtonStyle::Secondary)
                    .emoji(context.emoji(EmojiType::ArrowLeft)),
                CreateButton::new("pagination_row_right")
                    .style(ButtonStyle::Secondary)
                    .emoji(context.emoji(EmojiType::ArrowRight)),
            ]));
        }

        Some(CreateActionRow::Buttons(vec![
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
        ]))
    }

    fn matches(&self, custom_id: &str) -> bool {
        custom_id == "pagination_row_double_left"
            || custom_id == "pagination_row_left"
            || custom_id == "pagination_row_right"
            || custom_id == "pagination_row_double_right"
            || custom_id == "pagination_row_back"
    }

    async fn handle(
        &mut self,
        context: &Context,
        interaction: &ComponentInteraction,
    ) -> BotResult<InteractiveEmbedResponse> {
        match interaction.data.custom_id.as_str() {
            "pagination_row_double_left" => {
                let count = self.double_page_count();
                self.go_back(count);
            }
            "pagination_row_left" => {
                self.go_back(1);
            }
            "pagination_row_right" => {
                self.go_forward(1);
            }
            "pagination_row_double_right" => {
                let count = self.double_page_count();
                self.go_forward(count);
            }
            "pagination_row_back" => self.set_page(0),
            _ => {}
        };

        let embed = self.render_current_page(context).await?;
        Ok(InteractiveEmbedResponse::new().embed(embed))
    }
}
