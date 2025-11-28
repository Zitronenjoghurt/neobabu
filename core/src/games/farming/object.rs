use crate::rendering::o2d::object::Object;
use crate::rendering::o2d::render::O2DRenderable;

pub mod farmland;

#[derive(Debug, Clone, Copy)]
pub enum FarmObject {
    Farmland(farmland::Farmland),
}

impl FarmObject {
    pub fn untilled_farmland(x: u8, y: u8) -> Self {
        Self::Farmland(farmland::Farmland {
            x,
            y,
            is_tilled: false,
            is_wet: false,
        })
    }

    pub fn get_tile_coords(&self) -> (u8, u8) {
        match self {
            Self::Farmland(farmland) => (farmland.x, farmland.y),
        }
    }

    pub fn till(&mut self) {
        match self {
            Self::Farmland(farmland) => farmland.is_tilled = true,
            _ => {}
        }
    }

    pub fn water(&mut self) {
        match self {
            Self::Farmland(farmland) => farmland.is_wet = true,
            _ => {}
        }
    }
}

impl O2DRenderable for FarmObject {
    fn to_objects(&self) -> Vec<Object> {
        match self {
            Self::Farmland(farmland) => farmland.to_objects(),
        }
    }
}
