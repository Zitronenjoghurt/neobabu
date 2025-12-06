use crate::rendering::o2d::object::position::PositionO2D;
use crate::rendering::o2d::object::visual::{TextVisual, VisualO2D};

pub mod grid;
pub mod layer;
pub mod position;
pub mod visual;

#[derive(Debug, Clone)]
pub struct Object2D {
    pub visual: VisualO2D,
    pub position: PositionO2D,
}

impl Object2D {
    pub fn ambience(color: image::Rgba<u8>) -> Self {
        Self {
            visual: VisualO2D::Ambience(color),
            position: PositionO2D::default(),
        }
    }

    pub fn text(text: TextVisual, position: PositionO2D) -> Self {
        Self {
            visual: VisualO2D::Text(text),
            position,
        }
    }
}
