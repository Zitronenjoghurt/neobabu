use crate::database::Database;
use std::sync::Arc;

pub use sea_orm::{IntoActiveModel, Set};

pub mod apod;
pub mod dashboard_session;
pub mod guild;
pub mod guild_apod;
pub mod guild_birthday;
pub mod rps_games;
pub mod rps_user;
pub mod user;
pub mod user_birthday;
pub mod user_guild;

pub struct Stores {
    pub apod: Arc<apod::ApodStore>,
    pub dashboard_session: Arc<dashboard_session::DashboardSessionStore>,
    pub guild: Arc<guild::GuildStore>,
    pub guild_apod: Arc<guild_apod::GuildApodStore>,
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
            apod: apod::ApodStore::initialize(db),
            dashboard_session: dashboard_session::DashboardSessionStore::initialize(db),
            guild: guild::GuildStore::initialize(db),
            guild_apod: guild_apod::GuildApodStore::initialize(db),
            guild_birthday: guild_birthday::GuildBirthdayStore::initialize(db),
            rps_games: rps_games::RPSGamesStore::initialize(db),
            rps_user: rps_user::RPSUserStore::initialize(db),
            user: user::UserStore::initialize(db),
            user_birthday: user_birthday::UserBirthdayStore::initialize(db),
            user_guild: user_guild::UserGuildStore::initialize(db),
        })
    }
}
