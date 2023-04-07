mod date;
use chrono::{Datelike, NaiveDate};
use std::fmt;

pub enum DateRangeType {
    PrevWeek,
    PrevMonth,
    PrevThreeMonth,
}
impl DateRangeType {
    pub fn format((start, end): (NaiveDate, NaiveDate)) -> String {
        if start.year() == end.year() {
            return format!("{} - {}", start.format("%-d %b"), end.format("%-d %b %Y"));
        } else {
            return format!(
                "{} - {}",
                start.format("%-d %b %Y"),
                end.format("%-d %b %Y")
            );
        }
    }
}
impl fmt::Display for DateRangeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PrevWeek => write!(f, "{}", Self::format(date::prev_week())),
            Self::PrevMonth => write!(f, "{}", Self::format(date::prev_month())),
            Self::PrevThreeMonth => {
                write!(f, "{}", Self::format(date::prev_three_month()))
            }
        }
    }
}
