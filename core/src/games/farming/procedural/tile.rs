use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Default, Clone, Copy)]
    pub struct ProceduralTile: u8 {
        const GROUND = 0b0000_0001;
    }
}
