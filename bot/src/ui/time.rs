use std::ops::Add;

pub fn format_time_relative_in(duration: std::time::Duration) -> String {
    let timestamp = chrono::Utc::now().add(duration).timestamp();
    format!("<t:{}:R>", timestamp)
}

pub fn format_time_relative_at(date_time: chrono::DateTime<chrono::Utc>) -> String {
    format!("<t:{}:R>", date_time.timestamp())
}
