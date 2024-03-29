use crate::{date::PastDateRangeType, file::get_file_full_path};
use chrono::Local;
use clap::{Parser, Subcommand, ValueEnum};
use lazy_static::lazy_static;
use std::{
    default::Default,
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
#[derive(ValueEnum, Clone, Copy, Debug, Default)]
pub enum StatType {
    Weekly,
    #[default]
    Monthly,
    Quarterly,
}

impl StatType {
    pub fn as_str(&self) -> &'static str {
        match self {
            StatType::Weekly => "weekly",
            StatType::Monthly => "monthly",
            StatType::Quarterly => "quarterly",
        }
    }
}

impl fmt::Display for StatType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
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

    #[clap(short, long, value_enum, default_value = "monthly")]
    pub s_type: StatType,

    #[clap(short, long, value_parser, default_value=get_default_key())]
    pub key: String,

    #[clap(
        short,
        long,
        value_parser,
        default_value = get_default_file_name()
    )]
    pub migration_file_name: String,
    #[clap(short, long, value_parser, default_value = "false")]
    pub do_filter: bool,

    #[clap(short, long, value_parser, default_value = "false")]
    pub raise_pr: bool,
}
lazy_static! {
    static ref DEFAULT_FILE_NAME: OsString = OsString::from(get_default_migration_name());
}

fn get_default_file_name() -> &'static OsStr {
    DEFAULT_FILE_NAME.as_os_str()
}
fn get_default_key() -> &'static OsStr {
    Box::leak(Box::new(OsString::from(get_stat_key(&StatType::Monthly)))).as_os_str()
}

pub fn get_default_migration_name() -> String {
    let date = Local::now().format("%m%d").to_string();
    format!("SeedProfileStatsBatch{}", date)
}

pub fn get_stat_key(s_type: &StatType) -> String {
    match s_type {
        StatType::Weekly => PastDateRangeType::Week.to_string(),
        StatType::Monthly => PastDateRangeType::Month.to_string(),
        StatType::Quarterly => PastDateRangeType::ThreeMonth.to_string(),
    }
}
