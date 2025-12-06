use crate::rendering::o2d::sprite::SpriteId;
use crate::rendering::o2d::tileset::TilesetId;
use image::Rgba;

#[derive(Debug, Clone)]
pub enum VisualO2D {
    Ambience(Rgba<u8>),
    Color(Rgba<u8>),
    PointLight(PointLight),
    Sprite(SpriteId),
    Tile(TilesetId),
    Text(TextVisual),
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

#[derive(Debug, Clone)]
pub struct TextVisual {
    pub text: String,
    pub color: Rgba<u8>,
    pub wrapping: bool,
    pub max_width: Option<u32>,
}

impl TextVisual {
    pub fn new(text: impl Into<String>, color: impl Into<Rgba<u8>>) -> Self {
        Self {
            text: text.into(),
            color: color.into(),
            wrapping: false,
            max_width: None,
        }
    }

    pub fn with_wrapping(mut self) -> Self {
        self.wrapping = true;
        self
    }

    pub fn with_max_width(mut self, max_width: u32) -> Self {
        self.max_width = Some(max_width);
        self
    }
}
