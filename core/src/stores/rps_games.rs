use crate::database::entity::{rps_games, user};
use crate::database::Database;
use crate::error::CoreResult;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use std::sync::Arc;

pub struct RPSGamesStore {
    db: Arc<Database>,
}

impl RPSGamesStore {
    pub fn initialize(db: &Arc<Database>) -> Arc<Self> {
        Arc::new(Self { db: db.clone() })
    }

    fn id_pair(&self, user_id_1: impl AsRef<str>, user_id_2: impl AsRef<str>) -> (String, String) {
        let user_id_1 = user_id_1.as_ref().to_owned();
        let user_id_2 = user_id_2.as_ref().to_owned();

        if user_id_1 < user_id_2 {
            (user_id_1, user_id_2)
        } else {
            (user_id_2, user_id_1)
        }
    }

    pub async fn find_by_id(
        &self,
        user_1_id: impl AsRef<str>,
        user_2_id: impl AsRef<str>,
    ) -> CoreResult<Option<rps_games::Model>> {
        Ok(
            rps_games::Entity::find_by_id(self.id_pair(user_1_id, user_2_id))
                .one(self.db.conn())
                .await?,
        )
    }

    pub async fn fetch_or_create(
        &self,
        user_1: &user::Model,
        user_2: &user::Model,
    ) -> CoreResult<rps_games::Model> {
        let id = self.id_pair(&user_1.id, &user_2.id);
        if let Some(existing) = self.find_by_id(&id.0, &id.1).await? {
            return Ok(existing);
        };

        let new = rps_games::ActiveModel {
            user_id1: Set(id.0),
            user_id2: Set(id.1),
            ..Default::default()
        };

        Ok(new.insert(self.db.conn()).await?)
    }

    pub async fn update(&self, mut model: rps_games::ActiveModel) -> CoreResult<rps_games::Model> {
        model.updated_at = Set(chrono::Utc::now().naive_utc());
        Ok(model.update(self.db.conn()).await?)
    }
}
