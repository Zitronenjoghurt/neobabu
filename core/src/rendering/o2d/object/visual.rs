use crate::rendering::o2d::sprite::SpriteId;
use crate::rendering::o2d::tileset::TilesetId;
use image::Rgba;

#[derive(Debug, Clone, Copy)]
pub enum VisualO2D {
    Color(Rgba<u8>),
    Filter(Rgba<u8>),
    Sprite(SpriteId),
    Tile(TilesetId),
}

impl VisualO2D {
    pub fn is_effect(&self) -> bool {
        matches!(self, Self::Filter(_))
    }
}
