use crate::rendering::o2d::sprite::SpriteId;
use crate::rendering::o2d::tileset::TilesetId;
use image::Rgba;

#[derive(Debug, Clone, Copy)]
pub enum VisualO2D {
    Ambience(Rgba<u8>),
    Color(Rgba<u8>),
    PointLight(PointLight),
    Sprite(SpriteId),
    Tile(TilesetId),
}

impl VisualO2D {
    pub fn is_effect(&self) -> bool {
        matches!(self, VisualO2D::Ambience(_) | VisualO2D::PointLight { .. })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PointLight {
    pub color: Rgba<u8>,
    pub radius: f32,
    pub intensity: f32,
}
