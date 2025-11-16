use std::ops::Add;

pub fn format_time_relative_in(duration: std::time::Duration) -> String {
    let timestamp = chrono::Utc::now().add(duration).timestamp();
    format!("<t:{}:R>", timestamp)
}
