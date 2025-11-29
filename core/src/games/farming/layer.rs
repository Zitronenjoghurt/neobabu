use crate::rendering::o2d::prelude::LayerO2D;

#[derive(Debug, Copy, Clone)]
pub enum FarmLayer {
    Ground = 0,
    OnGround = 1,
}

impl LayerO2D for FarmLayer {
    fn get_z_index(&self) -> u8 {
        *self as u8
    }
}
