use crate::database::entity::youtube_video;

impl youtube_video::Model {
    pub fn url(&self) -> String {
        format!("https://www.youtube.com/watch?v={}", self.id)
    }
}
