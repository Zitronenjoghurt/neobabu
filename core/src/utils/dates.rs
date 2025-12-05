use chrono::{Datelike, NaiveDate, Utc};

pub fn upcoming_date(day: u32, month: u32) -> Option<NaiveDate> {
    let now = Utc::now();

    let (day, month) = if !is_leap_year(now.year()) && day == 29 && month == 2 {
        (1, 3)
    } else {
        (day, month)
    };

    let this_years_date = NaiveDate::from_ymd_opt(now.year(), month, day)?;
    if this_years_date > now.date_naive() {
        Some(this_years_date)
    } else {
        NaiveDate::from_ymd_opt(now.year() + 1, month, day)
    }
}

pub fn upcoming_date_time(day: u32, month: u32) -> Option<chrono::DateTime<Utc>> {
    upcoming_date(day, month).map(|date| date.and_hms_opt(9, 0, 0).map(|date| date.and_utc()))?
}

pub fn year_month_min_max_day(year: i32, month: u32) -> Option<(u32, u32)> {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => Some((1, 31)),
        4 | 6 | 9 | 11 => Some((1, 30)),
        2 => Some((1, if is_leap_year(year) { 29 } else { 28 })),
        _ => None,
    }
}

pub fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

pub fn iter_date_days_of_month(date: NaiveDate) -> impl Iterator<Item = NaiveDate> {
    let (min_day, max_day) = year_month_min_max_day(date.year(), date.month()).unwrap();
    (min_day..=max_day).filter_map(move |day| date.with_day(day))
}

pub fn iter_dates(start: NaiveDate, end: NaiveDate) -> impl Iterator<Item = NaiveDate> {
    let days = (end - start).num_days() + 1;
    (0..days).map(move |i| start + chrono::Duration::days(i))
}
