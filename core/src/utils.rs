use chrono::{Datelike, Utc};

pub fn upcoming_date(day: u32, month: u32) -> Option<chrono::NaiveDate> {
    let now = chrono::Utc::now();

    let (day, month) = if !is_leap_year(now.year()) && day == 29 && month == 2 {
        (1, 3)
    } else {
        (day, month)
    };

    let this_years_date = chrono::NaiveDate::from_ymd_opt(now.year(), month, day)?;
    if this_years_date > now.date_naive() {
        Some(this_years_date)
    } else {
        chrono::NaiveDate::from_ymd_opt(now.year() + 1, month, day)
    }
}

pub fn upcoming_date_time(day: u32, month: u32) -> Option<chrono::DateTime<Utc>> {
    upcoming_date(day, month).map(|date| date.and_hms_opt(0, 0, 0).map(|date| date.and_utc()))?
}

pub fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}
