use crate::ui::embed::interactive::rows::InteractiveRow;
use poise::serenity_prelude::CreateEmbed;

#[derive(Default)]
pub struct InteractiveEmbedResponse {
    pub embed: Option<CreateEmbed>,
    pub do_stop: bool,
    pub row_update: InteractiveEmbedRowUpdate,
}

impl InteractiveEmbedResponse {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn embed(mut self, embed: CreateEmbed) -> Self {
        self.embed = Some(embed);
        self
    }

    pub fn stop(mut self) -> Self {
        self.do_stop = true;
        self
    }

    pub fn stop_with_embed(embed: CreateEmbed) -> Self {
        Self::new().embed(embed).stop()
    }

    pub fn halt_with(embed: CreateEmbed) -> Self {
        Self::new().embed(embed).stop().remove_all_rows()
    }

    pub fn remove_row(mut self) -> Self {
        self.row_update = InteractiveEmbedRowUpdate::Remove;
        self
    }

    pub fn remove_all_rows(mut self) -> Self {
        self.row_update = InteractiveEmbedRowUpdate::RemoveAll;
        self
    }

    pub fn replace_row(mut self, row: impl InteractiveRow + 'static) -> Self {
        self.row_update = InteractiveEmbedRowUpdate::Replace(Box::new(row));
        self
    }

    pub fn replace_all_rows<I, R>(mut self, rows: I) -> Self
    where
        I: IntoIterator<Item = R>,
        R: InteractiveRow + 'static,
    {
        self.row_update = InteractiveEmbedRowUpdate::ReplaceAll(
            rows.into_iter()
                .map(|r| Box::new(r) as Box<dyn InteractiveRow>)
                .collect(),
        );
        self
    }

    pub fn has_no_change(&self) -> bool {
        self.embed.is_none()
            && !self.do_stop
            && matches!(self.row_update, InteractiveEmbedRowUpdate::Keep)
    }
}

#[derive(Default)]
pub enum InteractiveEmbedRowUpdate {
    #[default]
    Keep,
    Remove,
    RemoveAll,
    Replace(Box<dyn InteractiveRow>),
    ReplaceAll(Vec<Box<dyn InteractiveRow>>),
}
