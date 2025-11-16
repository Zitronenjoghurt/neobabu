#[derive(Debug, Clone)]
pub struct BirthdayNotification {
    pub user_id: String,
    pub guild_id: String,
    pub channel_id: String,
    pub age: Option<u8>,
    pub is_belated: bool,
}
