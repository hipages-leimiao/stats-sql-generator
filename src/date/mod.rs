mod past_date;
use chrono::{Datelike, NaiveDate};
use std::fmt;

pub enum PastDateRangeType {
    Week,
    Month,
    ThreeMonth,
}
impl PastDateRangeType {
    pub fn format((start, end): (NaiveDate, NaiveDate)) -> String {
        if start.year() == end.year() {
            format!("{} - {}", start.format("%-d %b"), end.format("%-d %b %Y"))
        } else {
            format!(
                "{} - {}",
                start.format("%-d %b %Y"),
                end.format("%-d %b %Y")
            )
        }
    }
}
impl fmt::Display for PastDateRangeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Week => write!(f, "{}", Self::format(past_date::prev_week())),
            Self::Month => write!(f, "{}", Self::format(past_date::prev_month())),
            Self::ThreeMonth => {
                write!(f, "{}", Self::format(past_date::prev_three_month()))
            }
        }
    }
}
