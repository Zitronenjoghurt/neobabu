use crate::database::entity::user;
use crate::types::user_permissions::UserPermissions;

impl user::Model {
    pub fn permissions(&self) -> UserPermissions {
        UserPermissions::from_bits_truncate(self.permissions as u64)
    }

    pub fn has_permissions(&self, permissions: UserPermissions) -> bool {
        self.permissions().has_permissions(permissions)
    }

    pub fn timezone(&self) -> Option<chrono_tz::Tz> {
        self.preferred_timezone
            .as_ref()
            .and_then(|tz_string| tz_string.parse().ok())
    }
}
