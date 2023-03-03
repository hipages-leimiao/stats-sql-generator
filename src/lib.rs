use anyhow::Result;
use chrono::{Datelike, Local};
use dirs;
use std::path::PathBuf;
pub mod cli;
pub mod file;

pub fn get_file_full_path(path: &str) -> Result<PathBuf> {
    let mut full_path = PathBuf::new();
    if path.starts_with("~/") {
        let home_dir = dirs::home_dir().unwrap();
        full_path.push(home_dir);
        full_path.push(path.trim_start_matches("~/"));
    } else {
        full_path.push(path);
    }
    Ok(full_path)
}

fn get_last_day_of_prev_month() -> String {
    let last_day = Local::now()
        .date_naive()
        .with_day(1)
        .unwrap()
        .pred_opt()
        .unwrap();
    last_day.format("%e %B %Y").to_string()
}

pub fn get_default_date_range() -> String {
    format!("1 September 2022 - {}", get_last_day_of_prev_month())
}
