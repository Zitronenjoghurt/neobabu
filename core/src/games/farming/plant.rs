use crate::games::farming::layer::FarmLayer;
use crate::rendering::o2d::prelude::{Object2D, PositionO2D, SpriteId, VisualO2D};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plant {
    pub id: PlantId,
    pub age: Duration,
}

impl Plant {
    pub fn new(id: PlantId) -> Self {
        Self {
            id,
            age: Duration::ZERO,
        }
    }

    pub fn render_objects(&self, x: u8, y: u8, is_watered: bool) -> Vec<Object2D> {
        let position = PositionO2D::from_tile_xy(x, y);

        let hole_sprite = if !is_watered {
            SpriteId::SoilHole2
        } else {
            SpriteId::SoilHole2Watered
        };

        vec![
            Object2D {
                position: position.with_layer(FarmLayer::AboveGround),
                visual: VisualO2D::Sprite(hole_sprite),
            },
            Object2D {
                position: position.with_layer(FarmLayer::Object),
                visual: VisualO2D::Sprite(self.get_sprite()),
            },
        ]
    }

    pub fn grow_by(&mut self, duration: Duration) {
        self.age += duration;
    }

    pub fn get_growth_percentage(&self) -> f32 {
        self.age.as_secs_f32() / self.id.ripe_duration().as_secs_f32()
    }

    pub fn is_ripe(&self) -> bool {
        self.get_growth_percentage() >= 1.0
    }

    pub fn get_sprite(&self) -> SpriteId {
        let stages = self.id.stages();
        if stages.is_empty() {
            return SpriteId::UntilledSoil;
        };

        let percentage = self.get_growth_percentage().clamp(0.0, 0.9999);
        let stage = (percentage * stages.len() as f32) as usize;
        stages[stage]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PlantId {
    Carrot,
}

impl PlantId {
    pub fn ripe_duration(&self) -> Duration {
        match self {
            Self::Carrot => Duration::from_hours(4),
        }
    }

    pub fn stages(&self) -> &[SpriteId] {
        match self {
            Self::Carrot => &[
                SpriteId::Carrot1,
                SpriteId::Carrot2,
                SpriteId::Carrot3,
                SpriteId::Carrot4,
                SpriteId::Carrot5,
            ],
        }
    }
}
