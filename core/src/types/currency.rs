use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::fmt::Display;
use strum::EnumIter;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, TryFromPrimitive, IntoPrimitive, EnumIter)]
#[repr(i16)]
pub enum Currency {
    Citrine = 0,
}

impl Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
