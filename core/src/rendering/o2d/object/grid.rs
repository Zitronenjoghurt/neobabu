use crate::rendering::o2d::object::{Object2D, VisualO2D};
use crate::rendering::o2d::prelude::TilesetId;
use crate::rendering::o2d::tileset::mask::TileMask;
use std::collections::HashMap;

pub struct O2DGrid {
    grid: HashMap<(u8, u8), Vec<Object2D>>,
    height: u8,
    width: u8,
}

impl O2DGrid {
    pub fn from_objects(objects: Vec<Object2D>, height: u8, width: u8) -> Self {
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

    pub fn sort(&mut self) {
        for (_, objects) in self.grid.iter_mut() {
            objects.sort_by_key(|o| (o.position.z_index, o.position.y_offset));
        }
    }

    pub fn iter_objects(&self) -> impl Iterator<Item = &Object2D> {
        self.grid.values().flatten()
    }

    pub fn add_object(&mut self, object: Object2D) {
        let key = (object.position.tile_x, object.position.tile_y);
        self.grid.entry(key).or_insert_with(Vec::new).push(object);
    }

    pub fn get_at(&self, tile_x: u8, tile_y: u8) -> Option<&[Object2D]> {
        self.grid.get(&(tile_x, tile_y)).map(|v| v.as_slice())
    }

    pub fn get_tile_at(&self, tile_x: u8, tile_y: u8, tileset_id: TilesetId) -> Option<&Object2D> {
        self.get_at(tile_x, tile_y).and_then(|objects| {
            objects
                .iter()
                .find(|obj| matches!(obj.visual, VisualO2D::Tile(id) if id == tileset_id))
        })
    }

    pub fn has_tile_at(&self, tile_x: u8, tile_y: u8, tileset_id: TilesetId) -> bool {
        if let Some(objects) = self.get_at(tile_x, tile_y) {
            objects
                .iter()
                .any(|obj| matches!(obj.visual, VisualO2D::Tile(id) if id == tileset_id))
        } else {
            false
        }
    }

    pub fn has_connecting_tile_at(&self, tile_x: u8, tile_y: u8, tileset_id: TilesetId) -> bool {
        let Some(objects) = self.get_at(tile_x, tile_y) else {
            return false;
        };
        objects.iter().any(|object| {
            let VisualO2D::Tile(other_tileset_id) = object.visual else {
                return false;
            };
            other_tileset_id == tileset_id || tileset_id.connects_with(other_tileset_id)
        })
    }

    pub fn determine_tile_mask(&self, tile_x: u8, tile_y: u8, tileset_id: TilesetId) -> TileMask {
        let n = if tile_y > 0 {
            self.has_connecting_tile_at(tile_x, tile_y - 1, tileset_id)
        } else {
            false
        };

        let e = if tile_x < self.width - 1 {
            self.has_connecting_tile_at(tile_x + 1, tile_y, tileset_id)
        } else {
            false
        };

        let s = if tile_y < self.height - 1 {
            self.has_connecting_tile_at(tile_x, tile_y + 1, tileset_id)
        } else {
            false
        };

        let w = if tile_x > 0 {
            self.has_connecting_tile_at(tile_x - 1, tile_y, tileset_id)
        } else {
            false
        };

        let ne = if tile_x < self.width - 1 && tile_y > 0 {
            self.has_connecting_tile_at(tile_x + 1, tile_y - 1, tileset_id)
        } else {
            false
        };

        let se = if tile_x < self.width - 1 && tile_y < self.height - 1 {
            self.has_connecting_tile_at(tile_x + 1, tile_y + 1, tileset_id)
        } else {
            false
        };

        let sw = if tile_x > 0 && tile_y < self.height - 1 {
            self.has_connecting_tile_at(tile_x - 1, tile_y + 1, tileset_id)
        } else {
            false
        };

        let nw = if tile_x > 0 && tile_y > 0 {
            self.has_connecting_tile_at(tile_x - 1, tile_y - 1, tileset_id)
        } else {
            false
        };

        TileMask::new(n, e, s, w, ne, se, sw, nw)
    }
}
