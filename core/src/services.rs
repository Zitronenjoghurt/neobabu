use crate::stores::Stores;
use std::sync::Arc;

mod birthday;
pub mod rock_paper_scissors;

pub struct Services {
    pub birthday: Arc<birthday::BirthdayService>,
    pub rps: Arc<rock_paper_scissors::RockPaperScissorsService>,
}

impl Services {
    pub fn initialize(stores: &Arc<Stores>) -> Arc<Self> {
        Arc::new(Self {
            birthday: birthday::BirthdayService::initialize(stores),
            rps: rock_paper_scissors::RockPaperScissorsService::initialize(stores),
        })
    }
}
