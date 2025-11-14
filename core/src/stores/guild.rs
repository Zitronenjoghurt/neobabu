use crate::database::Database;
use std::sync::Arc;

pub struct GuildStore {
    db: Arc<Database>,
}

impl GuildStore {
    pub fn initialize(db: &Arc<Database>) -> Arc<Self> {
        Arc::new(Self { db: db.clone() })
    }
}
