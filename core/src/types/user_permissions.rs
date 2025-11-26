use bitflags::bitflags;

bitflags! {
    pub struct UserPermissions: u64 {
        const ADMINISTRATOR = 0b10000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
    }
}

impl UserPermissions {
    pub fn is_administrator(&self) -> bool {
        self.contains(Self::ADMINISTRATOR)
    }

    pub fn has_permissions(&self, permissions: UserPermissions) -> bool {
        self.contains(permissions) || self.is_administrator()
    }
}
