use crate::games::farming::season::Season;
use chrono::Datelike;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub enum Hemisphere {
    Northern,
    Southern,
}

impl Hemisphere {
    pub fn current_season(&self) -> Season {
        let now = chrono::Utc::now();
        let month = now.month();

        let season = if (3..=5).contains(&month) {
            Season::Spring
        } else if (6..=8).contains(&month) {
            Season::Summer
        } else if (9..=11).contains(&month) {
            Season::Autumn
        } else {
            Season::Winter
        };

        match self {
            Self::Northern => season,
            Self::Southern => season.opposite(),
        }
    }
}
