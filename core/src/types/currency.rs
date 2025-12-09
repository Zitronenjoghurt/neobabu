use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Clone, Copy, Debug, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(i16)]
pub enum Currency {
    Citrine = 0,
}
