use crate::rendering::o2d::atlas::AtlasId;
use crate::rendering::o2d::sprite::{Rect, Sprite};
use crate::rendering::o2d::tileset::mask::{TileMask, TileMaskLayout};

pub mod mask;

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum TilesetId {
    FoliageSpring,
    FoliageSummer,
    FoliageAutumn,
    FoliageWinter,
    Rock,
    RockSpring,
    RockSummer,
    RockAutumn,
    RockWinter,
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
            Self::FoliageSpring
            | Self::FoliageSummer
            | Self::FoliageAutumn
            | Self::FoliageWinter
            | Self::Rock
            | Self::RockSpring
            | Self::RockSummer
            | Self::RockAutumn
            | Self::RockWinter => AtlasId::ExteriorTileset,
            Self::TilledSoil | Self::TilledWetSoil => AtlasId::FarmingTileset,
        }
    }

    pub fn layout(&self) -> TileMaskLayout {
        match self {
            Self::FoliageSpring
            | Self::FoliageSummer
            | Self::FoliageAutumn
            | Self::FoliageWinter
            | Self::TilledSoil
            | Self::TilledWetSoil => TileMaskLayout::Blob,
            Self::Rock
            | Self::RockSpring
            | Self::RockSummer
            | Self::RockAutumn
            | Self::RockWinter => TileMaskLayout::ThreeByThree2x2Hole,
        }
    }

    pub fn tile_size(&self) -> u8 {
        16
    }

    pub fn tile_offset(&self) -> (u8, u8) {
        match self {
            Self::FoliageSpring => (12, 0),
            Self::FoliageSummer => (24, 0),
            Self::FoliageAutumn => (36, 0),
            Self::FoliageWinter => (48, 0),
            Self::Rock => (5, 13),
            Self::RockSpring => (17, 13),
            Self::RockSummer => (29, 13),
            Self::RockAutumn => (41, 13),
            Self::RockWinter => (53, 13),
            Self::TilledSoil => (0, 0),
            Self::TilledWetSoil => (0, 4),
        }
    }

    pub fn connects_with(&self, tileset_id: TilesetId) -> bool {
        match self {
            Self::RockSpring => tileset_id == Self::FoliageSpring || tileset_id == Self::Rock,
            Self::FoliageSpring => tileset_id == Self::RockSpring,
            Self::RockSummer => tileset_id == Self::FoliageSummer || tileset_id == Self::Rock,
            Self::FoliageSummer => tileset_id == Self::RockSummer,
            Self::RockAutumn => tileset_id == Self::FoliageAutumn || tileset_id == Self::Rock,
            Self::FoliageAutumn => tileset_id == Self::RockAutumn,
            Self::RockWinter => tileset_id == Self::FoliageWinter || tileset_id == Self::Rock,
            Self::FoliageWinter => tileset_id == Self::RockWinter,
            _ => false,
        }
    }
}
