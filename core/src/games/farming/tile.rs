use crate::games::farming::layer::FarmLayer;
use crate::games::farming::plant::{Plant, PlantId};
use crate::games::farming::tile::ground::GroundFlags;
use crate::games::farming::world::FarmWorldDebugOptions;
use crate::rendering::o2d::prelude::*;
use crate::types::grid::cardinal::Cardinal;
use crate::types::grid::Grid;
use image::Rgba;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use strum::IntoEnumIterator;

pub mod ground;

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FarmTile {
    pub ground: GroundFlags,
    pub plant: Option<Plant>,
}

impl FarmTile {
    pub fn update(&mut self, elapsed: Duration) {
        if let Some(plant) = &mut self.plant {
            plant.grow_by(elapsed);
        }
    }

    pub fn new_land() -> Self {
        Self {
            ground: GroundFlags::GROUND | GroundFlags::FOLIAGE,
            plant: None,
        }
    }

    pub fn has_ground(&self) -> bool {
        self.ground.has_ground()
    }

    pub fn is_tilled(&self) -> bool {
        self.ground.is_tilled()
    }

    pub fn clear_ground(&mut self) {
        self.ground = GroundFlags::empty();
    }

    pub fn till(&mut self) {
        self.ground.insert(GroundFlags::TILLED);
    }

    pub fn water(&mut self) {
        self.ground.insert(GroundFlags::WATERED);
    }

    pub fn remove_foliage(&mut self) {
        self.ground.remove(GroundFlags::FOLIAGE);
    }

    pub fn add_plant(&mut self, plant_id: PlantId) {
        self.plant = Some(Plant::new(plant_id));
    }
}

// Rendering
impl FarmTile {
    pub fn render_objects(&self, ctx: TileContext) -> Vec<Object2D> {
        let mut objects = Vec::new();

        if !self.ground.has_ground() {
            objects.push(self.render_water(&ctx));
        } else {
            objects.push(self.render_rock(&ctx));
        }

        if self.ground.has_foliage() {
            objects.push(self.render_foliage(&ctx));
        }

        if self.ground.is_tilled() {
            objects.extend(self.render_soil(&ctx));
        }

        if let Some(plant) = &self.plant {
            objects.extend(plant.render_objects(ctx.x, ctx.y, self.ground.is_watered()));
        }

        if ctx.debug.tillability && ctx.is_tillable() {
            objects.push(Object2D {
                position: PositionO2D::from_tile_xy_layer(ctx.x, ctx.y, FarmLayer::Overlay),
                visual: VisualO2D::Color(Rgba([255, 0, 0, 128])),
            })
        }

        if ctx.debug.is_cliff && ctx.is_cliff() {
            objects.push(Object2D {
                position: PositionO2D::from_tile_xy_layer(ctx.x, ctx.y, FarmLayer::Overlay),
                visual: VisualO2D::Color(Rgba([0, 0, 255, 128])),
            })
        }

        if ctx.debug.has_ground && self.has_ground() {
            objects.push(Object2D {
                position: PositionO2D::from_tile_xy_layer(ctx.x, ctx.y, FarmLayer::Overlay),
                visual: VisualO2D::Color(Rgba([0, 255, 0, 128])),
            })
        }

        objects
    }

    fn render_water(&self, ctx: &TileContext) -> Object2D {
        let position = PositionO2D::from_tile_xy_layer(ctx.x, ctx.y, FarmLayer::Water);
        Object2D {
            position,
            visual: VisualO2D::Sprite(SpriteId::Water),
        }
    }

    fn render_rock(&self, ctx: &TileContext) -> Object2D {
        let position = PositionO2D::from_tile_xy_layer(ctx.x, ctx.y, FarmLayer::Ground);
        Object2D {
            position,
            visual: VisualO2D::Tile(TilesetId::Rock),
        }
    }

    fn render_foliage(&self, ctx: &TileContext) -> Object2D {
        if ctx.is_cliff() {
            let position = PositionO2D::from_tile_xy_layer(ctx.x, ctx.y, FarmLayer::Ground);
            Object2D {
                position,
                visual: VisualO2D::Tile(TilesetId::RockSpring),
            }
        } else {
            let position = PositionO2D::from_tile_xy_layer(ctx.x, ctx.y, FarmLayer::OnGround);
            Object2D {
                position,
                visual: VisualO2D::Tile(TilesetId::FoliageSpring),
            }
        }
    }

    fn render_soil(&self, ctx: &TileContext) -> Vec<Object2D> {
        let position = PositionO2D::from_tile_xy_layer(ctx.x, ctx.y, FarmLayer::Ground);

        let dry = Object2D {
            position,
            visual: VisualO2D::Tile(TilesetId::TilledSoil),
        };

        if !self.ground.is_watered() {
            return vec![dry];
        };

        let wet = Object2D {
            position: position.with_layer(FarmLayer::OnGround),
            visual: VisualO2D::Tile(TilesetId::TilledWetSoil),
        };

        vec![wet, dry]
    }
}

// Validation
impl FarmTile {
    pub fn validate(&self, ctx: TileContext) -> Option<FarmTile> {
        let mut new = self.clone();
        let is_tillable = ctx.is_tillable();

        if ctx.is_dangling_cliff() {
            new.clear_ground();
            new.plant = None;
        }

        if self.plant.is_some() && !is_tillable {
            new.plant = None;
        }

        if self.is_tilled() && !is_tillable {
            new.ground.remove(GroundFlags::TILLED);
            new.ground.remove(GroundFlags::WATERED);
        }

        if !is_tillable && self.has_ground() {
            new.ground.insert(GroundFlags::FOLIAGE);
        }

        if new != *self { Some(new) } else { None }
    }
}

pub struct TileContext<'a> {
    pub grid: &'a Grid<FarmTile>,
    pub debug: &'a FarmWorldDebugOptions,
    pub x: u8,
    pub y: u8,
}

impl TileContext<'_> {
    pub fn tile(&self) -> Option<&FarmTile> {
        self.grid.get_tile(self.x, self.y)
    }

    pub fn is_cliff(&self) -> bool {
        !self
            .grid
            .check_all_neighbors(self.x, self.y, |tile| tile.has_ground())
    }

    pub fn is_dangling_cliff(&self) -> bool {
        if !self.is_cliff() {
            return false;
        }

        let ns_water = self.cardinal_water(Cardinal::North) && self.cardinal_water(Cardinal::South);
        if ns_water {
            return true;
        }

        let ew_water = self.cardinal_water(Cardinal::East) && self.cardinal_water(Cardinal::West);
        if ew_water {
            return true;
        }

        let opposite_diagonal_water = (self.cardinal_water(Cardinal::NorthEast)
            && self.cardinal_water(Cardinal::SouthWest))
            || (self.cardinal_water(Cardinal::NorthWest)
                && self.cardinal_water(Cardinal::SouthEast));
        if opposite_diagonal_water {
            return true;
        }

        let front_diagonal_back = (self.cardinal_water(Cardinal::North)
            && (self.cardinal_water(Cardinal::SouthEast)
                || self.cardinal_water(Cardinal::SouthWest)))
            || (self.cardinal_water(Cardinal::East)
                && (self.cardinal_water(Cardinal::NorthWest)
                    || self.cardinal_water(Cardinal::SouthWest)))
            || (self.cardinal_water(Cardinal::South)
                && (self.cardinal_water(Cardinal::NorthWest)
                    || self.cardinal_water(Cardinal::NorthEast)))
            || (self.cardinal_water(Cardinal::West)
                && (self.cardinal_water(Cardinal::NorthEast)
                    || self.cardinal_water(Cardinal::SouthEast)));
        if front_diagonal_back && self.count_main_neighbors(true, |tile| !tile.has_ground()) == 1 {
            return true;
        }

        false
    }

    pub fn cardinal_water(&self, cardinal: Cardinal) -> bool {
        self.check_neighbor(cardinal, |tile| !tile.has_ground()) || !self.has_neighbor(cardinal)
    }

    pub fn count_main_neighbors_with(&self, f: impl Fn(&FarmTile) -> bool) -> usize {
        self.grid.count_main_neighbors_with(self.x, self.y, f)
    }

    pub fn check_neighbor(&self, cardinal: Cardinal, f: impl Fn(&FarmTile) -> bool) -> bool {
        self.grid.check_neighbor(cardinal, self.x, self.y, f)
    }

    pub fn has_neighbor(&self, cardinal: Cardinal) -> bool {
        self.grid
            .get_neighbor_coordinates(cardinal, self.x, self.y)
            .is_some()
    }

    pub fn count_main_neighbors(&self, include_none: bool, f: impl Fn(&FarmTile) -> bool) -> usize {
        let mut count = 0;
        for cardinal in Cardinal::iter_main() {
            let Some(tile) = self.grid.get_neighbor_tile(cardinal, self.x, self.y) else {
                if include_none {
                    count += 1;
                }
                continue;
            };
            if f(tile) {
                count += 1;
            }
        }
        count
    }

    pub fn count_diagonal_neighbors(
        &self,
        include_none: bool,
        f: impl Fn(&FarmTile) -> bool,
    ) -> usize {
        let mut count = 0;
        for cardinal in Cardinal::iter_diagonals() {
            let Some(tile) = self.grid.get_neighbor_tile(cardinal, self.x, self.y) else {
                if include_none {
                    count += 1;
                }
                continue;
            };
            if f(tile) {
                count += 1;
            }
        }
        count
    }

    pub fn count_main_neighbors_context(&self, f: impl Fn(&TileContext) -> bool) -> usize {
        let mut count = 0;
        for cardinal in Cardinal::iter_main() {
            let Some((x, y)) = self.grid.get_neighbor_coordinates(cardinal, self.x, self.y) else {
                continue;
            };
            if f(&TileContext {
                grid: self.grid,
                debug: self.debug,
                x,
                y,
            }) {
                count += 1;
            }
        }
        count
    }

    pub fn check_all_neighbors_context(&self, f: impl Fn(&TileContext) -> bool) -> bool {
        for cardinal in Cardinal::iter() {
            let Some((x, y)) = self.grid.get_neighbor_coordinates(cardinal, self.x, self.y) else {
                return false;
            };

            if !f(&TileContext {
                grid: self.grid,
                debug: self.debug,
                x,
                y,
            }) {
                return false;
            }
        }
        true
    }

    pub fn check_any_neighbor_context(&self, f: impl Fn(&TileContext) -> bool) -> bool {
        for cardinal in Cardinal::iter() {
            let Some((x, y)) = self.grid.get_neighbor_coordinates(cardinal, self.x, self.y) else {
                continue;
            };

            if f(&TileContext {
                grid: self.grid,
                debug: self.debug,
                x,
                y,
            }) {
                return true;
            }
        }

        false
    }

    pub fn is_tillable(&self) -> bool {
        let Some(tile) = self.tile() else {
            return false;
        };
        let any_neighbor_cliff = self.check_any_neighbor_context(|ctx| ctx.is_cliff());
        !self.is_cliff() && tile.has_ground() && !any_neighbor_cliff
    }
}
