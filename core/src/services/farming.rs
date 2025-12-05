use crate::database::entity::{farming_world, user};
use crate::error::{CoreError, CoreResult};
use crate::games::farming::hemisphere::Hemisphere;
use crate::games::farming::procedural::ProceduralWorld;
use crate::stores::Stores;
use sea_orm::Set;
use std::str::FromStr;
use std::sync::Arc;

pub struct FarmingService {
    stores: Arc<Stores>,
}

impl FarmingService {
    pub fn initialize(stores: &Arc<Stores>) -> Arc<Self> {
        Arc::new(Self {
            stores: stores.clone(),
        })
    }

    pub async fn create_world(
        &self,
        user: &user::Model,
        name: String,
        hemisphere: Hemisphere,
    ) -> CoreResult<farming_world::Model> {
        let Some(tz_string) = user.preferred_timezone.clone() else {
            return Err(CoreError::NoPreferredTimezone);
        };
        let tz = chrono_tz::Tz::from_str(&tz_string)?;

        let farming = self.stores.farming.fetch_or_create(user).await?;
        let world_count = self
            .stores
            .farming_world
            .count_by_user(user.id.as_str())
            .await?;

        if world_count >= farming.max_worlds() {
            return Err(CoreError::FarmingWorldMaximumReached(farming.max_worlds()));
        }

        let procedural = ProceduralWorld {
            height: 25,
            width: 25,
            hemisphere,
            tz,
            seed: rand::random(),
            scale: 10.0,
            threshold: -0.6,
        };

        let world = procedural.generate();
        let data = serde_json::to_value(&world)?;

        let new = farming_world::ActiveModel {
            user_id: Set(user.id.to_string()),
            index: Set(world_count as i32),
            name: Set(name),
            data: Set(data),
            ..Default::default()
        };

        self.stores.farming_world.insert(new).await
    }
}
