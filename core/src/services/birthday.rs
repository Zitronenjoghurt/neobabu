use crate::error::{CoreError, CoreResult};
use crate::stores::Stores;
use crate::types::feature::Feature;
use std::sync::Arc;

pub struct BirthdayService {
    stores: Arc<Stores>,
}

impl BirthdayService {
    pub fn initialize(stores: &Arc<Stores>) -> Arc<Self> {
        Arc::new(Self {
            stores: stores.clone(),
        })
    }

    pub fn validate_birthday(&self, day: i16, month: i16, year: Option<i16>) -> CoreResult<()> {
        todo!()
    }

    pub async fn set_birthday(
        &self,
        user_id: impl AsRef<str>,
        guild_id: impl AsRef<str>,
        day: i16,
        month: i16,
        year: Option<i16>,
    ) -> CoreResult<()> {
        let guild_birthday = self.stores.guild_birthday.fetch_or_create(guild_id).await?;
        if !guild_birthday.enabled {
            return Err(CoreError::FeatureNotEnabled(Feature::Birthday));
        };

        self.validate_birthday(day, month, year)?;

        todo!()
    }
}
