use crate::database::entity::apod;
use crate::error::CoreResult;
use crate::integrations::apis::nasa_apod::NasaApod;
use crate::integrations::apis::Apis;
use crate::stores::Stores;
use chrono::{Datelike, NaiveDate};
use sea_orm::Set;
use std::sync::Arc;

pub struct ApodService {
    apis: Arc<Apis>,
    stores: Arc<Stores>,
}

impl ApodService {
    pub fn initialize(apis: &Arc<Apis>, stores: &Arc<Stores>) -> Arc<Self> {
        Arc::new(Self {
            apis: apis.clone(),
            stores: stores.clone(),
        })
    }

    fn parse_date(date: &str) -> CoreResult<NaiveDate> {
        Ok(NaiveDate::parse_from_str(date, "%Y-%m-%d")?)
    }

    fn model_from_api(apod: &NasaApod) -> CoreResult<apod::ActiveModel> {
        let date = Self::parse_date(&apod.date)?;

        Ok(apod::ActiveModel {
            day: Set(date.day() as i16),
            month: Set(date.month() as i16),
            year: Set(date.year() as i16),
            title: Set(apod.title.clone()),
            explanation: Set(apod.explanation.clone()),
            url: Set(apod.url.clone()),
            hd_url: Set(apod.hdurl.clone()),
            thumbnail_url: Set(apod.thumbnail_url.clone()),
            media_type: Set(apod.media_type.clone()),
            copyright: Set(apod.copyright.clone()),
            was_announced: Set(false),
        })
    }

    async fn save_apod_if_not_exists(&self, apod: &NasaApod) -> CoreResult<apod::Model> {
        let active_model = Self::model_from_api(apod)?;
        let day = *active_model.day.as_ref();
        let month = *active_model.month.as_ref();
        let year = *active_model.year.as_ref();

        if let Some(model) = self.stores.apod.find(day, month, year).await? {
            Ok(model)
        } else {
            Ok(self.stores.apod.insert(active_model).await?)
        }
    }

    async fn fetch_and_cache_month(&self, date: &NaiveDate) -> CoreResult<Option<apod::Model>> {
        if *date > chrono::Utc::now().naive_utc().date() {
            return Ok(None);
        }

        let apods = self.apis.apod.year_month_of(date).await?;
        for apod in apods {
            self.save_apod_if_not_exists(&apod).await?;
        }

        let day = date.day() as i16;
        let month = date.month() as i16;
        let year = date.year() as i16;
        match self.stores.apod.find(day, month, year).await? {
            Some(model) => Ok(Some(model)),
            None => {
                let empty = apod::ActiveModel {
                    day: Set(day),
                    month: Set(month),
                    year: Set(year),
                    ..Default::default()
                };
                Ok(Some(self.stores.apod.insert(empty).await?))
            }
        }
    }

    pub async fn get(&self, date: &NaiveDate) -> CoreResult<Option<apod::Model>> {
        let day = date.day() as i16;
        let month = date.month() as i16;
        let year = date.year() as i16;

        if let Some(existing) = self.stores.apod.find(day, month, year).await? {
            return Ok(Some(existing));
        };

        self.fetch_and_cache_month(date).await
    }

    pub async fn today(&self) -> CoreResult<Option<apod::Model>> {
        let today_date = chrono::Utc::now().naive_utc().date();
        if let Some(existing) = self
            .stores
            .apod
            .find(
                today_date.day() as i16,
                today_date.month() as i16,
                today_date.year() as i16,
            )
            .await?
        {
            return Ok(Some(existing));
        }

        let apod = self.apis.apod.today().await?;
        let date = Self::parse_date(&apod.date)?;
        if date != today_date {
            return Ok(None);
        };

        Ok(Some(self.save_apod_if_not_exists(&apod).await?))
    }
}
