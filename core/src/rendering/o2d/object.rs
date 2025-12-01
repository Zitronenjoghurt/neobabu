use crate::rendering::o2d::object::position::PositionO2D;
use crate::rendering::o2d::object::visual::VisualO2D;

pub mod grid;
pub mod layer;
pub mod position;
pub mod visual;

#[derive(Debug, Clone, Copy)]
pub struct Object2D {
    pub visual: VisualO2D,
    pub position: PositionO2D,
}

impl Object2D {
    pub fn filter(color: image::Rgba<u8>) -> Self {
        Self {
            visual: VisualO2D::Filter(color),
            position: PositionO2D::default(),
        }
    }
}
