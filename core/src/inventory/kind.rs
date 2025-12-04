use num_enum::TryFromPrimitive;

#[derive(Debug, Clone, Copy, TryFromPrimitive)]
#[repr(i32)]
pub enum ItemKind {
    Carrot = 1500,
}
