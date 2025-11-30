use crate::rendering::o2d::atlas::AtlasId;

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl Rect {
    pub fn square(x: u32, y: u32, size: u32) -> Self {
        Self {
            x,
            y,
            width: size,
            height: size,
        }
    }

    pub const fn from_tiled(x: u8, y: u8, width: u8, height: u8) -> Self {
        Self {
            x: x as u32 * width as u32,
            y: y as u32 * height as u32,
            width: width as u32,
            height: height as u32,
        }
    }
}

pub struct Sprite {
    pub atlas_id: AtlasId,
    pub rect: Rect,
    // Offset from top of tile to bottom of sprite for y-layering
    pub y_sort_offset: u8,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum SpriteId {
    Carrot5,
    UntilledSoil,
    Water,
}

impl SpriteId {
    pub fn get_sprite(&self) -> Sprite {
        match self {
            Self::Carrot5 => Sprite {
                atlas_id: AtlasId::FarmingObjects,
                rect: Rect::from_tiled(4, 0, 16, 32),
                y_sort_offset: 0,
            },
            Self::UntilledSoil => Sprite {
                atlas_id: AtlasId::FarmingTileset,
                rect: Rect::from_tiled(10, 1, 16, 16),
                y_sort_offset: 0,
            },
            Self::Water => Sprite {
                atlas_id: AtlasId::ExteriorTileset,
                rect: Rect::from_tiled(10, 14, 16, 16),
                y_sort_offset: 0,
            },
        }
    }
}
