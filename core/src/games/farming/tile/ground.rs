use bitflags::bitflags;
use serde::{Deserialize, Serialize};

bitflags! {
    #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct GroundFlags: u8 {
        const GROUND = 0b0000_0001;
        const FOLIAGE = 0b0000_0010;
        const TILLED = 0b0000_0100;
        const WATERED = 0b0000_1000;
    }
}

impl GroundFlags {
    pub fn has_ground(&self) -> bool {
        self.contains(Self::GROUND)
    }

    pub fn has_foliage(&self) -> bool {
        self.contains(Self::FOLIAGE) && !self.is_tilled()
    }

    pub fn is_tilled(&self) -> bool {
        self.contains(Self::TILLED)
    }

    pub fn is_watered(&self) -> bool {
        self.contains(Self::WATERED)
    }
}
