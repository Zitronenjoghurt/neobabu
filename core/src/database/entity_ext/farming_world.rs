use crate::database::entity::farming_world;
use crate::error::CoreResult;
use crate::games::farming::world::FarmWorld;

impl farming_world::Model {
    pub fn data(&self) -> CoreResult<FarmWorld> {
        Ok(serde_json::from_value(self.data.clone())?)
    }
}
