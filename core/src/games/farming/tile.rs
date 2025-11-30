use crate::games::farming::layer::FarmLayer;
use crate::games::farming::plant::{Plant, PlantId};
use crate::games::farming::tile::computed::ComputedFlags;
use crate::games::farming::tile::ground::GroundFlags;
use crate::games::farming::world::FarmWorldDebugOptions;
use crate::rendering::o2d::prelude::*;
use image::Rgba;
use serde::{Deserialize, Serialize};
use std::time::Duration;

pub mod computed;
pub mod ground;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct FarmTile {
    pub ground: GroundFlags,
    pub plant: Option<Plant>,
    #[serde(skip, default)]
    pub computed: ComputedFlags,
}

impl FarmTile {
    pub fn update(&mut self, elapsed: Duration) {
        if let Some(plant) = &mut self.plant {
            plant.grow_by(elapsed);
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

    pub fn render_objects(&self, x: u8, y: u8, debug: &FarmWorldDebugOptions) -> Vec<Object2D> {
        let mut objects = Vec::new();

        if !self.ground.has_ground() {
            objects.push(self.render_water(x, y));
        } else {
            objects.push(self.render_rock(x, y));
        }

        if self.ground.has_foliage() {
            objects.push(self.render_foliage(x, y));
        }

        if self.ground.is_tilled() {
            objects.extend(self.render_soil(x, y));
        }

        if let Some(plant) = &self.plant {
            objects.extend(plant.render_objects(x, y, self.ground.is_watered()));
        }

        if debug.tillability && self.computed.is_tillable {
            objects.push(Object2D {
                position: PositionO2D::from_tile_xy_layer(x, y, FarmLayer::Overlay),
                visual: VisualO2D::Color(Rgba([255, 0, 0, 128])),
            })
        }

        objects
    }

    fn render_water(&self, x: u8, y: u8) -> Object2D {
        let position = PositionO2D::from_tile_xy_layer(x, y, FarmLayer::Water);
        Object2D {
            position,
            visual: VisualO2D::Sprite(SpriteId::Water),
        }
    }

    fn render_rock(&self, x: u8, y: u8) -> Object2D {
        let position = PositionO2D::from_tile_xy_layer(x, y, FarmLayer::Ground);
        Object2D {
            position,
            visual: VisualO2D::Tile(TilesetId::Rock),
        }
    }

    fn render_foliage(&self, x: u8, y: u8) -> Object2D {
        if self.computed.is_cliff() {
            let position = PositionO2D::from_tile_xy_layer(x, y, FarmLayer::Ground);
            Object2D {
                position,
                visual: VisualO2D::Tile(TilesetId::RockSpring),
            }
        } else {
            let position = PositionO2D::from_tile_xy_layer(x, y, FarmLayer::OnGround);
            Object2D {
                position,
                visual: VisualO2D::Tile(TilesetId::FoliageSpring),
            }
        }
    }

    fn render_soil(&self, x: u8, y: u8) -> Vec<Object2D> {
        let position = PositionO2D::from_tile_xy_layer(x, y, FarmLayer::Ground);

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
