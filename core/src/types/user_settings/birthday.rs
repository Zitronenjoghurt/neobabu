use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserBirthdaySettings {
    pub day: i16,
    pub month: i16,
    pub year: Option<i16>,
    pub updated_at: i64,
}
