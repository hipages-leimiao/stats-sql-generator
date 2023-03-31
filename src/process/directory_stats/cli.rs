use crate::{date::DateRangeType, file::get_file_full_path};
use chrono::{Datelike, Local};
use clap::{Parser, Subcommand, ValueEnum};
use std::{
    ffi::{OsStr, OsString},
    fmt,
    path::PathBuf,
};

pub use super::args;

#[derive(Subcommand, Debug)]
pub enum Action {
    Run(RunArgs),
    Parse,
}

#[derive(ValueEnum, Clone, Copy, Debug)]
pub enum StatType {
    Default,
    Weekly,
    Monthly,
    Quarterly,
}

impl Default for StatType {
    fn default() -> Self {
        StatType::Default
    }
}

impl StatType {
    pub fn as_str(&self) -> &'static str {
        match self {
            StatType::Default => "default",
            StatType::Weekly => "weekly",
            StatType::Monthly => "monthly",
            StatType::Quarterly => "quarterly",
        }
    }
}

impl fmt::Display for StatType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StatType::Default => write!(f, "default: {}", get_stat_key(&Self::Default)),
            StatType::Quarterly => write!(f, "quarterly: {}", get_stat_key(&Self::Quarterly)),
            StatType::Monthly => write!(f, "monthly: {}", get_stat_key(&Self::Monthly)),
            StatType::Weekly => write!(f, "weekly:  {}", get_stat_key(&Self::Weekly)),
        }
    }
}

#[derive(Parser, Debug, Clone, Default)]
pub struct RunArgs {
    #[clap(short, long, value_parser(get_file_full_path))]
    pub file: PathBuf,

    #[clap(short, long, value_enum, default_value = "default")]
    pub s_type: StatType,

    #[clap(short, long, value_parser, default_value=get_default_key())]
    pub key: String,

    #[clap(short, long, value_parser, default_value = "SeedProfileStatsBatch")]
    pub migration_file_name: String,

    #[clap(short, long, value_parser, default_value = "false")]
    pub raise_pr: bool,
}

fn get_default_key() -> &'static OsStr {
    Box::leak(Box::new(OsString::from(get_default_date_range()))).as_os_str()
}

pub fn get_default_date_range() -> String {
    let last_day = Local::now()
        .date_naive()
        .with_day(1)
        .unwrap()
        .pred_opt()
        .unwrap();

    format!(
        "1 September 2022 - {}",
        last_day.format("%-d %B %Y").to_string()
    )
}

pub fn get_stat_key(s_type: &StatType) -> String {
    match s_type {
        StatType::Default => get_default_date_range(),
        StatType::Weekly => DateRangeType::PrevWeek.to_string(),
        StatType::Monthly => DateRangeType::PrevMonth.to_string(),
        StatType::Quarterly => DateRangeType::PrevThreeMonth.to_string(),
    }
}
