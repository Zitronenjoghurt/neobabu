use crate::rendering::o2d::atlas::AtlasId;
use crate::rendering::o2d::sprite::{Rect, Sprite};
use crate::rendering::o2d::tileset::mask::{TileMask, TileMaskLayout};

pub mod mask;

pub struct Tileset {
    pub atlas_id: AtlasId,
    pub layout: TileMaskLayout,
    pub tile_size: u8,
    pub tile_offset_x: u8,
    pub tile_offset_y: u8,
}

impl Tileset {
    pub fn get_sprite(&self, mask: TileMask) -> Sprite {
        let rel = mask.tile_position(self.layout);
        let x = (rel.0 + self.tile_offset_x) * self.tile_size;
        let y = (rel.1 + self.tile_offset_y) * self.tile_size;
        let rect = Rect::square(x as u32, y as u32, self.tile_size as u32);
        Sprite {
            atlas_id: self.atlas_id,
            rect,
            y_sort_offset: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum TilesetId {
    Farmland,
    FarmlandWet,
}

impl TilesetId {
    pub fn tile_set(&self) -> &'static Tileset {
        match self {
            Self::Farmland => &Tileset {
                atlas_id: AtlasId::FarmingTileset,
                layout: TileMaskLayout::Blob,
                tile_size: 16,
                tile_offset_x: 0,
                tile_offset_y: 0,
            },
            Self::FarmlandWet => &Tileset {
                atlas_id: AtlasId::FarmingTileset,
                layout: TileMaskLayout::Blob,
                tile_size: 16,
                tile_offset_x: 0,
                tile_offset_y: 4,
            },
        }
    }
}
