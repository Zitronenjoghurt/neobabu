use crate::rendering::o2d::atlas::AtlasId;
use crate::rendering::o2d::sprite::{Rect, Sprite};
use crate::rendering::o2d::tileset::mask::{TileMask, TileMaskLayout};

pub mod mask;

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum TilesetId {
    FoliageSpring,
    Rock,
    RockSpring,
    TilledSoil,
    TilledWetSoil,
}

impl TilesetId {
    pub fn get_sprite(&self, mask: TileMask) -> Sprite {
        let rel = mask.tile_position(self.layout());
        let (tile_offset_x, tile_offset_y) = self.tile_offset();
        let x = (rel.0 + tile_offset_x) as u32 * self.tile_size() as u32;
        let y = (rel.1 + tile_offset_y) as u32 * self.tile_size() as u32;
        let rect = Rect::square(x, y, self.tile_size() as u32);
        Sprite {
            atlas_id: self.atlas_id(),
            rect,
            y_sort_offset: 0,
        }
    }

    pub fn atlas_id(&self) -> AtlasId {
        match self {
            Self::FoliageSpring | Self::Rock | Self::RockSpring => AtlasId::ExteriorTileset,
            Self::TilledSoil | Self::TilledWetSoil => AtlasId::FarmingTileset,
        }
    }

    pub fn layout(&self) -> TileMaskLayout {
        match self {
            Self::FoliageSpring | Self::TilledSoil | Self::TilledWetSoil => TileMaskLayout::Blob,
            Self::Rock | Self::RockSpring => TileMaskLayout::ThreeByThree2x2Hole,
        }
    }

    pub fn tile_size(&self) -> u8 {
        16
    }

    pub fn tile_offset(&self) -> (u8, u8) {
        match self {
            Self::FoliageSpring => (12, 0),
            Self::Rock => (5, 13),
            Self::RockSpring => (17, 13),
            Self::TilledSoil => (0, 0),
            Self::TilledWetSoil => (0, 4),
        }
    }

    pub fn connects_with(&self, tileset_id: TilesetId) -> bool {
        match self {
            Self::RockSpring => tileset_id == Self::FoliageSpring || tileset_id == Self::Rock,
            Self::FoliageSpring => tileset_id == Self::RockSpring,
            _ => false,
        }
    }
}
