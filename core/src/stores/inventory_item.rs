use crate::database::entity::inventory_item;
use crate::database::Database;
use crate::error::CoreResult;
use crate::inventory::kind::ItemKind;
use futures::StreamExt;
use sea_orm::prelude::*;
use sea_orm::{ExprTrait, Set};
use std::sync::Arc;

pub struct InventoryItemStore {
    db: Arc<Database>,
}

impl InventoryItemStore {
    pub fn initialize(db: &Arc<Database>) -> Arc<Self> {
        Arc::new(Self { db: db.clone() })
    }

    pub async fn stream_by_user_id(
        &self,
        user_id: impl AsRef<str>,
    ) -> CoreResult<impl futures::Stream<Item = CoreResult<inventory_item::Model>>> {
        Ok(inventory_item::Entity::find()
            .filter(inventory_item::Column::UserId.eq(user_id.as_ref().to_string()))
            .stream(self.db.conn())
            .await?
            .map(|model| Ok(model?)))
    }

    pub async fn stream_by_user_and_kind(
        &self,
        user_id: impl AsRef<str>,
        kind: ItemKind,
    ) -> CoreResult<impl futures::Stream<Item = CoreResult<inventory_item::Model>>> {
        Ok(inventory_item::Entity::find()
            .filter(
                inventory_item::Column::UserId
                    .eq(user_id.as_ref().to_string())
                    .and(inventory_item::Column::Kind.eq(kind as i32)),
            )
            .stream(self.db.conn())
            .await?
            .map(|model| Ok(model?)))
    }

    pub async fn insert(
        &self,
        model: inventory_item::ActiveModel,
    ) -> CoreResult<inventory_item::Model> {
        Ok(model.insert(self.db.conn()).await?)
    }

    pub async fn update(
        &self,
        mut model: inventory_item::ActiveModel,
    ) -> CoreResult<inventory_item::Model> {
        model.updated_at = Set(chrono::Utc::now().naive_utc());
        Ok(model.update(self.db.conn()).await?)
    }
}
