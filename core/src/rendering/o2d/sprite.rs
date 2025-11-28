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
}

pub struct Sprite {
    pub atlas_id: AtlasId,
    pub rect: Rect,
    // Offset from top of tile to bottom of sprite for y-layering
    pub y_sort_offset: u8,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum SpriteId {
    UntilledSoil,
}

impl SpriteId {
    pub fn get_sprite(&self) -> &'static Sprite {
        match self {
            Self::UntilledSoil => &Sprite {
                atlas_id: AtlasId::FarmingTileset,
                rect: Rect {
                    x: 160,
                    y: 16,
                    width: 16,
                    height: 16,
                },
                y_sort_offset: 0,
            },
        }
    }
}
