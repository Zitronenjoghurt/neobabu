use serde::{Deserialize, Serialize};
use std::time::Duration;

pub mod day_night;
pub mod hemisphere;
pub mod layer;
pub mod plant;
pub mod procedural;
pub mod season;
pub mod tile;
pub mod world;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FarmingGame {
    pub worlds: Vec<world::FarmWorld>,
    pub last_update: chrono::DateTime<chrono::Utc>,
}

impl FarmingGame {
    pub fn new() -> Self {
        Self {
            worlds: vec![],
            last_update: chrono::Utc::now(),
        }
    }

    pub fn update(&mut self) {
        let now = chrono::Utc::now();
        let elapsed_secs = (now - self.last_update).num_seconds();
        let elapsed = Duration::from_secs(elapsed_secs as u64);

        for world in &mut self.worlds {
            world.update(elapsed);
        }

        self.last_update = chrono::Utc::now();
    }
}
