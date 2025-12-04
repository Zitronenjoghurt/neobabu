use crate::error::CoreResult;
use crate::games::farming::day_night::day_night_color;
use crate::games::farming::hemisphere::Hemisphere;
use crate::games::farming::tile::{FarmTile, TileContext};
use crate::rendering::o2d::prelude::{O2DRenderer, Object2D};
use crate::types::grid::cardinal::Cardinal;
use crate::types::grid::Grid;
use chrono_tz::Tz;
use image::RgbaImage;
use serde::{Deserialize, Serialize};
use std::collections::{HashSet, VecDeque};
use std::time::Duration;
use strum::IntoEnumIterator;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FarmWorld {
    pub grid: Grid<FarmTile>,
    pub hemisphere: Hemisphere,
    pub tz: Tz,
}

impl FarmWorld {
    pub fn new_square_island(width: u8, height: u8, hemisphere: Hemisphere, tz: Tz) -> Self {
        let mut tiles = Vec::with_capacity(width as usize * height as usize);
        for x in 0..width {
            for y in 0..height {
                if x == 0 || y == 0 || x == width - 1 || y == height - 1 {
                    tiles.push(FarmTile::default())
                } else {
                    tiles.push(FarmTile::new_land());
                }
            }
        }
        Self {
            grid: Grid::new(tiles, width, height),
            hemisphere,
            tz,
        }
    }

    pub fn new_random(
        width: u8,
        height: u8,
        water_chance: f32,
        hemisphere: Hemisphere,
        tz: Tz,
    ) -> Self {
        let mut tiles = Vec::with_capacity(width as usize * height as usize);
        for _ in 0..width {
            for _ in 0..height {
                if rand::random::<f32>() < water_chance {
                    tiles.push(FarmTile::default())
                } else {
                    tiles.push(FarmTile::new_land());
                }
            }
        }
        Self {
            grid: Grid::new(tiles, width, height),
            hemisphere,
            tz,
        }
    }

    pub fn update(&mut self, elapsed: Duration) {
        self.validate();
        for tile in &mut self.grid.iter_tiles_mut() {
            tile.update(elapsed);
        }
    }
}

// Computing tiles
impl FarmWorld {
    #[tracing::instrument(level = "trace", skip_all)]
    pub fn validate(&mut self) {
        let mut visited = HashSet::new();
        let mut queue: VecDeque<(u8, u8)> = VecDeque::new();

        for x in 0..self.grid.width() {
            for y in 0..self.grid.height() {
                queue.push_back((x, y));
            }
        }

        while let Some((x, y)) = queue.pop_front() {
            if !visited.insert((x, y)) {
                continue;
            }

            let new_tile = {
                let ctx = TileContext {
                    grid: &self.grid,
                    debug: &Default::default(),
                    x,
                    y,
                    season: self.hemisphere.current_season(),
                };
                self.grid.get_tile(x, y).and_then(|t| t.validate(ctx))
            };

            if let Some(tile) = new_tile {
                *self.grid.get_tile_mut(x, y).unwrap() = tile;

                for cardinal in Cardinal::iter() {
                    if let Some((nx, ny)) = self.grid.get_neighbor_coordinates(cardinal, x, y) {
                        visited.remove(&(nx, ny));
                        queue.push_back((nx, ny));
                    }
                }
            }
        }
    }
}

// Rendering
impl FarmWorld {
    #[tracing::instrument(level = "trace", skip_all)]
    pub fn render(
        &mut self,
        o2d: &O2DRenderer,
        debug: FarmWorldDebugOptions,
    ) -> CoreResult<RgbaImage> {
        self.validate();

        let mut objects = Vec::new();
        for (tile, x, y) in self.grid.iter_tiles_coords() {
            let ctx = TileContext {
                grid: &self.grid,
                debug: &debug,
                x,
                y,
                season: self.hemisphere.current_season(),
            };
            objects.extend(tile.render_objects(ctx))
        }

        objects.push(Object2D::ambience(day_night_color(self.tz)));

        if debug.grid {
            o2d.render_debug(&objects, self.grid.height(), self.grid.width(), 16)
        } else {
            o2d.render(&objects, self.grid.height(), self.grid.width(), 16)
        }
    }
}

#[derive(Default)]
pub struct FarmWorldDebugOptions {
    pub grid: bool,
    pub tillability: bool,
    pub is_cliff: bool,
    pub has_ground: bool,
}
