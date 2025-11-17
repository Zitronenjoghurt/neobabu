use std::collections::HashMap;

#[derive(Default, serde::Deserialize)]
pub struct BotConfigEmoji(HashMap<String, u64>);

impl BotConfigEmoji {
    pub fn id(&self, emoji_name: &str) -> Option<&u64> {
        self.0.get(emoji_name)
    }
}
