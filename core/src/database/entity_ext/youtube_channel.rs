use crate::database::entity::youtube_channel;

impl youtube_channel::Model {
    pub fn url(&self) -> String {
        format!("https://www.youtube.com/channel/{}", self.id)
    }
}
