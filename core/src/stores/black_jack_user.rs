use crate::database::Database;
use std::sync::Arc;

pub struct BlackJackUserStore {
    db: Arc<Database>,
}

impl BlackJackUserStore {
    pub fn initialize(db: &Arc<Database>) -> Arc<Self> {
        Arc::new(Self { db: db.clone() })
    }
}
