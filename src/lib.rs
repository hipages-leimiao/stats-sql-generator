use anyhow::Result;
use clap::{command, Parser, Subcommand};
mod date;
pub mod file;
pub mod process;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli<T: Subcommand> {
    #[clap(subcommand)]
    pub action: T,
}

pub trait Processor {
    type Item;
    fn run(&self) -> Result<()>;

    fn load_data(&self) -> Result<Vec<Self::Item>>
    where
        Self: Sized;
    fn generate_sql(&self, data: &Vec<Self::Item>) -> Result<String>
    where
        Self: Sized;
    fn write_data(&self, sql: &str) -> Result<()>;
}

pub trait CliArgs
where
    Self: Sized,
{
    fn parse_args(&mut self) -> Result<Self>;
    fn parse_args_interactively(&mut self) -> Result<Self>;
}
