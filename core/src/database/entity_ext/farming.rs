use crate::database::entity::farming;

impl farming::Model {
    pub fn max_worlds(&self) -> u64 {
        1
    }
}
