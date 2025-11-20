use crate::error::CoreResult;
use crate::integrations::client::IntegrationClient;
use crate::integrations::request::RequestBuilder;
use crate::utils::year_month_min_max_day;
use chrono::Datelike;
use std::time::Duration;

pub struct NasaApodApi {
    client: IntegrationClient,
    token: Option<String>,
}

impl NasaApodApi {
    pub fn new(token: Option<String>) -> Self {
        let client = IntegrationClient::new(100, 1, Duration::from_secs(4));
        Self { client, token }
    }

    fn base_request(&'_ self) -> CoreResult<RequestBuilder<'_>> {
        let Some(token) = &self.token else {
            return Err(crate::error::CoreError::MissingNasaApiKey);
        };

        Ok(self
            .client
            .request("https://api.nasa.gov/planetary/apod")?
            .query("api_key", token)
            .query("thumbs", "true"))
    }

    pub async fn today(&self) -> CoreResult<NasaApod> {
        self.base_request()?.get_json().await
    }

    pub async fn year_month_of(&self, date: &chrono::NaiveDate) -> CoreResult<Vec<NasaApod>> {
        let year = date.year();
        let month = date.month();
        let (day_min, day_max) = year_month_min_max_day(year, month)
            .ok_or(crate::error::CoreError::InvalidMonth(month))?;

        let date_min = format!("{}-{:02}-{:02}", year, month, day_min);
        let date_max = format!("{}-{:02}-{:02}", year, month, day_max);

        self.base_request()?
            .query("start_date", date_min)
            .query("end_date", date_max)
            .get_json()
            .await
    }
}

#[derive(serde::Deserialize)]
pub struct NasaApod {
    pub date: String,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub explanation: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub hdurl: Option<String>,
    #[serde(default)]
    pub thumbnail_url: Option<String>,
    #[serde(default)]
    pub media_type: Option<String>,
    #[serde(default)]
    pub copyright: Option<String>,
}
