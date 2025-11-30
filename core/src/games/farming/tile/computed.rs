use crate::types::cardinal::CardinalFlags;

#[derive(Debug, Default, Clone)]
pub struct ComputedFlags {
    pub ground_around: CardinalFlags,
    pub tilled_around: CardinalFlags,
    pub is_tillable: bool,
}

impl ComputedFlags {
    pub fn is_cliff(&self) -> bool {
        !self.ground_around.has_all_cardinals()
    }
}
