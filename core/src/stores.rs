use crate::database::Database;
use std::sync::Arc;

pub use sea_orm::{IntoActiveModel, Set};

pub mod apod;
pub mod black_jack_user;
pub mod dashboard_session;
pub mod economy;
pub mod farming;
pub mod farming_world;
pub mod guild;
pub mod guild_apod;
pub mod guild_birthday;
pub mod guild_youtube;
pub mod guild_youtube_channel;
pub mod inventory_item;
pub mod rps_games;
pub mod rps_user;
pub mod user;
pub mod user_birthday;
pub mod user_guild;
pub mod youtube_channel;
pub mod youtube_video;

pub struct Stores {
    pub apod: Arc<apod::ApodStore>,
    pub bj_user: Arc<black_jack_user::BlackJackUserStore>,
    pub dashboard_session: Arc<dashboard_session::DashboardSessionStore>,
    pub economy: Arc<economy::EconomyStore>,
    pub farming: Arc<farming::FarmingStore>,
    pub farming_world: Arc<farming_world::FarmingWorldStore>,
    pub guild: Arc<guild::GuildStore>,
    pub guild_apod: Arc<guild_apod::GuildApodStore>,
    pub guild_birthday: Arc<guild_birthday::GuildBirthdayStore>,
    pub guild_youtube: Arc<guild_youtube::GuildYoutubeStore>,
    pub guild_youtube_channel: Arc<guild_youtube_channel::GuildYoutubeChannelStore>,
    pub item: Arc<inventory_item::InventoryItemStore>,
    pub rps_games: Arc<rps_games::RPSGamesStore>,
    pub rps_user: Arc<rps_user::RPSUserStore>,
    pub user: Arc<user::UserStore>,
    pub user_birthday: Arc<user_birthday::UserBirthdayStore>,
    pub user_guild: Arc<user_guild::UserGuildStore>,
    pub youtube_channel: Arc<youtube_channel::YoutubeChannelStore>,
    pub youtube_video: Arc<youtube_video::YoutubeVideoStore>,
}

impl Stores {
    pub fn initialize(db: &Arc<Database>) -> Arc<Self> {
        Arc::new(Self {
            apod: apod::ApodStore::initialize(db),
            bj_user: black_jack_user::BlackJackUserStore::initialize(db),
            dashboard_session: dashboard_session::DashboardSessionStore::initialize(db),
            economy: economy::EconomyStore::initialize(db),
            farming: farming::FarmingStore::initialize(db),
            farming_world: farming_world::FarmingWorldStore::initialize(db),
            guild: guild::GuildStore::initialize(db),
            guild_apod: guild_apod::GuildApodStore::initialize(db),
            guild_birthday: guild_birthday::GuildBirthdayStore::initialize(db),
            guild_youtube: guild_youtube::GuildYoutubeStore::initialize(db),
            guild_youtube_channel: guild_youtube_channel::GuildYoutubeChannelStore::initialize(db),
            item: inventory_item::InventoryItemStore::initialize(db),
            rps_games: rps_games::RPSGamesStore::initialize(db),
            rps_user: rps_user::RPSUserStore::initialize(db),
            user: user::UserStore::initialize(db),
            user_birthday: user_birthday::UserBirthdayStore::initialize(db),
            user_guild: user_guild::UserGuildStore::initialize(db),
            youtube_channel: youtube_channel::YoutubeChannelStore::initialize(db),
            youtube_video: youtube_video::YoutubeVideoStore::initialize(db),
        })
    }
}
