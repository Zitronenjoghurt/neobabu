use crate::database::Database;
use std::sync::Arc;

pub use sea_orm::{IntoActiveModel, Set};

mod guild;
mod guild_birthday;
mod rps_games;
mod rps_user;
mod user;
mod user_birthday;
mod user_guild;

pub struct Stores {
    pub guild: Arc<guild::GuildStore>,
    pub guild_birthday: Arc<guild_birthday::GuildBirthdayStore>,
    pub rps_games: Arc<rps_games::RPSGamesStore>,
    pub rps_user: Arc<rps_user::RPSUserStore>,
    pub user: Arc<user::UserStore>,
    pub user_birthday: Arc<user_birthday::UserBirthdayStore>,
    pub user_guild: Arc<user_guild::UserGuildStore>,
}

impl Stores {
    pub fn initialize(db: &Arc<Database>) -> Arc<Self> {
        Arc::new(Self {
            guild: guild::GuildStore::initialize(db),
            guild_birthday: guild_birthday::GuildBirthdayStore::initialize(db),
            rps_games: rps_games::RPSGamesStore::initialize(db),
            rps_user: rps_user::RPSUserStore::initialize(db),
            user: user::UserStore::initialize(db),
            user_birthday: user_birthday::UserBirthdayStore::initialize(db),
            user_guild: user_guild::UserGuildStore::initialize(db),
        })
    }
}
