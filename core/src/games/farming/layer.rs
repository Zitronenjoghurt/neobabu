use crate::rendering::o2d::prelude::LayerO2D;

#[derive(Debug, Copy, Clone)]
pub enum FarmLayer {
    Water = 0,
    Ground = 1,
    OnGround = 2,
    Overlay = 100,
}

impl LayerO2D for FarmLayer {
    fn get_z_index(&self) -> u8 {
        *self as u8
    }
}
