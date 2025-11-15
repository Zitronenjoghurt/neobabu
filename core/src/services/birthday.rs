use crate::database::entity::{guild, user, user_birthday};
use crate::error::{CoreError, CoreResult};
use crate::stores::Stores;
use crate::types::feature::Feature;
use sea_orm::{IntoActiveModel, Set};
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
        if day < 1 || day > 31 || month < 1 || month > 12 {
            return Err(CoreError::invalid_birthday("Day or month out of range."));
        }

        let verify_date = if let Some(year) = year {
            chrono::NaiveDate::from_ymd_opt(year as i32, month as u32, day as u32)
        } else {
            chrono::NaiveDate::from_ymd_opt(2000, month as u32, day as u32)
        };

        if verify_date.is_none() {
            return Err(CoreError::invalid_birthday("Invalid date."));
        }

        Ok(())
    }

    pub async fn set_birthday(
        &self,
        user: &user::Model,
        guild: &guild::Model,
        day: i16,
        month: i16,
        year: Option<i16>,
    ) -> CoreResult<()> {
        let guild_birthday = self
            .stores
            .guild_birthday
            .fetch_or_create(&guild.id)
            .await?;
        if !guild_birthday.enabled {
            return Err(CoreError::FeatureNotEnabled(Feature::Birthday));
        };

        self.validate_birthday(day, month, year)?;

        if let Some(user_birthday) = self.stores.user_birthday.find_by_user_id(&user.id).await? {
            let mut active = user_birthday.into_active_model();
            active.day = Set(day);
            active.month = Set(month);
            active.year = Set(year);
            self.stores.user_birthday.update(active).await?;
        } else {
            let new = user_birthday::ActiveModel {
                user_id: Set(user.id.clone()),
                day: Set(day),
                month: Set(month),
                year: Set(year),
                ..Default::default()
            };
            self.stores.user_birthday.insert(new).await?;
        }

        Ok(())
    }
}
