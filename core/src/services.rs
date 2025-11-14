use crate::stores::Stores;
use std::sync::Arc;

mod birthday;

pub struct Services {
    pub birthday: Arc<birthday::BirthdayService>,
}

impl Services {
    pub fn initialize(stores: &Arc<Stores>) -> Arc<Self> {
        Arc::new(Self {
            birthday: birthday::BirthdayService::initialize(stores),
        })
    }
}
