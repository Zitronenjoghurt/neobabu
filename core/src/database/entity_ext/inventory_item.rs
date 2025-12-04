use crate::database::entity::inventory_item;
use crate::error::{CoreError, CoreResult};
use crate::inventory::kind::ItemKind;
use crate::inventory::state::ItemState;
use sea_orm::Set;

impl inventory_item::ActiveModel {
    pub fn new(user_id: impl AsRef<str>, kind: ItemKind, state: ItemState) -> CoreResult<Self> {
        Ok(inventory_item::ActiveModel {
            id: Set(uuid::Uuid::new_v4()),
            user_id: Set(user_id.as_ref().to_string()),
            kind: Set(kind as i32),
            state: Set(state.serialize()?),
            ..Default::default()
        })
    }
}

impl inventory_item::Model {
    pub fn kind(&self) -> CoreResult<ItemKind> {
        ItemKind::try_from(self.kind).map_err(|_| CoreError::UnknownItemKind(self.kind))
    }

    pub fn state(&self) -> CoreResult<ItemState> {
        let kind = self.kind()?;
        let mut state = ItemState::deserialize(&self.state)?;
        state.sanitize(kind);
        Ok(state)
    }
}
