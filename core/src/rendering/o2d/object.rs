use crate::rendering::o2d::sprite::SpriteId;
use crate::rendering::o2d::tileset::mask::TileMask;
use crate::rendering::o2d::tileset::TilesetId;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub struct Object {
    pub visual: Visual,
    pub position: Position,
}

#[derive(Debug, Clone, Copy)]
pub enum Visual {
    Sprite(SpriteId),
    Tile(TilesetId),
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Position {
    pub tile_x: u8,
    pub tile_y: u8,
    pub x_offset: u8,
    pub y_offset: u8,
}

impl Position {
    pub fn from_tile_xy(tile_x: u8, tile_y: u8) -> Self {
        Self {
            tile_x,
            tile_y,
            ..Default::default()
        }
    }

    pub fn pixel_position(&self, tile_size: u8) -> (u32, u32) {
        let x = self.tile_x as u32 * tile_size as u32 + self.x_offset as u32;
        let y = self.tile_y as u32 * tile_size as u32 + self.y_offset as u32;
        (x, y as u32)
    }
}

pub struct ObjectGrid {
    grid: HashMap<(u8, u8), Vec<Object>>,
    height: u8,
    width: u8,
}

impl ObjectGrid {
    pub fn from_objects(objects: Vec<Object>, height: u8, width: u8) -> Self {
        let mut grid = HashMap::new();
        for object in objects {
            let key = (object.position.tile_x, object.position.tile_y);
            grid.entry(key).or_insert_with(Vec::new).push(object);
        }

        Self {
            grid,
            height,
            width,
        }
    }

    pub fn y_sort(&mut self) {
        for (_, objects) in self.grid.iter_mut() {
            objects.sort_by_key(|o| o.position.y_offset);
        }
    }

    pub fn iter_objects(&self) -> impl Iterator<Item = &Object> {
        self.grid.values().flatten()
    }

    pub fn add_object(&mut self, object: Object) {
        let key = (object.position.tile_x, object.position.tile_y);
        self.grid.entry(key).or_insert_with(Vec::new).push(object);
    }

    pub fn get_at(&self, tile_x: u8, tile_y: u8) -> Option<&[Object]> {
        self.grid.get(&(tile_x, tile_y)).map(|v| v.as_slice())
    }

    pub fn get_tile_at(&self, tile_x: u8, tile_y: u8, tileset_id: TilesetId) -> Option<&Object> {
        self.get_at(tile_x, tile_y).and_then(|objects| {
            objects
                .iter()
                .find(|obj| matches!(obj.visual, Visual::Tile(id) if id == tileset_id))
        })
    }

    pub fn has_tile_at(&self, tile_x: u8, tile_y: u8, tileset_id: TilesetId) -> bool {
        if let Some(objects) = self.get_at(tile_x, tile_y) {
            objects
                .iter()
                .any(|obj| matches!(obj.visual, Visual::Tile(id) if id == tileset_id))
        } else {
            false
        }
    }

    pub fn determine_tile_mask(&self, tile_x: u8, tile_y: u8, tileset_id: TilesetId) -> TileMask {
        let n = if tile_y > 0 {
            self.has_tile_at(tile_x, tile_y - 1, tileset_id)
        } else {
            false
        };

        let e = if tile_x < self.width - 1 {
            self.has_tile_at(tile_x + 1, tile_y, tileset_id)
        } else {
            false
        };

        let s = if tile_y < self.height - 1 {
            self.has_tile_at(tile_x, tile_y + 1, tileset_id)
        } else {
            false
        };

        let w = if tile_x > 0 {
            self.has_tile_at(tile_x - 1, tile_y, tileset_id)
        } else {
            false
        };

        let ne = if tile_x < self.width - 1 && tile_y > 0 {
            self.has_tile_at(tile_x + 1, tile_y - 1, tileset_id)
        } else {
            false
        };

        let se = if tile_x < self.width - 1 && tile_y < self.height - 1 {
            self.has_tile_at(tile_x + 1, tile_y + 1, tileset_id)
        } else {
            false
        };

        let sw = if tile_x > 0 && tile_y < self.height - 1 {
            self.has_tile_at(tile_x - 1, tile_y + 1, tileset_id)
        } else {
            false
        };

        let nw = if tile_x > 0 && tile_y > 0 {
            self.has_tile_at(tile_x - 1, tile_y - 1, tileset_id)
        } else {
            false
        };

        TileMask::new(n, e, s, w, ne, se, sw, nw)
    }
}
