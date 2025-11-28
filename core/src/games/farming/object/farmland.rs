use crate::rendering::o2d::object::{Object, Position, Visual};
use crate::rendering::o2d::render::O2DRenderable;
use crate::rendering::o2d::sprite::SpriteId;
use crate::rendering::o2d::tileset::TilesetId;

#[derive(Debug, Clone, Copy)]
pub struct Farmland {
    pub x: u8,
    pub y: u8,
    pub is_tilled: bool,
    pub is_wet: bool,
}

impl O2DRenderable for Farmland {
    fn to_objects(&self) -> Vec<Object> {
        let position = Position::from_tile_xy(self.x, self.y);

        if !self.is_tilled {
            return vec![Object {
                visual: Visual::Sprite(SpriteId::UntilledSoil),
                position,
            }];
        }

        let dry = Object {
            visual: Visual::Tile(TilesetId::Farmland),
            position,
        };

        if !self.is_wet {
            return vec![dry];
        }

        let wet = Object {
            visual: Visual::Tile(TilesetId::FarmlandWet),
            position,
        };

        vec![dry, wet]
    }
}
