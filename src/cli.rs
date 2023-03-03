use clap::{Parser, Subcommand};
use std::{
    ffi::{OsStr, OsString},
    path::PathBuf,
};

use crate::{get_default_date_range, get_file_full_path};
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

#[derive(Parser, Debug)]
pub struct RunArgs {
    #[clap(short, long, value_parser(get_file_full_path))]
    pub file: PathBuf,

    #[clap(short, long, value_parser, default_value = "true")]
    pub parsed: bool,

    #[clap(short, long, value_parser, default_value=get_default_key())]
    pub key: String,

    #[clap(short, long, value_parser, default_value = "SeedProfileStatsBatch")]
    pub migration_file_name: String,
}

fn get_default_key() -> &'static OsStr {
    Box::leak(Box::new(OsString::from(get_default_date_range()))).as_os_str()
}
