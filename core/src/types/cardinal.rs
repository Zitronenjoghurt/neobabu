use bitflags::bitflags;
use strum::EnumIter;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, EnumIter)]
pub enum Cardinal {
    North,
    East,
    South,
    West,
    NorthEast,
    SouthEast,
    SouthWest,
    NorthWest,
}

bitflags! {
    #[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct CardinalFlags: u8 {
        const NORTH = 0b0000_0001;
        const EAST = 0b0000_0010;
        const SOUTH = 0b0000_0100;
        const WEST = 0b0000_1000;
        const NORTH_EAST = 0b0001_0000;
        const SOUTH_EAST = 0b0010_0000;
        const SOUTH_WEST = 0b0100_0000;
        const NORTH_WEST = 0b1000_0000;
    }
}

impl CardinalFlags {
    pub fn set_cardinal(&mut self, cardinal: Cardinal) {
        match cardinal {
            Cardinal::North => self.insert(CardinalFlags::NORTH),
            Cardinal::East => self.insert(CardinalFlags::EAST),
            Cardinal::South => self.insert(CardinalFlags::SOUTH),
            Cardinal::West => self.insert(CardinalFlags::WEST),
            Cardinal::NorthEast => self.insert(CardinalFlags::NORTH_EAST),
            Cardinal::SouthEast => self.insert(CardinalFlags::SOUTH_EAST),
            Cardinal::SouthWest => self.insert(CardinalFlags::SOUTH_WEST),
            Cardinal::NorthWest => self.insert(CardinalFlags::NORTH_WEST),
        }
    }

    pub fn unset_cardinal(&mut self, cardinal: Cardinal) {
        match cardinal {
            Cardinal::North => self.remove(CardinalFlags::NORTH),
            Cardinal::East => self.remove(CardinalFlags::EAST),
            Cardinal::South => self.remove(CardinalFlags::SOUTH),
            Cardinal::West => self.remove(CardinalFlags::WEST),
            Cardinal::NorthEast => self.remove(CardinalFlags::NORTH_EAST),
            Cardinal::SouthEast => self.remove(CardinalFlags::SOUTH_EAST),
            Cardinal::SouthWest => self.remove(CardinalFlags::SOUTH_WEST),
            Cardinal::NorthWest => self.remove(CardinalFlags::NORTH_WEST),
        }
    }

    pub fn has_cardinal(&self, cardinal: Cardinal) -> bool {
        match cardinal {
            Cardinal::North => self.contains(CardinalFlags::NORTH),
            Cardinal::East => self.contains(CardinalFlags::EAST),
            Cardinal::South => self.contains(CardinalFlags::SOUTH),
            Cardinal::West => self.contains(CardinalFlags::WEST),
            Cardinal::NorthEast => self.contains(CardinalFlags::NORTH_EAST),
            Cardinal::SouthEast => self.contains(CardinalFlags::SOUTH_EAST),
            Cardinal::SouthWest => self.contains(CardinalFlags::SOUTH_WEST),
            Cardinal::NorthWest => self.contains(CardinalFlags::NORTH_WEST),
        }
    }

    pub fn has_all_cardinals(&self) -> bool {
        self.bits() == 0b1111_1111
    }

    pub fn has_any_opposite_with(&self, other: &CardinalFlags) -> bool {
        (self.contains(CardinalFlags::NORTH) && other.contains(CardinalFlags::SOUTH))
            || (self.contains(CardinalFlags::SOUTH) && other.contains(CardinalFlags::NORTH))
            || (self.contains(CardinalFlags::EAST) && other.contains(CardinalFlags::WEST))
            || (self.contains(CardinalFlags::WEST) && other.contains(CardinalFlags::EAST))
            || (self.contains(CardinalFlags::NORTH_EAST)
                && other.contains(CardinalFlags::SOUTH_WEST))
            || (self.contains(CardinalFlags::SOUTH_WEST)
                && other.contains(CardinalFlags::NORTH_EAST))
            || (self.contains(CardinalFlags::SOUTH_EAST)
                && other.contains(CardinalFlags::NORTH_WEST))
            || (self.contains(CardinalFlags::NORTH_WEST)
                && other.contains(CardinalFlags::SOUTH_EAST))
    }
}
