use clap::{Parser, Subcommand, ValueEnum};
use std::{
    ffi::{OsStr, OsString},
    fmt,
    path::PathBuf,
};

use crate::{get_default_date_range, get_file_full_path, get_stat_key};
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub action: Action,
}

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

#[derive(Parser, Debug)]
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
