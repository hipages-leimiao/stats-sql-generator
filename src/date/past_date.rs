use chrono::{Datelike, Duration, Local, NaiveDate};

// Get the date range for the previous week
pub fn prev_week() -> (NaiveDate, NaiveDate) {
    let today = Local::now().date_naive();
    let start_of_week =
        today - Duration::days(today.weekday().num_days_from_monday() as i64) - Duration::days(7);
    let end_of_week = start_of_week + Duration::days(6);
    (start_of_week, end_of_week)
}

// Get the date range for the previous month
pub fn prev_month() -> (NaiveDate, NaiveDate) {
    let today = Local::now().date_naive();
    let prev_month = today.with_day(1).unwrap() - Duration::days(1);
    let start_of_pre_month = prev_month.with_day(1).unwrap();
    (start_of_pre_month, prev_month)
}

// Get the date range for the previous quarter
pub fn prev_three_month() -> (NaiveDate, NaiveDate) {
    let (start_of_prev_month, prev_month) = prev_month();

    // Subtract three months
    let mut month = start_of_prev_month.month() as i32 - 2;
    let mut year = start_of_prev_month.year();
    if month < 1 {
        month += 12;
        year -= 1;
    }
    let start_of_prev_three_month = NaiveDate::from_ymd_opt(year, month as u32, 1).unwrap();
    (start_of_prev_three_month, prev_month)
}
#[allow(dead_code)]
pub fn prev_quarter() -> (NaiveDate, NaiveDate) {
    let today = Local::now().date_naive();
    let month = today.month();

    let start_month = match month {
        1..=3 => 10,
        4..=6 => 1,
        7..=9 => 4,
        10..=12 => 7,
        _ => unreachable!(),
    };

    let start = NaiveDate::from_ymd_opt(today.year(), start_month, 1).unwrap();
    let end = match start_month {
        10 => NaiveDate::from_ymd_opt(today.year(), 12, 31).unwrap(),
        1 => NaiveDate::from_ymd_opt(today.year(), 3, 31).unwrap(),
        4 => NaiveDate::from_ymd_opt(today.year(), 6, 30).unwrap(),
        7 => NaiveDate::from_ymd_opt(today.year(), 9, 30).unwrap(),
        _ => unreachable!(),
    };

    (start, end)
}
