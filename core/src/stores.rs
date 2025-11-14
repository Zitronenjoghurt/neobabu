use crate::database::Database;
use std::sync::Arc;

mod guild;
mod guild_birthday;
mod user;
mod user_birthday;

pub struct Stores {
    pub guild: Arc<guild::GuildStore>,
    pub guild_birthday: Arc<guild_birthday::GuildBirthdayStore>,
    pub user: Arc<user::UserStore>,
    pub user_birthday: Arc<user_birthday::UserBirthdayStore>,
}

impl Stores {
    pub fn initialize(db: &Arc<Database>) -> Arc<Self> {
        Arc::new(Self {
            guild: guild::GuildStore::initialize(db),
            guild_birthday: guild_birthday::GuildBirthdayStore::initialize(db),
            user: user::UserStore::initialize(db),
            user_birthday: user_birthday::UserBirthdayStore::initialize(db),
        })
    }
}
