use crate::error::CoreResult;
use crate::games::farming::tile::computed::ComputedFlags;
use crate::games::farming::tile::ground::GroundFlags;
use crate::games::farming::tile::FarmTile;
use crate::rendering::o2d::prelude::{O2DRenderer, Object2D};
use crate::types::cardinal::Cardinal;
use image::RgbaImage;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use strum::IntoEnumIterator;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FarmWorld {
    pub tiles: Vec<FarmTile>,
    pub height: u8,
    pub width: u8,
}

impl FarmWorld {
    pub fn new_empty(width: u8, height: u8) -> Self {
        let mut tiles = Vec::with_capacity(width as usize * height as usize);
        let mut land_tile = FarmTile::default();
        land_tile
            .ground
            .insert(GroundFlags::GROUND | GroundFlags::FOLIAGE);
        for x in 0..width {
            for y in 0..height {
                if x == 0 || y == 0 || x == width - 1 || y == height - 1 {
                    tiles.push(FarmTile::default())
                } else {
                    tiles.push(land_tile.clone());
                }
            }
        }
        Self {
            tiles,
            height,
            width,
        }
    }

    pub fn update(&mut self, elapsed: Duration) {
        self.compute();
        for tile in &mut self.tiles {
            tile.update(elapsed);
        }
    }

    pub fn get_tile(&self, x: u8, y: u8) -> Option<&FarmTile> {
        self.tiles
            .get(x as usize + y as usize * self.width as usize)
    }

    pub fn get_neighbor_tile(&self, cardinal: Cardinal, x: u8, y: u8) -> Option<&FarmTile> {
        self.get_neighbor_coordinates(cardinal, x, y)
            .and_then(|coord| self.get_tile(coord.0, coord.1))
    }

    pub fn get_neighbor_coordinates(&self, cardinal: Cardinal, x: u8, y: u8) -> Option<(u8, u8)> {
        match cardinal {
            Cardinal::North => {
                if y == 0 {
                    None
                } else {
                    Some((x, y - 1))
                }
            }
            Cardinal::East => {
                if x >= self.width - 1 {
                    None
                } else {
                    Some((x + 1, y))
                }
            }
            Cardinal::South => {
                if y >= self.height - 1 {
                    None
                } else {
                    Some((x, y + 1))
                }
            }
            Cardinal::West => {
                if x == 0 {
                    None
                } else {
                    Some((x - 1, y))
                }
            }
            Cardinal::NorthEast => {
                if x >= self.width - 1 || y == 0 {
                    None
                } else {
                    Some((x + 1, y - 1))
                }
            }
            Cardinal::SouthEast => {
                if x >= self.width - 1 || y >= self.height - 1 {
                    None
                } else {
                    Some((x + 1, y + 1))
                }
            }
            Cardinal::SouthWest => {
                if x == 0 || y >= self.height - 1 {
                    None
                } else {
                    Some((x - 1, y + 1))
                }
            }
            Cardinal::NorthWest => {
                if x == 0 || y == 0 {
                    None
                } else {
                    Some((x - 1, y - 1))
                }
            }
        }
    }

    pub fn get_tile_mut(&mut self, x: u8, y: u8) -> Option<&mut FarmTile> {
        self.tiles
            .get_mut(x as usize + y as usize * self.width as usize)
    }

    pub fn apply_at<F>(&mut self, x: u8, y: u8, f: F)
    where
        F: Fn(&mut FarmTile),
    {
        if let Some(tile) = self.get_tile_mut(x, y) {
            f(tile);
        }
    }

    pub fn apply_at_range<F>(&mut self, min: (u8, u8), max: (u8, u8), mut f: F)
    where
        F: FnMut(&mut FarmTile),
    {
        if min.0 > max.0 || min.1 > max.1 {
            return;
        }

        for x in min.0..=max.0 {
            for y in min.1..=max.1 {
                if let Some(tile) = self.get_tile_mut(x, y) {
                    f(tile);
                }
            }
        }
    }
}

// Computing tiles
impl FarmWorld {
    pub fn compute(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                self.compute_tile_cliffs(x, y);
            }
        }
        for x in 0..self.width {
            for y in 0..self.height {
                self.compute_cliff_integrity(x, y);
            }
        }
        for x in 0..self.width {
            for y in 0..self.height {
                self.compute_tile_tillability(x, y);
            }
        }
        for x in 0..self.width {
            for y in 0..self.height {
                self.compute_ground(x, y);
            }
        }
    }

    fn compute_tile_cliffs(&mut self, x: u8, y: u8) {
        let mut flags = ComputedFlags::default();
        for cardinal in Cardinal::iter() {
            let Some(tile) = self.get_neighbor_tile(cardinal, x, y) else {
                continue;
            };
            if tile.has_ground() {
                flags.ground_around.set_cardinal(cardinal)
            }
            if tile.is_tilled() {
                flags.tilled_around.set_cardinal(cardinal)
            }
        }
        self.apply_at(x, y, |tile| tile.computed = flags.clone());
    }

    fn compute_cliff_integrity(&mut self, x: u8, y: u8) {
        if let Some(tile) = self.get_tile_mut(x, y) {
            if tile.computed.ground_around.count_cardinals() < 4 {
                tile.clear_ground();
            }
        }
    }

    fn compute_tile_tillability(&mut self, x: u8, y: u8) {
        let mut is_tillable = true;
        for cardinal in Cardinal::iter() {
            let Some(tile) = self.get_tile(x, y) else {
                continue;
            };
            if !tile.has_ground() {
                is_tillable = false;
                break;
            }

            let Some(first_coords) = self.get_neighbor_coordinates(cardinal, x, y) else {
                continue;
            };
            let Some(first_tile) = self.get_tile(first_coords.0, first_coords.1) else {
                continue;
            };
            if first_tile.computed.is_cliff() || !first_tile.has_ground() {
                is_tillable = false;
                break;
            }
        }
        self.apply_at(x, y, |tile| tile.computed.is_tillable = is_tillable);
    }

    fn compute_ground(&mut self, x: u8, y: u8) {
        let Some(tile) = self.get_tile_mut(x, y) else {
            return;
        };

        if !tile.has_ground() {
            tile.ground = GroundFlags::empty();
        }

        if !tile.computed.is_tillable {
            tile.ground.remove(GroundFlags::TILLED);
            tile.ground.remove(GroundFlags::WATERED);
        }

        if tile.ground.is_tilled() {
            tile.ground.remove(GroundFlags::FOLIAGE);
        } else {
            tile.plant = None;
            tile.ground.remove(GroundFlags::WATERED);
        }
    }
}

// Rendering
impl FarmWorld {
    pub fn render(
        &mut self,
        o2d: &O2DRenderer,
        debug: FarmWorldDebugOptions,
    ) -> CoreResult<RgbaImage> {
        self.compute();
        self.compute();

        let mut objects = Vec::new();
        for x in 0..self.width {
            for y in 0..self.height {
                objects.extend(self.render_tile(x, y, &debug));
            }
        }

        if debug.grid {
            o2d.render_debug(&objects, self.height, self.width, 16)
        } else {
            o2d.render(&objects, self.height, self.width, 16)
        }
    }

    fn render_tile(&self, x: u8, y: u8, debug: &FarmWorldDebugOptions) -> Vec<Object2D> {
        let Some(tile) = self.get_tile(x, y) else {
            return vec![];
        };
        tile.render_objects(x, y, &debug)
    }
}

#[derive(Default)]
pub struct FarmWorldDebugOptions {
    pub grid: bool,
    pub tillability: bool,
}
