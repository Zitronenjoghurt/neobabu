use crate::database::entity::user;
use crate::types::user_permissions::UserPermissions;

impl user::Model {
    pub fn permissions(&self) -> UserPermissions {
        UserPermissions::from_bits_truncate(self.permissions as u64)
    }

    pub fn has_permissions(&self, permissions: UserPermissions) -> bool {
        self.permissions().has_permissions(permissions)
    }
}
