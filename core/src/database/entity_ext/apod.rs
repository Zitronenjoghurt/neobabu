use crate::database::entity::apod;
use chrono::NaiveDate;

impl apod::Model {
    pub fn site_url(&self) -> String {
        let year = self.year.to_string().drain(2..).collect::<String>();
        let month = format!("{:02}", self.month);
        let day = format!("{:02}", self.day);
        format!("https://apod.nasa.gov/apod/ap{year}{month}{day}.html")
    }

    pub fn date_string(&self) -> String {
        format!("{}-{:02}-{:02}", self.year, self.month, self.day)
    }

    pub fn date_time(&self) -> Option<NaiveDate> {
        NaiveDate::parse_from_str(&self.date_string(), "%Y-%m-%d").ok()
    }
}
