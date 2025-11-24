use serde::{Deserialize, Serialize};

pub mod birthday;

#[derive(Serialize, Deserialize)]
pub struct UserSettings {
    pub birthday: Option<birthday::UserBirthdaySettings>,
}
