use clap::{Parser, Subcommand};
use std::path::PathBuf;
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
    #[clap(short, long, value_parser)]
    pub file: PathBuf,

    #[clap(short, long, value_parser)]
    pub parsed: bool,
    
    #[clap(short, long, value_parser)]
    pub key: String,

    #[clap(short, long, value_parser)]
    pub migration_file_name: String,
}
