use crate::error::CoreResult;
use crate::rendering::o2d::atlas::{Atlas, AtlasId};
use std::sync::Arc;

pub struct AtlasCache {
    pub farming_icons: Atlas,
    pub farming_objects: Atlas,
    pub farming_tileset: Atlas,
}

impl AtlasCache {
    pub fn initialize() -> CoreResult<Arc<Self>> {
        let cache = Self {
            farming_icons: AtlasId::FarmingIcons.load()?,
            farming_objects: AtlasId::FarmingObjects.load()?,
            farming_tileset: AtlasId::FarmingTileset.load()?,
        };
        Ok(Arc::new(cache))
    }

    pub fn get(&self, atlas_id: AtlasId) -> CoreResult<&Atlas> {
        Ok(match atlas_id {
            AtlasId::FarmingIcons => &self.farming_icons,
            AtlasId::FarmingObjects => &self.farming_objects,
            AtlasId::FarmingTileset => &self.farming_tileset,
        })
    }
}
