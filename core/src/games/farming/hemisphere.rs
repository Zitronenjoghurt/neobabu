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

        let season = if month >= 3 && month <= 5 {
            Season::Spring
        } else if month >= 6 && month <= 8 {
            Season::Summer
        } else if month >= 9 && month <= 11 {
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
