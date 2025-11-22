use crate::database::entity::dashboard_session;
use crate::database::Database;
use crate::error::CoreResult;
use sea_orm::{ActiveModelTrait, EntityTrait, ModelTrait};
use std::sync::Arc;

#[derive(Debug)]
pub struct DashboardSessionStore {
    db: Arc<Database>,
}

impl DashboardSessionStore {
    pub fn initialize(db: &Arc<Database>) -> Arc<Self> {
        Arc::new(Self { db: db.clone() })
    }

    pub async fn find(&self, id: impl AsRef<str>) -> CoreResult<Option<dashboard_session::Model>> {
        Ok(dashboard_session::Entity::find_by_id(id.as_ref())
            .one(self.db.conn())
            .await?)
    }

    pub async fn insert(
        &self,
        model: dashboard_session::ActiveModel,
    ) -> CoreResult<dashboard_session::Model> {
        Ok(model.insert(self.db.conn()).await?)
    }

    pub async fn update(
        &self,
        model: dashboard_session::ActiveModel,
    ) -> CoreResult<dashboard_session::Model> {
        Ok(model.update(self.db.conn()).await?)
    }

    pub async fn delete(&self, model: dashboard_session::Model) -> CoreResult<()> {
        let _ = model.delete(self.db.conn()).await?;
        Ok(())
    }

    pub async fn delete_id(&self, id: impl AsRef<str>) -> CoreResult<()> {
        if let Some(model) = self.find(id).await? {
            self.delete(model).await?;
        }
        Ok(())
    }
}
