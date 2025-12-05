use crate::error::CoreResult;
use image::RgbaImage;

pub mod cache;

#[derive(Debug, Clone)]
pub struct Atlas(RgbaImage);

impl Atlas {
    pub fn load(data: &[u8]) -> CoreResult<Self> {
        Ok(Self(image::load_from_memory(data)?.to_rgba8()))
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Option<&image::Rgba<u8>> {
        self.0.get_pixel_checked(x, y)
    }
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum AtlasId {
    ExteriorTileset,
    FarmingIcons,
    FarmingObjects,
    FarmingTileset,
}

impl AtlasId {
    pub fn data(&self) -> &'static [u8] {
        match self {
            Self::ExteriorTileset => include_bytes!("../../../assets/exterior_tileset.png"),
            Self::FarmingIcons => include_bytes!("../../../assets/farming_icons.png"),
            Self::FarmingObjects => include_bytes!("../../../assets/farming_objects.png"),
            Self::FarmingTileset => include_bytes!("../../../assets/farming_tileset.png"),
        }
    }

    pub fn load(&self) -> CoreResult<Atlas> {
        Atlas::load(self.data())
    }
}
