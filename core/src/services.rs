use crate::integrations::apis::Apis;
use crate::stores::Stores;
use std::sync::Arc;

mod apod;
mod birthday;
mod rock_paper_scissors;
mod user;
mod youtube;

pub struct Services {
    pub apod: Arc<apod::ApodService>,
    pub birthday: Arc<birthday::BirthdayService>,
    pub rps: Arc<rock_paper_scissors::RockPaperScissorsService>,
    pub user: Arc<user::UserService>,
    pub youtube: Arc<youtube::YoutubeService>,
}

impl Services {
    pub fn initialize(apis: &Arc<Apis>, stores: &Arc<Stores>) -> Arc<Self> {
        Arc::new(Self {
            apod: apod::ApodService::initialize(apis, stores),
            birthday: birthday::BirthdayService::initialize(stores),
            rps: rock_paper_scissors::RockPaperScissorsService::initialize(stores),
            user: user::UserService::initialize(stores),
            youtube: youtube::YoutubeService::initialize(apis, stores),
        })
    }
}
