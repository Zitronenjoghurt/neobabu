use crate::database::Database;
use std::sync::Arc;

pub struct BlackJackRivalsStore {
    db: Arc<Database>,
}

impl BlackJackRivalsStore {
    pub fn initialize(db: &Arc<Database>) -> Arc<Self> {
        Arc::new(Self { db: db.clone() })
    }
}
