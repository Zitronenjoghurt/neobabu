use crate::error::CoreResult;
use serenity::all::{GuildId, GuildInfo, GuildPagination};
use std::collections::VecDeque;
use std::sync::Arc;

pub struct GuildIterator {
    http: Arc<serenity::http::Http>,
    buffer: VecDeque<GuildInfo>,
    last_id: Option<GuildId>,
    exhausted: bool,
}

impl GuildIterator {
    pub fn new(http: &Arc<serenity::http::Http>) -> Self {
        Self {
            http: http.clone(),
            buffer: VecDeque::new(),
            last_id: None,
            exhausted: false,
        }
    }

    async fn fetch_next_page(&mut self) -> CoreResult<()> {
        if self.exhausted {
            return Ok(());
        }

        let pagination = self.last_id.map(GuildPagination::After);
        let guilds = self.http.get_guilds(pagination, Some(100)).await?;

        if guilds.len() < 100 {
            self.exhausted = true;
        }

        if let Some(last) = guilds.last() {
            self.last_id = Some(last.id);
        }

        self.buffer.extend(guilds);
        Ok(())
    }

    pub async fn next(&mut self) -> CoreResult<Option<GuildInfo>> {
        if self.buffer.is_empty() && !self.exhausted {
            self.fetch_next_page().await?;
        }
        Ok(self.buffer.pop_front())
    }

    pub async fn collect(mut self) -> CoreResult<Vec<GuildInfo>> {
        while !self.exhausted {
            self.fetch_next_page().await?;
        }
        Ok(self.buffer.into())
    }
}
