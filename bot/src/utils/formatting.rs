use chrono_tz::Tz;

pub fn format_bool(value: bool) -> String {
    if value {
        "Yes".to_string()
    } else {
        "No".to_string()
    }
}

pub fn humane_datetime(datetime: chrono::DateTime<Tz>) -> String {
    let date = datetime.format("%A, %B %-d, %Y").to_string();
    format!("**`{date}`** at {})", humane_time(datetime))
}

pub fn humane_time(datetime: chrono::DateTime<Tz>) -> String {
    let local_time_12h = datetime.format("%-I:%M %p").to_string();
    let local_time_24h = datetime.format("%H:%M").to_string();
    format!("**`{local_time_12h}`** (`{local_time_24h}`)")
}
